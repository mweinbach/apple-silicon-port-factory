#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b6c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14162b9f0; const P2: u64 = 0x14162b8e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x58u64)))).to_bits() as u128); rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x64u64))) as u32 as u64; r14 = r8; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x68u64)))).to_bits() as u128); xmm2 = (xmm1) as u128; rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm2 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80000000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm0 = (((rcx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm2 = crate::simd_ops::vunpcklps(xmm2, xmm2); rax = (crate::simd_ops::vmovmskps(xmm2) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm2 = (((rcx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vcvtdq2ps(xmm2); bb = 3; continue; }
            3 => { r15 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; r13 = 0x80; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((xmm1 as u32)) as u32); rax = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x64u64), ((rax as u32)) as u32); r12 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x6cu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x64u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 25; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x6cu64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), (0xff00u16) as u16); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x44u64))) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xf0u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x46u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x45u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x24u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rcx as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x25u64), ((rdx as u8)) as u8); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r13 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); bb = 13; continue; }
            13 => { rdx = 0x20; rsi = rbx.wrapping_add(0x20); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); xmm0 = 0u128; crate::simd_ops::wu64(rax.wrapping_add(0xcu64), r12); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r12); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rbp as u32))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x54u64)))).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x5cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(r14, rdx); bb = 18; continue; }
            16 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdx); crate::simd_ops::wu64(r14, rdx); bb = 18; continue; }
            17 => { rsi = rbx.wrapping_add(0x20); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x6cu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = 0x1; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x5cu64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x30; const P3: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    r8 = rdx;
    r8 = ((r8).wrapping_sub(rax)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); }
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rbx = rcx.wrapping_add(0x24); bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_sub(0x1cu64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x6u64), (0x0u8) as u8); crate::simd_ops::wu64(rbx.wrapping_sub(0xcu64), r9); crate::simd_ops::wu32(rbx.wrapping_sub(0x4u64), (0xffffffffu32) as u32); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 3; continue; }
            3 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rdx = (((rdx as u32)).wrapping_shl(7u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0x4u64), ((rdx as u32)) as u32); rax = ((rdx as u32)) as u32 as u64; rcx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rdx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_sub(0xcu64), r9); rdx = ((((rdx as u32)) & (0x7fu32))) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_sub(0x4u64), ((rdx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 6; continue; }
            6 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 8; continue; }
            8 => { fl_l = r9; fl_r = (crate::simd_ops::ru64(rbx.wrapping_sub(0x14u64))) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; rdx = r9; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x4u64), ((rax as u16)) as u16); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { r9 = rbx; rcx = (rcx & !0xffu64) | (((0x6u8) as u8 as u64)); fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x6u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); r9 = rbx.wrapping_add(0x1); rcx = (rcx & !0xffu64) | (((0x5u8) as u8 as u64)); bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_sub(0x14u64))) as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (((r8 as u8)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rcx = (rcx & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); bb = 15; continue; }
            15 => { r8 = (((rcx as u8)) as u32) as u32 as u64; rcx = r9; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 17; continue; }
            17 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            18 => { crate::simd_ops::wu32(rbx.wrapping_sub(0x4u64), (0xffffffffu32) as u32); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162b9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162b540; const P1: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r11 = rcx; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = rcx.wrapping_add(0x24); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; r10 = (((rbp as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); r10 = (r10 & !0xffu64) | ((((((r10 as u8)) & (0xf0u8))) as u8 as u64)); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x25u64))) as u8 as u64; fl_r = (0x80u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (0x0u64) as u64); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x4u64), ((rax as u16)) as u16); crate::simd_ops::wu32(r11.wrapping_add(0x20u64), (0xffffffffu32) as u32); crate::simd_ops::wu8(r11.wrapping_add(0x2au64), ((rax as u8)) as u8); bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = ((r10.wrapping_add(0x40u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xefu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0x3; bb = 23; continue; }
            11 => { rcx = 0x2; bb = 23; continue; }
            12 => { rax = ((rbp.wrapping_add(0x10u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 7; continue; } bb = 13; continue; }
            13 => { rcx = 0x8081; fl_l = ((((rcx as u32)) >> ((((rax as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) == 0) { bb = 7; continue; } bb = 14; continue; }
            14 => { rax = r11.wrapping_add(0x25); rsi = 0; fl_l = 0; fl_r = 0; r9 = 0; fl_l = 0; fl_r = 0; r8 = rax.wrapping_add(0x1); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; r8 = (if (((fl_l as u8)) != ((fl_r as u8))) { rax } else { r8 }) as u64; bb = 15; continue; }
            15 => { rdx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fu32))) as u32 as u64; rsi = ((rcx as u32)) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rsi = ((((rsi as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 17; continue; }
            17 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 18; continue; }
            18 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 19; continue; }
            19 => { rax = ((rsi as u32)) as u32 as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 20; continue; }
            20 => { rcx = (((rcx as u32)).wrapping_sub((r11 as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xffffffdcu32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 21; continue; }
            21 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru8(r14)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(7u32)) as u8 as u64)); rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; bb = 25; continue; }
            25 => { rcx = ((rcx).wrapping_add(r14)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rcx); fl_l = rcx; fl_r = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 26; continue; }
            26 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0xf0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { crate::simd_ops::wu8(r11.wrapping_add(0x2au64), ((rbp as u8)) as u8); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (0x0u64) as u64); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x4u64), ((rax as u16)) as u16); crate::simd_ops::wu8(r11.wrapping_add(0x2au64), ((rax as u8)) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(r11.wrapping_add(0x20u64), (0xffffffffu32) as u32); bb = 8; continue; }
            32 => { rdx = r11.wrapping_add(0x20); rcx = r11.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(r11.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 34; continue; }
            34 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x4u64), ((rax as u16)) as u16); rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 35; continue; }
            35 => { rcx = (rcx & !0xffu64) | (((0x6u8) as u8 as u64)); fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 38; continue; } bb = 36; continue; }
            36 => { rax = ((crate::simd_ops::ru8(r11.wrapping_add(0x2au64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); rcx = (rcx & !0xffu64) | (((0x5u8) as u8 as u64)); fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 38; continue; }
            38 => { r8 = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = (((r8 as u8)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rcx = (rcx & !0xffu64) | ((((fl_l as u8).wrapping_sub(fl_r as u8)) as u8 as u64)); bb = 40; continue; }
            40 => { r8 = (((rcx as u8)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 41; continue; }
            41 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162bc20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162b8e0; const P1: u64 = 0x40; const P2: u64 = 0x5c; const P3: u64 = 0x64; const P4: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = 0x1;
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rcx);
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162bc50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162b8e0; const P1: u64 = 0x14162b9f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x6cu64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm1) as u128; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rdx = 0x1; rcx = ((rcx).wrapping_add(0x20u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x58u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x5cu64), (0x0u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rcx as u32)) as u32); xmm1 = (xmm6) as u128; rdx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm1 as u32)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80000000; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { xmm0 = (((rdx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm1 = crate::simd_ops::vunpcklps(xmm1, xmm1); rax = (crate::simd_ops::vmovmskps(xmm1) as u32) as u32 as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm1 = (((rdx as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x64u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((xmm6 as u32)) as u32); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x6cu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x64u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 11; continue; }
            11 => { rcx = rbx.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x6cu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = 0x1; rcx = rbx.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x5cu64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; bb = 5; continue; }
            15 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x2; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162bd60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690894; const P1: u64 = 0x142690890; const P2: u64 = 0x14162b8e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x6cu64))) as u32 as u64; rbx = rcx; rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x6cu64), ((rax as u32)) as u32); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x6cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x14u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((rdi as u32)) as u32); r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1u64))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((r8 as u32)) as u32); if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x4u64))) as u32) as u32 as u64; xmm1 = 0u128; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x5u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x3u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); rax = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x20u64), ((rdi as u8)) as u8); xmm0 = (((rcx as u32)) as u32) as u128; rcx = rbx.wrapping_add(0x20); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = rdx.wrapping_add(0x6); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); rax = ((rdx.wrapping_add(r9)) as u64) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x2au64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::lk_or_u32(rbx.wrapping_add(0x6cu64), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rax); xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x54u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x58u64), ((xmm1 as u32)) as u32); bb = 5; continue; }
            4 => { fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (((rax as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffdu32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x5cu64), ((rdi as u32)) as u32); rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x64u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x6cu64), ((rdx as u32)) as u32); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdi); rax = 0x2; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rdi as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162beb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162b8e0; const P1: u64 = 0x40; const P2: u64 = 0x5c; const P3: u64 = 0x60; const P4: u64 = 0x64; const P5: u64 = 0x6c; const P6: u64 = 0x38; const P7: u64 = 0x20; const P8: u64 = 0x14162b9f0; const P9: u64 = 0x4; const P10: u64 = 0x8; const P11: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rbx = rcx; rdx = 0x1; rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P5))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0x1; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::lk_add_u32(rbx.wrapping_add(P3), ((rax as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P2), (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P9))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P11))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162bf60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = (((rdx as u8)) as u32) as u32 as u64;
    rax = ((((rax as u32)) & (0xfffffffbu32))) as u32 as u64;
    r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64;
    fl_l = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_r = fl_l;
    r8 = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162bf80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x1c; const P2: u64 = 0x142690894; const P3: u64 = 0x142690890; const P4: u64 = 0x54; const P5: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64;
    if !((f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan())) { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm1 as u32)) as u32); }
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    xmm1 = 0u128;
    xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P0)))).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((xmm1 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160f710; const P1: u64 = 0x1415c8c30; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = rax;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c0b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((rax as u8)) as u8);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415adef0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = rcx; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415c9a50; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x1401de410; const P5: u64 = 0x10; const P6: u64 = 0x1426894c0; const P7: u64 = 0x20; const P8: u64 = 0x220;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); rsi = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm8) as u128); xmm7 = (xmm2) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((xmm7 as u32)) as u32); xmm6 = (xmm0) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (xmm0) as u128; xmm8 = (xmm0) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rdi.wrapping_add(P2), (0x0u32) as u32); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(P5), (0x0u64) as u64); crate::simd_ops::wu8(rdi.wrapping_add(P3), (0x0u8) as u8); bb = 4; continue; }
            4 => { xmm0 = 0u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rdx = P6; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; r8 = rsp.wrapping_add(0x30); xmm3 = (xmm8) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm7 as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x34u64), (0x5u16) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32);
    rbx = rcx;
    rdx = ((rdx as u32)) as u32 as u64;
    rcx = 0x5;
    rdx = ((rdx).wrapping_shl(4u32)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = 0x34; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = r9; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = ((r8 as u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; r14 = rdx; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0x8u64)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rbp = ((r8.wrapping_add(0x5u64)) as u32) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = 0x5; r8 = ((rbp as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0x34; bb = 14; continue; }
            3 => { crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; xmm1 = 0u128; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; xmm0 = 0u128; r8 = 0; fl_l = 0; fl_r = 0; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = ((rdi.wrapping_sub(0x4u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; r10 = r14.wrapping_add(0x10); fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; r8 = ((rax.wrapping_mul(4)) as u32) as u32 as u64; rdx = ((rax.wrapping_mul(4)) as u64) as u64; bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(r10.wrapping_sub(0x10u64))) as u128; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r10 = r10.wrapping_add(0x40); rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x4cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsi))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); xmm0 = (crate::simd_ops::ru128(r10.wrapping_sub(0x40u64))) as u128; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x3cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsi))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); xmm0 = (crate::simd_ops::ru128(r10.wrapping_sub(0x30u64))) as u128; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x2cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsi))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); xmm0 = (crate::simd_ops::ru128(r10.wrapping_sub(0x20u64))) as u128; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = (crate::simd_ops::ru32(r10.wrapping_sub(0x1cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsi))) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 13; continue; }
            9 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdx = r8; bb = 11; continue; }
            11 => { rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r14)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r8 = ((rdi as u32)) as u32 as u64; bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = rdx.wrapping_add(0x10); rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0xcu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsi))) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x18u64), (0x0u8) as u8); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rax = 0x1; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = 0; fl_l = 0; fl_r = 0; rdi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 3; continue; }
            3 => { rsi = ((rbx as u32)) as u32 as u64; rsi = ((rsi).wrapping_add(rsi)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rax; rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(P0), rbp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; bb = 5; continue; }
            5 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 7; continue; }
            7 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rbp); crate::simd_ops::wu32(rdi, ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { crate::simd_ops::wu32(rcx, ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r11 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdi = ((rdx as u32)) as u32 as u64; r10 = ((r8 as u32)) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((r9 as u32)) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            7 => { rax = r8; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x270; const P3: u64 = 0x143930f90; const P4: u64 = 0x14159ab70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = ((r9 as u32)) as u32 as u64; r14 = ((r8 as u32)) as u32 as u64; rsi = rdx; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; bb = 7; continue; }
            7 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162c240;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = r9; r15 = r8; r13 = rdx; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { r11 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = ((r10 as u32)) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rdx = r14.wrapping_add(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((r9.wrapping_add(rsi)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; fl_l = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x12u64))) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; rdx = ((r10 as u32)) as u32 as u64; rcx = r13; rdx = ((rdx).wrapping_shl(4u32)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32); rdx = ((rdx).wrapping_add(r14)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); r9 = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x270u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rbp = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((rax as u32)) as u32); bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x141524330; const P2: u64 = 0x14183acbc; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((r9 as u32)) as u32 as u64; r14 = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x5; rsi = r8; rdx = ((rbx.wrapping_add(P0)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((rbx as u32)) as u32 as u64; rdx = rsi; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); rdx = ((rbp as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rax = 0x1; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P3), rdi); bb = 3; continue; }
            2 => { rax = 0x34; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r11 = (crate::simd_ops::ru32(rcx)) as u32 as u64; rdi = rcx; fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((r11 as u32)) as u32 as u64; fl_r = 0x1; r11 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; r10 = ((r11 as u32)) as u32 as u64; rax = 0x10; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; r8 = ((rdx.wrapping_add(rcx)) as u64) as u64; r9 = ((rax.wrapping_add(rcx)) as u64) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx).wrapping_add(P1))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(r9)) as u128; rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); xmm1 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); crate::simd_ops::wu128(r9, (xmm1) as u128); bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x10u64)) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { fl_l = (((r11 as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; r11 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 11; continue; }
            11 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x18; const P2: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rdi = r8; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r11 = ((rbp as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; r10 = ((rbp as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rsi).wrapping_add(P2))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((r9.wrapping_add(rbx)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rax.wrapping_add(rsi).wrapping_add(P2), ((rbp as u32)) as u32); fl_l = (((r11 as u32)).wrapping_add(1)) as u32 as u64; r11 = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::lk_add_u32(rdx.wrapping_add(P1), ((r11 as u32)) as u32); crate::simd_ops::lk_sub_u32(rdx.wrapping_add(P0), ((r11 as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(P0), ((r11 as u32)) as u32)) as u32 as u64; fl_r = fl_l; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            7 => { rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c8a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415cf9e0; const P1: u64 = 0x142690910; const P2: u64 = 0x44; const P3: u64 = 0x142690a18; const P4: u64 = 0x50; const P5: u64 = 0x28; const P6: u64 = 0x142690ac0; const P7: u64 = 0x30; const P8: u64 = 0x142690b18; const P9: u64 = 0x38; const P10: u64 = 0x1426908e8; const P11: u64 = 0x60; const P12: u64 = 0x142689070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = P1; crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(P2), (0xffu32) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rax); rax = P3; crate::simd_ops::wu64(rbx.wrapping_add(P4), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); rax = P8; crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); rax = P10; crate::simd_ops::wu64(rbx.wrapping_add(P11), rax); rax = P12; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rcx); crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x80u64), ((rcx as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x88u64), (0xffu8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x90u64), (0xffu8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xd0u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), rcx); crate::simd_ops::wu32(rbx.wrapping_add(0xe8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c994(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162c9b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0x30u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162c9b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0x38u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162c9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690910; const P1: u64 = 0xf8; const P2: u64 = 0x142690a18; const P3: u64 = 0x28; const P4: u64 = 0x142690ac0; const P5: u64 = 0x30; const P6: u64 = 0x142690b18; const P7: u64 = 0x38; const P8: u64 = 0x1415242e0; const P9: u64 = 0x60; const P10: u64 = 0x20; const P11: u64 = 0x14158fa60; const P12: u64 = 0x141646970; const P13: u64 = 0x142689028; const P14: u64 = 0x14268e6d0; const P15: u64 = 0x14268dac0; const P16: u64 = 0x14268b678; const P17: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rsi = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rcx; crate::simd_ops::wu64(rcx, rax); rax = P2; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); rax = P4; crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); rax = P6; crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), (0x0u64) as u64); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = P13; crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rax); rax = P14; crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rax); rax = P15; crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); rax = P16; crate::simd_ops::wu64(rdi, rax); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0x100; rcx = rdi; rax = crate::simd_ops::call_stk(P17, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158e990;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = 0x40001;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x305; const P2: u64 = 0x2a0; const P3: u64 = 0xf0; const P4: u64 = 0x38; const P5: u64 = 0x2fc; const P6: u64 = 0x143426494; const P7: u64 = 0x1438152ac; const P8: u64 = 0x306; const P9: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rdi) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(P5)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); rdx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(P7)) as u32 as u64; rax = (((rax) as i64).wrapping_mul((rdi) as i64) as u64) as u64; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rdx = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u8(r8.wrapping_add(P1), (0x3fu8) as u8); crate::simd_ops::lk_and_u8(r8.wrapping_add(P8), (0xfeu8) as u8); crate::simd_ops::wu32(r8.wrapping_add(P5), (0x0u32) as u32); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x40; const P2: u64 = 0x142690b38; const P3: u64 = 0x1434268d0; const P4: u64 = 0x14267648c; const P5: u64 = 0x143426e1c; const P6: u64 = 0x143426eb8; const P7: u64 = 0x14267664c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(23u32)) as u32 as u64; xmm0 = (((rcx as u32)) as u32) as u128; xmm2 = (xmm0) as u128; rcx = (((rax as u8)) as u32) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm3 = (xmm0) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm3) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = 0u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cc60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524320;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; fl_r = (0x0u64) as u64; rdi = rdx; rsi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru16(rdx.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), (0x11u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xf8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x44u64))) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x48u64))) as u32 as u64; r8 = ((rax.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(6u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x3ffu32))) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((rcx as u32)) as i32) as u32) as u32 as u64; rcx = ((rax.wrapping_add(0x5u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0xf8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), (0x2u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::wu64(rdi, rax); rdx = rdi; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rbx as u32)) as u32); crate::simd_ops::wu16(rdi.wrapping_add(0x12u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), (0x2bu32) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), (0x2bu32) as u32); crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu16(rdx.wrapping_add(0x12u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rdx.wrapping_add(0x10u64), ((rax as u16)) as u16); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), (0x1u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), (0x2u32) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cd70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f90; const P1: u64 = 0xfffffffffffffff0; const P2: u64 = 0x270; const P3: u64 = 0x14159a3d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rcx;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ce10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe8; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x3a;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P3))) as u32) as u32 as u64;
    fl_l = ((((rax as u8)) & (0x7u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ce40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe8; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x78;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ce70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe8; const P1: u64 = 0x20; const P2: u64 = 0x141605350; const P3: u64 = 0x18; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = r8; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = rax; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (if (((fl_l as u32)) >= ((fl_r as u32))) { crate::simd_ops::ru32(rbx) } else { (rcx as u32) }) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; rdx = ((r8.wrapping_mul(8)) as u64) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rdi), rcx); rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P3))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff0; const P1: u64 = 0x348; const P2: u64 = 0x350; const P3: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64) as u128;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cf20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe8; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = ((crate::simd_ops::ru16(rax.wrapping_add(P3))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cf40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff0; const P1: u64 = 0x300;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru16(rax.wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cf50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff0; const P1: u64 = 0x298; const P2: u64 = 0x38; const P3: u64 = 0x194; const P4: u64 = 0x1; const P5: u64 = 0x20; const P6: u64 = 0x1416454a0; const P7: u64 = 0x14162ee70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    r8 = 0;
    fl_l = 0; fl_r = 0;
    loop { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; fl_r = 0x0; rdx = ((r8.wrapping_add(P4)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; rdx = (if (((fl_l as u32)) != ((fl_r as u32))) { (r8 as u32) } else { (rdx as u32) }) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    rax = 0x2;
    fl_l = (crate::simd_ops::lk_and_u32(rdx, (0xffu32) as u32)) as u32 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162cfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x141646b20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; r13 = rcx.wrapping_add(0x28); rax = (crate::simd_ops::ru64(r13)) as u64; r15 = r9; r12 = r8; r14 = ((rdx as u32)) as u32 as u64; r11 = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; r10 = ((rax.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); fl_l = rax; fl_r = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rdx = rax.wrapping_add(0x8); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_sub(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x290u64))) as u64; rbp = (crate::simd_ops::ru32(rcx)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rsi = rbx.wrapping_add(0x378); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x3b8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 9; continue; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x3b0u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; bb = 14; continue; }
            14 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 20; continue; }
            16 => { rsi = rbx.wrapping_add(0x378); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 8; continue; } bb = 17; continue; }
            17 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            18 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            19 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 22; continue; } bb = 24; continue; }
            24 => { bb = 29; continue; }
            25 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3a0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x3a8u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            28 => { rsi = rbx.wrapping_add(0x378); bb = 29; continue; }
            29 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 31; continue; }
            31 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(r14.wrapping_mul(4)))) as u32 as u64; bb = 34; continue; }
            33 => { rbx = ((rbp as u32)) as u32 as u64; bb = 34; continue; }
            34 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            36 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { r8 = ((rbx as u32)) as u32 as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = r13; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(r15, ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r12, rax); bb = 40; continue; }
            39 => { crate::simd_ops::wu64(r12, rbp); crate::simd_ops::wu32(r15, ((rbp as u32)) as u32); bb = 40; continue; }
            40 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffe0; const P1: u64 = 0x310;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P1)))))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    crate::simd_ops::lk_and_u32(rdx, (0xffu32) as u32);
    rax = ((xmm0 as u32)) as u32 as u64;
    crate::simd_ops::wu128(rsp, (xmm0) as u128);
    rax = (((rax as u8)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rsp, ((rax as u32)) as u32);
    rax = 0x2;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu128(r8, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x50; const P2: u64 = 0x10; const P3: u64 = 0xf0; const P4: u64 = 0x8; const P5: u64 = 0x141646c90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((r8 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rdx; rdi = rcx; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rbx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x28u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d2b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x50; const P2: u64 = 0x8; const P3: u64 = 0xf0; const P4: u64 = 0x141646c90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = rdx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdi = rcx; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rbx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = rsi; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x28u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf8; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x50; const P2: u64 = 0x58; const P3: u64 = 0x8; const P4: u64 = 0x141646950; const P5: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64;
    rdi = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { crate::simd_ops::wu32(rsi.wrapping_add(P2), (0x0u32) as u32); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    loop { rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::wu32(rsi.wrapping_add(P2), (0x0u32) as u32);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = (crate::simd_ops::ru64(rsi)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d3e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x14158e830; const P5: u64 = 0x14158f340; const P6: u64 = 0x1416087b0; const P7: u64 = 0x1438152ac; const P8: u64 = 0x143815b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r12); r14 = rcx; crate::simd_ops::wu8(rax.wrapping_sub(0x38u64), ((r12 as u8)) as u8); rbp = 0x2; crate::simd_ops::wu32(rax.wrapping_sub(0x34u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_sub(0x2au64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); crate::simd_ops::wu32(rax.wrapping_sub(0x30u64), (0x1u32) as u32); crate::simd_ops::wu16(rax.wrapping_sub(0x2cu64), (0x100u16) as u16); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x290u64))) as u64; r15 = (crate::simd_ops::ru32(rcx)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x378); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x3b8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x3b0u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdi = r12; bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            11 => { rsi = rbx.wrapping_add(0x378); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            13 => { rdi = r12; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x370u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x370u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3a0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x3a8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x378); bb = 23; continue; }
            23 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x88); crate::simd_ops::wu32(r14.wrapping_add(0xe8u64), ((rcx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r8 = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xe8u64))) as u32 as u64; rdx = (rdx & !0xffu64) | (((0x2u8) as u8 as u64)); crate::simd_ops::wu64(r14.wrapping_add(0xf0u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 33; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = r14.wrapping_add(0x60); fl_l = rax; fl_r = rax; rdx = rdi; r8 = rax.wrapping_add(0x18); r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { r8 }) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rbx = r14.wrapping_add(0x40); crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); r8 = r14.wrapping_add(0x28); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = P8; rax = ((((rax as u32)) & (0xfffd0120u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x4101u32) as u32); rax = ((((rax as u32)) | (0x50120u32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = r14; fl_r = r14; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0xf0u64))) as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(0xb0u64))) as u64; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { r8 }) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((((rcx as u32)) >> (18u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x44u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; rax = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; rax = ((((rax as u32)) & (0xffc0u32))) as u32 as u64; fl_l = ((((rax as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x48u64), ((rax as u32)) as u32); bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0xf0u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x2a0u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x2a8u64), ((rax as u32)) as u32); bb = 35; continue; }
            33 => { rbp = 0x2; bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { rax = ((rbp as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x60; const P2: u64 = 0x20;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x58; const P2: u64 = 0x1415cfd50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = 0x2d;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415d11a0; const P1: u64 = 0x142690b40; const P2: u64 = 0x90; const P3: u64 = 0x98; const P4: u64 = 0xa0;
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
    crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690b40; const P1: u64 = 0x98; const P2: u64 = 0x1415242e0; const P3: u64 = 0x90; const P4: u64 = 0x1415d11f0; const P5: u64 = 0x141834d30;
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
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64); }
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xa8; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x1415d12d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16);
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xa2; const P1: u64 = 0x88; const P2: u64 = 0x34;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r10 = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    let d = ((r10 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64);
    if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; }
    r10 = ((((r10 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64;
    crate::simd_ops::wu32(r8, ((r10 as u32)) as u32);
    rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as i32) as u32) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(P2)))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(r9, ((rax as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162d870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1d; const P1: u64 = 0x143930f68; const P2: u64 = 0x1415d0ee0; const P3: u64 = 0x1415d1520; const P4: u64 = 0x1438152b4; const P5: u64 = 0x141524320; const P6: u64 = 0x14183acbc; const P7: u64 = 0x141651ad0; const P8: u64 = 0x141651d10; const P9: u64 = 0x141524330; const P10: u64 = 0x1415d0330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x20u64))) as u32) as u32 as u64; r14 = rdx; rbx = rcx; rdi = 0x2d; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rdx = rsp.wrapping_add(0xa0); rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x68u64))) as u32 as u64; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax.wrapping_sub(0x2du64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdi = ((rax as u32)) as u32 as u64; bb = 10; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x20u64), (0xfdu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 10; continue; }
            7 => { rdi = 0x2e; bb = 10; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((((rax as u8)) & (0xfdu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x20u64), ((rax as u8)) as u8); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x400000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rdx = rsp.wrapping_add(0x90); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((rax as u32)) as u32); fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; xmm0 = 0u128; rdx = (((rdx as u32)).wrapping_add((rbp as u32))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x30u64)))).to_bits() as u128); rdx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), ((rdx as u32)) as u32); fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rbp as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0xacu64), (0x1u32) as u32); bb = 17; continue; }
            15 => { crate::simd_ops::wu32(rsp.wrapping_add(0xacu64), (0x11u32) as u32); bb = 17; continue; }
            16 => { crate::simd_ops::wu32(rsp.wrapping_add(0xacu64), (0x2u32) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r9 = rsp.wrapping_add(0xa8); rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rbx; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x270u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu32(r14.wrapping_add(0x30u64), ((rdi as u32)) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            20 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(r14.wrapping_add(0x30u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_add(0x2a0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); r13 = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); rsi = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rdi = ((crate::simd_ops::ru8(rax.wrapping_add(0x4u64))) as u32) as u32 as u64; r15 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rsi = (((rsi as u32)).wrapping_shr(6u32)) as u32 as u64; rsi = ((((rsi as u32)) & (0x3ffu32))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rsi as u32)) as u32); fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = ((r15 as u32)) as u32 as u64; rcx = ((r13.wrapping_add(0x5u64)) as u32) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; r8 = ((r13.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rax); r13 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 25; continue; }
            25 => { rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0xa2u64))) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rsi as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rax as u32)) as u32); rax = ((r8 as u32)) as u32 as u64; let d = ((rdi as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r14 = ((rax as u32)) as u32 as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xa0u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 36; continue; } bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x98u64)))) as u64; r8 = (((r8 as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0xa4u64))) as u16 as u64; fl_r = (0x2u16) as u16 as u64; rsi = ((rbp as u32)) as u32 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 31; continue; } bb = 27; continue; }
            27 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 35; continue; } bb = 28; continue; }
            28 => { rbp = r13; bb = 29; continue; }
            29 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; r8 = 0x1; rdx = rbp; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x98u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rbp = ((rbp).wrapping_add(0x2u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 29; continue; } bb = 30; continue; }
            30 => { bb = 34; continue; }
            31 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { rbp = r13; bb = 33; continue; }
            33 => { r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(0xa2u64))) as u32) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rcx = rbp; rdx = ((((rdx as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x98u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rbp = ((rbp).wrapping_add(0x2u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 33; continue; } bb = 34; continue; }
            34 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xa0u64))) as u32) as u32 as u64; r12 = rbx.wrapping_add(0x6c); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(0xa0u64), ((rbp as u16)) as u16); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x68u64), ((rcx as u32)) as u32); crate::simd_ops::lk_add_u32(r12, ((rcx as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0xa2u64))) as u32) as u32 as u64; r13 = ((r13).wrapping_add(rax)) as u64; fl_l = ((r15 as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; r15 = (r15 & !0xffffu64) | ((((fl_l as u16).wrapping_sub(fl_r as u16)) as u16 as u64)); bb = 37; continue; }
            36 => { r12 = rbx.wrapping_add(0x6c); bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx as u16)) as u32) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rsi = ((rbp as u32)) as u32 as u64; rax = (((r15 as u16)) as u32) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r15 = ((rax as u32)) as u32 as u64; r15 = (if (((fl_l as u32)) < ((fl_r as u32))) { (r8 as u32) } else { (r15 as u32) }) as u32 as u64; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0xa4u64))) as u16 as u64; fl_r = (0x2u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 42; continue; } bb = 38; continue; }
            38 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 46; continue; } bb = 39; continue; }
            39 => { rbp = r13; bb = 40; continue; }
            40 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; r8 = ((r15 as u32)) as u32 as u64; rdx = rbp; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x60u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rbp = ((rbp).wrapping_add(0x2u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 40; continue; } bb = 41; continue; }
            41 => { bb = 46; continue; }
            42 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 46; continue; } bb = 43; continue; }
            43 => { rbp = r13; bb = 44; continue; }
            44 => { r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(0xa2u64))) as u32) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rcx = rbp; rdx = ((((rdx as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; r8 = ((((r8 as u32)) as i32).wrapping_mul(((r15 as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x60u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rbp = ((rbp).wrapping_add(0x2u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 44; continue; } bb = 45; continue; }
            45 => { r12 = rbx.wrapping_add(0x6c); bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; rax = ((r15 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as i32) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x90u64)))) as u64; rax = ((rax).wrapping_add(r13)) as u64; rdx = if (rax as i64) < 0 { 0xffffffffffffffffu64 } else { 0u64 }; let d = (rcx) as i64 as i128; let n = (((rdx as u128) << 64) | (rax as u128)) as i128; if d != 0 { rax = n.wrapping_div(d) as i64 as u64; rdx = n.wrapping_rem(d) as i64 as u64; } rdx = ((r8 as u32)) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((r15 as u32)) as i32) as u32) as u32 as u64; rdi = rax; crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x68u64), ((rdx as u32)) as u32); crate::simd_ops::lk_add_u32(r12, ((rdx as u32)) as u32); rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (rcx) as u64); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 53; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 50; continue; } bb = 48; continue; }
            48 => { rdx = ((r8 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(0x5u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x30u64), (0x2u32) as u32); bb = 54; continue; }
            50 => { r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(0x68u64))) as u32) as u32 as u64; rcx = rax; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu16(rbx.wrapping_add(0xa0u64), ((r8 as u16)) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::lk_add_u32(r12, ((rcx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x82u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x88u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            52 => { fl_l = ((((rax as u8)) & (0xfdu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x82u64), ((rax as u8)) as u8); bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r8 = (((rdi as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; r9 = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            54 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162dd50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415cf650; const P1: u64 = 0x1415d1d10; const P2: u64 = 0x143426494;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbx = rcx; r8 = r11.wrapping_sub(0x28); rcx = 0; fl_l = 0; fl_r = 0; rax = rdx; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rcx as u8)) as u8); crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rcx); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; rsi = rbx.wrapping_add(0x34); crate::simd_ops::wu32(r11.wrapping_sub(0x38u64), ((rcx as u32)) as u32); r15 = rbx.wrapping_add(0x30); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rcx); r14 = rbx.wrapping_add(0x3c); rcx = r11.wrapping_sub(0x38); crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rcx); r12 = rbx.wrapping_add(0x38); crate::simd_ops::wu64(r11.wrapping_sub(0x80u64), rsi); r9 = rbx.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(0xa4u64), ((rax as u16)) as u16); fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0x8311; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x2u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; rax = ((((rax as u32)) & (0xfff80010u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(7u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xffc0u32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = ((((rcx as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x2a0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rax.wrapping_add(0x2a8u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x80); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { r9 = ((crate::simd_ops::ru16(rdi.wrapping_add(0xcu64))) as u32) as u32 as u64; rax = 0x40; crate::simd_ops::wu32(rbx.wrapping_add(0x88u64), ((r9 as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rbx.wrapping_add(0xa2u64), ((rax as u16)) as u16); r11 = (crate::simd_ops::ru32(r15)) as u32 as u64; rax = ((r11 as u32)) as u32 as u64; rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0xcu64))) as u32) as u32 as u64; r10 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rsi = rbx.wrapping_add(0x54); rbp = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r8 = ((r10.wrapping_add(r11)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rax as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rsi)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(r12)) as u32 as u64; rdx = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(6u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(6u32)) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((r9 as u32)) as i32) as u32) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((r9 as u32)) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r10 as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((r10 as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((rdx as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r12)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = 0x7; bb = 19; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((r10 as u32)) as u32); rax = ((r11 as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x7cu64), ((r8 as u32)) as u32); let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (((rax as u32)).wrapping_shl(6u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rdx = rsp.wrapping_add(0x70); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r14)) as u32 as u64; rdx = (crate::simd_ops::ru32(r12)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru16(rsi)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; xmm2 = 0u128; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0xcu64))) as u32) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xa2u64))) as u32) as u32 as u64; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm1 = (((rax as u32)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm2 as u32)) as u32); if (((fl_l as u16)) == ((fl_r as u16))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = rsp.wrapping_add(0x70); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x310u64)))))) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x7du64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            18 => { rax = 0x2; bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 20; continue; }
            20 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162dfe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x1415242e0; const P2: u64 = 0x1415d1ce0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162e020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415d1e60; const P1: u64 = 0xa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) > ((fl_r as u32)))) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rdi.wrapping_add(P1), ((rax as u16)) as u16); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162e050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xb8; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x4c; const P6: u64 = 0x50; const P7: u64 = 0x54; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x28; const P11: u64 = 0x30; const P12: u64 = 0x1416419b0; const P13: u64 = 0x143815b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rdi as u8)) as u8); rbx = rcx; crate::simd_ops::lk_and_u32(rcx.wrapping_add(P1), (0xffu32) as u32); crate::simd_ops::wu64(rcx, rdi); crate::simd_ops::wu64(rcx.wrapping_add(P2), rdi); crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P4), rdi); crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P6), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P7), (0xffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P8), rdi); crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P10), rdi); crate::simd_ops::wu64(rcx.wrapping_add(P11), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), (0x3f800000u32) as u32); crate::simd_ops::wu16(rcx.wrapping_add(0x6cu64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(0x6eu64), (0x1u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x70u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x74u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rdi); crate::simd_ops::wu32(rcx.wrapping_add(0x90u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rdi); crate::simd_ops::wu8(rcx.wrapping_add(0xa8u64), ((rdi as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0xb0u64), (0x32u8) as u8); rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = P13; crate::simd_ops::wu8(rbx.wrapping_add(0x10cu64), ((rdi as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x10cu64), (0xffu32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xf0u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0x108u64), ((rdi as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x110u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162e180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x68; const P2: u64 = 0x58; const P3: u64 = 0x10; const P4: u64 = 0x6c; const P5: u64 = 0x80; const P6: u64 = 0x88; const P7: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdi as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rdi as u32)) as u32); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rbx.wrapping_add(P6), ((rdi as u8)) as u8); }
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162e1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690d10; const P1: u64 = 0x1416382b0; const P2: u64 = 0x14162f410; const P3: u64 = 0x141651fc0; const P4: u64 = 0x141642fc0; const P5: u64 = 0x1401de410; const P6: u64 = 0x14162e180; const P7: u64 = 0x1415242e0; const P8: u64 = 0x142690c78; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rsi = rcx; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x158u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = ((rdx as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x148u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { r14 = (((rbp) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rax = ((rbp as u32)) as u32 as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; r14 = r14.wrapping_sub(0x118); rdx = (((rax) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rbp = (((rbp as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((r14.wrapping_add(r8)) as u64) as u64; rdx = ((rdx).wrapping_add(0xfffffffffffffee8u64)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x148u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x140u64)))) as u64; rcx = rdi.wrapping_sub(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_sub(0x58); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_sub(0x58); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_sub(0xf8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x148u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x148u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 7; continue; }
            7 => { r14 = rsi.wrapping_add(0x28); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x118u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x120u64))) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rbp = rbx.wrapping_add(0xc0); bb = 10; continue; }
            10 => { rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbp.wrapping_sub(0xa0); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x118u64)) as u64; rbp = ((rbp).wrapping_add(0x118u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x118u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r14.wrapping_add(0x120u64), ((rbx as u32)) as u32); fl_l = r14; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(r14.wrapping_add(0x118u64), rbx); crate::simd_ops::wu32(r14.wrapping_add(0x124u64), ((rbx as u32)) as u32); bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = P8; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsi, rax); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = 0x1a0; rcx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rax = rsi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162e3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415a5850; const P1: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; rdi = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; bb = 2; continue; }
            2 => { rcx = ((r8 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r9 as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = rdx; rbx = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((((r9 as u32)) as i32 as i64) as u64) as u64; rbx = ((r10.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_sub(r10)) as u64; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r14 = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rdx).wrapping_add(r14)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (((rax as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x8); rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = ((rax).wrapping_add(r14)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 17; continue; }
            12 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 17; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162e500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162f410; const P1: u64 = 0x141651fc0; const P2: u64 = 0x141642fc0; const P3: u64 = 0x1401de410; const P4: u64 = 0x14162e180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x148u64))) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; rbp = rcx; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r15 = (((r14) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; r15 = r15.wrapping_sub(0x118); r14 = (((r14 as u32)).wrapping_sub(1)) as u32 as u64; rax = ((r15.wrapping_add(r8)) as u64) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x10cu64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x110u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x148u64))) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0xfffffffffffffee8u64)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x148u64))) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x140u64)))) as u64; rcx = rsi.wrapping_sub(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_sub(0x58); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_sub(0x58); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_sub(0xf8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x148u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            9 => { crate::simd_ops::wu32(rax.wrapping_add(0x10cu64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(0x110u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(r15).wrapping_add(0x110u64), (0x1u8) as u8); bb = 10; continue; }
            10 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 11; continue; }
            11 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162e630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524320; const P1: u64 = 0x142690d10; const P2: u64 = 0x141634020; const P3: u64 = 0x14162e7a0; const P4: u64 = 0x141636850;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rbp = r8; rcx = 0x5; r14 = rdx; rdx = 0x1a0; r15 = r9; r8 = ((rcx.wrapping_add(0xbu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x198u64))) as u32 as u64; rax = P1; rcx = ((((rcx as u32)) & (0xfffffffeu32))) as u32 as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x194u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u32) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0x3f800000u32) as u32); rcx = ((((rcx as u32)) & (0xfffffffdu32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), (0x3f800000u32) as u32); rax = (((rax as u32)).wrapping_add((rax as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x3f800000u32) as u32); rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), (0x3f800000u32) as u32); rcx = ((((rcx as u32)) & (0xfffffffbu32))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x198u64), ((rcx as u32)) as u32); rcx = rbp.wrapping_add(0x70); crate::simd_ops::wu64(rbx.wrapping_add(0x148u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x150u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x158u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x160u64), r15); crate::simd_ops::wu64(rbx.wrapping_add(0x168u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x178u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x180u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x188u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x18cu64), (0xc5800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x190u64), (0xc5800000u32) as u32); fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x170u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbx); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); bb = 8; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rax); rax = rbx; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbx); bb = 8; continue; }
            7 => { rax = rdi; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162e7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162f160; const P1: u64 = 0x14162e050; const P2: u64 = 0x143931de0; const P3: u64 = 0x143427a60; const P4: u64 = 0x143427660; const P5: u64 = 0x1416419b0; const P6: u64 = 0x143815b50; const P7: u64 = 0x141651fc0; const P8: u64 = 0x141642fc0; const P9: u64 = 0x1401de410; const P10: u64 = 0x14162e180;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x40); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x148u64))) as u32 as u64; r14 = ((r8 as u32)) as u32 as u64; r15 = rdx; rsi = rcx; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x14cu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x28u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x14cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x148u64))) as u32 as u64; rbx = (((rax) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x148u64), ((rax as u32)) as u32); fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x140u64)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x158u64))) as u64; rcx = 0x1; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x168u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); let xa_src: u64 = rcx; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P2, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x48u64), (xmm0) as u128); rcx = ((rcx).wrapping_add(1)) as u64; rdx = (0xffffffffffffffu64) as u64; xmm0 = (crate::simd_ops::ru128(P4)) as u128; rcx = (((rcx) & (rdx))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x78u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = rbp.wrapping_sub(0x20); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x68u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); crate::simd_ops::wu128(rbp.wrapping_sub(0x48u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), (0xffffu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x3f800000u32) as u32); crate::simd_ops::wu16(rbp.wrapping_sub(0x74u64), ((rdi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x72u64), (0x1u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x6cu64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rdi); crate::simd_ops::wu32(rbp.wrapping_sub(0x50u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), ((rdi as u8)) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x30u64), (0x32u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x48u64))) as u128; rcx = P6; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x58u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rcx); r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r9 = (r9 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx, rcx); xmm2 = 0u128; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rcx); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x50u64))) as u32 as u64; crate::simd_ops::wu128(rbx.wrapping_add(0x28u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x90u64), ((rcx as u32)) as u32); crate::simd_ops::wu128(rbx.wrapping_add(0x38u64), (xmm1) as u128); rax = (((rax as u8)) as u32) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x78u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rdi); crate::simd_ops::wu128(rbx.wrapping_add(0x48u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rdi as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x58u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x70u64))) as u32) as u128; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x6cu64))) as u32) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((xmm1 as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x68u64))) as u128; crate::simd_ops::wu8(rbp.wrapping_add(0x28u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x74u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x78u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x2cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x30u64), ((r9 as u8)) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x18u64), (xmm2) as u128); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xa8u64))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x30u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2cu64))) as u32 as u64; r8 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x28u64))) as u32) as u32 as u64; xmm2 = (crate::simd_ops::ru128(rbp.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu8(rbx.wrapping_add(0xa8u64), (0x0u8) as u8); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rcx); rcx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x30u64))) as u32) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rdi); crate::simd_ops::wu8(rbx.wrapping_add(0xb0u64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; crate::simd_ops::wu128(rbx.wrapping_add(0xc0u64), (xmm0) as u128); crate::simd_ops::wu32(rbx.wrapping_add(0xb8u64), ((rcx as u32)) as u32); rcx = rbp.wrapping_add(0x8); xmm0 = ((crate::simd_ops::ru64(rbp)) as u64) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0xd0u64), (xmm1) as u128); crate::simd_ops::wu8(rbx.wrapping_add(0x108u64), ((r8 as u8)) as u8); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x10cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rbx.wrapping_add(0xe8u64), (xmm1) as u128); crate::simd_ops::wu8(rbx.wrapping_add(0x110u64), ((r9 as u8)) as u8); crate::simd_ops::wu128(rbx.wrapping_add(0xf8u64), (xmm2) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rax = rbx; bb = 12; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; bb = 12; continue; }
            12 => { r11 = rsp.wrapping_add(0x140); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162eab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x170; const P5: u64 = 0xffffffffffffffe8; const P6: u64 = 0xffffffffffffffd8; const P7: u64 = 0xfffffffffffffff0; const P8: u64 = 0x14162e3d0; const P9: u64 = 0x150; const P10: u64 = 0x1415914d0; const P11: u64 = 0x30; const P12: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rbp = ((rdx as u32)) as u32 as u64; rdi = rcx; r14 = 0; fl_l = 0; fl_r = 0; rdx = rsi; crate::simd_ops::wu64(rax.wrapping_add(P5), r14); rcx = ((rax.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P7), r14); rbx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax, rsi); rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; r9 = ((rbp as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P1), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P3), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P11))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); rbx = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 4; continue; }
            3 => { rbx = 0x2; bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r14 as u32)) as u32); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ebd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x150; const P1: u64 = 0x8; const P2: u64 = 0x18c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(3u32)) as u8 as u64));
    fl_l = ((((rax as u8)) & (0x7u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ebf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x150; const P1: u64 = 0x8; const P2: u64 = 0x15c; const P3: u64 = 0x1434264e4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ed20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x150; const P1: u64 = 0x8; const P2: u64 = 0x1e8; const P3: u64 = 0x1f0; const P4: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((rdx as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rbx = ((rbx).wrapping_add(0x88u64)) as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P2)))) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162edc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x150; const P1: u64 = 0x8; const P2: u64 = 0x1c0; const P3: u64 = 0xf8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ede0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x170; const P1: u64 = 0x14162e3d0; const P2: u64 = 0x150; const P3: u64 = 0x8; const P4: u64 = 0x1415912d0; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rsi = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x20); rdx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax, rdi); rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; bb = 3; continue; }
            2 => { rbx = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0x4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ee80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x158;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((rax).wrapping_add(0x70u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ee90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x158; const P1: u64 = 0x168;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    rax = rdx;
    crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162eeb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x150; const P1: u64 = 0x1434264e4; const P2: u64 = 0x143427080; const P3: u64 = 0x1434268d0; const P4: u64 = 0x8; const P5: u64 = 0x154; const P6: u64 = 0x4; const P7: u64 = 0x158;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm2 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    xmm3 = ((crate::simd_ops::ru32(P3)) as u32) as u128;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128);
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64;
    if !((f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { xmm0 = (xmm2) as u128; } else { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); }
    crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128);
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64;
    if !((f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { xmm0 = (xmm2) as u128; } else { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); }
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((xmm0 as u32)) as u32);
    xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32) as u128;
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128);
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64;
    if !((f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { crate::simd_ops::wu32(rdx.wrapping_add(P4), ((xmm2 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((xmm1 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162ef40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x148; const P1: u64 = 0x140; const P2: u64 = 0xb8; const P3: u64 = 0xc0; const P4: u64 = 0xe8; const P5: u64 = 0x100; const P6: u64 = 0x14183acbc; const P7: u64 = 0xf8; const P8: u64 = 0x14183acc2;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rsi = r8; r11 = rdx; rdi = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; r10 = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x300; rax = (((r10 as u8)) as u32) as u32 as u64; r9 = (if (((fl_l as u32)) == ((fl_r as u32))) { (r10 as u32) } else { (r9 as u32) }) as u32 as u64; rbx = (((r9 as u8)) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0x7u32)) as u32 as u64; rbx = ((((rbx as u32)) & (0xfffffff8u32))) as u32 as u64; rbx = ((((rbx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; rcx = r11; r8 = ((rbx as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = rsi; r8 = ((rbx as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rbx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((r8).wrapping_shl(2u32)) as u64; rcx = r11; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r8 = ((rbx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((r8).wrapping_shl(2u32)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162f160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524320; const P1: u64 = 0x14162e050; const P2: u64 = 0x14162f410; const P3: u64 = 0x141651fc0; const P4: u64 = 0x141642fc0; const P5: u64 = 0x1401de410; const P6: u64 = 0x14162e180; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x124u64))) as u32 as u64; rbp = rcx; r12 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r12 = (((r12 as u32)).wrapping_shr(1u32)) as u32 as u64; r12 = (((r12 as u32)).wrapping_add((rdx as u32))) as u32 as u64; r15 = (crate::simd_ops::ru32(rbp.wrapping_add(0x120u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r12 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r12 as u32) }) as u32 as u64; r12 = (((r12 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = ((r12 as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; fl_l = rdx; fl_r = (0x118u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r14 = rbp; bb = 3; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; r8 = ((rcx.wrapping_add(0x8u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = rax; bb = 3; continue; }
            3 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x118u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x118u64))) as u64; rbx = rsi; rbx = ((rbx).wrapping_sub(r14)) as u64; rcx = rsi; rdx = ((rdx).wrapping_add(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x118u64))) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; rcx = rdi.wrapping_add(0xe8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x118u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x118u64))) as u64; fl_l = rbp; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbp.wrapping_add(0x118u64), r14); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbp.wrapping_add(0x124u64), ((r12 as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162f2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x150; const P1: u64 = 0x8; const P2: u64 = 0x160;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(3u32)) as u8 as u64));
    fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162f2f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rdx; crate::simd_ops::wu64(rcx, rax); rbp = 0; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x8u64))) as u128; rbx = rcx; crate::simd_ops::wu128(rcx.wrapping_add(0x8u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x18u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x28u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x38u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), ((xmm0 as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), ((rbp as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x6cu64), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x6cu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x6cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rbp); crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rbp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x88u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(0x88u64), ((rbp as u8)) as u8); bb = 5; continue; }
            5 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rbp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rax); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x88u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rbp); crate::simd_ops::wu8(rbx.wrapping_add(0x88u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rdi.wrapping_add(0x88u64), ((rbp as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x90u64))) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x90u64), ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162f410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x14162f2f0; const P4: u64 = 0xb8; const P5: u64 = 0xc0; const P6: u64 = 0x10c; const P7: u64 = 0x108; const P8: u64 = 0x110; const P9: u64 = 0x141642ff0; const P10: u64 = 0xe8; const P11: u64 = 0x141651fc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = rdx; crate::simd_ops::wu64(rcx, rax); rbx = rcx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rcx = ((rcx).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P2), rsi); rdx = ((rdx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rdx = ((rdi.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P7), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(P8))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xf0u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xf0u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x100u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x100u64), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xe8u64), rsi); crate::simd_ops::wu32(rdi.wrapping_add(0xf0u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0xf8u64), rsi); crate::simd_ops::wu64(rdi.wrapping_add(0x100u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162f510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141631060; const P2: u64 = 0x141630e40; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = (crate::simd_ops::ru64(r8)) as u64; rsi = r9; r15 = r8; rbp = rdx; rdi = r14; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r8.wrapping_add(0x8u64), r14); rdi = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r8, rdi); bb = 3; continue; }
            3 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); r12 = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 4; continue; }
            4 => { rdx = ((rbx.wrapping_add(0x38u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax, rbx); rcx = 0x10; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rbx); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((rcx as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x8); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 9; continue; }
            9 => { rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rbx); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rbx as u32)) as u32); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rbx as u32)) as u32); bb = 16; continue; }
            16 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = 0x34; bb = 27; continue; }
            18 => { crate::simd_ops::wu64(rdi, rbx); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rdi); bb = 21; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_add_u32(rbp, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r15, rdi); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), r14); bb = 22; continue; }
            22 => { rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = ((rax as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x8); rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r12 as u32) } else { (rdx as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rbx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xf8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xfcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x100u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x104u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x108u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = ((r12 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10cu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xc0u64))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xb4u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xc8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 27; continue; }
            27 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162f730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141631440; const P2: u64 = 0x141631140; const P3: u64 = 0x1415242e0; const P4: u64 = 0x141630b90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(r8)) as u64; r15 = r9; r14 = r8; rbp = rdx; rdi = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rsi); rdi = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(r8, rdi); bb = 3; continue; }
            3 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); r12 = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 4; continue; }
            4 => { rdx = ((rbx.wrapping_add(0x38u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax, rbx); r13 = 0x10; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rbx); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((r13 as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x8); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 8; continue; }
            8 => { r13 = (((r13 as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = rdi.wrapping_add(0x18); rdx = ((r13 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rbx); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rbx as u32)) as u32); bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rbx as u32)) as u32); bb = 15; continue; }
            15 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rax = 0x34; bb = 28; continue; }
            17 => { crate::simd_ops::wu64(rdi, rbx); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rdi); bb = 20; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_add_u32(rbp, (1) as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rsi); bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = ((rax as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x8); rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r12 as u32) } else { (rdx as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rbx); bb = 26; continue; }
            26 => { rbx = rcx; bb = 27; continue; }
            27 => { rdx = rbx; rcx = r15; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = ((r12 as u32)) as u32 as u64; bb = 28; continue; }
            28 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162f910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141631760; const P2: u64 = 0x141630e40; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(r8)) as u64; rbp = r9; r15 = r8; r14 = rdx; rdi = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rsi); rdi = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(r8, rdi); bb = 3; continue; }
            3 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); r12 = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 4; continue; }
            4 => { rdx = ((rbx.wrapping_add(0x38u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax, rbx); rcx = 0x10; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rbx); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((rcx as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x8); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 9; continue; }
            9 => { rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = 0x34; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rbx); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rbx as u32)) as u32); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rbx as u32)) as u32); bb = 16; continue; }
            16 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = 0x34; bb = 27; continue; }
            18 => { crate::simd_ops::wu64(rdi, rbx); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rdi); bb = 21; continue; }
            20 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_add_u32(r14, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r15, rdi); crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rsi); bb = 22; continue; }
            22 => { rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = ((rax as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x8); rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r12 as u32) } else { (rdx as u32) }) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); rcx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xf8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xfcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = ((r12 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x100u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xc0u64))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xb4u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 27; continue; }
            27 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162fb20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426d88; const P1: u64 = 0x1434268d0; const P2: u64 = 0x1415242e0; const P3: u64 = 0x141524320; const P4: u64 = 0x14183ad76; const P5: u64 = 0x141624720;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); r13 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); r15 = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm9) as u128); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { xmm8 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(P1)) as u32) as u128; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x2cu64))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(2u32)) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x2cu64), ((rbx as u32)) as u32); crate::simd_ops::wu64(r15.wrapping_add(0x30u64), rbx); r8 = ((rcx.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r15.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x2cu64), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r12 = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; rsi = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 8; continue; }
            8 => { rbp = ((r13.wrapping_sub(0x1u64)) as u32) as u32 as u64; r14 = rdi.wrapping_add(0x18); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu32(r14.wrapping_sub(0x10u64), (0xffffffffu32) as u32); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x48u64))) as u128; xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x38u64))) as u128; rax = ((xmm1 as u64)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu128(r14, (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(r14.wrapping_add(0x10u64), (xmm1) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm1 = (xmm8) as u128; bb = 13; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { xmm1 = (xmm9) as u128; bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; xmm1 = 0u128; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; xmm6 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32) as u128; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r13 as u32))) as u32 as u64; xmm7 = (xmm6) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm7 = (xmm0) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (xmm0) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 19; continue; }
            19 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32) as u128; rax = rsp.wrapping_add(0xd0); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); xmm3 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); r8 = ((r13 as u32)) as u32 as u64; rdx = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm7 as u32)) as u32); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r12.wrapping_add(rbp.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_sub(0xcu64), ((rcx as u32)) as u32); bb = 26; continue; }
            20 => { crate::simd_ops::wu64(rdi, (0x0u64) as u64); xmm3 = 0u128; xmm1 = (crate::simd_ops::ru128(r14)) as u128; rax = ((rbp as u32)) as u32 as u64; xmm2 = (crate::simd_ops::ru128(r14.wrapping_add(0x10u64))) as u128; xmm0 = (xmm1) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm1, 0xffu8); rax = ((xmm2 as u64)) as u64; xmm1 = crate::simd_ops::vshufps(xmm1, xmm1, 0x93u8); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); rax = ((rax).wrapping_shr(32u32)) as u64; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | (xmm3 & 0xffff_ffffu128); xmm1 = crate::simd_ops::vshufps(xmm1, xmm1, 0x39u8); crate::simd_ops::wu128(r14, (xmm1) as u128); crate::simd_ops::wu128(r14.wrapping_add(0x10u64), (xmm2) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { xmm1 = (xmm8) as u128; bb = 24; continue; }
            22 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { xmm1 = (xmm9) as u128; bb = 24; continue; }
            24 => { xmm0 = (xmm3) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); bb = 25; continue; }
            25 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32) as u128; rax = rsp.wrapping_add(0xd0); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = r14.wrapping_sub(0xc); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); xmm3 = (xmm7) as u128; r8 = 0x1; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm7 as u32)) as u32); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = ((crate::simd_ops::ru32(r14.wrapping_sub(0xcu64))) as u32) as u128; rbp = ((r13.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu32(r14.wrapping_sub(0x8u64), ((xmm6 as u32)) as u32); rdi = ((rdi).wrapping_add(0x38u64)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; r14 = ((r14).wrapping_add(0x38u64)) as u64; rbx = ((rbx).wrapping_add(0x58u64)) as u64; r12 = ((r12.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 27; continue; }
            27 => { r15 = (crate::simd_ops::ru64(r15)) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 28; continue; }
            28 => { rax = 0x1; bb = 29; continue; }
            29 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { rax = 0x2; bb = 29; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14162fdf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426a48; const P1: u64 = 0x1415242e0; const P2: u64 = 0x141524320; const P3: u64 = 0x1438152ac;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rsi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); rbp = ((rdx as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { xmm7 = ((crate::simd_ops::ru64(P0)) as u64) as u128; r14 = 0; fl_l = 0; fl_r = 0; xmm6 = 0u128; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(2u32)) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), r14); r8 = ((rcx.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 8; continue; }
            8 => { r11 = r8.wrapping_add(0x10); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r8, rdi); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(r11, ((r14 as u32)) as u32); crate::simd_ops::wu32(r11.wrapping_sub(0x8u64), (0xffffffffu32) as u32); r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((r10 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((rbx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r10 = ((rdx as u32)) as u32 as u64; bb = 13; continue; }
            13 => { xmm0 = 0u128; rax = (crate::simd_ops::ru32(P3)) as u32 as u64; xmm1 = 0u128; xmm2 = (xmm7) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((((rbx as u32)) as i32) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((r10 as u32)) as i32) as f64).to_bits() as u128); xmm3 = crate::simd_ops::vcvtpd2ps(xmm1); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm1 = crate::simd_ops::vcvtpd2ps(xmm0); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm3 = (xmm1) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r11, ((xmm3 as u32)) as u32); bb = 28; continue; }
            15 => { xmm1 = (xmm3) as u128; crate::simd_ops::wu32(r11.wrapping_sub(0x8u64), (0x3u32) as u32); xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r11, ((xmm3 as u32)) as u32); bb = 28; continue; }
            16 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = ((r10 as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((rbx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r10 = ((rdx as u32)) as u32 as u64; bb = 19; continue; }
            19 => { rdx = ((rdi.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(P3)) as u32 as u64; xmm0 = 0u128; xmm1 = 0u128; xmm2 = (xmm7) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | (((((rbx as u32)) as i32) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); rax = rdi; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) / f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((((r10 as u32)) as i32) as f64).to_bits() as u128); xmm3 = crate::simd_ops::vcvtpd2ps(xmm1); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm1 = crate::simd_ops::vcvtpd2ps(xmm0); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm3 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { xmm0 = (xmm1) as u128; xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x0u8); fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { crate::simd_ops::wu128(rax, (xmm0) as u128); rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 23; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru32(r11)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r11, ((xmm0 as u32)) as u32); bb = 28; continue; }
            24 => { xmm0 = (xmm3) as u128; xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x0u8); fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { crate::simd_ops::wu128(rax, (xmm0) as u128); rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 27; continue; }
            27 => { xmm0 = ((crate::simd_ops::ru32(r11)) as u32) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r11, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(r11.wrapping_sub(0x8u64), (0x3u32) as u32); bb = 28; continue; }
            28 => { crate::simd_ops::wu32(r11.wrapping_sub(0x4u64), ((xmm1 as u32)) as u32); r8 = ((r8).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; r11 = ((r11).wrapping_add(0x18u64)) as u64; rcx = ((rcx).wrapping_add(0x28u64)) as u64; rdi = ((rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 29; continue; }
            29 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 30; continue; }
            30 => { rax = 0x1; bb = 31; continue; }
            31 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            32 => { rax = 0x2; bb = 31; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141630060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143426ff0; const P2: u64 = 0x141632050; const P3: u64 = 0x143428760;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xd8u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm7) as u128); rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r12); xmm7 = 0u128; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x40u64), r15); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm8) as u128); rax = r8; rbp = rax.wrapping_add(0x10); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm9) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm10) as u128); r8 = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm12) as u128); xmm12 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add(0x2u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r14 = ((((r14 as u32)) & (0xfffffffcu32))) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32) as u128; r15 = ((((r15 as u32)) & (0xfffffffcu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); r15 = (((r15 as u32)).wrapping_add((r14 as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r13 = (((r13 as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; r13 = (((r13 as u32)).wrapping_add(0x2u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r14 as u32)) as u32); r13 = ((((r13 as u32)) & (0xfffffffcu32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x108u64), ((rax as u32)) as u32); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((r13 as u32)) as u32 as u64; xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm8 = 0u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = (((r15 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; r15 = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            4 => { rax = ((r13 as u32)) as u32 as u64; xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm12) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm8 = 0u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rax = (((rax as u32)).wrapping_add((r13 as u32))) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = (((r15 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; r15 = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            5 => { rax = ((r13 as u32)) as u32 as u64; xmm8 = 0u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); bb = 6; continue; }
            6 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32) as u128; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r9); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rax = ((r14 as u32)) as u32 as u64; xmm0 = 0u128; rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rcx = ((r9.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { crate::simd_ops::wu128(r9, (xmm0) as u128); r9 = ((r9).wrapping_add(0x10u64)) as u64; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { xmm0 = ((crate::simd_ops::ru32(rbp)) as u32) as u128; rsi = (((rsi as u32)).wrapping_add((rdx as u32))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm7 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbp, ((xmm0 as u32)) as u32); bb = 13; continue; }
            13 => { xmm9 = ((crate::simd_ops::ru32(P1)) as u32) as u128; r12 = ((r14.wrapping_add(r13)) as u32) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rbx); r12 = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r15 as u32) } else { (r12 as u32) }) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm3 = (xmm7) as u128; bb = 17; continue; }
            16 => { xmm3 = ((crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm8 as u32)).to_bits() as u128); bb = 17; continue; }
            17 => { rax = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rsp.wrapping_add(0x38); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = ((rdi as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rsi = (((rsi as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm2 = (xmm0) as u128; bb = 18; continue; }
            18 => { rdx = ((r14.wrapping_add(r13.wrapping_mul(2))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r15 as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x118u64), ((rdx as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 19; continue; }
            19 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { xmm3 = (xmm7) as u128; bb = 22; continue; }
            21 => { xmm3 = (xmm12) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x20u64)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm8 as u32)).to_bits() as u128); bb = 22; continue; }
            22 => { rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rsp.wrapping_add(0x38); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = ((rdi as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x118u64))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm2 = (xmm0) as u128; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r8 = ((r15 as u32)) as u32 as u64; xmm10 = 0u128; r14 = ((rdx.wrapping_add(rax)) as u32) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; xmm10 = (xmm10 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); r8 = (if (((fl_l as u32)) < ((fl_r as u32))) { (r14 as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = ((xmm10 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { xmm3 = (xmm7) as u128; bb = 27; continue; }
            26 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; xmm3 = ((crate::simd_ops::ru32(rax.wrapping_add(0x24u64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm10 as u32)).to_bits() as u128); bb = 27; continue; }
            27 => { rax = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rsp.wrapping_add(0x38); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = ((rdi as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x118u64))) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rbx as u32))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm2 = (xmm0) as u128; bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32) as u128; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 51; continue; } bb = 29; continue; }
            29 => { xmm1 = (xmm12) as u128; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 30; continue; }
            30 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { xmm3 = (xmm7) as u128; xmm0 = (xmm7) as u128; bb = 33; continue; }
            32 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; xmm3 = (xmm12) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x20u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm8 as u32)).to_bits() as u128); bb = 33; continue; }
            33 => { rax = ((r12 as u32)) as u32 as u64; xmm1 = 0u128; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x34u64)))) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((rdx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 34; continue; }
            34 => { xmm8 = (xmm1) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 50; continue; } bb = 35; continue; }
            35 => { fl_l = ((xmm10 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { xmm1 = (xmm7) as u128; bb = 39; continue; }
            37 => { xmm1 = (xmm6) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm10 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 39; continue; } bb = 38; continue; }
            38 => { xmm0 = (xmm8) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0) ^ (crate::simd_ops::ru128(P3)); rbp = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; bb = 39; continue; }
            39 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((r15 as u32))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rax = ((r14 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rbp as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rbp = ((r9 as u32)) as u32 as u64; bb = 42; continue; }
            41 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rbp = (if (((fl_l as u32)) < ((fl_r as u32))) { (r14 as u32) } else { (rbp as u32) }) as u32 as u64; bb = 42; continue; }
            42 => { rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; xmm3 = 0u128; rbp = (((rbp as u32)).wrapping_add(0x2u32)) as u32 as u64; rbp = ((((rbp as u32)) & (0xfffffffcu32))) as u32 as u64; xmm3 = (xmm3 & !0xffff_ffffu128) | ((((r9) as i64) as f32).to_bits() as u128); rcx = ((r8.wrapping_add(rbp)) as u32) as u32 as u64; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { xmm0 = (xmm9) as u128; bb = 45; continue; }
            44 => { xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); bb = 45; continue; }
            45 => { rax = ((rbp as u32)) as u32 as u64; xmm5 = 0u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm4 = (xmm5) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 62; continue; } bb = 46; continue; }
            46 => { fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 48; continue; } bb = 47; continue; }
            47 => { xmm3 = (xmm7) as u128; bb = 49; continue; }
            48 => { xmm3 = 0u128; rax = ((r8 as u32)) as u32 as u64; xmm0 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((rdx as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm3) ^ (crate::simd_ops::ru128(P3)); bb = 49; continue; }
            49 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (((rcx as u32)).wrapping_sub((rsi as u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rbx = ((rdi as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rcx as u32) } else { (rbx as u32) }) as u32 as u64; rcx = rsp.wrapping_add(0x38); rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rsi = (((rsi as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm2 = (xmm0) as u128; bb = 52; continue; }
            50 => { xmm6 = (xmm1) as u128; bb = 51; continue; }
            51 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 52; continue; }
            52 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; bb = 53; continue; }
            53 => { xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rdx = ((r15 as u32)) as u32 as u64; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rdx = (((rdx as u32)).wrapping_sub((rsi as u32))) as u32 as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rdi as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 59; continue; } bb = 54; continue; }
            54 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rdi as u32) } else { (rdx as u32) }) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 59; continue; } bb = 55; continue; }
            55 => { xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32) as u128; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (xmm2) as u128; xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x0u8); r8 = ((rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 58; continue; } bb = 56; continue; }
            56 => { bb = 57; continue; }
            57 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 57; continue; } bb = 58; continue; }
            58 => { xmm0 = (xmm2) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r8); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r14)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r14)) }).to_bits() as u128); rsi = (((rsi as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(r14, ((xmm0 as u32)) as u32); bb = 59; continue; }
            59 => { r9 = (((r9 as u32)).wrapping_sub((rbp as u32))) as u32 as u64; rbp = ((r9.wrapping_add(r15)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 65; continue; } bb = 60; continue; }
            60 => { rax = ((r9 as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 63; continue; } bb = 61; continue; }
            61 => { xmm6 = (xmm7) as u128; bb = 64; continue; }
            62 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; bb = 53; continue; }
            63 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm6) ^ (crate::simd_ops::ru128(P3)); bb = 64; continue; }
            64 => { rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rsp.wrapping_add(0x38); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = ((rdi as u32)) as u32 as u64; xmm3 = (xmm6) as u128; rbx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (((rsi as u32)).wrapping_add((rbx as u32))) as u32 as u64; xmm2 = (xmm0) as u128; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 65; continue; }
            65 => { xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 71; continue; } bb = 66; continue; }
            66 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 71; continue; } bb = 67; continue; }
            67 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = 0u128; rax = ((rdi as u32)) as u32 as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 70; continue; } bb = 68; continue; }
            68 => { bb = 69; continue; }
            69 => { crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 69; continue; } bb = 70; continue; }
            70 => { xmm0 = ((crate::simd_ops::ru32(r14)) as u32) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm7 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm7 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r14, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x3u32) as u32); bb = 72; continue; }
            71 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm2 as u32)) as u32); bb = 72; continue; }
            72 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; rsp = ((rsp).wrapping_add(0xd8u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141630640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143426ff0; const P2: u64 = 0x143428760;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = ((rsp).wrapping_sub(0x88u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); r9 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); xmm4 = 0u128; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); r10 = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r13); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r14); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm10) as u128); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rax as u32)) as u32); r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; r15 = ((((r15 as u32)) & (0xfffffffcu32))) as u32 as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((r15 as u32))) as u32 as u64; xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32) as u128; rdx = (((rdx as u32)).wrapping_sub((r9 as u32))) as u32 as u64; r11 = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; r13 = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; rbx = (((rbx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp, (xmm11) as u128); xmm11 = ((crate::simd_ops::ru32(P0)) as u32) as u128; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32) as u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rbx as u32)) as u32 as u64; xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm5 = 0u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            4 => { rax = ((rbx as u32)) as u32 as u64; xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm11) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm5 = 0u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 6; continue; }
            5 => { rax = ((rbx as u32)) as u32 as u64; xmm5 = 0u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32) as u128; bb = 8; continue; }
            8 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32) as u128; rcx = ((r9 as u32)) as u32 as u64; rax = ((r15 as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 10; continue; }
            10 => { xmm8 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rsi = ((rbx.wrapping_add(r15)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdi as u32) } else { (rsi as u32) }) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm0 = 0u128; bb = 14; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(r10.wrapping_add(0x20u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); bb = 14; continue; }
            14 => { xmm1 = 0u128; rcx = ((rsi as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); bb = 15; continue; }
            15 => { r14 = ((r15.wrapping_add(rbx.wrapping_mul(2))) as u32) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; r14 = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdi as u32) } else { (r14 as u32) }) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 16; continue; }
            16 => { fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = 0u128; bb = 19; continue; }
            18 => { xmm0 = (xmm11) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(r10.wrapping_add(0x20u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); bb = 19; continue; }
            19 => { xmm1 = 0u128; rcx = ((r14 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); bb = 20; continue; }
            20 => { r12 = ((r14.wrapping_add(r11)) as u32) as u32 as u64; rbp = ((rdi as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; xmm6 = 0u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((((r11) as i64) as f32).to_bits() as u128); rbp = (if (((fl_l as u32)) < ((fl_r as u32))) { (r12 as u32) } else { (rbp as u32) }) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 25; continue; } bb = 21; continue; }
            21 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { xmm0 = 0u128; bb = 24; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru32(r10.wrapping_add(0x24u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 24; continue; }
            24 => { xmm1 = 0u128; rcx = ((rbp as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); bb = 25; continue; }
            25 => { xmm2 = ((crate::simd_ops::ru32(r10.wrapping_add(0x24u64))) as u32) as u128; r11 = 0; fl_l = 0; fl_r = 0; xmm10 = ((crate::simd_ops::ru32(P2)) as u32) as u128; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 46; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); xmm1 = (xmm11) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { xmm7 = 0u128; xmm0 = 0u128; bb = 30; continue; }
            29 => { xmm0 = ((crate::simd_ops::ru32(r10.wrapping_add(0x20u64))) as u32) as u128; xmm7 = (xmm11) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(crate::simd_ops::ru32(r10.wrapping_add(0x20u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); bb = 30; continue; }
            30 => { rax = ((rsi as u32)) as u32 as u64; xmm1 = 0u128; rax = (((rax as u32)).wrapping_sub((r15 as u32))) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((r14 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 31; continue; }
            31 => { xmm9 = (xmm1) as u128; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 44; continue; } bb = 32; continue; }
            32 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 37; continue; } bb = 33; continue; }
            33 => { xmm1 = 0u128; bb = 34; continue; }
            34 => { rax = ((r11 as u32)) as u32 as u64; bb = 35; continue; }
            35 => { r11 = ((r12 as u32)) as u32 as u64; xmm5 = 0u128; r11 = (((r11 as u32)).wrapping_sub((rdi as u32))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; xmm5 = (xmm5 & !0xffff_ffffu128) | ((((r13) as i64) as f32).to_bits() as u128); r11 = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (r11 as u32) }) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; r11 = (if (((fl_l as u32)) < ((fl_r as u32))) { (r13 as u32) } else { (r11 as u32) }) as u32 as u64; r11 = (((r11 as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; rcx = ((r11.wrapping_add(rbp)) as u32) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 39; continue; } bb = 36; continue; }
            36 => { xmm0 = 0u128; bb = 40; continue; }
            37 => { xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm5 = (xmm1) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 34; continue; } bb = 38; continue; }
            38 => { xmm0 = (xmm9) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0) ^ (xmm10); rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; bb = 35; continue; }
            39 => { xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0) ^ (xmm10); bb = 40; continue; }
            40 => { rax = ((r11 as u32)) as u32 as u64; xmm7 = 0u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm6 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm6) ^ (xmm10); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 45; continue; } bb = 41; continue; }
            41 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; xmm5 = 0u128; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rax = ((rbp as u32)) as u32 as u64; xmm0 = 0u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((r14 as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) / f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm5 = (xmm5) ^ (xmm10); bb = 43; continue; }
            43 => { rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; xmm1 = 0u128; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); bb = 45; continue; }
            44 => { xmm2 = (xmm1) as u128; bb = 45; continue; }
            45 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 46; continue; }
            46 => { xmm11 = (crate::simd_ops::ru128(rsp)) as u128; rcx = ((rdi as u32)) as u32 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = ((r9 as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 49; continue; } bb = 47; continue; }
            47 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (r9 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { xmm3 = ((crate::simd_ops::ru32(r10.wrapping_add(0x24u64))) as u32) as u128; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32) as u128; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); bb = 49; continue; }
            49 => { r13 = (((r13 as u32)).wrapping_sub((r11 as u32))) as u32 as u64; r10 = ((rdi.wrapping_add(r13)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 54; continue; } bb = 50; continue; }
            50 => { rax = ((r13 as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 52; continue; } bb = 51; continue; }
            51 => { xmm2 = 0u128; bb = 53; continue; }
            52 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2) ^ (xmm10); bb = 53; continue; }
            53 => { xmm1 = 0u128; rcx = ((r10 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r9 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm3 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm3 as u32) }).to_bits() as u128); xmm3 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); bb = 54; continue; }
            54 => { xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 57; continue; } bb = 55; continue; }
            55 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(xmm4 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm4 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), (0x3u32) as u32); rsp = ((rsp).wrapping_add(0x88u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            57 => { crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((xmm3 as u32)) as u32); rsp = ((rsp).wrapping_add(0x88u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141630b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xbc; const P1: u64 = 0x60; const P2: u64 = 0x10; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_add_u32(rdx.wrapping_add(P0), (1) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), (0x0u64) as u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { fl_l = (crate::simd_ops::lk_add_u32(rcx, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx); return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P1), rdx);
    fl_l = (crate::simd_ops::lk_add_u32(rcx, (1) as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141630b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143426d88; const P2: u64 = 0x14183ad76; const P3: u64 = 0x1438152ac; const P4: u64 = 0x1434269f8; const P5: u64 = 0x14183ad7c; const P6: u64 = 0x143428740; const P7: u64 = 0x14183ad70; const P8: u64 = 0x142674d68; const P9: u64 = 0x14183ad6a; const P10: u64 = 0x143426ac0; const P11: u64 = 0x143426a48; const P12: u64 = 0x142674d58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xa8u64))) as u64; rdi = rdx; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x18u64))) as u128; xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rax = ((xmm1 as u64)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x118u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rcx.wrapping_add(0x128u64), (xmm1) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm1 = (xmm9) as u128; bb = 5; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { xmm1 = (xmm8) as u128; bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x124u64), ((xmm0 as u32)) as u32); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x138u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm10) as u128); xmm1 = 0u128; xmm10 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10cu64))) as u32) as u128; fl_l = ((xmm10 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm3 = (xmm8) as u128; bb = 10; continue; }
            9 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm7) as u128); xmm7 = 0u128; xmm7 = (xmm7 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm1 = crate::simd_ops::vcvtps2pd(xmm1); xmm6 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) * f64::from_bits(crate::simd_ops::ru64(P4))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((if f64::from_bits(xmm1 as u64) < f64::from_bits(xmm7 as u64) { f64::from_bits(xmm1 as u64) } else { f64::from_bits(xmm7 as u64) }).to_bits() as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm1 as u64)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = crate::simd_ops::vcvtps2pd(xmm10); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm1 as u64)).to_bits() as u128); xmm0 = (xmm0) ^ (crate::simd_ops::ru128(P6)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm6 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) / f64::from_bits(xmm7 as u64)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P8))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru64(P10)) as u64) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) - f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(crate::simd_ops::ru64(P11))).to_bits() as u128); xmm0 = crate::simd_ops::vsqrtpd(xmm1); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm1 = crate::simd_ops::vcvtpd2ps(xmm0); xmm3 = (xmm1) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; xmm2 = 0u128; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); bb = 13; continue; }
            12 => { xmm0 = (xmm8) as u128; bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rbx.wrapping_add(0x128u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x104u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x128u64), ((xmm0 as u32)) as u32); bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rbx.wrapping_add(0x11cu64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x120u64), ((xmm1 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x12cu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 16; continue; }
            16 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x124u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P12))).to_bits() as u128); bb = 20; continue; }
            18 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x124u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rbx.wrapping_add(0x124u64), ((xmm0 as u32)) as u32); bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rbx.wrapping_add(0x12cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x138u64), (0x0u8) as u8); bb = 22; continue; }
            22 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xf8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xfcu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x100u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x104u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x110u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x118u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x38u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x128u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x48u64), (xmm1) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc0u64))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xb4u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xd0u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141630e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524350; const P1: u64 = 0x141524320; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; r9 = 0x10; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r12 = 0; fl_l = 0; fl_r = 0; r10 = ((r12 as u32)) as u32 as u64; fl_l = rsi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r9 = rsi.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rdx = rdi.wrapping_add(0xc); r15 = (0xfffffffffffffff4u64) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; r14 = 0x24; r11 = 0x3c; r15 = ((r15).wrapping_sub(rdi)) as u64; r14 = ((r14).wrapping_sub(rdi)) as u64; r11 = ((r11).wrapping_sub(rdi)) as u64; fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; r10 = ((r9.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = rdx.wrapping_sub(0xc); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx, r12); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r15.wrapping_add(rdx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rcx = rdx.wrapping_add(0xc); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), r12); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x28u64))) as u64) as u128; r8 = rdx.wrapping_add(0x24); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r8, r12); crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), r12); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); r8 = rdx.wrapping_add(0x3c); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(r8, r12); crate::simd_ops::wu32(rdx.wrapping_add(0x44u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x48u64), r12); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(r11)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rdx).wrapping_add(0x10u64))) as u64) as u128; rdx = ((rdx).wrapping_add(0x60u64)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 19; continue; }
            19 => { fl_l = r10; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; fl_l = rsi; fl_r = r10; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 21; continue; }
            21 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xcu64), r12); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdx; rcx = ((rcx).wrapping_sub(rdi)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141631060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524350; const P3: u64 = 0x141524320; const P4: u64 = 0x30; const P5: u64 = 0xffffffffffffffd0; const P6: u64 = 0xffffffffffffffe0; const P7: u64 = 0xfffffffffffffff0; const P8: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; r9 = 0x10; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 11; continue; }
            3 => { r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P6))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P6), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P7))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P7), (xmm0) as u128); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141631140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524350; const P1: u64 = 0x141524320; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((r15 as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rcx; r9 = 0x10; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { rdx = rcx; r8 = 0x10; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r9 = (0xffffffffffffffe0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); r9 = ((r9).wrapping_sub(rsi)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rbp = ((r12 as u32)) as u32 as u64; fl_l = rdi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r10 = rdi.wrapping_sub(0x4); r14 = 0x88; r10 = ((r10).wrapping_shr(2u32)) as u64; r11 = 0x50; rdx = rsi.wrapping_add(0x20); r14 = ((r14).wrapping_sub(rsi)) as u64; r11 = ((r11).wrapping_sub(rsi)) as u64; fl_l = ((r10).wrapping_add(1)) as u64; r10 = ((fl_l as u64)) as u64; rbp = ((r10.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = rdx.wrapping_sub(0x20); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu32(rdx.wrapping_sub(0x18u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0x14u64), r12); crate::simd_ops::wu64(rdx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r12); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rdx.wrapping_add(r9)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); rcx = rdx.wrapping_add(0x18); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rcx, r12); crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x24u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x38u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x40u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x48u64), r12); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x58u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x68u64))) as u64) as u128; r8 = rdx.wrapping_add(0x50); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r8, r12); crate::simd_ops::wu32(rdx.wrapping_add(0x58u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x5cu64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x80u64), r12); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r11))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(r11).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r11).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(r11).wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); r8 = rdx.wrapping_add(0x88); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(r8, r12); crate::simd_ops::wu32(rdx.wrapping_add(0x90u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x94u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0xa0u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0xb0u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0xb8u64), r12); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(r14)) as u64) as u64; rdx = ((rdx).wrapping_add(0xe0u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 19; continue; }
            19 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rdx = (((rbp) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x20u64)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = rdi; fl_r = rbp; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { r8 = rdx.wrapping_sub(0x20); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(r8, r12); crate::simd_ops::wu32(rdx.wrapping_sub(0x18u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0x14u64), r12); crate::simd_ops::wu64(rdx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r12); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r9.wrapping_add(rdx)) as u64) as u64; rdx = ((rdx).wrapping_add(0x38u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rsi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r15 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141631440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524350; const P1: u64 = 0x141524320; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((r15 as u32)) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rcx; r9 = 0x10; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { rdx = rcx; r8 = 0x10; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { r9 = (0xffffffffffffffc0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); r12 = 0; fl_l = 0; fl_r = 0; r9 = ((r9).wrapping_sub(rsi)) as u64; r11 = ((r12 as u32)) as u32 as u64; fl_l = rdi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r10 = rdi.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rdx = rsi.wrapping_add(0x40); r14 = 0x70; r10 = ((r10).wrapping_shr(2u32)) as u64; rbp = 0xc8; r14 = ((r14).wrapping_sub(rsi)) as u64; rbp = ((rbp).wrapping_sub(rsi)) as u64; fl_l = ((r10).wrapping_add(1)) as u64; r10 = ((fl_l as u64)) as u64; r11 = ((r10.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { r8 = rdx.wrapping_sub(0x40); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r12); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(r9)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); r8 = rdx.wrapping_add(0x18); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx.wrapping_add(0x50u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), r12); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x58u64))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x68u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x78u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x88u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x98u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0xa8u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); r8 = rdx.wrapping_add(0x70); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdx.wrapping_add(0xa8u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0xb0u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0xb8u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0xc0u64), r12); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(r14)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); r8 = rdx.wrapping_add(0xc8); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdx.wrapping_add(0x100u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x108u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x110u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x118u64), r12); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rbp)) as u64) as u64; rdx = ((rdx).wrapping_add(0x160u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 19; continue; }
            19 => { fl_l = r11; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rdx = (((r11) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0x40u64)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = rdi; fl_r = r11; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { r8 = rdx.wrapping_sub(0x40); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rdx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r12); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r9.wrapping_add(rdx)) as u64) as u64; rdx = ((rdx).wrapping_add(0x58u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rsi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r15 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141631760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524350; const P3: u64 = 0x141524320; const P4: u64 = 0x28; const P5: u64 = 0xffffffffffffffd8; const P6: u64 = 0xffffffffffffffe8; const P7: u64 = 0xfffffffffffffff8; const P8: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbp.wrapping_mul(4)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; r9 = 0x10; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 11; continue; }
            3 => { r8 = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(P6))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rdi).wrapping_add(P6), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rdi).wrapping_add(P7), ((xmm0 as u64)) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rbp as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141631840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162f910; const P1: u64 = 0x14162f510; const P2: u64 = 0x14162f730; const P3: u64 = 0x1415242e0; const P4: u64 = 0x141630e40; const P5: u64 = 0x141631140; const P6: u64 = 0x14162fb20; const P7: u64 = 0x141524320; const P8: u64 = 0x141630640; const P9: u64 = 0x141630060; const P10: u64 = 0x14162fdf0; const P11: u64 = 0x141624450; const P12: u64 = 0x1416258c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), (0x1u32) as u32); r12 = ((rdx as u32)) as u32 as u64; rbp = (rcx) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rax.wrapping_add(0x28u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rax.wrapping_add(0x28u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rax.wrapping_add(0x28u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r13) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((r12 as u32)) as u32 as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xc0u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 23; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xc4u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xccu64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xc4u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xccu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xd4u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x90u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = ((rbp.wrapping_add(0x48u64)) as u64) as u64; r9 = (rdi) as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 22; continue; }
            20 => { rdx = ((rbp.wrapping_add(0x18u64)) as u64) as u64; r9 = (rdi) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 22; continue; }
            21 => { rdx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; r9 = (rdi) as u64; r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rax as u32)) as u32); rcx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 24; continue; }
            24 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 25; continue; }
            25 => { fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x14u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 41; continue; } bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(r15)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 28; continue; }
            28 => { bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (rsi) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((r13 as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rax) as u64); bb = 32; continue; }
            31 => { crate::simd_ops::wu64(r15, (rax) as u64); bb = 32; continue; }
            32 => { fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), (r15) as u64); bb = 34; continue; }
            34 => { crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x30u64), (1) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r15)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 36; continue; }
            36 => { crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (r13) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r13 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r13) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r13 as u32)) as u32); bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r13 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (r13) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((r13 as u32)) as u32); bb = 40; continue; }
            40 => { rdx = (rbx) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 41; continue; }
            41 => { fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 73; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 57; continue; } bb = 43; continue; }
            43 => { rsi = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 44; continue; }
            44 => { bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (rsi) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((r13 as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 47; continue; } bb = 46; continue; }
            46 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); bb = 48; continue; }
            47 => { crate::simd_ops::wu64(r14, (rax) as u64); bb = 48; continue; }
            48 => { fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 50; continue; } bb = 49; continue; }
            49 => { crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), (r14) as u64); bb = 50; continue; }
            50 => { crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x18u64), (1) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 52; continue; }
            52 => { crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (r13) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r13 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r13) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r13 as u32)) as u32); bb = 54; continue; }
            54 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r13 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (r13) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((r13 as u32)) as u32); bb = 56; continue; }
            56 => { rdx = (rbx) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 57; continue; }
            57 => { fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 73; continue; } bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 73; continue; } bb = 59; continue; }
            59 => { rsi = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 73; continue; } bb = 60; continue; }
            60 => { bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (rsi) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x10u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((r13 as u32)) as u32); fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 63; continue; } bb = 62; continue; }
            62 => { crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), (rax) as u64); bb = 64; continue; }
            63 => { crate::simd_ops::wu64(r14, (rax) as u64); bb = 64; continue; }
            64 => { fl_l = (rsi) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 66; continue; } bb = 65; continue; }
            65 => { crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (r14) as u64); bb = 66; continue; }
            66 => { crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x48u64), (1) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 68; continue; }
            68 => { crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), (r13) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 69; continue; }
            69 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((r13 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r13) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r13 as u32)) as u32); bb = 70; continue; }
            70 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 72; continue; } bb = 71; continue; }
            71 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r13 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (r13) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((r13 as u32)) as u32); bb = 72; continue; }
            72 => { rdx = (rbx) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rsi) as u64 as u64; fl_r = (rsi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 61; continue; } bb = 73; continue; }
            73 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 111; continue; } bb = 74; continue; }
            74 => { rbp = (0x1u32) as u32 as u64; bb = 75; continue; }
            75 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc0u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(0x3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 108; continue; } bb = 76; continue; }
            76 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xc4u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 78; continue; } bb = 77; continue; }
            77 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xccu64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 108; continue; } bb = 78; continue; }
            78 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xc4u64))) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 80; continue; } bb = 79; continue; }
            79 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xccu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xd4u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 108; continue; } bb = 80; continue; }
            80 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x90u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 99; continue; } bb = 81; continue; }
            81 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 91; continue; } bb = 82; continue; }
            82 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 109; continue; } bb = 83; continue; }
            83 => { r13 = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 85; continue; } bb = 84; continue; }
            84 => { rdx = ((rax as u32)) as u32 as u64; rcx = ((r15.wrapping_add(0x18u64)) as u64) as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rbp as u32) } else { (rdx as u32) }) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 88; continue; } bb = 85; continue; }
            85 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 88; continue; } bb = 86; continue; }
            86 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x20u64), ((rax as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 89; continue; } bb = 87; continue; }
            87 => { crate::simd_ops::wu64(rcx, (r13) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), (r13) as u64); bb = 89; continue; }
            88 => { r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((r13 as u32)) as u32 as u64; bb = 89; continue; }
            89 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), (rcx) as u64); crate::simd_ops::lk_add_u32(r15.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 109; continue; } bb = 90; continue; }
            90 => { r15 = (crate::simd_ops::ru64(r15)) as u64; bb = 109; continue; }
            91 => { r13 = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 93; continue; } bb = 92; continue; }
            92 => { rdx = ((rax as u32)) as u32 as u64; rcx = ((r14.wrapping_add(0x18u64)) as u64) as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rbp as u32) } else { (rdx as u32) }) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 96; continue; } bb = 93; continue; }
            93 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 96; continue; } bb = 94; continue; }
            94 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x20u64), ((rax as u32)) as u32); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 97; continue; } bb = 95; continue; }
            95 => { crate::simd_ops::wu64(rcx, (r13) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), (r13) as u64); bb = 97; continue; }
            96 => { r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((r13 as u32)) as u32 as u64; bb = 97; continue; }
            97 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), (rcx) as u64); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 109; continue; } bb = 98; continue; }
            98 => { r14 = (crate::simd_ops::ru64(r14)) as u64; bb = 109; continue; }
            99 => { r12 = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 101; continue; } bb = 100; continue; }
            100 => { rdx = ((rax as u32)) as u32 as u64; rcx = ((rsi.wrapping_add(0x18u64)) as u64) as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rbp as u32) } else { (rdx as u32) }) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 105; continue; } bb = 101; continue; }
            101 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x24u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 105; continue; } bb = 102; continue; }
            102 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x18u64)))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 104; continue; } bb = 103; continue; }
            103 => { r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rcx, (r13) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0xcu64), (r13) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (r13) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r13) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), (r13) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), (r13) as u64); bb = 106; continue; }
            104 => { r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 106; continue; }
            105 => { r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((r13 as u32)) as u32 as u64; bb = 106; continue; }
            106 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xd0u64))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), (rcx) as u64); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 109; continue; } bb = 107; continue; }
            107 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; bb = 109; continue; }
            108 => { crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), (r13) as u64); bb = 109; continue; }
            109 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 75; continue; } bb = 110; continue; }
            110 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; bb = 111; continue; }
            111 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 114; continue; } bb = 112; continue; }
            112 => { bb = 113; continue; }
            113 => { crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), (r13) as u64); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 113; continue; } bb = 114; continue; }
            114 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rdx = ((r12 as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 128; continue; } bb = 115; continue; }
            115 => { bb = 116; continue; }
            116 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x2cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 121; continue; } bb = 117; continue; }
            117 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 119; continue; } bb = 118; continue; }
            118 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 119; continue; }
            119 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(2u32)) as u64; crate::simd_ops::wu32(r15.wrapping_add(0x2cu64), ((r13 as u32)) as u32); crate::simd_ops::wu64(r15.wrapping_add(0x30u64), (r13) as u64); r8 = ((rcx.wrapping_add(0x10u64)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(r15.wrapping_add(0x30u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 128; continue; } bb = 120; continue; }
            120 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x2cu64), ((rax as u32)) as u32); bb = 121; continue; }
            121 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x10u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x18u64))) as u64; r14 = (crate::simd_ops::ru64(r15.wrapping_add(0x30u64))) as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 127; continue; } bb = 122; continue; }
            122 => { bb = 123; continue; }
            123 => { crate::simd_ops::wu64(rdi, (r14) as u64); r9 = (r14) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; r8 = (rdi) as u64; rdx = ((r12 as u32)) as u32 as u64; rcx = (rbx) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 125; continue; } bb = 124; continue; }
            124 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 126; continue; }
            125 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 126; continue; }
            126 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; rbx = ((rbx).wrapping_add(0x30u64)) as u64; r14 = ((r14.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 123; continue; } bb = 127; continue; }
            127 => { r15 = (crate::simd_ops::ru64(r15)) as u64; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 116; continue; } bb = 128; continue; }
            128 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rdx = ((r12 as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 130; continue; } bb = 129; continue; }
            129 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 129; continue; } bb = 130; continue; }
            130 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rbx = (r13) as u64; r8 = (r13) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 143; continue; } bb = 131; continue; }
            131 => { bb = 132; continue; }
            132 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xc0u64))) as u32 as u64; rdx = (rcx) as u64; rax = (((rax as u32)).wrapping_sub(0x3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 136; continue; } bb = 133; continue; }
            133 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xc4u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 135; continue; } bb = 134; continue; }
            134 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0xccu64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 136; continue; } bb = 135; continue; }
            135 => { r8 = (rcx) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; bb = 142; continue; }
            136 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 138; continue; } bb = 137; continue; }
            137 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), (rax) as u64); bb = 139; continue; }
            138 => { crate::simd_ops::wu64(r8.wrapping_add(0x60u64), (rax) as u64); bb = 139; continue; }
            139 => { fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 141; continue; } bb = 140; continue; }
            140 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (r8) as u64); bb = 141; continue; }
            141 => { crate::simd_ops::lk_sub_u32(rbp, (1) as u32); rcx = (rax) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r13 } else { rbx }) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), (rbx) as u64); rbx = (rdx) as u64; bb = 142; continue; }
            142 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 132; continue; } bb = 143; continue; }
            143 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 148; continue; } bb = 144; continue; }
            144 => { bb = 145; continue; }
            145 => { rdi = (rbx) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xbcu64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xbcu64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 147; continue; } bb = 146; continue; }
            146 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rdi) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 147; continue; }
            147 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 145; continue; } bb = 148; continue; }
            148 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141632050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426bf8; const P1: u64 = 0x143426d10; const P2: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm2) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm7) as u128); xmm7 = (xmm3) as u128; r9 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; xmm1 = (xmm3) as u128; xmm4 = (xmm3) as u128; rax = ((rdx as u32)) as u32 as u64; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp, (0x0u32) as u32); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); xmm5 = 0u128; rdx = ((rax.wrapping_mul(4)) as u64) as u64; rax = r8; xmm0 = (crate::simd_ops::ru128(rsp)) as u128; rcx = ((rdx.wrapping_add(r8)) as u64) as u64; xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0xe1u8); xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm7 & 0xffff_ffffu128); xmm2 = crate::simd_ops::vshufps(xmm2, xmm2, 0x0u8); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0xc6u8); xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x0u8); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x27u8); xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm4 & 0xffff_ffffu128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x39u8); xmm2 = crate::simd_ops::vaddps(xmm2, xmm0); crate::simd_ops::wu128(r8, (xmm2) as u128); r8 = ((r8).wrapping_add(0x10u64)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { xmm0 = (xmm3) as u128; xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(rax)); rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu128(r8, (xmm0) as u128); r8 = ((r8).wrapping_add(0x10u64)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { xmm2 = (crate::simd_ops::ru128(rax)) as u128; bb = 5; continue; }
            5 => { crate::simd_ops::lk_add_u64(r9, (rdx) as u64); xmm2 = crate::simd_ops::vaddps(xmm2, xmm3); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm5 = (xmm2) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm1 = (xmm5) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(xmm6 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); xmm0 = (xmm5) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(rax)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rax)) }).to_bits() as u128); crate::simd_ops::wu32(rax, ((xmm1 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141632150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141624420; const P1: u64 = 0x1416258c0; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rbp = 0; fl_l = 0; fl_r = 0; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbp); bb = 3; continue; }
            3 => { crate::simd_ops::lk_sub_u32(rsi, (1) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xbcu64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(0xbcu64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(rsi, ((rbp as u32)) as u32); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rbx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbp as u32)) as u32); rdi = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rbp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rbp as u32)) as u32); bb = 15; continue; }
            15 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((rbp as u32)) as u32); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rbx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbp as u32)) as u32); rdi = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rbp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rbp as u32)) as u32); bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rbp as u32)) as u32); bb = 24; continue; }
            24 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rbp); crate::simd_ops::wu32(rsi.wrapping_add(0x30u64), ((rbp as u32)) as u32); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { rbx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbp as u32)) as u32); rdi = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rbp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rbp as u32)) as u32); bb = 31; continue; }
            31 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rbp as u32)) as u32); bb = 33; continue; }
            33 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x50u64), rbp); crate::simd_ops::wu32(rsi.wrapping_add(0x48u64), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141632370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141632390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141632cf0; const P1: u64 = 0x142690db0; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; r14 = rdx; rdx = ((rax.wrapping_add(rax)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r8 = P1; r10 = (crate::simd_ops::ru64(r14)) as u64; r9 = 0x12d; rcx = r14; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_add(0x5u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rbp as u32)) as u32); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0x5; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rbp as u32)) as u32); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rbp); rax = 0x34; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 13; continue; }
            12 => { rax = 0x1; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141632490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428720; const P1: u64 = 0x142690e10; const P2: u64 = 0x1434268d0; const P3: u64 = 0x8; const P4: u64 = 0x143427008; const P5: u64 = 0x143426460; const P6: u64 = 0x4; const P7: u64 = 0x1c; const P8: u64 = 0x20; const P9: u64 = 0x2c; const P10: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); xmm5 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32) as u128; xmm0 = (xmm5) as u128; xmm0 = (xmm0) & (xmm1); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm0 = (xmm4) as u128; xmm0 = (xmm0) & (xmm1); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm5 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm4 = 0u128; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P4)) as u32) as u128; crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P5)) as u32) as u128; bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; xmm0 = (xmm5) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P7)))).to_bits() as u128); xmm3 = (xmm4) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P8)))).to_bits() as u128); xmm2 = (xmm4) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P9)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm5) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P10)))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x0u8); xmm1 = crate::simd_ops::vrcpps(xmm0); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rdx.wrapping_mul(4)), ((xmm3 as u32)) as u32); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(r9.wrapping_mul(4)), ((xmm2 as u32)) as u32); bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0x40u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 10; continue; }
            10 => { xmm7 = (crate::simd_ops::ru128(rsp)) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416325a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427008; const P1: u64 = 0x142690b38;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = r8; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r10 = rdx; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); r8 = rax.wrapping_add(0x14); xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm10) as u128); xmm10 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm11) as u128); xmm11 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); bb = 2; continue; }
            2 => { xmm2 = (xmm9) as u128; xmm0 = (xmm10) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_sub(0x8u64)))).to_bits() as u128); xmm1 = (xmm8) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_sub(0x4u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(r8))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { xmm3 = ((crate::simd_ops::ru64(r10)) as u64) as u128; xmm2 = (xmm8) as u128; xmm4 = (xmm3) as u128; xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x8u64)))).to_bits() as u128); xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x20u64)))).to_bits() as u128); xmm4 = crate::simd_ops::vshufps(xmm4, xmm4, 0x55u8); xmm5 = (xmm4) as u128; xmm6 = (xmm4) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x14u64)))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x18u64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x1cu64)))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm3) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x10u64)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0xcu64)))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x24u64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0x28u64)))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm6) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm11 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x0u8); xmm1 = crate::simd_ops::vrcpps(xmm0); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(rdx.wrapping_mul(4)), ((xmm5 as u32)) as u32); rdx = (crate::simd_ops::ru32(r8.wrapping_sub(0x10u64))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(rdx.wrapping_mul(4)), ((xmm0 as u32)) as u32); rdx = (crate::simd_ops::ru32(r8.wrapping_sub(0xcu64))) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(rdx.wrapping_mul(4)), ((xmm1 as u32)) as u32); bb = 8; continue; }
            8 => { rax = ((rax).wrapping_add(0x40u64)) as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141632760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14342698c; const P1: u64 = 0x14267564c; const P2: u64 = 0x143428720; const P3: u64 = 0x142675648; const P4: u64 = 0x142675640; const P5: u64 = 0x142675644; const P6: u64 = 0x14183acc2; const P7: u64 = 0x1416325a0; const P8: u64 = 0x141632490; const P9: u64 = 0x141632f70;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; xmm4 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rsi = rcx; xmm5 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm6 = (xmm2) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; rdi = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm8 = (xmm1) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm3 = (xmm5) as u128; bb = 3; continue; }
            2 => { xmm3 = (xmm4) as u128; bb = 3; continue; }
            3 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm9 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm10 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm3) as u128; xmm0 = (xmm0) & (xmm9); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm1) as u128; xmm2 = (xmm2) & (xmm9); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm5 = (xmm4) as u128; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm5) as u128; xmm0 = (xmm5) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm0 = (xmm0) & (xmm9); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm8) as u128; xmm0 = (xmm0) & (xmm9); xmm7 = (xmm1) as u128; xmm7 = (xmm7) & (xmm9); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm8) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm8 = (xmm1) as u128; xmm8 = (xmm8) & (xmm9); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm6) as u128; xmm1 = (xmm1) & (xmm9); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm10 = (xmm2) as u128; xmm10 = (xmm10) & (xmm9); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = rdi; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((r8).wrapping_shl(2u32)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { xmm0 = 0u128; r8 = rbx; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rsi.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm7 as u32)) as u32); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm10 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            9 => { xmm0 = (xmm7) as u128; rcx = rsi.wrapping_add(0x18); xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm8); rdx = ((xmm0 as u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rdx = rbx; rcx = ((rdi as u32)) as u32 as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141632a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434264e4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbx = rdx; rdi = ((((r8 as u32)) as i32 as i64) as u64) as u64; rdx = (((crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as i32 as i64) as u64) as u64; rsi = rcx; r10 = ((((r9 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm14) as u128); xmm14 = ((crate::simd_ops::ru32(rbx.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(r10.wrapping_mul(8)))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rsp, (xmm15) as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm15 = ((crate::simd_ops::ru32(rbx.wrapping_add(rdi.wrapping_mul(8)))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            2 => { r10 = (crate::simd_ops::ru32(rsi)) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; r8 = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { (r9 as u32) } else { (r8 as u32) }) as u32 as u64; r9 = (if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { (rdx as u32) } else { (r9 as u32) }) as u32 as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm6) as u128); rdx = rbx; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm9) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm10) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm11) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm12) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm13) as u128); r11 = ((((r8 as u32)) as i32 as i64) as u64) as u64; rsi = ((((r9 as u32)) as i32 as i64) as u64) as u64; bb = 4; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm0 = (xmm14) as u128; xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm6 = (xmm1) as u128; xmm11 = ((crate::simd_ops::ru32(rbx.wrapping_add(r11.wrapping_mul(8)))) as u32) as u128; xmm7 = (xmm2) as u128; xmm4 = ((crate::simd_ops::ru32(rbx.wrapping_add(rsi.wrapping_mul(8)))) as u32) as u128; xmm8 = (xmm11) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm3 = (xmm4) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm9 = (xmm15) as u128; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm13 = (xmm15) as u128; xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm8) as u128; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm12 = (xmm14) as u128; xmm12 = (xmm12 & !0xffff_ffffu128) | ((f32::from_bits(xmm12 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; xmm10 = (xmm0) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rsi.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; xmm5 = (xmm0) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm10) as u128; xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm11) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 9; continue; }
            9 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 10; continue; }
            10 => { xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 11; continue; }
            11 => { xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; xmm15 = (crate::simd_ops::ru128(rsp)) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141632cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = 0x5; rdx = ((r15 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r9 = (0xffffffffffffffecu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); r9 = ((r9).wrapping_sub(rsi)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rbp = ((r12 as u32)) as u32 as u64; fl_l = rdi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r11 = rdi.wrapping_sub(0x4); r14 = 0x6c; r11 = ((r11).wrapping_shr(2u32)) as u64; r10 = 0xac; rdx = rsi.wrapping_add(0x14); r14 = ((r14).wrapping_sub(rsi)) as u64; r10 = ((r10).wrapping_sub(rsi)) as u64; fl_l = ((r11).wrapping_add(1)) as u64; r11 = ((fl_l as u64)) as u64; rbp = ((r11.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = rdx.wrapping_sub(0x14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu32(rdx, ((r12 as u32)) as u32); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((rdx.wrapping_add(r9)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r8))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(r8).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r8).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(r8).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); rcx = rdx.wrapping_add(0x2c); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx.wrapping_add(0x38u64), r12); crate::simd_ops::wu32(rdx.wrapping_add(0x40u64), ((r12 as u32)) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x70u64))) as u128; r8 = rdx.wrapping_add(0x6c); crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), r12); crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), ((r12 as u32)) as u32); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r14.wrapping_add(rdx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); r8 = rdx.wrapping_add(0xac); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdx.wrapping_add(0xb8u64), r12); crate::simd_ops::wu32(rdx.wrapping_add(0xc0u64), ((r12 as u32)) as u32); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = ((rdx).wrapping_add(0x100u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r10))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(r10).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r10).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(r10).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 19; continue; }
            19 => { fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rdx = rbp; rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(0x14u64)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = rdi; fl_r = rbp; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { r8 = rdx.wrapping_sub(0x14); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rdx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu32(rdx, ((r12 as u32)) as u32); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r9.wrapping_add(rdx)) as u64) as u64; rdx = ((rdx).wrapping_add(0x40u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rsi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r15 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141632f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; r10 = 0; fl_l = 0; fl_r = 0; r9 = ((rcx as u32)) as u32 as u64; xmm3 = 0u128; xmm1 = 0u128; r8 = ((r10 as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp, rdi); rdi = 0xfffffff8; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { r11 = ((rcx as u32)) as u32 as u64; xmm2 = 0u128; fl_l = ((((r11 as u32)) & ((rdi as u32)))) as u32 as u64; fl_r = fl_l; r11 = ((fl_l as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = ((r8 as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128; rax = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0x8u32)) as u32 as u64; xmm2 = crate::simd_ops::vaddps(xmm2, xmm0); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128; xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 4; continue; }
            4 => { xmm2 = crate::simd_ops::vaddps(xmm2, xmm1); xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((xmm2 >> 64) & 0xffff_ffff_ffff_ffffu128); xmm1 = crate::simd_ops::vaddps(xmm1, xmm2); xmm0 = (xmm1) as u128; xmm0 = crate::simd_ops::vshufps(xmm0, xmm1, 0xf5u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 5; continue; }
            5 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rax = (((rax as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rbx = ((r8 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = ((r9 as u32)) as u32 as u64; rcx = rdx.wrapping_add(0x8); rax = (((rax as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = ((rcx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; r8 = ((r8.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rbx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            8 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x8u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x4u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = ((r9 as u32)) as u32 as u64; rcx = ((rdx.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            11 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; bb = 13; continue; }
            13 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 27; continue; } bb = 14; continue; }
            14 => { xmm2 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm2) as u128; xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x0u8); fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { r8 = ((r9 as u32)) as u32 as u64; r11 = 0x8; fl_l = ((((r8 as u32)) & (0xfffffff0u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; bb = 16; continue; }
            16 => { xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(r10.wrapping_mul(4)))) as u128; rax = ((r11.wrapping_sub(0x4u64)) as u32) as u32 as u64; xmm0 = crate::simd_ops::vmulps(xmm0, xmm3); crate::simd_ops::wu128(rdx.wrapping_add(r10.wrapping_mul(4)), (xmm0) as u128); r10 = (((r10 as u32)).wrapping_add(0x10u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128; xmm0 = crate::simd_ops::vmulps(xmm0, xmm3); crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(4)), (xmm0) as u128); rax = ((r11 as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128; xmm0 = crate::simd_ops::vmulps(xmm0, xmm3); crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(4)), (xmm0) as u128); rax = ((r11.wrapping_add(0x4u64)) as u32) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(0x10u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rax.wrapping_mul(4)))) as u128; xmm0 = crate::simd_ops::vmulps(xmm0, xmm3); crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(4)), (xmm0) as u128); fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 17; continue; }
            17 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 18; continue; }
            18 => { rax = ((r9 as u32)) as u32 as u64; r11 = ((r10 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 22; continue; } bb = 19; continue; }
            19 => { rax = ((r9 as u32)) as u32 as u64; rcx = rdx.wrapping_add(0x8); rax = (((rax as u32)).wrapping_sub((r10 as u32))) as u32 as u64; rcx = ((rcx.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; rax = (((rax as u32)).wrapping_sub(0x4u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r10 = ((r10.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; r11 = ((r11.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; bb = 20; continue; }
            20 => { rcx = rcx.wrapping_add(0x10); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x18u64)))).to_bits() as u128); xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x14u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_sub(0x18u64), ((xmm0 as u32)) as u32); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x10u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_sub(0x14u64), ((xmm1 as u32)) as u32); xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_sub(0x10u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_sub(0xcu64), ((xmm1 as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { r8 = ((r9 as u32)) as u32 as u64; rcx = ((rdx.wrapping_add(r11.wrapping_mul(4))) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 23; continue; }
            23 => { rcx = rcx.wrapping_add(0x4); xmm0 = (xmm2) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_sub(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_sub(0x4u64), ((xmm0 as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rax = ((r9.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = (((rax) & (rdi))) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = crate::simd_ops::vsqrtps(xmm0); crate::simd_ops::wu128(rdx, (xmm1) as u128); rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 27; continue; }
            27 => { rdi = (crate::simd_ops::ru64(rsp)) as u64; bb = 28; continue; }
            28 => { rsp = ((rsp).wrapping_add(0x8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416331b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143428760; const P1: u64 = 0x143426c2c; const P2: u64 = 0x1434270c8; const P3: u64 = 0x143426d88; const P4: u64 = 0x14342698c; const P5: u64 = 0x143427094; const P6: u64 = 0x143427104; const P7: u64 = 0x141835533; const P8: u64 = 0x14183ad94; const P9: u64 = 0x14183ada0; const P10: u64 = 0x141633590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rbp = rsp.wrapping_add(0x20); r9 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r12 = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0xf8u64), rbx); r13 = rdx; crate::simd_ops::wu128(rbp.wrapping_add(0xa0u64), (xmm6) as u128); rcx = (0xffffffffffffff0u64) as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x90u64), (xmm7) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x80u64), (xmm8) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x70u64), (xmm9) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x60u64), (xmm10) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x50u64), (xmm11) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x40u64), (xmm12) as u128); r9 = ((r9).wrapping_shl(3u32)) as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm13) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm14) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm15) as u128); rax = r9.wrapping_add(0xf); fl_l = rax; fl_r = r9; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = rcx; bb = 2; continue; }
            2 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = r9.wrapping_add(0xf); r15 = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp, r15); fl_l = rax; fl_r = r9; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rcx; bb = 4; continue; }
            4 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; xmm9 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rdx = r12.wrapping_add(0x28); xmm10 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rsp = ((rsp).wrapping_sub(rax)) as u64; xmm12 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm13 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm14 = ((crate::simd_ops::ru32(P5)) as u32) as u128; r14 = rsp.wrapping_add(0x20); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 5; continue; }
            5 => { xmm15 = ((crate::simd_ops::ru32(P6)) as u32) as u128; rdi = r14.wrapping_add(0x4); rax = r15; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r14; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r15 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xf0u64), rax); bb = 6; continue; }
            6 => { xmm6 = ((crate::simd_ops::ru32(rsi.wrapping_add(r13).wrapping_add(0x4u64))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm10 as u32)) as u32 as u64; xmm7 = ((crate::simd_ops::ru32(rsi.wrapping_add(r13).wrapping_add(0x8u64))) as u32) as u128; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm15 as u32)).to_bits() as u128); bb = 10; continue; }
            8 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm12 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); bb = 10; continue; }
            10 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm13 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm12 as u32)).to_bits() as u128); bb = 14; continue; }
            12 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm14 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(r12.wrapping_add(0x28u64))) as u64; xmm1 = (xmm7) as u128; xmm2 = (xmm6) as u128; xmm0 = (xmm1) as u128; xmm1 = (xmm2) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(rbx).wrapping_add(0x8u64), ((xmm0 as u32)) as u32); xmm3 = (xmm0) as u128; xmm3 = crate::simd_ops::vshufps(xmm3, xmm0, 0xeu8); xmm2 = (xmm0) as u128; xmm2 = crate::simd_ops::vshufps(xmm2, xmm0, 0x1u8); xmm1 = (xmm3) as u128; xmm1 = crate::simd_ops::vshufps(xmm1, xmm3, 0x1u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsi.wrapping_add(rbx), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(rbx).wrapping_add(0x4u64), ((xmm3 as u32)) as u32); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(rdi), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(rdi).wrapping_sub(0x4u64), ((xmm6 as u32)) as u32); rax = (crate::simd_ops::ru64(r12.wrapping_add(0x28u64))) as u64; xmm8 = ((crate::simd_ops::ru32(rsi.wrapping_add(rax))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(rsi.wrapping_add(rax).wrapping_add(0x8u64))) as u32) as u128; xmm8 = (xmm8) ^ (xmm9); xmm7 = ((crate::simd_ops::ru32(rsi.wrapping_add(rax).wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm8) as u128; xmm0 = (xmm6) as u128; xmm7 = (xmm7) ^ (xmm9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (xmm7) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm9 = (xmm0) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_sub(0x4u64), ((xmm9 as u32)) as u32); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32)).sqrt().to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm7) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm10 as u32)) as u32 as u64; crate::simd_ops::wu32(rdi, ((xmm0 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); bb = 18; continue; }
            16 => { fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm12 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rdi.wrapping_sub(0x4u64), ((xmm9 as u32)) as u32); bb = 19; continue; }
            19 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm13 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); bb = 23; continue; }
            21 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm14 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rdi, ((xmm0 as u32)) as u32); bb = 24; continue; }
            24 => { xmm9 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rsi = ((rsi).wrapping_add(0xcu64)) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 25; continue; }
            25 => { r15 = (crate::simd_ops::ru64(rbp)) as u64; rdx = r12.wrapping_add(0x28); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(r12)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)), (0xb33bbd2eu32) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64), (0x80000000u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8u64), (0xbf800000u32) as u32); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(rax.wrapping_mul(8)), (0x0u32) as u32); crate::simd_ops::wu32(r15.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x4u64), (0xbfc90fdbu32) as u32); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; xmm8 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8u64))) as u32) as u128; xmm8 = (xmm8) ^ (xmm9); xmm7 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm8) as u128; xmm0 = (xmm6) as u128; xmm7 = (xmm7) ^ (xmm9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (xmm7) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm9 = (xmm0) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(r14.wrapping_add(rbx.wrapping_mul(8)), ((xmm9 as u32)) as u32); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32)).sqrt().to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm7) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm10 as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x4u64), ((xmm0 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); bb = 30; continue; }
            28 => { fl_l = ((xmm9 as u32)) as u32 as u64; fl_r = ((xmm12 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 31; continue; } bb = 29; continue; }
            29 => { xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); bb = 30; continue; }
            30 => { crate::simd_ops::wu32(r14.wrapping_add(rbx.wrapping_mul(8)), ((xmm9 as u32)) as u32); bb = 31; continue; }
            31 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm13 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); bb = 35; continue; }
            33 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm14 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 36; continue; } bb = 34; continue; }
            34 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); bb = 35; continue; }
            35 => { crate::simd_ops::wu32(r14.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x4u64), ((xmm0 as u32)) as u32); bb = 36; continue; }
            36 => { r9 = r12.wrapping_add(0x18); rdx = r15; r8 = r12.wrapping_add(0x8); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = r12.wrapping_add(0x18); rdx = r14; r8 = r12.wrapping_add(0x8); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf8u64))) as u64; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0xa0u64))) as u128; xmm7 = (crate::simd_ops::ru128(rbp.wrapping_add(0x90u64))) as u128; xmm8 = (crate::simd_ops::ru128(rbp.wrapping_add(0x80u64))) as u128; xmm9 = (crate::simd_ops::ru128(rbp.wrapping_add(0x70u64))) as u128; xmm10 = (crate::simd_ops::ru128(rbp.wrapping_add(0x60u64))) as u128; xmm11 = (crate::simd_ops::ru128(rbp.wrapping_add(0x50u64))) as u128; xmm12 = (crate::simd_ops::ru128(rbp.wrapping_add(0x40u64))) as u128; xmm13 = (crate::simd_ops::ru128(rbp.wrapping_add(0x30u64))) as u128; xmm14 = (crate::simd_ops::ru128(rbp.wrapping_add(0x20u64))) as u128; xmm15 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; rsp = rbp.wrapping_add(0xb0); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141633590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690e14; const P1: u64 = 0x143428760; const P2: u64 = 0x142690b38; const P3: u64 = 0x1434268d0; const P4: u64 = 0x141632a70; const P5: u64 = 0x141632cf0; const P6: u64 = 0x1434266dc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; r10 = (crate::simd_ops::ru32(rcx)) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; r13 = r9; r14 = rcx; rax = ((r10.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm6) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm7) as u128); xmm7 = 0u128; crate::simd_ops::wu128(r11.wrapping_sub(0x68u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x78u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x88u64), (xmm10) as u128); xmm10 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x98u64), (xmm11) as u128); xmm11 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0xa8u64), (xmm12) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0xb8u64), (xmm13) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0xc8u64), (xmm14) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm15) as u128); bb = 2; continue; }
            2 => { rax = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = ((r10.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 36; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 35; continue; } bb = 5; continue; }
            5 => { rax = ((r10.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            9 => { r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rdi = r13; r9 = ((rsi as u32)) as u32 as u64; rdi = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { r8 } else { rdi }) as u64; rcx = r14; r8 = ((rbp as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r15 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r15 as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rcx = 0x1; rdx = (((rdx as u8)) as u32) as u32 as u64; rdx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; bb = 17; continue; }
            17 => { rax = ((rax).wrapping_add(0x40u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 18; continue; }
            18 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = 0x1; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; r9 = ((r9).wrapping_shl(6u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((r9).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r9.wrapping_add(0xcu64), r15); crate::simd_ops::wu32(r9.wrapping_add(0x14u64), ((r15 as u32)) as u32); bb = 25; continue; }
            24 => { r9 = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 26; continue; }
            26 => { xmm0 = 0u128; r8 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu128(r9, (xmm0) as u128); rcx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu128(r9.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu128(r9.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(r9.wrapping_add(0x30u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r8 = (if (((fl_l as u32)) != ((fl_r as u32))) { (rbp as u32) } else { (r8 as u32) }) as u32 as u64; crate::simd_ops::wu32(r9, ((r8 as u32)) as u32); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) != ((fl_r as u32))) { (rbx as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(r9.wrapping_add(0x4u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; xmm6 = ((crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)))) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)))) as u32) as u128; xmm2 = (xmm6) as u128; xmm5 = ((crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm3) as u128; xmm4 = ((crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm10 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 33; continue; } bb = 28; continue; }
            28 => { xmm1 = (xmm11) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm3 = (xmm3) ^ (xmm9); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm4 = (xmm4) ^ (xmm9); crate::simd_ops::wu32(r9.wrapping_add(0x1cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x20u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x24u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x28u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x2cu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu128(r9.wrapping_add(0x30u64), (xmm7) as u128); bb = 33; continue; }
            29 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rax = ((rbp as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rsi as u32)) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x8u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm10 = (xmm0) as u128; xmm4 = ((crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)))) as u32) as u128; xmm6 = (xmm2) as u128; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x150u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x8u64))) as u32) as u128; xmm9 = (xmm4) as u128; xmm3 = ((crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm12 = (xmm4) as u128; xmm5 = ((crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)))) as u32) as u128; xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm8 = (xmm5) as u128; rax = ((rbx as u32)) as u32 as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm11 = (xmm5) as u128; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); r10 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm3 as u32)) as u32); xmm13 = ((crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)))) as u32) as u128; xmm14 = ((crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x8u64))) as u32) as u128; xmm3 = (xmm9) as u128; xmm15 = ((crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm6) as u128; xmm11 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm1 as u32)) as u32); xmm12 = (xmm12 & !0xffff_ffffu128) | ((f32::from_bits(xmm12 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm8) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm2 = (xmm11) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm0 = (xmm10) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm12) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P2)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { xmm5 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x150u64))) as u32) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm12 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32) as u128; xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) / f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm14) as u128; xmm3 = (xmm13) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm2 = (xmm14) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); xmm4 = (xmm15) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x30u64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm1 = (xmm15) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4))))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm12) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm13 as u32)).to_bits() as u128); xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm15) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); crate::simd_ops::wu32(r9, ((rbp as u32)) as u32); xmm15 = (xmm15 & !0xffff_ffffu128) | ((f32::from_bits(xmm15 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x30u64)))).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((rbx as u32)) as u32); xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm14 = (xmm14 & !0xffff_ffffu128) | ((f32::from_bits(xmm14 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm14 = (xmm14 & !0xffff_ffffu128) | ((f32::from_bits(xmm14 as u32) - f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm14 = (xmm14 & !0xffff_ffffu128) | ((f32::from_bits(xmm14 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x1cu64), ((xmm14 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x20u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x24u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x28u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x2cu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x30u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x38u64), ((xmm13 as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0x3cu64), ((xmm9 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x8u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x8u64)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x8u64)))).to_bits() as u128); xmm12 = ((crate::simd_ops::ru32(P6)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4))))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm4 = (xmm1) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm2); crate::simd_ops::wu64(r9.wrapping_add(0xcu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(0x14u64), ((xmm1 as u32)) as u32); xmm1 = (xmm1) as u128; xmm3 = ((crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(r9.wrapping_add(0xcu64))) as u32) as u128; xmm0 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm5 = (xmm6) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4))))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r10.wrapping_mul(4)).wrapping_add(0x8u64)))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm6) as u128; xmm0 = (xmm3) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4))))).to_bits() as u128); xmm1 = (xmm4) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(r8.wrapping_mul(4)).wrapping_add(0x8u64)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x8u64)))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((if f32::from_bits(xmm5 as u32) < f32::from_bits(xmm6 as u32) { f32::from_bits(xmm5 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((xmm5 as u32)) as u32); bb = 32; continue; }
            32 => { xmm11 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm9 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm8 = ((crate::simd_ops::ru32(P0)) as u32) as u128; bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(r14)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; r10 = ((rcx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; bb = 35; continue; }
            35 => { rsi = ((rax as u32)) as u32 as u64; r10 = ((rcx as u32)) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 36; continue; }
            36 => { rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x2u64)) as u32) as u32 as u64; r10 = ((rcx as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 37; continue; }
            37 => { xmm15 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm14 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm13 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0xc0u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0xd0u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xf0u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x100u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; bb = 38; continue; }
            38 => { rsp = ((rsp).wrapping_add(0x130u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141633c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x5; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rsi as u32)) as u32); rcx = ((rsi.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x5; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P5), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rsi as u32)) as u32); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(P8), rsi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141633c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141633cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x60; const P3: u64 = 0x64; const P4: u64 = 0x1415af330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rcx, rdx); rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P1), r8); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(r8.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = r8; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32); bb = 7; continue; }
            7 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141633d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = rcx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141633d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x60; const P2: u64 = 0x64; const P3: u64 = 0x1415af340; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141633dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x60; const P2: u64 = 0x64; const P3: u64 = 0x1415af340; const P4: u64 = 0x1415242e0; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rdx; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rcx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = 0u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P5), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsi, (xmm0) as u128); crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141633e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415af330; const P1: u64 = 0x1415af340; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x0u32) as u32); rcx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x100u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x20); rdi = 0x1; bb = 12; continue; }
            9 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 10; continue; }
            10 => { rcx = rsp.wrapping_add(0x20); rdi = 0x1; bb = 12; continue; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x38); xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rbx = 0; fl_l = 0; fl_r = 0; rdi = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu128(rcx, (xmm0) as u128); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = ((((rdi as u32)) & (0xfffffffdu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            17 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            18 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 31; continue; }
            30 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            31 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 32; continue; }
            32 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 36; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x114;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = rdx; return rax; }
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    rax = rdx;
    fl_l = (crate::simd_ops::lk_and_u32(rdx, (0xffu32) as u32)) as u32 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1b0; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r9 = rcx;
    rbx = r8;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(r8)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rdi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(r8, rdi);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::wu16(rbx.wrapping_add(P4), ((rdi as u16)) as u16); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu16(rbx.wrapping_add(P4), ((rdi as u16)) as u16);
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416340d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1b8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
    crate::simd_ops::wu32(r9, (0xffffffffu32) as u32);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c9c80; const P1: u64 = 0x14; const P2: u64 = 0x18; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rdx;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rdi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx, rdi);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::wu16(rbx.wrapping_add(P3), ((rdi as u16)) as u16); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu16(rbx.wrapping_add(P3), ((rdi as u16)) as u16);
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c9f00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } crate::simd_ops::wu32(r8, (0xffffffffu32) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416341c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xa; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = (((rcx as u8)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416341f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x143930f78; const P3: u64 = 0x1415f37e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx, rdx);
    rdx = r8;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x8; const P4: u64 = 0x40; const P5: u64 = 0x4c; const P6: u64 = 0x58; const P7: u64 = 0x60; const P8: u64 = 0x68; const P9: u64 = 0x70; const P10: u64 = 0x78; const P11: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_and_u32(rcx.wrapping_add(P0), (0xffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu16(rcx.wrapping_add(P5), ((rax as u16)) as u16); crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); crate::simd_ops::wu64(rcx.wrapping_add(P8), rax); crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P10), rax); crate::simd_ops::wu64(rcx.wrapping_add(P11), rax); crate::simd_ops::wu8(rcx.wrapping_add(0x88u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x98u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rax); rax = rcx; crate::simd_ops::wu32(rcx.wrapping_add(0x2cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x30u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x34u64), (0xffffu32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x48u64), (0x3f800000u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x4eu64), (0x1u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x50u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), (0x3f800000u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x90u64), (0x32u8) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0xa4u64), (0x2bu64) as u64); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141634fd0; const P1: u64 = 0x1415242e0; const P2: u64 = 0x14162e180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rsi = rbx.wrapping_add(0xa8); rdx = rbx; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_sub(0x4u64))) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x20u64), (1) as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(0xffffffffffffff40u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbp = ((rbp).wrapping_shl(6u32)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), r14); bb = 9; continue; }
            9 => { rcx = rbp.wrapping_sub(0xc0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 11; continue; }
            10 => { rbx = ((rbx).wrapping_add(0xc0u64)) as u64; crate::simd_ops::wu64(rsi.wrapping_sub(0x10u64), r14); crate::simd_ops::wu64(rsi.wrapping_sub(0x4u64), (0x11u64) as u64); fl_l = ((rsi).wrapping_add(0xc0u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 13; continue; }
            13 => { rdx = rbx; rsi = rbx.wrapping_add(0xa8); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_sub(0x4u64))) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 15; continue; }
            15 => { rax = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(0xffffffffffffff40u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbp = ((rbp).wrapping_shl(6u32)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), r14); bb = 19; continue; }
            19 => { rcx = rbp.wrapping_sub(0xc0); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            20 => { rbx = ((rbx).wrapping_add(0xc0u64)) as u64; crate::simd_ops::wu64(rsi.wrapping_sub(0x10u64), r14); crate::simd_ops::wu64(rsi.wrapping_sub(0x4u64), (0x11u64) as u64); fl_l = ((rsi).wrapping_add(0xc0u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416344c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268e670;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rsi = rdx; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rbx as u32)) as u32); r14 = rcx; r15 = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x14u64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xcdu32) as u32); rdx = r10; r9 = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = 0x2; bb = 13; continue; }
            4 => { r9 = 0xcc; r8 = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rbp); crate::simd_ops::wu32(r14.wrapping_add(0x14u64), ((r15 as u32)) as u32); bb = 9; continue; }
            9 => { crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((r15 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; rdx = ((rcx.wrapping_mul(8)) as u64) as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(rax), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(rax).wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 12; continue; }
            12 => { rax = 0x1; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416345c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415dc3f0; const P1: u64 = 0x141634dd0; const P2: u64 = 0x143815b50; const P3: u64 = 0x1416344c0; const P4: u64 = 0x141634f50; const P5: u64 = 0x14162f2f0; const P6: u64 = 0x141634c40; const P7: u64 = 0x141634250;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbp = rsp.wrapping_add(0x30); r15 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rsi = r9; rdi = (crate::simd_ops::ru64(rdx)) as u64; r14 = r8; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r13 = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rbx); crate::simd_ops::wu128(rbp.wrapping_add(0x60u64), (xmm6) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x50u64), (xmm7) as u128); fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 1; continue; }
            1 => { r8 = (((rax as u8)) as u32) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xdu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_shr(12u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = ((((rdx as u32)) & (0x637u32))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = (((rax as u8)) as u32) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; fl_l = 0; fl_r = ((rax as u32)) as u32 as u64; rax = (0u32.wrapping_sub(fl_r as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) < ((fl_r as u32))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(12u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x100u32))) as u32 as u64; r8 = ((((r8 as u32)) | ((rcx as u32)))) as u32 as u64; r8 = ((((r8 as u32)) | ((rdx as u32)))) as u32 as u64; rdx = (0xffffffffffffff0u64) as u64; rbx = (((r8 as u8)) as u32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp, rbx); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_add(0xfu64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = rdx; bb = 5; continue; }
            5 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = ((rbx.wrapping_mul(4)) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_add(0xfu64)) as u64; r12 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r12); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = rdx; bb = 7; continue; }
            7 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(0x8u64))) as u128; rdx = ((crate::simd_ops::ru8(r9.wrapping_add(0x4cu64))) as u32) as u32 as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; xmm1 = (crate::simd_ops::ru128(r9.wrapping_add(0x18u64))) as u128; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x28u64))) as u32 as u64; rcx = rbp.wrapping_add(0x20); xmm3 = ((crate::simd_ops::ru32(r9.wrapping_add(0x2cu64))) as u32) as u128; r15 = rsp.wrapping_add(0x30); crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32) as u128; xmm0 = 0u128; xmm7 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32) as u128; r8 = ((crate::simd_ops::ru8(rsi.wrapping_add(0x90u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0xb8u64), ((r8 as u8)) as u8); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rbx) as i64) as f32).to_bits() as u128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x0u8); xmm1 = crate::simd_ops::vrsqrtps(xmm0); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; r9 = 0x3; r8 = (((rdi as u8)) as u32) as u32 as u64; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rax.wrapping_add(0x58); rax = P2; rdx = rsi.wrapping_add(0x58); crate::simd_ops::wu64(rcx, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rdx = ((crate::simd_ops::ru16(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(r14.wrapping_add(0x12u64))) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(r15)) as u32) as u128; rax = (((rdi as u8)) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu16(rbx.wrapping_add(0xa8u64), ((rdx as u16)) as u16); crate::simd_ops::wu16(rbx.wrapping_add(0xaau64), ((r8 as u16)) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), (0x4101u32) as u32); rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rcx); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa4u64), ((rax as u32)) as u32); rax = (((rdi as u8)) as u32) as u32 as u64; fl_l = ((rax).wrapping_shl(2u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = ((crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; rax = (crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xb8u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x90u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x54u64), ((xmm7 as u32)) as u32); bb = 14; continue; }
            14 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; r15 = ((r15).wrapping_add(0x4u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(r13)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rdi)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rdi = ((rdi).wrapping_add(0xc0u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 18; continue; }
            18 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x68u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), (0x0u64) as u64); crate::simd_ops::wu32(rsi.wrapping_add(0x6cu64), (0x0u32) as u32); bb = 55; continue; }
            21 => { rdx = ((crate::simd_ops::ru16(r14.wrapping_add(0x12u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; xmm2 = ((crate::simd_ops::ru32(r15)) as u32) as u128; crate::simd_ops::wu16(rdi.wrapping_add(0xa8u64), ((rcx as u16)) as u16); rcx = rdi.wrapping_add(0x58); crate::simd_ops::wu16(rdi.wrapping_add(0xaau64), ((rdx as u16)) as u16); rdx = rsi.wrapping_add(0x58); crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0xa0u64), (0x4101u32) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xa4u64), ((rax as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(r12.wrapping_add(0x4u64))) as u32) as u128; rax = (crate::simd_ops::ru32(r12)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x54u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x90u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((xmm2 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = 0x1; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 55; continue; } bb = 22; continue; }
            22 => { r9 = r15.wrapping_add(0x4); r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), r9); rdx = P2; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(r13)) as u64; rax = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; r10 = (((r12 as u8)) as u32) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r10); rcx = ((rcx).wrapping_shl(56u32)) as u64; rcx = (((rcx) | (crate::simd_ops::ru64(rsi)))) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rbx = ((rbx).wrapping_add(0xc0u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 27; continue; }
            27 => { bb = 35; continue; }
            28 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 34; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rdx); rcx = rbx.wrapping_add(0x58); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x8u64))) as u128; rdx = rdi.wrapping_add(0x58); crate::simd_ops::wu128(rbx.wrapping_add(0x8u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x18u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x28u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x38u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), ((xmm0 as u64)) as u64); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x88u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x88u64), (0x0u8) as u8); bb = 33; continue; }
            33 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x78u64), rax); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x90u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x90u64), ((rax as u8)) as u8); bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((crate::simd_ops::ru16(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru16(r14.wrapping_add(0x12u64))) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(r9)) as u32) as u128; crate::simd_ops::wu16(rbx.wrapping_add(0xa8u64), ((rdx as u16)) as u16); rdx = P2; crate::simd_ops::wu16(rbx.wrapping_add(0xaau64), ((r8 as u16)) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0xa0u64), (0x4101u32) as u32); rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rcx); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xa4u64), ((rax as u32)) as u32); rax = (((r10 as u8)) as u32) as u32 as u64; fl_l = ((rax).wrapping_shl(2u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = ((crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x8u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x4u64))) as u32) as u128; rax = (crate::simd_ops::ru32(r15.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xb8u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x90u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x54u64), ((xmm7 as u32)) as u32); bb = 35; continue; }
            35 => { r9 = ((r9).wrapping_add(0x4u64)) as u64; r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), r9); fl_l = ((r12 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 23; continue; } bb = 36; continue; }
            36 => { bb = 55; continue; }
            37 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; rdx = rsi; rcx = rdi; crate::simd_ops::wu128(rdi.wrapping_add(0x98u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            39 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; r9 = 0x3; r8 = 0; fl_l = 0; fl_r = 0; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 40; continue; }
            40 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; rdx = rsi; rcx = rax; crate::simd_ops::wu128(rax.wrapping_add(0x98u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0xa8u64), ((xmm1 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            41 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 54; continue; } bb = 42; continue; }
            42 => { r12 = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), r12); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xcu64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 44; continue; } bb = 43; continue; }
            43 => { r9 = ((r15 as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; bb = 53; continue; }
            44 => { rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 47; continue; } bb = 45; continue; }
            45 => { rdx = ((rax as u32)) as u32 as u64; r12 = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r12 as u32) } else { (rdx as u32) }) as u32 as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 52; continue; } bb = 46; continue; }
            46 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; bb = 47; continue; }
            47 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r13.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 52; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(0x18u64))) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rdi).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 50; continue; }
            50 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 51; continue; }
            51 => { crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu32(rdi.wrapping_add(0xb0u64), ((r15 as u32)) as u32); bb = 53; continue; }
            52 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 53; continue; }
            53 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 54; continue; }
            54 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x98u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x8u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x18u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x28u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x38u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), ((xmm0 as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); bb = 55; continue; }
            55 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; rax = rdi; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x60u64))) as u128; xmm7 = (crate::simd_ops::ru128(rbp.wrapping_add(0x50u64))) as u128; rsp = rbp.wrapping_add(0x70); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141634250; const P2: u64 = 0x141634fd0; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14162e180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r12 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rdi = rbx; rdi = ((rdi).wrapping_sub(r14)) as u64; rcx = rbx; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xb8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0xb8u64), r12); bb = 10; continue; }
            10 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xc0u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 11; continue; }
            11 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rsi, r14); crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xc; const P2: u64 = 0x33; const P3: u64 = 0x141634c40; const P4: u64 = 0x1c; const P5: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32); rbx = rcx; rdi = 0x1; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rsi = ((rdi.wrapping_add(P2)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = ((rsi as u32)) as u32 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rdi as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rcx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = 0x64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rsi = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rdi as u32) } else { (rsi as u32) }) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) != ((fl_r as u32))) { (rsi as u32) } else { (rdi as u32) }) as u32 as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x141634c40; const P3: u64 = 0x1; const P4: u64 = 0x141634250; const P5: u64 = 0xb0; const P6: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = ((r9 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r14 = rdx; rbp = (((r8 as u8)) as u32) as u32 as u64; rdi = rcx; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(6u32)) as u64; rax = ((rax.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rax as u32)) as u32); fl_l = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = rbp; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rsi as u32)) as u32); rcx = ((rcx).wrapping_shl(56u32)) as u64; rcx = (((rcx) | (r14))) as u64; crate::simd_ops::wu64(rbx, rcx); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(P6), (1) as u32)) as u32 as u64; bb = 9; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xb8; const P2: u64 = 0x1415242e0; const P3: u64 = 0x14162e180; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0xc0u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsi, rbp); crate::simd_ops::wu32(rsi.wrapping_add(P4), ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141634e90; const P1: u64 = 0x10; const P2: u64 = 0x20;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x0u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rbp = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rbp as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(P0), rbp); crate::simd_ops::wu64(r14.wrapping_add(P1), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141634fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162f2f0; const P1: u64 = 0x98; const P2: u64 = 0xa8; const P3: u64 = 0xb0; const P4: u64 = 0xa4; const P5: u64 = 0xb8; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P1))) as u128;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu128(rdi.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(rdi.wrapping_add(P2), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rsi);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x2bu64) as u64);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rsi);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r11 = rdx; r14 = ((crate::simd_ops::ru8(r9.wrapping_add(0x4cu64))) as u32) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(12u32)) as u32 as u64; rsi = rcx; r10 = ((r8 as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; r10 = ((((r10 as u32)) & (0xffff7u32))) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_sub((r10 as u32))) as u32 as u64; xmm0 = 0u128; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; r10 = (((r10 as u32)).wrapping_shl(12u32)) as u32 as u64; rdx = (((rcx as u8)) as u32) as u32 as u64; rdx = ((((rdx as u32)) | ((rax as u32)))) as u32 as u64; rdx = ((((rdx as u32)) | ((r10 as u32)))) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rbp = ((((rbp as u32)) & (0xf00u32))) as u32 as u64; crate::simd_ops::wu128(r11, (xmm0) as u128); fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x48u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x100; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(12u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0x63fu32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu8(r11.wrapping_add(0xcu64), ((rbx as u8)) as u8); rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(0xc0u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 10; continue; }
            10 => { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { bb = 19; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r9)) as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 15; continue; }
            15 => { rax = ((rax).wrapping_add(0xc0u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(6u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx).wrapping_add(0xc0u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 19; continue; }
            19 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { rax = rcx; bb = 21; continue; }
            21 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = 0x1; crate::simd_ops::wu64(r11, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x8u64), ((rcx as u32)) as u32); bb = 32; continue; }
            23 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x60u64))) as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r9 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x3840009; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(r11.wrapping_add(0x8u64), ((rcx as u32)) as u32); bb = 28; continue; }
            28 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(r10.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(r11.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu32(r11.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru32(r11.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; r8 = (((rdx as u8)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 34; continue; }
            34 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu32(r11.wrapping_add(0x8u64), (0x3u32) as u32); bb = 48; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(r10.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 40; continue; } bb = 37; continue; }
            37 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 38; continue; }
            38 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = (((rdi as u8)) as u32) as u32 as u64; bb = 41; continue; }
            40 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(r10.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu8(r11.wrapping_add(0xdu64), ((rax as u8)) as u8); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 47; continue; } bb = 42; continue; }
            42 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 47; continue; } bb = 43; continue; }
            43 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 47; continue; } bb = 44; continue; }
            44 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x100; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 47; continue; } bb = 45; continue; }
            45 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rdi = 0x2; bb = 47; continue; }
            47 => { crate::simd_ops::wu32(r11.wrapping_add(0x8u64), ((rdi as u32)) as u32); bb = 48; continue; }
            48 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = r11; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416352a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xb0; const P2: u64 = 0x98; const P3: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r10 = rdx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; r9 = ((rax.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx), r9); rdx = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64; crate::simd_ops::wu64(r8.wrapping_add(rdx), rax); fl_l = ((r8).wrapping_add(0x8u64)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0xc0u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416352f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690ee0; const P1: u64 = 0x141633d10; const P2: u64 = 0x142690fa0; const P3: u64 = 0x142691018;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), (0xc5800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdi); rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x38u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rdi); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x60u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf9u8))) as u8 as u64)); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdi); rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x1u8))) as u8 as u64)); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x60u64), ((rax as u8)) as u8); rax = P0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rdi); rcx = ((rcx).wrapping_add(0x70u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x98u64), (0x80u8) as u8); rax = P2; crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x99u64), (0xe0u8) as u8); crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x60u64), (0x4u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x90u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x94u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rbx, rax); rax = P3; crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xb0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xb8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xc8u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xccu64), (0x1000000u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xd0u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0xd2u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xe8u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xf0u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xf8u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xfcu64), (0x1000000u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x100u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x102u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x108u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x110u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x118u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x120u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x128u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x12cu64), (0x1000000u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x130u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x132u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x138u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x140u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x148u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x150u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x158u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x15cu64), (0x1000000u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x160u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x162u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x168u64), ((rdi as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x168u64), (0xffu32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x18cu64), ((rdi as u8)) as u8); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x18cu64), (0xffu32) as u32); crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x1b4u64), (0xfcu8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x170u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x178u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x180u64), (0x20100ffu32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x184u64), (0x3u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x188u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x190u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x198u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x1a0u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x1a8u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x1b0u64), ((rdi as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416352f0; const P1: u64 = 0x1426910e8; const P2: u64 = 0x142691160; const P3: u64 = 0xa0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
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
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690fa0; const P1: u64 = 0x142691018; const P2: u64 = 0x1416383a0; const P3: u64 = 0x1416384a0; const P4: u64 = 0x1415242e0; const P5: u64 = 0x141646970; const P6: u64 = 0x141635770; const P7: u64 = 0x14163b610; const P8: u64 = 0x14268dac0; const P9: u64 = 0x142690ee0; const P10: u64 = 0x141633d40; const P11: u64 = 0x1416440f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); r14 = rcx; rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(0xa0u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u8(r14.wrapping_add(0x1b4u64), (0xfcu8) as u8); rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x198u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = ((r15.wrapping_add(0x5u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14.wrapping_add(0x198u64), r15); bb = 2; continue; }
            2 => { rcx = r14.wrapping_add(0x1a0); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x190u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0xe; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14.wrapping_add(0x190u64), r15); bb = 4; continue; }
            4 => { rsi = 0x4; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbx = r14.wrapping_add(0x168); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x30u64))) as u64; rbx = ((rbx).wrapping_sub(0x30u64)) as u64; rsi = ((rsi).wrapping_sub(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, r15); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r15 as u32)) as u32); bb = 7; continue; }
            7 => { rbp = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r15 as u32)) as u32); bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 10; continue; }
            10 => { rax = P8; crate::simd_ops::wu64(r14.wrapping_add(0xa0u64), rax); rcx = r14.wrapping_add(0x70); rax = P9; crate::simd_ops::wu64(r14, rax); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426910e8; const P1: u64 = 0x142691160; const P2: u64 = 0xa0; const P3: u64 = 0x141645320; const P4: u64 = 0x1416383a0; const P5: u64 = 0x8; const P6: u64 = 0x1b4; const P7: u64 = 0x58; const P8: u64 = 0x10; const P9: u64 = 0x1415242e0; const P10: u64 = 0x170; const P11: u64 = 0x14163b610; const P12: u64 = 0x141635540;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::lk_and_u8(rbx.wrapping_add(P6), (0xfcu8) as u8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x5; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x170u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x170u64), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x17cu64), (0x0u32) as u32); bb = 6; continue; }
            6 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x5; const P1: u64 = 0x1415242e0; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x30; const P5: u64 = 0x28; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rsi.wrapping_add(P0)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P2), rsi); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rsi); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rsi); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = 0x50; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142690ee0; const P1: u64 = 0x141633d40; const P2: u64 = 0x1416440f0; const P3: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0x70u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xa0; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416358d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426910e8; const P1: u64 = 0x142691160; const P2: u64 = 0xa0; const P3: u64 = 0x141645320; const P4: u64 = 0x1416383a0; const P5: u64 = 0x8; const P6: u64 = 0x1b4; const P7: u64 = 0x58; const P8: u64 = 0x10; const P9: u64 = 0x1415242e0; const P10: u64 = 0x170; const P11: u64 = 0x14163b610; const P12: u64 = 0x141635540; const P13: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rsi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); rbx = rcx; rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::lk_and_u8(rbx.wrapping_add(P6), (0xfcu8) as u8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x5; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x170u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x170u64), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x17cu64), (0x0u32) as u32); bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0x1c0; rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416359c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158fa60; const P1: u64 = 0x141834d30;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xa0; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141638ec0; const P1: u64 = 0x143931de0; const P2: u64 = 0x1438152b4; const P3: u64 = 0x1416399c0; const P4: u64 = 0x1415242e0; const P5: u64 = 0x14162e180; const P6: u64 = 0x141635770;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; r15 = ((r8 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; rbp = rcx; r14 = 0x1; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r14 as u32) } else { (rdx as u32) }) as u32 as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = ((rdi).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x0u8) as u8); crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x8u64), (0xffu32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x24u64), ((r12 as u8)) as u8); crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x24u64), (0xffu32) as u32); crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu64(rdi.wrapping_add(0xcu64), (0x2bu64) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), r12); crate::simd_ops::wu8(rdi.wrapping_add(0x20u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), r12); let xa_src: u64 = r14; let xa_old: u64 = crate::simd_ops::atomic_xadd_u64(P1, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r14 = (xa_old) as u64; r9 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rdx = r14.wrapping_add(0x1); rax = (0xffffffffffffffu64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r15 as u32)) as u32); rdx = (((rdx) & (rax))) as u64; r8 = ((rbx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = rdi; bb = 14; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x10u64)))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_sub(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0x5; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x30u64), r12); crate::simd_ops::wu16(rbx.wrapping_sub(0x1eu64), ((r12 as u16)) as u16); bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_sub(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_sub(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0xe; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), r12); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x181; const P1: u64 = 0xb8; const P2: u64 = 0xc;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    r11 = (((rdx as u8)) as u32) as u32 as u64;
    rbx = rcx;
    rax = ((((r11 as u8)) as i8 as i64) as u64) as u64;
    rax = (((crate::simd_ops::ru8(rax.wrapping_add(rcx).wrapping_add(P0))) as i8 as i32) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64;
    if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { rax = (((r11 as u8)) as u32) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u32 as u64; fl_r = 0x11; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
    loop { r11 = (((rax as u8)) as u32) as u32 as u64; rdx = ((((rax as u8)) as i8 as i64) as u64) as u64; rax = (((crate::simd_ops::ru8(rdx.wrapping_add(rcx).wrapping_add(P0))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if !(((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32))) { break; } }
    rax = (((r11 as u8)) as u32) as u32 as u64;
    rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1))) as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P2))) as u32 as u64; fl_r = 0x11;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141635b80; const P1: u64 = 0x141638530; const P2: u64 = 0x14163c8a0; const P3: u64 = 0x1416382e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = rcx; rax = (((r8 as u8)) as u32) as u32 as u64; r13 = rdx; crate::simd_ops::wu8(r14, (0x0u8) as u8); rcx = ((crate::simd_ops::ru16(rdx.wrapping_add(0x1b0u64))) as u32) as u32 as u64; r15 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r15 = ((r15).wrapping_shl(4u32)) as u64; r15 = ((r15).wrapping_add(rdx)) as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0xcfu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru16(r15.wrapping_add(0xd4u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = rsi.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r9); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x24u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rsi.wrapping_add(0x2cu64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((((fl_l as u16)) & ((fl_r as u16))) as i16) > 0) { bb = 20; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; r14 = ((((r8 as u8)) as i8 as i64) as u64) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; rdi = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rdx = (((crate::simd_ops::ru8(r14.wrapping_add(r13).wrapping_add(0x181u64))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = ((rbx as u32)) as u32 as u64; rcx = r13; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0xb8u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rdi).wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x11; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 13; continue; }
            13 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rsi.wrapping_add(0x10); rdx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rdi = ((rdi).wrapping_sub(0x30u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } bb = 16; continue; }
            16 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 49; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 49; continue; } bb = 19; continue; }
            19 => { bb = 48; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 49; continue; } bb = 21; continue; }
            21 => { bb = 48; continue; }
            22 => { r10 = (r10 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((r10 as u8)) as u8); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x27u64))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 49; continue; } bb = 24; continue; }
            24 => { rax = ((((rbx as u32)) as i32 as i64) as u64) as u64; rdi = ((rax.wrapping_mul(8)) as u64) as u64; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 29; continue; } bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x88u64))) as u64; bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 28; continue; }
            28 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 29; continue; }
            29 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r10 = (r10 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            30 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = ((rdi).wrapping_sub(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 25; continue; } bb = 31; continue; }
            31 => { bb = 45; continue; }
            32 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 49; continue; } bb = 33; continue; }
            33 => { rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rbp = ((((r8 as u8)) as i8 as i64) as u64) as u64; r12 = (((r8 as u8)) as u32) as u32 as u64; rdi = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((fl_l as u64)) as u64; r14 = ((rcx.wrapping_mul(8)) as u64) as u64; bb = 34; continue; }
            34 => { rdx = (((crate::simd_ops::ru8(rbp.wrapping_add(r13).wrapping_add(0x181u64))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 36; continue; } bb = 35; continue; }
            35 => { r8 = ((rbx as u32)) as u32 as u64; rcx = r13; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r10 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; bb = 37; continue; }
            36 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0xb8u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rdi).wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x11; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 37; continue; }
            37 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 39; continue; }
            39 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(rdx))) as u64; r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x88u64))) as u64; bb = 40; continue; }
            40 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 41; continue; }
            41 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 40; continue; } bb = 42; continue; }
            42 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x10); rdx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r10 = (r10 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((r10 as u8)) as u8); bb = 43; continue; }
            43 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = ((rdi).wrapping_sub(0x30u64)) as u64; r14 = ((r14).wrapping_sub(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 34; continue; } bb = 44; continue; }
            44 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 45; continue; }
            45 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 49; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 49; continue; } bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x27u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(r14, ((rcx as u8)) as u8); bb = 48; continue; }
            48 => { rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141635e90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc2; const P1: u64 = 0x14163c730;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((xmm3 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; xmm4 = ((crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32) as u128; xmm5 = 0u128; xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; rbp = rcx; xmm3 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64) as u128; xmm0 = (xmm2) as u128; xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm6 = 0u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm7 = (xmm2) as u128; xmm2 = (crate::simd_ops::ru128(r8.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm8 = (xmm1) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); xmm10 = (xmm4) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm12) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x88u64), (xmm13) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x98u64), (xmm14) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xa8u64), (xmm15) as u128); xmm15 = ((crate::simd_ops::ru32(r8.wrapping_add(0x10u64))) as u32) as u128; xmm14 = ((crate::simd_ops::ru32(r8)) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32) as u128; xmm12 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; xmm13 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm3 as u64)) as u64); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) * f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm10 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm4) as u128; xmm4 = ((crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm3 as u64) - f64::from_bits(xmm4 as u64)).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm4 as u64)) as u64); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm15) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm4 = crate::simd_ops::vcvtpd2ps(xmm3); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm15) as u128; xmm0 = (xmm1) as u128; xmm1 = crate::simd_ops::vunpckhpd(xmm1, xmm1); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm2 = crate::simd_ops::vunpckhpd(xmm2, xmm2); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) - f64::from_bits(xmm2 as u64)).to_bits() as u128); xmm2 = (xmm8) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | (((f64::from_bits(xmm0 as u64)) as f32).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | (((f64::from_bits(xmm1 as u64)) as f32).to_bits() as u128); xmm0 = (xmm10) as u128; xmm1 = (xmm7) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm0 = (xmm6) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((xmm2 as u32)) as u32); xmm2 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm5 = ((crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm5) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((xmm3 as u32)) as u32); xmm3 = (xmm9) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm2 as u32)) as u32); xmm6 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32) as u128; xmm0 = (xmm13) as u128; xmm4 = ((crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32) as u128; xmm2 = (xmm13) as u128; rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x170u64))) as u32 as u64; xmm1 = (xmm12) as u128; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x160u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x178u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); rcx = (((rbx as u8)) as u32) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); rcx = (((rcx as u32)).wrapping_add(0x7u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffff8u32))) as u32 as u64; rax = (((rdi as u8)) as u32) as u32 as u64; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm15) as u128; xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm15 = (xmm15 & !0xffff_ffffu128) | ((f32::from_bits(xmm15 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; xmm0 = (xmm4) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm9 = ((crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm3 as u32)) as u32); xmm3 = ((crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32) as u128; xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm9) as u128; xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm5) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((xmm2 as u32)) as u32); xmm13 = (xmm13 & !0xffff_ffffu128) | ((f32::from_bits(xmm13 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm8 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm8) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm9) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm14 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((xmm13 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm15 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) + f32::from_bits(xmm4 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((xmm2 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm9 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((xmm9 as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((rcx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((r8).wrapping_shl(2u32)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x168u64))) as u32 as u64; rcx = rbp.wrapping_sub(0xa0); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x158u64))) as u32) as u128; rdx = rsp.wrapping_add(0x50); xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x150u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x148u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x120); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x70u64))) as u128; xmm13 = (crate::simd_ops::ru128(r11.wrapping_sub(0x80u64))) as u128; xmm14 = (crate::simd_ops::ru128(r11.wrapping_sub(0x90u64))) as u128; xmm15 = (crate::simd_ops::ru128(r11.wrapping_sub(0xa0u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416361f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x14183acc2; const P2: u64 = 0x141835533; const P3: u64 = 0x143427a60; const P4: u64 = 0x14163c730;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); xmm0 = 0u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); rbp = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); xmm7 = (xmm3) as u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); xmm8 = (xmm2) as u128; xmm9 = (xmm1) as u128; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xf0u64))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as u32 as u64; rcx = (((rbx as u8)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x7u32)) as u32 as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rcx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((r8).wrapping_shl(2u32)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { xmm0 = 0u128; xmm1 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm8 & 0xffff_ffffu128); xmm1 = (xmm1 & !0xffff_ffffu128) | (xmm9 & 0xffff_ffffu128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; rcx = rbp.wrapping_sub(0xa0); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbx as u32)) as u32); rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); xmm3 = (xmm0) as u128; xmm3 = crate::simd_ops::vshufps(xmm3, xmm0, 0xeu8); xmm2 = (xmm0) as u128; xmm1 = (xmm3) as u128; xmm2 = crate::simd_ops::vshufps(xmm2, xmm0, 0x1u8); xmm3 = crate::simd_ops::vshufps(xmm3, xmm3, 0x1u8); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x110u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x6cu64), ((xmm0 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm3 as u32)) as u32); xmm3 = (xmm7) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x3f800000u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; bb = 8; continue; }
            7 => { rax = 0x1f; bb = 8; continue; }
            8 => { r11 = rsp.wrapping_add(0xc0); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141636370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf; const P1: u64 = 0x10; const P2: u64 = 0x141515eb0; const P3: u64 = 0x141572890;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rbx); r14 = rdx; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rsi); rsi = ((r9 as u32)) as u32 as u64; rdi = ((r9 as u32)) as u32 as u64; rax = ((rsi.wrapping_mul(4)) as u64) as u64; r10 = ((rax.wrapping_add(P0)) as u64) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = (0xffffffffffffff0u64) as u64; bb = 2; continue; }
            2 => { r10 = (((r10) & (0xfffffffffffffff0u64))) as u64; rax = r10; rsp = ((rsp).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rbp, r8); r9 = ((rcx.wrapping_add(P1)) as u64) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rbx = rsp.wrapping_add(0x30); rcx = rbp; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((r14.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rbx); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rdx = ((rsi as u32)) as u32 as u64; rcx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_shr(2u32)) as u64; xmm2 = 0u128; rdi = ((rdi).wrapping_shl(4u32)) as u64; fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; rbx = ((rbx).wrapping_add(0x10u64)) as u64; xmm0 = crate::simd_ops::vmulps(xmm0, xmm0); xmm0 = crate::simd_ops::vaddps(xmm0, xmm2); xmm2 = (xmm0) as u128; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; xmm1 = (xmm0) as u128; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; xmm1 = crate::simd_ops::vshufps(xmm1, xmm0, 0xb1u8); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); xmm0 = (xmm1) as u128; xmm0 = crate::simd_ops::vshufps(xmm0, xmm1, 0x4eu8); xmm0 = crate::simd_ops::vaddps(xmm0, xmm1); rsp = rbp.wrapping_add(0x10); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141636450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691240; const P1: u64 = 0x141636370; const P2: u64 = 0x142691238;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r14 = r8; r15 = ((crate::simd_ops::ru16(r8.wrapping_add(0x10u64))) as u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = (xmm1) as u128; rbp = rcx; xmm6 = 0u128; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x100; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 1; continue; }
            1 => { rdi = (((rdi as u32)).wrapping_shr(12u32)) as u32 as u64; rbx = 0x1; rdi = ((((rdi as u32)) & (0xffff7u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 22; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P0)) as u32) as u128; bb = 3; continue; }
            3 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((rdi as u32)) as u32 as u64; r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = (((r8 as u8)) as u32) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_add(1)) as u8 as u64)); fl_l = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 11; continue; }
            8 => { rcx = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((0x0u8) as u8 as u64)); fl_l = ((((rcx as u32)) & ((rdi as u32)))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add(1)) as u8 as u64)); rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { rcx = ((crate::simd_ops::ru16(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; r9 = ((r15 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rdx = (((rdx as u8)) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x38u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x600; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); bb = 13; continue; }
            13 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 14; continue; }
            14 => { rbx = (((rbx as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 15; continue; }
            15 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 22; continue; }
            16 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x200; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rbx = 0x1; bb = 19; continue; }
            18 => { rbx = (((rdi as u8)) as u32) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; rdi = ((rsi as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rcx = ((crate::simd_ops::ru16(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; r9 = ((r15 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((rsi) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); rdi = ((rdi).wrapping_add(0x90u64)) as u64; fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm0 = 0u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r15) as i64) as f32).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((xmm7 as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141636600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415e9250; const P1: u64 = 0xc8; const P2: u64 = 0x1415dc7d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128);
    rcx = ((rcx).wrapping_add(0xffffffffffffffd0u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128);
    xmm6 = (xmm3) as u128;
    xmm7 = (xmm1) as u128;
    rbx = ((r8 as u32)) as u32 as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rax.wrapping_add(P1)) as u64) as u64; xmm3 = (xmm6) as u128; r8 = ((rbx as u32)) as u32 as u64; xmm1 = (xmm7) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128;
    rax = 0x2;
    xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141636660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415e9250; const P1: u64 = 0xc8; const P2: u64 = 0x1415dc9f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = ((rcx).wrapping_add(0xffffffffffffffd0u64)) as u64;
    rbx = ((r9 as u32)) as u32 as u64;
    rdi = ((r8 as u32)) as u32 as u64;
    rsi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r9 = ((rax.wrapping_add(P1)) as u64) as u64; } else { r9 = 0; fl_l = 0; fl_r = 0; }
    r8 = ((rbx as u32)) as u32 as u64;
    rdx = ((rdi as u32)) as u32 as u64;
    rcx = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}
