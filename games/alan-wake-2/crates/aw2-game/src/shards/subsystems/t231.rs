#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141603910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815b38; const P1: u64 = 0x14158ec20;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; rsi = rdx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r15 = rcx; crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm8) as u128); xmm8 = (xmm3) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x58u64), (xmm9) as u128); xmm9 = (xmm2) as u128; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdi.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32) as u128; bb = 2; continue; }
            2 => { xmm6 = (xmm9) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x10u64)))).to_bits() as u128); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 17; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x1u32) as u32); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 11; continue; }
            11 => { bb = 17; continue; }
            12 => { rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x1u32) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x28u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 18; continue; }
            18 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 19; continue; }
            19 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524350; const P1: u64 = 0x141524320; const P2: u64 = 0x14183acbc; const P3: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; rbx = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (0x18u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = rbx; bb = 5; continue; }
            3 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r9 = 0x10; r8 = rdx; rdx = rcx; rcx = ((r9.wrapping_sub(0xcu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; bb = 5; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { bb = 35; continue; }
            7 => { rcx = 0x4; r8 = ((rcx.wrapping_add(0xcu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { r8 = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u32) as u32 as u64; rdx = rbx; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            9 => { fl_l = rdx; fl_r = (0x18u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdi = rbx; bb = 12; continue; }
            11 => { rcx = 0x4; r8 = ((rcx.wrapping_add(0xcu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 15; continue; }
            15 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); r12 = 0; fl_l = 0; fl_r = 0; r10 = ((r12 as u32)) as u32 as u64; fl_l = rsi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 28; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r9 = rsi.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); rcx = rdi.wrapping_add(0x10); r14 = (0xfffffffffffffff0u64) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; rbp = 0x20; r11 = 0x38; r14 = ((r14).wrapping_sub(rdi)) as u64; rbp = ((rbp).wrapping_sub(rdi)) as u64; r11 = ((r11).wrapping_sub(rdi)) as u64; fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; r10 = ((r9.wrapping_mul(4)) as u64) as u64; bb = 18; continue; }
            18 => { rdx = rcx.wrapping_sub(0x10); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_sub(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rcx, (0x3f800000u32) as u32); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; r8 = ((r14.wrapping_add(rcx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r8))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(r8).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = rcx.wrapping_add(0x8); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), (0x3f800000u32) as u32); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(rax).wrapping_add(0x28u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = rcx.wrapping_add(0x20); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x30u64), (0x3f800000u32) as u32); bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(rbp)) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = rcx.wrapping_add(0x38); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x48u64), (0x3f800000u32) as u32); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(r11)) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx).wrapping_add(0x10u64))) as u64) as u128; rcx = ((rcx).wrapping_add(0x60u64)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 27; continue; }
            27 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 28; continue; }
            28 => { fl_l = r10; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 33; continue; } bb = 29; continue; }
            29 => { rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; fl_l = rsi; fl_r = r10; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rdx = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 30; continue; }
            30 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x3f800000u32) as u32); bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = rdx; rcx = ((rcx).wrapping_sub(rdi)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((r15 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xffffffffffffffe8; const P2: u64 = 0x18; const P3: u64 = 0x28; const P4: u64 = 0xfffffffffffffffd; const P5: u64 = 0x30; const P6: u64 = 0x141590920; const P7: u64 = 0x48; const P8: u64 = 0x40; const P9: u64 = 0x8; const P10: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = rdx; rbx = rcx; rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { r8 = ((rax.wrapping_add(P2)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; rax = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = ((r8).wrapping_sub(rax)) as u64; rdx = ((rax.wrapping_add(P9)) as u64) as u64; r8 = ((((r8 as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P7), (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158c580; const P1: u64 = 0x14158d2f0; const P2: u64 = 0x141603630; const P3: u64 = 0x141603d50; const P4: u64 = 0x14183acc8; const P5: u64 = 0x1416034e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r13 = ((crate::simd_ops::ru8(rcx.wrapping_add(0xe0u64))) as u32) as u32 as u64; rbp = rdx; crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((r13 as u8)) as u8); rdi = rcx; fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xe0u64))) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rax.wrapping_add(0x20); bb = 4; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 4; continue; }
            4 => { rcx = rdi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rcx = rdi; rax = 0x20; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rdi.wrapping_add(0xe0u64), (0x1u8) as u8); r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            6 => { rax = 0x20; bb = 7; continue; }
            7 => { r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r12 = ((rax.wrapping_add(rcx)) as u64) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); r13 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 11; continue; }
            11 => { r11 = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { r8 = ((rax as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((r8 as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; xmm0 = (crate::simd_ops::ru128(r11.wrapping_add(rdx.wrapping_mul(8)))) as u128; rsi = ((r11.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rcx = ((xmm0 as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = rcx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = rcx; fl_r = r10; bb = 15; continue; }
            15 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) < ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { (r9 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; r9 = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 16; continue; }
            16 => { bb = 21; continue; }
            17 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rdx = rsi; rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_sub(rsi)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rsi; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x48u64), (1) as u32)) as u32 as u64; r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 22; continue; }
            22 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r13 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu8(rdi.wrapping_add(0xe0u64), ((r13 as u8)) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), (0x0u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), (0x0u32) as u32); bb = 26; continue; }
            26 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603fb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x48; const P2: u64 = 0x68; const P3: u64 = 0x1416034e0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141603fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141603ac0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; rbx = rcx; rsi = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((rsi as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = ((rsi as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { r8 = rsi; r10 = rdi; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 6; continue; }
            6 => { rax = rsi; r11 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = rax; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 7; continue; }
            7 => { r9 = rsi; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_sub(rdi)) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; r9 = ((r9).wrapping_sub(0x4u64)) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; r10 = ((rdi.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x3f800000u32) as u32); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x3f800000u32) as u32); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(0x30u64)) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x3f800000u32) as u32); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdx = ((rdx).wrapping_add(0x48u64)) as u64; fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x3f800000u32) as u32); bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_add(0x60u64)) as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 17; continue; }
            17 => { fl_l = r10; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = r8; fl_r = r10; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x3f800000u32) as u32); bb = 21; continue; }
            21 => { rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rsi as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141604140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141603420; const P1: u64 = 0x141603ac0; const P2: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; r14 = rdx; rbx = r8; rdx = rsp.wrapping_add(0x20); rdi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = rax; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((r8 as u32)) as u32 as u64; rax = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = rdi; rdx = (((rdx as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rbx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rcx = rbx.wrapping_add(0x18); rdx = rbx; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x3f800000u32) as u32); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x28u64), (1) as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 15; continue; }
            10 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = ((r8 as u32)) as u32 as u64; rax = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = rdi; rdx = (((rdx as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x2cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x0u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x3f800000u32) as u32); bb = 15; continue; }
            15 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); bb = 17; continue; }
            17 => { rax = rcx; bb = 19; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416042c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x14158c580; const P2: u64 = 0x20; const P3: u64 = 0x14158d2f0; const P4: u64 = 0x141603630; const P5: u64 = 0x48; const P6: u64 = 0x40; const P7: u64 = 0x10; const P8: u64 = 0x1416034e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm1) as u128; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u64) as u64; bb = 4; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rbx.wrapping_add(P0), ((rax as u8)) as u8); r8 = (((rax as u8)) as u32) as u32 as u64; bb = 7; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rax.wrapping_add(P7), ((xmm6 as u32)) as u32); rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141604370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe1; const P1: u64 = 0x1416044a0; const P2: u64 = 0x1416034e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rcx = ((rcx).wrapping_add(0x70u64)) as u64; r8 = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416043c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141603800; const P1: u64 = 0x141604140; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x18; const P5: u64 = 0xfffffffffffffffd; const P6: u64 = 0x30; const P7: u64 = 0x1415aaf70; const P8: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rbp = rdx; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rsi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm6 = (xmm2) as u128; rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rbp)) as u128; r8 = rsp.wrapping_add(0x60); rcx = rsi; rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rax; rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P3))) as u32 as u64; r10 = ((r8.wrapping_add(P4)) as u64) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax.wrapping_add(P5)) as u64) as u64; rax = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r10; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(r10)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rsi.wrapping_add(P6)) as u64) as u64; r8 = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(P8), ((xmm6 as u32)) as u32); bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = (((rbx as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416044a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141603800; const P1: u64 = 0x141604140; const P2: u64 = 0x1415aaf70; const P3: u64 = 0x1416044a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; rsi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r15); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; r13 = (r13 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rax = rsi.wrapping_add(0x48); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((r9 as u32)) as u32); r13 = (r13 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rax, ((r9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); bb = 2; continue; }
            2 => { r15 = ((r9 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rdi); rdi = rdx.wrapping_add(0xc); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r12); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm8) as u128); xmm8 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r14); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); r12 = ((r8 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 5; continue; }
            5 => { xmm6 = ((crate::simd_ops::ru32(rdi)) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rdi.wrapping_sub(0xcu64))) as u64; rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { r8 = rsp.wrapping_add(0xd0); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); rdx = rsp.wrapping_add(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = rbp; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rdx = rbx.wrapping_add(0x18); rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = rax.wrapping_sub(0x3); rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rsi.wrapping_add(0x30); rdx = rbp; r8 = rsp.wrapping_add(0xe8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); r13 = (r13 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            14 => { r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { r13 = (r13 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            17 => { fl_l = (((r15 as u32)).wrapping_add(1)) as u32 as u64; r15 = ((fl_l as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xe0u64))) as u32 as u64; rax = rsi.wrapping_add(0x48); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; bb = 20; continue; }
            20 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = rsi; crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rax, ((r9 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rax = (((r13 as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416046a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x1416046e0; const P2: u64 = 0x1416034e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416046e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141590920; const P1: u64 = 0x141603d50; const P2: u64 = 0x14183acc8; const P3: u64 = 0x141604140; const P4: u64 = 0x1415aaf70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r15 = rdx; rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); rax = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); rsi = rbx; r12 = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_sub(rbx)) as u64; rax = r12; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rbx; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbx.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x28u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; bb = 8; continue; }
            7 => { fl_l = ((rbx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rax = ((r14 as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((rax as u32)) as u32 as u64; rax = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 11; continue; }
            11 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r14 as u32)) as u32); rcx = rdi; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { fl_l = rsi; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = rax.wrapping_sub(0x18); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rdx = rax.wrapping_add(0x18); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = rax.wrapping_sub(0x3); rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rdx)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = rdi.wrapping_add(0x30); rdx = rsi; r8 = rsp.wrapping_add(0x78); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 22; continue; }
            21 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x18u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = (((rbp as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141604890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x14158c580; const P2: u64 = 0x20; const P3: u64 = 0x14158d2f0; const P4: u64 = 0x141603630; const P5: u64 = 0x1416043c0; const P6: u64 = 0x1416034e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rbx = rcx; xmm6 = (xmm2) as u128; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P2)) as u64) as u64; bb = 4; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rcx = rbx; rax = 0x20; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x1u8) as u8); rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            6 => { rax = 0x20; bb = 7; continue; }
            7 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rcx = ((rcx).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm2 = (xmm6) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((((rax as u8)) | ((rdi as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141604950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416049d0; const P1: u64 = 0x70; const P2: u64 = 0xc0; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1415242e0; const P6: u64 = 0x1c; const P7: u64 = 0x143931d18; const P8: u64 = 0xe1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdi = ((rbx.wrapping_add(P2)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_l = (crate::simd_ops::ru64(P7)) as u64; fl_r = rbx; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru64(P7)) as u64; fl_r = rbx; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu32(rdi.wrapping_add(P4), (0x0u32) as u32);
    fl_l = rdi; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = 0x4; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rdi.wrapping_add(P3), (0x0u64) as u64);
    crate::simd_ops::wu32(rdi.wrapping_add(P6), (0x0u32) as u32);
    fl_l = (crate::simd_ops::ru64(P7)) as u64; fl_r = rbx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rax as u8)) as u8);
    fl_l = (crate::simd_ops::ru64(P7)) as u64; fl_r = rbx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu8(rbx.wrapping_add(P8), ((rax as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416049d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x4; const P3: u64 = 0x1415242e0; const P4: u64 = 0x2c; const P5: u64 = 0x40; const P6: u64 = 0x18; const P7: u64 = 0x10; const P8: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdi as u32)) as u32); fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rdi as u32)) as u32); fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0x4; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P7), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rdi as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141604a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14268ec08; const P1: u64 = 0x8; const P2: u64 = 0x14392d4b0; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x14392d4b8; const P6: u64 = 0x14392d4a8; const P7: u64 = 0x14268ec18; const P8: u64 = 0x7c; const P9: u64 = 0x3a; const P10: u64 = 0x20; const P11: u64 = 0x28;
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
    crate::simd_ops::lk_add_u32(P6, (1) as u32);
    rax = P7;
    crate::simd_ops::wu64(P2, rcx);
    crate::simd_ops::lk_or_u32(rcx.wrapping_add(P8), (0x1u32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P9), (0xf8u8) as u8)) as u8 as u64; fl_r = fl_l;
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rdx);
    crate::simd_ops::wu16(rcx.wrapping_add(0x38u64), ((rdx as u16)) as u16);
    crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x60u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(0x78u64), (0x3f800000u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141604ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14268ebe8; const P1: u64 = 0x8; const P2: u64 = 0x14392d498; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x14392d4a0; const P6: u64 = 0x14392d490; const P7: u64 = 0x14268ebf8; const P8: u64 = 0x20; const P9: u64 = 0x28; const P10: u64 = 0x30; const P11: u64 = 0x38;
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
    crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141604b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x5; const P2: u64 = 0x1415242e0; const P3: u64 = 0x10; const P4: u64 = 0x8; const P5: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rsi as u16)) as u16); rcx = ((rsi.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rsi); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rsi as u32)) as u32); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rbx = rdx; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = rbx; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rsi as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P4), rsi); crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rsi as u32)) as u32); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141604bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268ec18; const P1: u64 = 0x40; const P2: u64 = 0x48; const P3: u64 = 0x1415242e0; const P4: u64 = 0x4c; const P5: u64 = 0x60; const P6: u64 = 0x50; const P7: u64 = 0x18; const P8: u64 = 0x10; const P9: u64 = 0x1c; const P10: u64 = 0x20; const P11: u64 = 0x141604b30; const P12: u64 = 0x14268ec08; const P13: u64 = 0x14392d4b8; const P14: u64 = 0x14392d4b0; const P15: u64 = 0x14392d4a8; const P16: u64 = 0x142688f88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rsi as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdi = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rsi as u32)) as u32); fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0x4; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi.wrapping_add(P8), rsi); crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rsi as u32)) as u32); bb = 6; continue; }
            6 => { rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = P12; crate::simd_ops::wu64(rbx, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P13)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P13, rax); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(P14)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::lk_sub_u32(P15, (1) as u32); crate::simd_ops::wu64(P14, rax); rax = P16; crate::simd_ops::wu64(rbx, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141604cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268ebf8; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415242e0; const P6: u64 = 0x14268ebe8; const P7: u64 = 0x14392d4a0; const P8: u64 = 0x14392d498; const P9: u64 = 0x14392d490; const P10: u64 = 0x142688f88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = P0; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r14 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(P1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xcu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbp = rdi.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rsi = r12; bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            13 => { rbp = rdi.wrapping_add(0x8); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            15 => { rsi = r12; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; rdi = rsi.wrapping_sub(0x10); rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rdi }) as u64; bb = 52; continue; }
            24 => { rbp = rdi.wrapping_add(0x8); bb = 25; continue; }
            25 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = rsi; fl_r = rsi; rdi = rsi.wrapping_sub(0x10); rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rdi }) as u64; bb = 52; continue; }
            27 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 38; continue; } bb = 28; continue; }
            28 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 29; continue; }
            29 => { rbp = rdi.wrapping_add(0x60); bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 31; continue; }
            31 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 33; continue; } bb = 35; continue; }
            35 => { rsi = r12; bb = 36; continue; }
            36 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 41; continue; }
            38 => { rbp = rdi.wrapping_add(0x60); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 30; continue; } bb = 39; continue; }
            39 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            40 => { rsi = r12; bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 48; continue; } bb = 42; continue; }
            42 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 42; continue; } bb = 44; continue; }
            44 => { bb = 49; continue; }
            45 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 50; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 50; continue; } bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 50; continue; }
            48 => { rbp = rdi.wrapping_add(0x60); bb = 49; continue; }
            49 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 50; continue; }
            50 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 51; continue; }
            51 => { rdi = rsi.wrapping_sub(0x10); bb = 52; continue; }
            52 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            55 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 56; continue; }
            56 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 57; continue; }
            57 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x20u64), rax); fl_l = rdx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 60; continue; } bb = 59; continue; }
            59 => { crate::simd_ops::wu64(r14.wrapping_add(0x28u64), r12); bb = 60; continue; }
            60 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 63; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x40u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 63; continue; } bb = 62; continue; }
            62 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = (crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x44u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rdx); bb = 57; continue; }
            63 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x44u64), (1) as u32)) as u32 as u64; bb = 57; continue; }
            64 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 67; continue; } bb = 66; continue; }
            66 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x40u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 68; continue; } bb = 67; continue; }
            67 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 68; continue; }
            68 => { rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 65; continue; } bb = 69; continue; }
            69 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 71; continue; } bb = 70; continue; }
            70 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14.wrapping_add(0x38u64), r12); bb = 71; continue; }
            71 => { crate::simd_ops::wu32(r14.wrapping_add(0x40u64), ((r12 as u32)) as u32); rax = P6; crate::simd_ops::wu64(r14.wrapping_add(0x20u64), r12); crate::simd_ops::wu64(r14.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(r14.wrapping_add(0x30u64), r12); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(r14, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 73; continue; } bb = 72; continue; }
            72 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); bb = 73; continue; }
            73 => { fl_l = r14; fl_r = (crate::simd_ops::ru64(P7)) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 75; continue; } bb = 74; continue; }
            74 => { crate::simd_ops::wu64(P7, rax); bb = 76; continue; }
            75 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 76; continue; }
            76 => { rax = (crate::simd_ops::ru64(P8)) as u64; fl_l = r14; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::lk_sub_u32(P9, (1) as u32); crate::simd_ops::wu64(P8, rax); rax = P10; crate::simd_ops::wu64(r14, rax); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268dfb0; const P1: u64 = 0x14160c070; const P2: u64 = 0x68; const P3: u64 = 0x70; const P4: u64 = 0x1415242e0; const P5: u64 = 0x74; const P6: u64 = 0x1415af1b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rdx = rsp.wrapping_add(0x20);
    rdi = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32);
    rcx = ((rcx).wrapping_add(0x68u64)) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rsi); crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rsi as u32)) as u32); }
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff0; const P1: u64 = 0x141604bd0; const P2: u64 = 0x141834ee0; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rsi = rcx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rbx = ((rbx).wrapping_shl(7u32)) as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0xffffffffffffff80u64)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = ((rsi.wrapping_add(P0)) as u64) as u64; rdx = ((rdx).wrapping_shl(7u32)) as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rsi.wrapping_add(P0)) as u64) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = 0x80; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rsi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416052a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x60; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x8; const P4: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r9 = rdx; rbx = rcx; r8 = ((rdi as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = (crate::simd_ops::ru64(r11.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = ((r11.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((r8.wrapping_add(P2)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r8 as u32) } else { (rcx as u32) }) as u32 as u64; fl_l = rdx; fl_r = r9; r8 = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = ((rdi.wrapping_add(P2)) as u32) as u32 as u64; rax = ((r11.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_sub(rcx)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14158c580;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14158c580;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = ((rax).wrapping_add(0xc0u64)) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14158c580; const P3: u64 = 0x38; const P4: u64 = 0xd8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; }
    rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P3))) as u32) as u32 as u64;
    rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as i32) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416053b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415f4cb0; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; r15 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r15 = (((r15 as u32)).wrapping_shr(1u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r15 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r15 as u32) }) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rbp.wrapping_add(0x14); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = (0xffffffffffffffecu64) as u64; r14 = ((r14).wrapping_sub(rbp)) as u64; r12 = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rcx = rbx.wrapping_sub(0x14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_sub(0xcu64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x4u64), r12); bb = 8; continue; }
            8 => { rdx = ((r14.wrapping_add(rbx)) as u64) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r12 = 0; fl_l = 0; fl_r = 0; r10 = ((r12 as u32)) as u32 as u64; fl_l = rsi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r9 = rsi.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rdx = rdi.wrapping_add(0x8); r15 = (0xfffffffffffffff8u64) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; r14 = 0x18; r11 = 0x28; r15 = ((r15).wrapping_sub(rdi)) as u64; r14 = ((r14).wrapping_sub(rdi)) as u64; r11 = ((r11).wrapping_sub(rdi)) as u64; fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; r10 = ((r9.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = rdx.wrapping_sub(0x8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rcx, ((r12 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r12); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), (0x42c80000u32) as u32); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r15.wrapping_add(rdx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = rdx.wrapping_add(0x8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rcx, ((r12 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0xcu64), r12); crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), (0x42c80000u32) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x10u64))) as u128; r8 = rdx.wrapping_add(0x18); crate::simd_ops::wu128(rcx, (xmm0) as u128); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r8, ((r12 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x1cu64), r12); crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), (0x42c80000u32) as u32); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); r8 = rdx.wrapping_add(0x28); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(r8, ((r12 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x2cu64), r12); crate::simd_ops::wu32(rdx.wrapping_add(0x34u64), (0x42c80000u32) as u32); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(r11)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx))) as u128; rdx = ((rdx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu128(r8, (xmm0) as u128); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 19; continue; }
            19 => { fl_l = r10; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rdx = r10; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; fl_l = rsi; fl_r = r10; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), (0x42c80000u32) as u32); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdx; rcx = ((rcx).wrapping_sub(rdi)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141605490;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = rcx; rsi = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rsi as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = ((rsi as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { r8 = rsi; r10 = rdi; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 6; continue; }
            6 => { rax = rsi; r11 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = rax; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 7; continue; }
            7 => { r9 = rsi; rdx = rdi; r9 = ((r9).wrapping_sub(rdi)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_sub(0x4u64)) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; r10 = ((rdi.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), (0x42c80000u32) as u32); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), (0x42c80000u32) as u32); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), (0x42c80000u32) as u32); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), (0x42c80000u32) as u32); bb = 16; continue; }
            16 => { rdx = ((rdx).wrapping_add(0x40u64)) as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 17; continue; }
            17 => { fl_l = r10; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = r10; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = r10; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), (0x42c80000u32) as u32); bb = 21; continue; }
            21 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416057b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1416053b0; const P3: u64 = 0x1; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; r8 = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; r8 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbp = 0; fl_l = 0; fl_r = 0; rsi = ((rax.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = r8; rsi = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P0), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r8.wrapping_add(P0), rbp); rax = ((r8.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(P4), rbp); crate::simd_ops::wu64(r8, rdi); bb = 11; continue; }
            9 => { rax = rbp; bb = 11; continue; }
            10 => { rax = rsi; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415aaf70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0x50u64)) as u64;
    r8 = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = 0; fl_r = rax;
    rax = (0u64.wrapping_sub(fl_r as u64)) as u64;
    rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7c; const P1: u64 = 0x1416059a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_or_u32(rcx.wrapping_add(P0), (0x1u32) as u32);
    fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416058a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd4; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x4; const P4: u64 = 0x30; const P5: u64 = 0x10; const P6: u64 = 0x141524330; const P7: u64 = 0x28; const P8: u64 = 0x44;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; r8 = (r8 & !0xffu64) | (((((r8 as u8)).wrapping_shr(7u32)) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((r8 as u8)) as u8); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rcx = ((rax.wrapping_add(P2)) as u64) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = ((rax.wrapping_add(P5)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rcx, rax); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P7), rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(P8), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); bb = 14; continue; }
            14 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416059a0; const P1: u64 = 0x7c;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm2 as u32)) as u32);
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm3 as u32)) as u32);
    r9 = 0x1;
    r8 = rsp.wrapping_add(0x30);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::lk_or_u32(rbx.wrapping_add(P1), (0x1u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416059a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416057b0; const P1: u64 = 0x141605660;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdi = ((r9 as u32)) as u32 as u64; rsi = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rcx = rax; rbp = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rdi as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = ((rax as u32)) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { r8 = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; r8 = ((r8).wrapping_add(r8)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = rdx; fl_r = rbp; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rcx.wrapping_add(0x10); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r8.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rax).wrapping_sub(0x10u64), (xmm0) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r9 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = ((rdi.wrapping_sub(0x4u64)) as u32) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; rdx = rsi.wrapping_add(0xc); fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; r9 = ((rax.wrapping_mul(4)) as u32) as u32 as u64; r11 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x8u64))) as u32 as u64; r8 = r8.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdx.wrapping_add(0x20); crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_sub(0x3cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x2cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(r8.wrapping_add(rcx).wrapping_sub(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x20u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_sub(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x24u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_sub(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x18u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_sub(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x1cu64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_sub(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_sub(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(r8).wrapping_sub(0x10u64), ((rax as u32)) as u32); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 11; continue; }
            11 => { bb = 17; continue; }
            12 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r11 = r9; bb = 14; continue; }
            14 => { rdx = r11; r8 = ((rsi.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rdi as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; rdx = rdx.wrapping_add(0x10); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = r8.wrapping_add(0x8); crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_sub(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_sub(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx).wrapping_sub(0x10u64), ((rax as u32)) as u32); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { r8 = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(r8.wrapping_mul(8)))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)), ((rax as u32)) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x14183acc2; const P3: u64 = 0x143931a60; const P4: u64 = 0x1415e71a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); rbx = rcx; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); rbp = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), r12); rdi = rcx.wrapping_add(0x20); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r13); r13 = rdx; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r14); r14 = ((r8 as u32)) as u32 as u64; r12 = (((r14) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu16(rdi.wrapping_add(0x18u64), ((rsi as u16)) as u16); rcx = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rsi); bb = 4; continue; }
            4 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = r12; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { r8 = r12; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu16(rdi.wrapping_add(0x18u64), ((r14 as u16)) as u16); bb = 7; continue; }
            7 => { fl_l = ((rsi as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rdi.wrapping_add(0x18u64))) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rax = ((rsi as u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(r13))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rdx), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(r13).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rdx).wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(r13).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rdx).wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdx.wrapping_add(r13).wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(rdx).wrapping_add(0x30u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rbp = (rbp & !0xffu64) | ((((((rbp as u8)) ^ (crate::simd_ops::ru8(rbx.wrapping_add(0x3au64))))) as u8 as u64)); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (rbp & !0xffu64) | ((((((rbp as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::lk_xor_u8(rbx.wrapping_add(0x3au64), ((rbp as u8)) as u8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x68u64))) as u32 as u64; fl_r = 0x0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = rbx.wrapping_add(0x50); rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::lk_or_u32(rbx.wrapping_add(0x7cu64), (0x1u32) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14158f480; const P2: u64 = 0x48; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x7c; const P5: u64 = 0x14160af00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = ((rdx as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru16(rax.wrapping_add(P2))) as u32) as u32 as u64; rcx = ((rdi.wrapping_add(P3)) as u32) as u32 as u64; r8 = 0x1; r8 = (((r8 as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rcx = (((rdx as u16)) as u32) as u32 as u64; rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) | ((r8 as u16)))) as u16 as u64)); crate::simd_ops::wu16(rax.wrapping_add(P2), ((rcx as u16)) as u16); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((!((rax as u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = r9; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_and_u32(rbx.wrapping_add(P4), (0xfffffffeu32) as u32)) as u32 as u64; fl_r = fl_l; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x7c; const P1: u64 = 0x8; const P2: u64 = 0x14158f480; const P3: u64 = 0x14160af00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    rbx = rcx;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P0), (0xfffffffeu32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268ec28; const P1: u64 = 0x1415fe280; const P2: u64 = 0x20; const P3: u64 = 0x18; const P4: u64 = 0x14; const P5: u64 = 0x1415bd160; const P6: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); rbx = rcx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = rax; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rcx); rcx = 0xffffffff; let xa_src: u32 = (rcx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rax.wrapping_add(P4), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rcx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0x30; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = r8; rbp = ((rdx as u32)) as u32 as u64; r15 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(0xeu64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x160u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x160u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x168); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x1a8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x1a0u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rbx.wrapping_add(0x168); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x160u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x160u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x160u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x160u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 19; continue; } bb = 21; continue; }
            21 => { bb = 26; continue; }
            22 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x190u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x198u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            25 => { rsi = rbx.wrapping_add(0x168); bb = 26; continue; }
            26 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = ((rdi.wrapping_add(0xfu64)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            29 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), (0x0u64) as u64); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(r15.wrapping_add(0x20u64), rdi); bb = 32; continue; }
            31 => { crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); bb = 32; continue; }
            32 => { crate::simd_ops::wu64(r14, rdi); rax = 0x1; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffd1; const P1: u64 = 0x141524330; const P2: u64 = 0x1415bd140; const P3: u64 = 0x14268ec28; const P4: u64 = 0x28; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0x30;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu32(rbx.wrapping_add(P4), (0x0u32) as u32); crate::simd_ops::wu64(rbx, rax); rax = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P5), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141605fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x34; const P2: u64 = 0x14160f710; const P3: u64 = 0x68; const P4: u64 = 0x10; const P5: u64 = 0x14; const P6: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r15 = r9; r12 = r8; r13 = rdx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbp); rax = 0x403; rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rsi); rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rdi); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P1))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { r10 = (crate::simd_ops::ru64(rax)) as u64; r9 = r15; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); r8 = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); rdx = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rcx = rax; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P5), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rbx = rax; fl_l = rax; fl_r = rax; rax = 0x403; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 8; continue; }
            8 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160f710;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r15 as u32)) as u32); r14 = r8; rsi = rdx; r13 = ((r15.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rbp = ((r15.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r12); r12 = 0x403; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0x34u64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(rsp.wrapping_add(0x60u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; bb = 9; continue; }
            8 => { rdx = ((r15 as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(r14.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x4u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu32(r14.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x8u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r9 = rsp.wrapping_add(0x60); r8 = r14; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r13 as u32)) as u32); rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x10); rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 14; continue; }
            14 => { bb = 17; continue; }
            15 => { rcx = rdi.wrapping_add(0x10); let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; rax = 0x3; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (r13 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rsi, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416061d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x3;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416061e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x5; const P2: u64 = 0x141605de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(P1)) as u64) as u64; rdi = ((rax as u32)) as u32 as u64; rbp = rcx; rdi = ((((rdi as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fu32))) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; rdi = ((rcx as u32)) as u32 as u64; rdi = ((((rdi as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; r8 = rsp.wrapping_add(0x38); rcx = rbp; rbx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 9; continue; }
            8 => { rax = 0x1; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268ec48;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = P0; crate::simd_ops::wu64(rcx, rax); r9 = rcx; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((r8 as u32)) as u32); rdi = ((r8 as u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r10 = ((r8 as u32)) as u32 as u64; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); r11 = ((r8 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; rax = 0x1; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; r10 = ((rcx as u32)) as u32 as u64; rax = 0x7; rcx = 0; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; rdi = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; rax = 0x80000000; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80000006; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x80000006; rcx = 0; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0xau64))) as u32) as u32 as u64; rdx = (((rcx as u8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rax = 0x400; rdx = 0x40; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(r9.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(r9.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = ((((r11 as u32)) >> (25u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(r9.wrapping_add(0x8u64), (0x1u32) as u32); r8 = 0x1; bb = 7; continue; }
            7 => { fl_l = ((((r11 as u32)) >> (26u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((((r8 as u32)) | (0x2u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 9; continue; }
            9 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((((r8 as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 11; continue; }
            11 => { fl_l = ((((r10 as u32)) >> (9u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((((r8 as u32)) | (0x8u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = ((((r10 as u32)) >> (19u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((((r8 as u32)) | (0x10u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 15; continue; }
            15 => { fl_l = ((((r10 as u32)) >> (28u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((((r8 as u32)) | (0x20u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 17; continue; }
            17 => { fl_l = ((((r10 as u32)) >> (29u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((((r8 as u32)) | (0x40u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 19; continue; }
            19 => { rax = r9; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((((r8 as u32)) >> (7u32)) & 1) as u64; r8 = (((r8 as u32)) | ((1u32) << (7u32))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x10u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x10u64)) as u64; r10 = 0; fl_l = 0; fl_r = 0; r8 = rcx; rcx = 0; fl_l = 0; fl_r = 0; rax = 0; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); r9 = ((r10 as u32)) as u32 as u64; r11 = ((r10 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; rax = 0x1; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; r9 = ((rcx as u32)) as u32 as u64; rax = 0x7; rcx = 0; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; r10 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; rax = 0x80000000; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80000006; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x80000006; rcx = 0; fl_l = 0; fl_r = 0; let (cp_a, cp_b, cp_c, cp_d) = crate::simd_ops::cpuid_x64(rax as u32, rcx as u32); rax = cp_a; rbx = cp_b; rcx = cp_c; rdx = cp_d; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0xau64))) as u32) as u32 as u64; rdx = (((rcx as u8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { rax = 0x400; rdx = 0x40; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((rax as u32)) as u32); fl_l = ((((r11 as u32)) >> (25u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_or_u32(r8.wrapping_add(0x8u64), (0x1u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 7; continue; }
            7 => { fl_l = ((((r11 as u32)) >> (26u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_or_u32(r8.wrapping_add(0x8u64), (0x2u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 9; continue; }
            9 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_or_u32(r8.wrapping_add(0x8u64), (0x4u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 11; continue; }
            11 => { fl_l = ((((r9 as u32)) >> (9u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_or_u32(r8.wrapping_add(0x8u64), (0x8u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 13; continue; }
            13 => { fl_l = ((((r9 as u32)) >> (19u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_or_u32(r8.wrapping_add(0x8u64), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 15; continue; }
            15 => { fl_l = ((((r9 as u32)) >> (28u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_or_u32(r8.wrapping_add(0x8u64), (0x20u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 17; continue; }
            17 => { fl_l = ((((r9 as u32)) >> (29u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_or_u32(r8.wrapping_add(0x8u64), (0x40u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 19; continue; }
            19 => { fl_l = ((r10 as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_or_u32(r8.wrapping_add(0x8u64), (0x80u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 21; continue; }
            21 => { rsp = ((rsp).wrapping_add(0x10u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416064f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d1e0; const P1: u64 = 0x143931d38; const P2: u64 = 0x143931d20; const P3: u64 = 0x141834a78; const P4: u64 = 0x141606280; const P5: u64 = 0x142644450; const P6: u64 = 0x141834d18; const P7: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru32(P0)) as u32 as u64; rdx = 0x74; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = P2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = P1; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbx = P2; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = P1; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14268ec68; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu16(rcx.wrapping_add(P3), ((rax as u16)) as u16);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416065c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(r9, ((r8 as u32)) as u32);
    rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = ((((rax as u32)) as i32).wrapping_mul(((r8 as u32)) as i32) as u32) as u32 as u64;
    crate::simd_ops::wu64(rdx, (0x1u64) as u64);
    crate::simd_ops::wu32(r9.wrapping_add(P1), ((rax as u32)) as u32);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416065e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415d2150; const P1: u64 = 0x1415242e0; const P2: u64 = 0x141524330; const P3: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; r14 = r8; r12 = (((r9 as u16)) as u32) as u32 as u64; rbx = rdx; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = r8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; r9 = ((crate::simd_ops::ru16(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); rbp = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = ((r9 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x42u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rbp = (rbp & !0xffffu64) | (((if (((fl_l as u32)) < ((fl_r as u32))) { (r12 as u16) } else { (rbp as u16) }) as u16 as u64)); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(0x5u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = (((r15 as u16)) as u32) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (0x0u64) as u64); bb = 8; continue; }
            8 => { r13 = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (((rbp as u16)) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r15 as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32); r12 = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; r12 = (((r12 as u32)).wrapping_sub((rax as u32))) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 9; continue; }
            9 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { rdx = (((r9 as u16)) as u32) as u32 as u64; rcx = 0x5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = rax; r8 = (((r12 as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r8 as u32)) as u32); rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x18u64)))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 15; continue; }
            12 => { r15 = (((r15 as u32)).wrapping_sub((rax as u32))) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx, (0x2u64) as u64); bb = 16; continue; }
            14 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = rax; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; r8 = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r13 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbp = 0x1; rax = (((r15 as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0x0u32) as u32); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); crate::simd_ops::wu64(rax, r13); crate::simd_ops::wu16(rax.wrapping_add(0x10u64), ((rbp as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(0x12u64), ((rbp as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), ((rbp as u16)) as u16); rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u16)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((rax as u32)).wrapping_add(0x2du32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); bb = 16; continue; }
            16 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rbx; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426494;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rax = 0xfffe; r10 = r8; r11 = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru16(rbx)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdx, (0x60u64) as u64); rax = rdx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(0xeu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rdx = ((((rdx as u32)) & (0x3fu32))) as u32 as u64; r8 = ((crate::simd_ops::ru16(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; r8 = ((((r8 as u32)) & (0x3ffu32))) as u32 as u64; crate::simd_ops::wu32(r9, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x4u64), ((rcx as u32)) as u32); rax = ((((rax as u32)) & (0xfff80000u32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(6u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; r8 = ((((r8 as u32)) | ((rdx as u32)))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r9.wrapping_add(0x8u64), ((r8 as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x10u64), ((rax as u32)) as u32); rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(0x10u64))) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } crate::simd_ops::wu32(r9.wrapping_add(0x14u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru32(r10.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(0x14u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(r10.wrapping_add(0x10u64)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            4 => { rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(r10.wrapping_add(0x18u64))) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(r10.wrapping_add(0x14u64)))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r10.wrapping_add(0x1cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rcx as u32)) as i32) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(r10.wrapping_add(0x14u64)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(r9.wrapping_add(0x24u64), ((rax as u32)) as u32); xmm0 = 0u128; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as i32) as u32) as u32 as u64; crate::simd_ops::wu64(r11, (0x1u64) as u64); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P0))).to_bits() as u128); crate::simd_ops::wu32(r9.wrapping_add(0x1cu64), ((xmm0 as u32)) as u32); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = 0x2; crate::simd_ops::wu16(r9.wrapping_add(0x2cu64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x18u64), ((rax as u16)) as u16); rax = r11; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x42; const P5: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::lk_add_u64(rdx.wrapping_add(P1), (r8) as u64);
    crate::simd_ops::lk_sub_u32(rdx.wrapping_add(P2), ((r8 as u32)) as u32);
    crate::simd_ops::lk_add_u32(rdx.wrapping_add(P3), ((r8 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P4))) as u32) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x38u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xdfu8))) as u8 as u64)); fl_l = ((((rcx as u8)) | (0x18u8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8); return rax; } rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416068c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14; const P2: u64 = 0x5; const P3: u64 = 0x1415242e0; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    rbx = rdx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rsi as u32)) as u32);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rsi.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P0), rsi); crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rsi as u32)) as u32); }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = rbx;
    crate::simd_ops::wu64(rbx, (0x1u64) as u64);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415242e0; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x30; const P2: u64 = 0x38; const P3: u64 = 0x88; const P4: u64 = 0x90; const P5: u64 = 0x8; const P6: u64 = 0x18; const P7: u64 = 0x22; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x40; const P11: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::wu16(rcx.wrapping_add(P7), ((rax as u16)) as u16);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu16(rcx.wrapping_add(P9), ((rax as u16)) as u16);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(0x7cu64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(0x78u64), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1415242e0; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64;
    if !((((fl_l as u16)) != ((fl_r as u16)))) { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P1), (0x0u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x0u32) as u32); } rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_sub(1)) as u16 as u64));
    crate::simd_ops::wu16(rbx.wrapping_add(P0), ((rax as u16)) as u16);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416069f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x28; const P2: u64 = 0x141607120; const P3: u64 = 0x20; const P4: u64 = 0x22; const P5: u64 = 0x8; const P6: u64 = 0x2d; const P7: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (((rax as u8)) as u32) as u32 as u64; bb = 9; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P3))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add(1)) as u16 as u64)); rcx = ((((rcx as u32)) & (0x2u32))) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(P3), ((rax as u16)) as u16); fl_l = ((rax as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rbx.wrapping_add(P4))) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = ((r8.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 9; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 8; continue; }
            8 => { crate::simd_ops::wu16(rbx.wrapping_add(P3), ((rdi as u16)) as u16); rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x11; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { crate::simd_ops::wu16(rbx.wrapping_add(P7), ((rdi as u16)) as u16); rax = 0x1; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rax = 0x11; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x30; const P5: u64 = 0xffffffffffffffd0; const P6: u64 = 0xffffffffffffffe0; const P7: u64 = 0xfffffffffffffff0; const P8: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P5), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(P6))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P6), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(P7))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P7), (xmm0) as u128); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141606aa0; const P1: u64 = 0x1415d7980; const P2: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; r13 = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; rbp = rcx; r8 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = rcx.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14)) as u64; r15 = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rcx; rsi = rax.wrapping_add(0x8); rsi = (if (((fl_l as u64)) == ((fl_r as u64))) { r12 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = r15; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rsi).wrapping_add(crate::simd_ops::ru64(r14))) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 22; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsi, rdi); rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 12; continue; }
            12 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 13; continue; }
            13 => { r9 = (crate::simd_ops::ru64(r14)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x90u64))) as u32 as u64; rcx = r9; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 17; continue; }
            17 => { bb = 22; continue; }
            18 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = ((r8).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = rcx.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x90u64), (1) as u32)) as u32 as u64; bb = 22; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsi, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rsi.wrapping_add(0x10u64), (xmm1) as u128); rax = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x18u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 23; continue; }
            23 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 24; continue; }
            24 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141606aa0; const P1: u64 = 0x1415d7980; const P2: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx.wrapping_add(0x88); rbx = rdx; rax = (crate::simd_ops::ru64(rsi)) as u64; rbp = rcx; r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; rdi = rax.wrapping_add(0x8); fl_l = rax; fl_r = r8; rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rdi }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rdi).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi, rbx); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x90u64))) as u32 as u64; rcx = r9; rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { rax = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = ((r8).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = rcx.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x90u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            20 => { xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x10u64), (xmm1) as u128); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; rax = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rcx as u32)) as u32); bb = 16; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x0;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141606eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416071f0; const P1: u64 = 0x1438152a8; const P2: u64 = 0x141607120; const P3: u64 = 0x1434268d0; const P4: u64 = 0x143426448; const P5: u64 = 0x1439310e8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x24u64))) as u32) as u32 as u64; rsi = rdi.wrapping_add(0x58); r14 = rdi.wrapping_add(0x5c); r15 = rdi.wrapping_add(0x60); fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = ((rax as u32)) as u32 as u64; rdx = rsi; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add(1)) as u16 as u64)); rbx = ((rbx).wrapping_shl(4u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; rcx = rdi; crate::simd_ops::wu16(rdi.wrapping_add(0x24u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(r14, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(r15, ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = 0x1; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(P1))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = (crate::simd_ops::ru32(P1)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x4cu64), ((rax as u32)) as u32); bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x24u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 16; continue; } bb = 3; continue; }
            3 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (((rax as u8)) as u32) as u32 as u64; bb = 12; continue; }
            6 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add(1)) as u16 as u64)); rcx = ((((rcx as u32)) & (0x2u32))) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rax as u16)) as u16); fl_l = ((rax as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rdi.wrapping_add(0x22u64))) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rax = r8.wrapping_add(0x18); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); bb = 12; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); bb = 11; continue; }
            11 => { crate::simd_ops::wu16(rdi.wrapping_add(0x20u64), ((rbx as u16)) as u16); rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x2du64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu16(rdi.wrapping_add(0x24u64), ((rbx as u16)) as u16); rsi = rdi.wrapping_add(0x58); r14 = rdi.wrapping_add(0x5c); r15 = rdi.wrapping_add(0x60); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r10 = (0x5851f42d4c957f2du64) as u64; rdx = ((crate::simd_ops::ru16(rdi.wrapping_add(0x24u64))) as u32) as u32 as u64; xmm5 = ((crate::simd_ops::ru32(P3)) as u32) as u128; rdx = ((rdx).wrapping_add(rdx)) as u64; xmm3 = ((crate::simd_ops::ru32(P4)) as u32) as u128; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64) as u128; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(P5)) as u64; rdx = (((rdx) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; crate::simd_ops::wu64(P5, rdx); r8 = rdx; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r8 = (((r8) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; rdx = ((rdx).wrapping_shr(33u32)) as u64; r8 = ((r8).wrapping_add(1)) as u64; crate::simd_ops::wu64(P5, r8); rcx = r8; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = (((rcx) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm4 = ((crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32) as u128; xmm0 = (((rdx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); r8 = ((r8).wrapping_shr(33u32)) as u64; rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(P5, rcx); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); rcx = ((rcx).wrapping_shr(33u32)) as u64; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32) as u128; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x44u64), ((rax as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (((rcx as u32)) as u32) as u128; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); fl_l = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((rax as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rsi))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x64u64), ((xmm0 as u32)) as u32); xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = crate::simd_ops::vshufps(xmm0, xmm0, 0x55u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (((r9 as u32)) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(r14))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((xmm1 as u32)) as u32); xmm1 = (((r8 as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(r15))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x6cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x50u64), ((xmm5 as u32)) as u32); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { crate::simd_ops::wu32(rdi, ((rbx as u32)) as u32); bb = 17; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x54;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    crate::simd_ops::wu32(rcx, (0x2u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439310e8; const P1: u64 = 0x22; const P2: u64 = 0x28; const P3: u64 = 0x20; const P4: u64 = 0x8; const P5: u64 = 0x18; const P6: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(P0)) as u64; r10 = rcx; rdx = 0; fl_l = 0; fl_r = 0; rcx = (0x5851f42d4c957f2du64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P0, rax); r8 = ((crate::simd_ops::ru16(r10.wrapping_add(P1))) as u32) as u32 as u64; rax = ((rax).wrapping_shr(33u32)) as u64; let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } fl_l = (crate::simd_ops::ru8(r10.wrapping_add(P2))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; crate::simd_ops::wu16(r10.wrapping_add(P3), ((rdx as u16)) as u16); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (((rdx as u16)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(r10.wrapping_add(P5), rcx); bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(r10.wrapping_add(P6))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 3; continue; }
            3 => { r11 = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = ((r8 as u32)) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r9)) as u64; rcx = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            8 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu8(r8.wrapping_add(rcx), (0x0u8) as u8); r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(r10.wrapping_add(P6))) as u64; r8 = ((crate::simd_ops::ru16(r10.wrapping_add(P3))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu8(r8.wrapping_add(rdx), (0x1u8) as u8); return rax; }
            15 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416071f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439310e8; const P1: u64 = 0x143426448; const P2: u64 = 0x1434268d0; const P3: u64 = 0x18; const P4: u64 = 0xc; const P5: u64 = 0x10; const P6: u64 = 0x8; const P7: u64 = 0x14; const P8: u64 = 0x4;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(P0)) as u64; r8 = (0x5851f42d4c957f2du64) as u64; xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P0, rax); rax = ((rax).wrapping_shr(33u32)) as u64; xmm0 = (((rax as u32)) as u32) as u128; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P4)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P0, rax); rax = ((rax).wrapping_shr(33u32)) as u64; xmm0 = (((rax as u32)) as u32) as u128; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P5)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P6)))).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P6), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P0, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm0 = (((rax as u32)) as u32) as u128; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(P7)))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P8)))).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P8), ((xmm0 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416072c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x54; const P1: u64 = 0x44; const P2: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (((rdx as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(P0)))) as u32 as u64;
    crate::simd_ops::lk_add_u32(rcx.wrapping_add(P1), ((rdx as u32)) as u32);
    fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P2), ((rdx as u32)) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rcx, (0x1u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416072e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x141607120;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x0; r13 = ((r9 as u32)) as u32 as u64; r14 = ((r8 as u32)) as u32 as u64; rsi = rcx; r15 = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 1; continue; }
            1 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rdx); crate::simd_ops::wu16(rcx.wrapping_add(0x22u64), ((r14 as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x2du64), ((rax as u8)) as u8); rbp = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdi = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 6; continue; }
            6 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rbx.wrapping_add(0x10u64), ((rax as u16)) as u16); bb = 8; continue; }
            8 => { rax = rbp; crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); fl_l = (crate::simd_ops::lk_add_u16(rbp.wrapping_add(0x10u64), (1) as u16)) as u16 as u64; rax = ((crate::simd_ops::ru16(r12)) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x20u64), ((rax as u16)) as u16); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rcx); bb = 33; continue; }
            9 => { crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdx); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rcx.wrapping_add(0x20u64), ((rdi as u16)) as u16); rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdi.wrapping_add(0x18u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rdi); crate::simd_ops::wu16(rax.wrapping_add(0x10u64), ((rdi as u16)) as u16); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 11; continue; }
            11 => { rdx = r14; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rax; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx, rbp); rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r14 as u32)) as u32); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax), ((rdi as u8)) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 17; continue; }
            17 => { bb = 22; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 20; continue; }
            20 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rbx = rdi; bb = 22; continue; }
            22 => { r14 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 23; continue; }
            23 => { rax = ((crate::simd_ops::ru16(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(r14)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rdi); crate::simd_ops::wu32(r14.wrapping_add(0xcu64), ((rdi as u32)) as u32); bb = 26; continue; }
            26 => { rdx = r14; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            27 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(r14.wrapping_add(0x10u64), ((rax as u16)) as u16); bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::lk_add_u16(rbx.wrapping_add(0x10u64), (1) as u16)) as u16 as u64; bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 31; continue; }
            31 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { r15 = 0x1; bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((r15 as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416074f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438152a8; const P1: u64 = 0x1416071f0; const P2: u64 = 0x1434268d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(0x2cu64), (0x1u8) as u8); fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu16(rcx.wrapping_add(0x24u64), ((rax as u16)) as u16); rbp = 0x1; rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x24u64), ((rbp as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); xmm3 = ((crate::simd_ops::ru32(rax)) as u32) as u128; crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), ((xmm3 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), ((xmm1 as u32)) as u32); xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32) as u128; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((xmm2 as u32)) as u32); rax = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(P0))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = (crate::simd_ops::ru32(P0)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rbp as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4cu64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r8)) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = rbx.wrapping_add(0x58); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x5cu64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 10; continue; }
            8 => { rdx = rbx.wrapping_add(0x58); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x20); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x60u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x5cu64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x58u64)))).to_bits() as u128); bb = 10; continue; }
            9 => { xmm0 = 0u128; xmm1 = 0u128; xmm2 = 0u128; bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx.wrapping_add(0x64u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x6cu64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1b8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x58u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x5cu64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x60u64)))).to_bits() as u128); xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm1); crate::simd_ops::wu64(rdx.wrapping_add(0x4u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((xmm2 as u32)) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64) as u128; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x4cu64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), ((xmm0 as u64)) as u64); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = ((rcx.wrapping_add(rsi)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((rax as u32)) as u32); rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(rbx, ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x44u64), ((rsi as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((xmm1 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { rax = 0x25; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x38; const P2: u64 = 0x1415242e0; const P3: u64 = 0x3c; const P4: u64 = 0x28; const P5: u64 = 0x10; const P6: u64 = 0x8; const P7: u64 = 0xc; const P8: u64 = 0x88; const P9: u64 = 0x90; const P10: u64 = 0x94;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = rcx; crate::simd_ops::wu32(rcx, ((rsi as u32)) as u32); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rsi as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(P5))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rsi as u32)) as u32); bb = 7; continue; }
            7 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rdi.wrapping_add(P5), ((rax as u16)) as u16); bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), rsi); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P8), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rsi as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416077b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x141606eb0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(0x44u64)))) as u32 as u64; xmm3 = 0u128; r9 = ((rdx as u32)) as u32 as u64; r8 = rcx; xmm3 = (xmm3 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x50u64)))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm3 = (xmm0) as u128; bb = 4; continue; }
            2 => { xmm0 = 0u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm3 = 0u128; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; xmm4 = (xmm3) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x64u64)))).to_bits() as u128); xmm5 = (xmm3) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x68u64)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x6cu64)))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x58u64)))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x5cu64)))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x60u64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm4 as u32)) as u32); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x70u64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm5 as u32)) as u32); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x74u64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm3 as u32)) as u32); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x78u64)))).to_bits() as u128); rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rax)) as u64; xmm0 = (xmm4) as u128; xmm1 = (xmm5) as u128; xmm2 = (xmm3) as u128; rax = ((rax).wrapping_add(0x8u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1b8u64))) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x4u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x8u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0xcu64)))).to_bits() as u128); xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm1); crate::simd_ops::wu64(rdx.wrapping_add(0x4u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((xmm2 as u32)) as u32); rdx = (crate::simd_ops::ru32(r8.wrapping_add(0x38u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x30u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu64(r8.wrapping_add(0x70u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r8.wrapping_add(0x78u64), ((rax as u32)) as u32); fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x48u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = r8; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            9 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416078b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x38; const P2: u64 = 0x58; const P3: u64 = 0x1b8; const P4: u64 = 0x8; const P5: u64 = 0x4; const P6: u64 = 0xc; const P7: u64 = 0x5c; const P8: u64 = 0x60; const P9: u64 = 0x70; const P10: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = rcx;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64;
    fl_l = rax; fl_r = r8;
    if (((fl_l as u64)) == ((fl_r as u64))) { xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64) as u128; rax = (crate::simd_ops::ru32(r9.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu64(r9.wrapping_add(P9), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r9.wrapping_add(P10), ((rax as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rax)) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32) as u128; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(P5)))).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(P7)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(r9.wrapping_add(P8)))).to_bits() as u128); xmm0 = crate::simd_ops::vunpcklps(xmm0, xmm1); crate::simd_ops::wu64(rdx.wrapping_add(P5), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P6), ((xmm2 as u32)) as u32); rdx = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    xmm0 = ((crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64) as u128;
    rax = (crate::simd_ops::ru32(r9.wrapping_add(P8))) as u32 as u64;
    crate::simd_ops::wu64(r9.wrapping_add(P9), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(r9.wrapping_add(P10), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14268ecd8; const P1: u64 = 0x14268ecd0; const P2: u64 = 0x4; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x28; const P8: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    xmm0 = ((crate::simd_ops::ru64(P1)) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rdx as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416079b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x2; const P2: u64 = 0x1415242e0; const P3: u64 = 0x28; const P4: u64 = 0x30; const P5: u64 = 0x20; const P6: u64 = 0x40; const P7: u64 = 0x10; const P8: u64 = 0x8; const P9: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rsi.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rsi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x2; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rsi); bb = 4; continue; }
            4 => { crate::simd_ops::lk_and_u32(rbx.wrapping_add(P4), (0x80000000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rsi as u32)) as u32); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(P7))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rsi as u32)) as u32); bb = 8; continue; }
            8 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rdi.wrapping_add(P7), ((rax as u16)) as u16); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbx.wrapping_add(P6), rsi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1415242e0; const P4: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(P1))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rbx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rbx as u32)) as u32); bb = 4; continue; }
            4 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P0), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = (((rax as u16)).wrapping_sub(1)) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); crate::simd_ops::wu16(rdi.wrapping_add(P1), ((rax as u16)) as u16); bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x30; const P2: u64 = 0x18; const P3: u64 = 0x28; const P4: u64 = 0x8; const P5: u64 = 0x4; const P6: u64 = 0xc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rdi = r9; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r10 = 0; fl_l = 0; fl_r = 0; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; bb = 4; continue; }
            4 => { rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P4))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(r10.wrapping_mul(8)).wrapping_add(P5))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 5; continue; }
            5 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; r9 = (((r9 as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(r8)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 18; continue; } bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 18; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P4))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 18; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(P6))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 11; continue; }
            11 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; r8 = ((r8).wrapping_add(0x10u64)) as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 7; continue; } bb = 12; continue; }
            12 => { xmm0 = ((crate::simd_ops::ru32(r8)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 18; continue; } bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P5))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 18; continue; } bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(r8.wrapping_add(P4))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 18; continue; } bb = 15; continue; }
            15 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = ((r8).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 17; continue; }
            17 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); rsi = r9; r13 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r15 = ((r8 as u32)) as u32 as u64; rbx = rcx; rbp = 0x1; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rdi); bb = 4; continue; }
            4 => { crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x30u64), (0x80000000u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rdi as u32)) as u32); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 6; continue; }
            6 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { rax = ((r15 as u32)) as u32 as u64; rcx = 0x2; rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; r12 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { r8 = ((r12 as u32)) as u32 as u64; rdx = r13; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = ((r14.wrapping_add(r14.wrapping_mul(2))) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r15 as u32)) as u32); rdx = (((rdx as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = 0x2; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x30u64), (0x80000000u32) as u32); rax = ((r14 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; r9 = ((rdi as u32)) as u32 as u64; crate::simd_ops::lk_or_u32(rbx.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { rdx = rdi; r8 = rsi; bb = 12; continue; }
            12 => { r10 = (crate::simd_ops::ru32(rsi)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; rsi = rsi.wrapping_add(0x8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(0x8u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = ((r10 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x18u64)))) as u64; r8 = ((r8).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(rax), rcx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(0xcu64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(0x10u64), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(rdx).wrapping_add(0x14u64), ((rdi as u32)) as u32); rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 14; continue; }
            14 => { bb = 18; continue; }
            15 => { rbp = 0x2; bb = 18; continue; }
            16 => { rbp = 0x34; bb = 18; continue; }
            17 => { rbp = 0x1f; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x7fffffff; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = (((crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as i32 as i64) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = ((r8).wrapping_add(r8)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_sub(0x4u64), ((rcx as u32)) as u32); bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((((rcx as u32)) >> (31u32)) & 1) as u64; rcx = (((rcx as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 20; continue; } bb = 23; continue; }
            23 => { rax = ((rbp as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x28; const P2: u64 = 0x10; const P3: u64 = 0x143930fa8; const P4: u64 = 0x1415f83b0;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rax = ((r9 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8);
    fl_l = ((((r9 as u32)) >> (31u32)) & 1) as u64;
    r9 = (((r9 as u32)) & !((1u32) << (31u32))) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (((rax as u32)).wrapping_shr(31u32)) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(P3)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x30; const P2: u64 = 0x28; const P3: u64 = 0x8; const P4: u64 = 0xfffffffffffffffc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x7fffffff; r9 = rcx; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = (((crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P3))) as i32 as i64) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = ((r8).wrapping_add(r8)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(P4), ((rcx as u32)) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r9.wrapping_add(P1))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((((rcx as u32)) >> (31u32)) & 1) as u64; rcx = (((rcx as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141607e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0; const P3: u64 = 0x143930f44; const P4: u64 = 0x141608360;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = r8; r12 = rdx; rbx = rcx; r13 = 0x1; r9 = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rcx, rax); r15 = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 1; continue; }
            1 => { rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x50u64))) as u64; rdx = ((r9 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(r8.wrapping_add(0x58u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { bb = 13; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = rbp; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(rdx.wrapping_mul(8)), rcx); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r14.wrapping_add(0x50u64), rdi); rax = (((r13 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x5cu64), ((r15 as u32)) as u32); bb = 13; continue; }
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
            23 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdi as u32)) as u32); rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); r8 = (((r13 as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); r9 = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = ((((rdi as u32)) as i32).wrapping_mul((0xfffffff4u32) as i32) as u32) as u32 as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(2u32)) as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::lk_add_u64(rbx, (rcx) as u64); crate::simd_ops::lk_add_u32(r12, ((rax as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 25; continue; }
            25 => { r13 = 0x1; bb = 26; continue; }
            26 => { rax = ((r13 as u32)) as u32 as u64; bb = 27; continue; }
            27 => { r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd140; const P1: u64 = 0x14268ece0; const P2: u64 = 0x18; const P3: u64 = 0x14164cca0; const P4: u64 = 0x30; const P5: u64 = 0x14268ed00; const P6: u64 = 0x38; const P7: u64 = 0x70; const P8: u64 = 0x14268ed30; const P9: u64 = 0x40; const P10: u64 = 0x48; const P11: u64 = 0x50; const P12: u64 = 0x1426452b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rdi, rax);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), (0xffffffffu32) as u32);
    rax = P5;
    crate::simd_ops::wu64(rdi.wrapping_add(P6), rbx);
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    rax = P8;
    crate::simd_ops::wu64(rdi.wrapping_add(P9), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P10), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(P11), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rbx);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rbx);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416080c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268ed00; const P1: u64 = 0x14268ed30; const P2: u64 = 0x18; const P3: u64 = 0x1415c2b30; const P4: u64 = 0x60; const P5: u64 = 0x68; const P6: u64 = 0x1415242e0; const P7: u64 = 0x6c; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x4c; const P11: u64 = 0x38; const P12: u64 = 0x143815b40; const P13: u64 = 0x1426452a0; const P14: u64 = 0x14164ccd0; const P15: u64 = 0x1415bd160;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rcx = ((rcx).wrapping_add(0x50u64)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P8), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P10), ((rbp as u32)) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P12; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = rbx.wrapping_add(0x70); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P15, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608188(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141608200;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0xa8u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268ef10; const P1: u64 = 0x14268ef48; const P2: u64 = 0x18; const P3: u64 = 0x14268efc8; const P4: u64 = 0xa8; const P5: u64 = 0x14160e9b0; const P6: u64 = 0xb0; const P7: u64 = 0x14160e340; const P8: u64 = 0x1416080c0; const P9: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xd8; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416080c0; const P1: u64 = 0x141834d30;
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xa8; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c2bf0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x14160a710;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32 as u64; rdi = rcx; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rbx = (((r9 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); r9 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x50u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rdx = 0x3; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbx as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r8 as u8)) as u8); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { crate::simd_ops::wu8(rax.wrapping_add(0x2u64), ((rbx as u8)) as u8); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 10; continue; }
            10 => { rdx = rbx; r8 = rbx; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0x8u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = r8; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rdx = rbx.wrapping_add(0x10); rcx = ((((rcx as u32)) & (0xfffffff0u32))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x68u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; bb = 16; continue; }
            15 => { fl_l = ((rbx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rax = ((rcx as u32)) as u32 as u64; rdx = r8; bb = 16; continue; }
            16 => { rcx = ((rax as u32)) as u32 as u64; r8 = rdx; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 22; continue; }
            21 => { xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32) as u128; rdx = (((rsi as u16)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 23; continue; }
            23 => { rax = 0x1; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416084f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x1416089b0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; r15 = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); rbx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = (crate::simd_ops::ru32(r15.wrapping_add(0x68u64))) as u32 as u64; rdx = ((rbx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 51; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; bb = 3; continue; }
            3 => { rbp = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm7 as u32)) as u32); xmm0 = (xmm7) as u128; xmm6 = (xmm7) as u128; bb = 6; continue; }
            5 => { xmm0 = 0u128; xmm6 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); bb = 6; continue; }
            6 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 43; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x60u64))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; rdx = r14; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru16(rax.wrapping_add(rcx).wrapping_add(0x8u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 10; continue; }
            10 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(rcx).wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rcx))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx).wrapping_add(0xcu64))) as u32) as u128; bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 14; continue; }
            14 => { bb = 43; continue; }
            15 => { r10 = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 36; continue; } bb = 16; continue; }
            16 => { rax = ((r14.wrapping_sub(0x4u64)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; r10 = ((rax.wrapping_mul(4)) as u32) as u32 as u64; r8 = ((rax.wrapping_mul(4)) as u64) as u64; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(rcx).wrapping_add(0x8u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 21; continue; } bb = 18; continue; }
            18 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            20 => { xmm6 = ((crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0xcu64))) as u32) as u128; bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(rcx).wrapping_add(0x18u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 25; continue; } bb = 22; continue; }
            22 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x1cu64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 25; continue; }
            24 => { xmm6 = ((crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x1cu64))) as u32) as u128; bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rdx).wrapping_add(0x28u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 29; continue; } bb = 26; continue; }
            26 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(0x2cu64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            28 => { xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(0x2cu64))) as u32) as u128; bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rdx).wrapping_add(0x38u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 33; continue; } bb = 30; continue; }
            30 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(0x3cu64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 33; continue; }
            32 => { xmm6 = ((crate::simd_ops::ru32(rcx.wrapping_add(rdx).wrapping_add(0x3cu64))) as u32) as u128; bb = 33; continue; }
            33 => { rdx = ((rdx).wrapping_add(0x40u64)) as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 34; continue; }
            34 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 37; continue; } bb = 35; continue; }
            35 => { bb = 43; continue; }
            36 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 37; continue; }
            37 => { r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; rcx = ((r14 as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru16(r8.wrapping_add(0x8u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 42; continue; } bb = 39; continue; }
            39 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 42; continue; }
            41 => { xmm6 = ((crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32) as u128; bb = 42; continue; }
            42 => { r8 = ((r8).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 43; continue; }
            43 => { r9 = rsp.wrapping_add(0x70); r8 = (((rbp as u8)) as u32) as u32 as u64; rdx = (((rsi as u16)) as u32) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 46; continue; }
            46 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 49; continue; } bb = 47; continue; }
            47 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); bb = 49; continue; }
            49 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x50u64))) as u64; r8 = rsp.wrapping_add(0x70); r9 = 0x4; rdx = (((rsi as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(r12)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 50; continue; }
            50 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 51; continue; }
            51 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 52; continue; }
            52 => { rsp = ((rsp).wrapping_add(0x58u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160a410;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    r9 = r8;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416087b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14160a410;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r9 = ((r8.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416087d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141608040; const P2: u64 = 0xb0; const P3: u64 = 0x14268ef10; const P4: u64 = 0xa8; const P5: u64 = 0x14268ef48; const P6: u64 = 0x18; const P7: u64 = 0x14268efc8; const P8: u64 = 0x8; const P9: u64 = 0xc8; const P10: u64 = 0xd0; const P11: u64 = 0xc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0xd8;
    rcx = 0x2;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = P3; crate::simd_ops::wu64(rbx, rax); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = P5; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); rax = P7; crate::simd_ops::wu64(rdx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P8), rax); crate::simd_ops::wu64(rbx.wrapping_add(P9), rax); crate::simd_ops::wu64(rbx.wrapping_add(P10), rax); rax = rbx; crate::simd_ops::wu64(rcx, rdx); crate::simd_ops::wu64(rbx.wrapping_add(P11), rcx); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416088f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160f380; const P1: u64 = 0x14160e370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((r8 as u32)) as u32 as u64;
    rbx = rdx;
    rdx = ((rax as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x20; const P2: u64 = 0xb8; const P3: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64; return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((crate::simd_ops::ru8(rax.wrapping_add(P3))) as u32) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; return rax; }
    rax = (((rax as u8)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160f380;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0xc0u64)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416089b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x141609d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r15 = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rbp = r9; rdi = (((rdx as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = r14.wrapping_add(0x18); fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); r12 = 0xfffffff8; xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { r9 = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rdx.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 13; continue; } bb = 7; continue; }
            7 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 8; continue; }
            8 => { xmm6 = (xmm7) as u128; bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; xmm3 = (xmm6) as u128; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rbp))).to_bits() as u128); rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbp, ((xmm6 as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 27; continue; }
            13 => { rax = ((r9.wrapping_mul(2).wrapping_add(0x9u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rax = (((rax) & (r12))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 8; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 15; continue; }
            15 => { xmm6 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; bb = 9; continue; }
            16 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 18; continue; }
            18 => { r12 = 0xfffffff8; bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { r9 = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; bb = 21; continue; }
            21 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rdx.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 28; continue; } bb = 22; continue; }
            22 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 21; continue; } bb = 23; continue; }
            23 => { xmm6 = 0u128; bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; xmm3 = (xmm6) as u128; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rbp))).to_bits() as u128); rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbp, ((xmm6 as u32)) as u32); bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (((rsi as u8)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            28 => { rax = ((r9.wrapping_mul(2).wrapping_add(0x9u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rax = (((rax) & (r12))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 23; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 30; continue; }
            30 => { xmm6 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; bb = 24; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rdx; fl_r = rdx;
    rax = ((rdx.wrapping_add(P0)) as u64) as u64;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r12 = 0; fl_l = 0; fl_r = 0; r10 = ((r12 as u32)) as u32 as u64; fl_l = rsi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r9 = rsi.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rdx = rdi.wrapping_add(0x8); r15 = (0xfffffffffffffff8u64) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; r14 = 0x18; r11 = 0x28; r15 = ((r15).wrapping_sub(rdi)) as u64; r14 = ((r14).wrapping_sub(rdi)) as u64; r11 = ((r11).wrapping_sub(rdi)) as u64; fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; r10 = ((r9.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = rdx.wrapping_sub(0x8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rcx, ((r12 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_sub(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu16(rdx, ((r12 as u16)) as u16); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((r12 as u32)) as u32); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r15.wrapping_add(rdx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = rdx.wrapping_add(0x8); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rcx, ((r12 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((r12 as u32)) as u32); crate::simd_ops::wu16(rdx.wrapping_add(0x10u64), ((r12 as u16)) as u16); crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((r12 as u32)) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(rax).wrapping_add(0x10u64))) as u128; r8 = rdx.wrapping_add(0x18); crate::simd_ops::wu128(rcx, (xmm0) as u128); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(r8, ((r12 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); crate::simd_ops::wu16(rdx.wrapping_add(0x20u64), ((r12 as u16)) as u16); crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), ((r12 as u32)) as u32); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); r8 = rdx.wrapping_add(0x28); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(r8, ((r12 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), ((r12 as u32)) as u32); crate::simd_ops::wu16(rdx.wrapping_add(0x30u64), ((r12 as u16)) as u16); crate::simd_ops::wu32(rdx.wrapping_add(0x34u64), ((r12 as u32)) as u32); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(r11)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rdx))) as u128; rdx = ((rdx).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu128(r8, (xmm0) as u128); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 19; continue; }
            19 => { fl_l = r10; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rdx = r10; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; fl_l = rsi; fl_r = r10; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rdx, r12); crate::simd_ops::wu16(rdx.wrapping_add(0x8u64), ((r12 as u16)) as u16); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((r12 as u32)) as u32); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdx; rcx = ((rcx).wrapping_sub(rdi)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; r12 = (((r12 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((r12 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 25; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); r13 = 0; fl_l = 0; fl_r = 0; r11 = ((r13 as u32)) as u32 as u64; fl_l = rsi; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); r10 = rsi.wrapping_sub(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); rdx = rdi.wrapping_add(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r14 = 0xc; r15 = (0xfffffffffffffffcu64) as u64; r10 = ((r10).wrapping_shr(2u32)) as u64; rbp = 0x14; r15 = ((r15).wrapping_sub(rdi)) as u64; r14 = ((r14).wrapping_sub(rdi)) as u64; rbp = ((rbp).wrapping_sub(rdi)) as u64; fl_l = ((r10).wrapping_add(1)) as u64; r10 = ((fl_l as u64)) as u64; r11 = ((r10.wrapping_mul(4)) as u64) as u64; bb = 9; continue; }
            9 => { r8 = rdx.wrapping_sub(0x4); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu16(r8, ((r13 as u16)) as u16); crate::simd_ops::wu8(rdx.wrapping_sub(0x2u64), ((r13 as u8)) as u8); crate::simd_ops::wu32(rdx, ((r13 as u32)) as u32); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((rdx.wrapping_add(r15)) as u64) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax))) as u64; crate::simd_ops::wu64(r8, rcx); r8 = rdx.wrapping_add(0x4); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu16(r8, ((r13 as u16)) as u16); crate::simd_ops::wu8(rdx.wrapping_add(0x6u64), ((r13 as u8)) as u8); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r13 as u32)) as u32); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rax).wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, rcx); r8 = rdx.wrapping_add(0xc); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu16(r8, ((r13 as u16)) as u16); crate::simd_ops::wu8(rdx.wrapping_add(0xeu64), ((r13 as u8)) as u8); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((r13 as u32)) as u32); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(r14))) as u64; crate::simd_ops::wu64(r8, rax); r8 = rdx.wrapping_add(0x14); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu16(r8, ((r13 as u16)) as u16); crate::simd_ops::wu8(rdx.wrapping_add(0x16u64), ((r13 as u8)) as u8); crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((r13 as u32)) as u32); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rdx = ((rdx).wrapping_add(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rbp))) as u64; crate::simd_ops::wu64(r8, rax); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 19; continue; }
            19 => { fl_l = r11; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rdx = ((rdi.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = r11; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 21; continue; }
            21 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu16(rdx, ((r13 as u16)) as u16); crate::simd_ops::wu8(rdx.wrapping_add(0x2u64), ((r13 as u8)) as u8); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((r13 as u32)) as u32); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdx; rcx = ((rcx).wrapping_sub(rdi)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64; crate::simd_ops::wu64(rdx, rcx); rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r12 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141608f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0x141529ed0; const P2: u64 = 0x14160a240;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; rax = rcx.wrapping_sub(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r14 = rcx; xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 1; continue; }
            1 => { r10 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x9cu64))) as u32) as u32 as u64; r8 = ((r10 as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(32u32)) as u64; fl_l = (((r8) | (rax))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rax))) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbp); rbp = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rbp = ((rbp).wrapping_shl(32u32)) as u64; rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rsi); fl_l = (((rbp) | (rax))) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rdi); bb = 6; continue; }
            6 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rax))) as u64; fl_l = rcx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32) as u128; bb = 11; continue; }
            9 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x3cu64))) as u32) as u128; bb = 11; continue; }
            11 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; r8 = rsp.wrapping_add(0xb0); xmm1 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0xb0u64), ((rax as u32)) as u32); rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 15; continue; }
            14 => { xmm0 = 0u128; bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x30u64), ((xmm6 as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x38u64), ((xmm0 as u32)) as u32); fl_l = (crate::simd_ops::lk_xor_u8(rdi.wrapping_add(0x14u64), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xcu64))) as u32) as u32 as u64; rdx = 0x1; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = r15; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x78u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x70u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; bb = 18; continue; }
            18 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416090f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160f0e0; const P1: u64 = 0x24; const P2: u64 = 0x29; const P3: u64 = 0x14160f420; const P4: u64 = 0x10; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((r8 as u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rbp = rcx; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rbx = (((r9 as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P2))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = (((rbx as u8)) as u32) as u32 as u64; rcx = rbp.wrapping_add(0xc0); r8 = ((rsi as u32)) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; rcx = rbp.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; rcx = rbp.wrapping_add(0xa8); rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x141609d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r15 = rcx; rbx = r9; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r14 = r8; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rdi = (((rdx as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rcx = ((r8.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru16(r8)) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(0x4u64)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); rsi = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); r13 = ((crate::simd_ops::ru8(r8.wrapping_add(0x2u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((r13 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); r12 = 0xfffffff8; xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { r9 = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rdx.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 18; continue; } bb = 12; continue; }
            12 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 13; continue; }
            13 => { xmm6 = (xmm7) as u128; bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; xmm3 = (xmm6) as u128; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(r14))).to_bits() as u128); rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(r14, ((xmm6 as u32)) as u32); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 17; continue; }
            17 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 32; continue; }
            18 => { rax = ((r9.wrapping_mul(2).wrapping_add(0x9u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rax = (((rax) & (r12))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 13; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 20; continue; }
            20 => { xmm6 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; bb = 14; continue; }
            21 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 23; continue; }
            23 => { r12 = 0xfffffff8; bb = 24; continue; }
            24 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { r9 = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rdx.wrapping_mul(2)))) as u16 as u64; fl_r = ((rdi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 37; continue; } bb = 27; continue; }
            27 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 26; continue; } bb = 28; continue; }
            28 => { xmm6 = 0u128; bb = 29; continue; }
            29 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; xmm3 = (xmm6) as u128; rdx = (((rdi as u16)) as u32) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(r14))).to_bits() as u128); rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(r14, ((xmm6 as u32)) as u32); bb = 31; continue; }
            31 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 33; continue; }
            33 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu8(rbx, ((r13 as u8)) as u8); bb = 35; continue; }
            35 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 36; continue; }
            36 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            37 => { rax = ((r9.wrapping_mul(2).wrapping_add(0x9u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rax = (((rax) & (r12))) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 28; continue; } bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 39; continue; }
            39 => { xmm6 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; bb = 29; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416093a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141609190; const P1: u64 = 0x1415c2f80; const P2: u64 = 0x143930fa0; const P3: u64 = 0x1415c4320; const P4: u64 = 0x141608b80;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = r11.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), ((r13 as u32)) as u32); r15 = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rbx); rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rbx); rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), r14); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x2u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 4; continue; }
            4 => { r14 = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r9 = rbp.wrapping_add(0x6f); xmm7 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32) as u128; r8 = rbp.wrapping_add(0x7f); rdx = (((r14 as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm7 as u32)) as u32); rcx = r15; crate::simd_ops::wu8(rbp.wrapping_add(0x6fu64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; r12 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x6fu64))) as u32) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(0x50u64)) as u64; rdx = rbp.wrapping_sub(0x39); r8 = ((r14 as u32)) as u32 as u64; r13 = ((r14 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), (0x3f800000u32) as u32); fl_l = ((r12 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), (0x0u32) as u32); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = r15.wrapping_add(0x10); rcx = (crate::simd_ops::ru64(P2)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; xmm6 = (xmm0) as u128; r14 = (crate::simd_ops::ru32(r15.wrapping_add(0x68u64))) as u32 as u64; r8 = 0x1; crate::simd_ops::wu32(rbp.wrapping_sub(0x1du64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((r8 as u32)) as u32); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x21u64), ((r13 as u16)) as u16); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x6cu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x25u64), ((rdx as u32)) as u32); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = ((rcx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = r15.wrapping_add(0x60); rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r8 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x6cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x68u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rcx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x68u64), ((rcx as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x60u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; fl_l = ((r12 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 17; continue; }
            16 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); bb = 18; continue; }
            18 => { rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; r14 = (((r13 as u16)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; bb = 21; continue; }
            21 => { r13 = 0; fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 24; continue; } bb = 23; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); bb = 29; continue; }
            24 => { rsi = (crate::simd_ops::ru32(r15.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(0x6cu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0xdu64), ((xmm7 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (0x0u64) as u64); crate::simd_ops::wu16(rbp.wrapping_sub(0x11u64), ((r14 as u16)) as u16); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = ((rax as u32)) as u32 as u64; rcx = r15.wrapping_add(0x60); rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = 0x1; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x6cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(r15.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x68u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(r15.wrapping_add(0x60u64)))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu64(rcx, (0x0u64) as u64); crate::simd_ops::wu128(rcx, (xmm0) as u128); bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_add(0x7f); r9 = 0x4; rdx = (((r14 as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x58u64))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rbx); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 32; continue; }
            32 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 34; continue; }
            34 => { rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa0; const P1: u64 = 0x58; const P2: u64 = 0x68; const P3: u64 = 0x60; const P4: u64 = 0x2; const P5: u64 = 0x4; const P6: u64 = 0x14160a710; const P7: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rsi as u32)) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm2 = ((crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32) as u128; rcx = rdi; rdx = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0xb;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0xc;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd1d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0xa;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0xb8; const P2: u64 = 0x4; const P3: u64 = 0x5; const P4: u64 = 0xc0; const P5: u64 = 0x14160f0e0; const P6: u64 = 0x24; const P7: u64 = 0x29; const P8: u64 = 0x14160f420; const P9: u64 = 0xa8; const P10: u64 = 0x10; const P11: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; rbx = rcx; rdi = ((rdx as u32)) as u32 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbp = ((crate::simd_ops::ru8(rcx.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P3))) as u32) as u32 as u64; rsi = ((rdi.wrapping_mul(8)) as u64) as u64; r14 = (crate::simd_ops::ru32(rsi.wrapping_add(rcx))) as u32 as u64; bb = 5; continue; }
            4 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = ((rdi.wrapping_mul(8)) as u64) as u64; r14 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { r8 = 0; fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(rax).wrapping_add(P6))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rax).wrapping_add(P7))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = (((rbp as u8)) as u32) as u32 as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; r8 = ((r14 as u32)) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141608b80;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = rcx; rsi = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((rsi as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = ((rsi as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { r8 = rsi; r10 = rdi; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 6; continue; }
            6 => { rax = rsi; r11 = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = rax; fl_r = (0x4u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 7; continue; }
            7 => { r9 = rsi; rdx = rdi; r9 = ((r9).wrapping_sub(rdi)) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_sub(0x4u64)) as u64; r9 = ((r9).wrapping_shr(2u32)) as u64; fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; r10 = ((rdi.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu16(rcx.wrapping_add(0x8u64), ((r11 as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((r11 as u32)) as u32); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu16(rcx.wrapping_add(0x8u64), ((r11 as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((r11 as u32)) as u32); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu16(rcx.wrapping_add(0x8u64), ((r11 as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((r11 as u32)) as u32); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu16(rcx.wrapping_add(0x8u64), ((r11 as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((r11 as u32)) as u32); bb = 16; continue; }
            16 => { rdx = ((rdx).wrapping_add(0x40u64)) as u64; fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 17; continue; }
            17 => { fl_l = r10; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 18; continue; }
            18 => { rdx = r10; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = r10; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rcx, r11); crate::simd_ops::wu16(rcx.wrapping_add(0x8u64), ((r11 as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((r11 as u32)) as u32); bb = 21; continue; }
            21 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815b40; const P1: u64 = 0x14158e9f0; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x38; const P5: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rbx = rcx; r8 = rsp.wrapping_add(0x20); rcx = P0; rsi = ((r9 as u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P0; r9 = ((rsi as u32)) as u32 as u64; r8 = rbp; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = P0; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi as u32)) as u32 as u64; bb = 8; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx.wrapping_add(P4), rsi); bb = 7; continue; }
            7 => { rax = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rdi as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141609a70; const P1: u64 = 0xb0; const P2: u64 = 0x14160e140; const P3: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = r8;
    rbp = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdi = ((rax as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; r8 = rsi; rdx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815b40; const P1: u64 = 0x14158e9f0; const P2: u64 = 0x141535d60; const P3: u64 = 0x141607e30; const P4: u64 = 0x141608d60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::lk_add_u64(rdx, (0x4u64) as u64); r12 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = r8; rbx = rdx; r14 = rcx; rbp = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rdx, rax); rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; r15 = rax.wrapping_add(0x4); crate::simd_ops::wu64(rdx, r15); fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { r8 = rsp.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); rdx = ((rbp as u32)) as u32 as u64; rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P0; r9 = ((rsi as u32)) as u32 as u64; r8 = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdx = P0; r8 = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u64(rbx, (rsi) as u64); rax = ((r15 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rdi, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 37; continue; }
            5 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { crate::simd_ops::wu32(r14.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x38u64), r15); crate::simd_ops::lk_add_u64(rbx, (rsi) as u64); fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rdi, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 10; continue; }
            8 => { crate::simd_ops::wu32(r14.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::lk_add_u64(rbx, (rsi) as u64); fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rdi, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 10; continue; }
            9 => { crate::simd_ops::lk_add_u64(rdx, (rsi) as u64); fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(r8, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); rbp = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x48u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x4cu64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rdx = ((rbp as u32)) as u32 as u64; rcx = r14.wrapping_add(0x40); rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = 0x34; bb = 37; continue; }
            15 => { rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x48u64), ((rax as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = r12; r8 = rbp; bb = 17; continue; }
            17 => { crate::simd_ops::lk_add_u64(rbx, (1) as u64); rdx = rdx.wrapping_add(0x4); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rax).wrapping_sub(0x4u64), ((rcx as u32)) as u32); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { r8 = r14; rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = r14.wrapping_add(0x18); r8 = rdi; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbx, rcx); rsi = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x6cu64))) as u32 as u64; rcx = r14.wrapping_add(0x60); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { r15 = 0x1; rbp = ((r15 as u32)) as u32 as u64; bb = 24; continue; }
            23 => { rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rbp = 0x34; r15 = 0x1; rbp = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r15 as u32) } else { (rbp as u32) }) as u32 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 24; continue; }
            24 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 25; continue; }
            25 => { rdi = r14.wrapping_add(0x60); r14 = rsi; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r8 = rax.wrapping_add(0x1); r9 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r8); r9 = ((((r9 as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { rdx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r8); rcx = ((((rcx as u32)) & (0x7fu32))) as u32 as u64; r9 = ((rcx as u32)) as u32 as u64; r9 = ((((r9 as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 28; continue; } bb = 29; continue; }
            29 => { rcx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rax = r8.wrapping_add(0x1); crate::simd_ops::wu64(rbx, rax); xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x58u64), ((r9 as u16)) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x5au64), ((rcx as u8)) as u8); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r15 as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; bb = 32; continue; }
            32 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((r8 as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rdx, rcx); bb = 35; continue; }
            35 => { fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 36; continue; }
            36 => { rax = ((rbp as u32)) as u32 as u64; bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x60; const P2: u64 = 0xc; const P3: u64 = 0x141608b80; const P4: u64 = 0x8; const P5: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = ((rcx.wrapping_add(P1)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x2u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r8 as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x28u64), ((rdx as u16)) as u16); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu16(rcx.wrapping_add(P4), ((rax as u16)) as u16); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); bb = 5; continue; }
            5 => { rax = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; fl_l = rcx; fl_r = rcx; crate::simd_ops::wu128(rcx, (xmm0) as u128); rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rax; fl_r = rax; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609de0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x30; const P4: u64 = 0x141609ea0; const P5: u64 = 0x58; const P6: u64 = 0x143930f70; const P7: u64 = 0x1415d9620;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rsi = r8; rdi = rdx; rbx = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P3), (xmm0) as u128); bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P6)) as u64; r9 = rsp.wrapping_add(0x48); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x2u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); r8 = ((r8).wrapping_add(0x50u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu16(rsp.wrapping_add(0x50u64), (0xff00u16) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x78; const P2: u64 = 0x8; const P3: u64 = 0x1415cca50; const P4: u64 = 0x10; const P5: u64 = 0x143930fa0; const P6: u64 = 0x1415c4320; const P7: u64 = 0xc; const P8: u64 = 0x4; const P9: u64 = 0x14160a240;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; r8 = ((rdi.wrapping_add(P4)) as u64) as u64; rcx = (crate::simd_ops::ru64(P5)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = 0x1; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P7))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P8))) as u32 as u64; rcx = rdi; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x141609f90;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdi = rcx;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rdi.wrapping_add(P0), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141609f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x50; const P2: u64 = 0x58; const P3: u64 = 0x48; const P4: u64 = 0x10; const P5: u64 = 0x30; const P6: u64 = 0x70; const P7: u64 = 0x1426452b0; const P8: u64 = 0xa0; const P9: u64 = 0x1426452a8; const P10: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rsi = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rsi); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P3))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P4))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(P0))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P0), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm0) as u128); bb = 2; continue; }
            2 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = ((rsi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P8))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsi.wrapping_add(P8), rbx); crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); crate::simd_ops::wu64(rsi.wrapping_add(P8), rbx); bb = 6; continue; }
            6 => { rcx = ((rsi.wrapping_add(P6)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(P10), (1) as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160a090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ccba0; const P1: u64 = 0x1416093a0; const P2: u64 = 0x143815b50; const P3: u64 = 0x1426452b0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x80u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm1) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); bb = 2; continue; }
            2 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rbp); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { rcx = rdi.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; rdx = rbp; rax = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rcx = rdi.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rbp); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((rbp as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x6cu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), r8); bb = 17; continue; }
            16 => { crate::simd_ops::wu64(rdx.wrapping_add(0x70u64), r8); bb = 17; continue; }
            17 => { rcx = rdi.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160a1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x30; const P2: u64 = 0x10; const P3: u64 = 0x20; const P4: u64 = 0x1415f2900; const P5: u64 = 0xb8; const P6: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((crate::simd_ops::ru8(rax.wrapping_add(P3))) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; r8 = (((rax as u8)) as u32) as u32 as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((crate::simd_ops::ru8(rax.wrapping_add(P6))) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; r8 = (((rax as u8)) as u32) as u32 as u64; return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    r8 = (((rax as u8)) as u32) as u32 as u64;
    rcx = ((r9 as u32)) as u32 as u64;
    return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160a220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffff68; const P1: u64 = 0xffffffffffffff88; const P2: u64 = 0x1415f28a0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = ((rdx as u32)) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160a240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x141608b80;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rcx; rbp = ((r9 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; r11 = ((r8 as u32)) as u32 as u64; r15 = ((rdx as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rsp.wrapping_add(0x78); r9 = 0x4; rdx = (((rbp as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; bb = 6; continue; }
            5 => { xmm0 = 0u128; bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru32(r14.wrapping_add(0x68u64))) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 7; continue; }
            7 => { r9 = r8; bb = 8; continue; }
            8 => { rcx = ((crate::simd_ops::ru16(r8.wrapping_add(0x8u64))) as u32) as u32 as u64; rdx = r9; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32) as u128; rax = r8; crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x60u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; bb = 12; continue; }
            12 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { xmm1 = ((crate::simd_ops::ru32(r8.wrapping_add(0xcu64))) as u32) as u128; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 17; continue; }
            16 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); bb = 18; continue; }
            18 => { r10 = (crate::simd_ops::ru32(r14.wrapping_add(0x68u64))) as u32 as u64; r8 = ((r8).wrapping_add(0x10u64)) as u64; rcx = ((r10 as u32)) as u32 as u64; r9 = rdx; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 19; continue; }
            19 => { rsi = ((r10 as u32)) as u32 as u64; r8 = rdx; bb = 20; continue; }
            20 => { rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 21; continue; }
            21 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32) as u128; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x6cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r11 as u32)) as u32); crate::simd_ops::wu16(rsp.wrapping_add(0x28u64), ((rbp as u16)) as u16); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = r14.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x6cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x68u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x60u64)))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x78u64)))).to_bits() as u128); bb = 28; continue; }
            27 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x78u64)))).to_bits() as u128); bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; r8 = rsp.wrapping_add(0x50); r9 = 0x4; rdx = (((rbp as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 30; continue; }
            30 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160a410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815b50; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x1415c2cc0; const P4: u64 = 0x141609810; const P5: u64 = 0x141609ea0; const P6: u64 = 0x143930f70; const P7: u64 = 0x1415d9620; const P8: u64 = 0x1416084f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r15 = r9; r14 = r8; rdi = rdx; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; bb = 35; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x70); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), r12); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rbx); bb = 6; continue; }
            6 => { rcx = rdi.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rcx = rbx.wrapping_add(0x78); rdx = ((rdx).wrapping_add(0x50u64)) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x80u64), ((r12 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x68u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru64(r9.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x58u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(r9.wrapping_add(0x68u64))) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = ((rdi.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rdx = (if (((fl_l as u8)) == ((fl_r as u8))) { (rdi as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64; rcx = r9.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rsi = rax; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = ((r12 as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { rdx = ((((rdx as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as i32) as u32) as u32 as u64; fl_l = (((rdi as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rsi = rbx.wrapping_add(0x60); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = ((rdi as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 32; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; r8 = ((r12 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x68u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; rdx = ((r8 as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; crate::simd_ops::wu16(rdx.wrapping_add(0x8u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x68u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 20; continue; }
            20 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x8u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r14.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); bb = 23; continue; }
            23 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x1c0u64))) as u64; rdx = rbp.wrapping_sub(0x38); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r12); crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r12); crate::simd_ops::wu16(rbp.wrapping_sub(0x30u64), (0xff00u16) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rax = r14.wrapping_add(0x78); bb = 30; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x48u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { xmm0 = (crate::simd_ops::ru128(r15)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r15.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r15.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); bb = 28; continue; }
            28 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(r15)) as u64; xmm0 = 0u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), (0x2u32) as u32); crate::simd_ops::wu16(rbp.wrapping_sub(0x30u64), (0xff00u16) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), r12); bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(P6)) as u64; r9 = rbp.wrapping_sub(0x38); r8 = ((r8).wrapping_add(0x50u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rbp.wrapping_sub(0x50); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((r12 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            32 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; bb = 35; continue; }
            35 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160a710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141609190; const P1: u64 = 0x1415c2f80; const P2: u64 = 0x1434268d0; const P3: u64 = 0x143930fa0; const P4: u64 = 0x1415c4320; const P5: u64 = 0x141608b80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); r9 = rbp.wrapping_add(0x77); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), rsi); r8 = rbp.wrapping_add(0x7f); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r14); r14 = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm7) as u128); xmm7 = (xmm2) as u128; r13 = (((rdx as u16)) as u32) as u32 as u64; rdx = (((r13 as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm7 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x77u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rsi = 0x1; r15 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x77u64))) as u32) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x50u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); r8 = ((r13 as u32)) as u32 as u64; rdx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xcu64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; bb = 7; continue; }
            6 => { xmm0 = 0u128; bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); bb = 8; continue; }
            8 => { rcx = r14.wrapping_add(0x10); rdi = r14.wrapping_add(0x60); r14 = rcx; bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = r14; rcx = (crate::simd_ops::ru64(P3)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; xmm6 = (xmm0) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x5u64), ((xmm6 as u32)) as u32); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x9u64), ((r13 as u16)) as u16); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0xdu64), ((rdx as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((rcx as u32)) as u32 as u64; rax = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = rdi; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rcx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rcx as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 16; continue; }
            15 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); bb = 17; continue; }
            17 => { rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rsi = 0x1; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; rsi = 0x1; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; bb = 20; continue; }
            20 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 21; continue; }
            21 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x7fu64), ((xmm0 as u32)) as u32); bb = 28; continue; }
            23 => { rdi = (crate::simd_ops::ru32(r14.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(0x6cu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x5u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (0x0u64) as u64); crate::simd_ops::wu16(rbp.wrapping_sub(0x9u64), ((r13 as u16)) as u16); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = ((rax as u32)) as u32 as u64; rcx = r14.wrapping_add(0x60); rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rsi as u32) } else { (rdx as u32) }) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x6cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x68u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x60u64)))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x11u64))) as u128; crate::simd_ops::wu64(rcx, (0x0u64) as u64); crate::simd_ops::wu128(rcx, (xmm0) as u128); bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_add(0x7f); r9 = 0x4; rdx = (((r13 as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rsp = ((rsp).wrapping_add(0x88u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160a980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142689150; const P1: u64 = 0x20; const P2: u64 = 0x28; const P3: u64 = 0x1415242e0; const P4: u64 = 0x2c; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x3c; const P8: u64 = 0x10; const P9: u64 = 0x142689140; const P10: u64 = 0x18; const P11: u64 = 0x1439315b8; const P12: u64 = 0x1439315b0; const P13: u64 = 0x1439315a8; const P14: u64 = 0x142688f88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rdi as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rdi as u32)) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rdi as u32)) as u32); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; rax = P9; crate::simd_ops::wu64(rbx, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P10), rax); bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P11)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(P11, rax); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(P12)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::lk_sub_u32(P13, (1) as u32); crate::simd_ops::wu64(P12, rax); rax = P14; crate::simd_ops::wu64(rbx, rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160aa40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ea4; const P1: u64 = 0x1415d7950; const P2: u64 = 0x14158c580; const P3: u64 = 0x14158d2f0; const P4: u64 = 0x14160b590; const P5: u64 = 0x14160b2c0; const P6: u64 = 0x14160b200; const P7: u64 = 0x1415d5250;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rbp = rsp.wrapping_add(0x20); r12 = ((crate::simd_ops::ru16(rdx.wrapping_add(0x38u64))) as u32) as u32 as u64; r13 = r9; rax = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm6) as u128); rax = ((rax).wrapping_shl(4u32)) as u64; r9 = r8; r15 = rdx; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((r12 as u32)) as u32); r14 = rcx; r10 = rax.wrapping_add(0xf); fl_l = r10; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = (0xffffffffffffff0u64) as u64; bb = 2; continue; }
            2 => { r10 = (((r10) & (0xfffffffffffffff0u64))) as u64; rax = r10; rbx = (crate::simd_ops::ru64(r13.wrapping_add(0xd0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(r13.wrapping_add(0xd8u64))) as u32 as u64; rsi = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rsi); rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 52; continue; } bb = 3; continue; }
            3 => { xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rdi = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(r9.wrapping_add(0x28u64))) as u32 as u64; r8 = ((rdi as u32)) as u32 as u64; r11 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r8 as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = r11; r10 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 7; continue; }
            7 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x20u64))) as u64; rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdx = ((((r8 as u32)) as i32 as i64) as u64) as u64; r10 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; bb = 10; continue; }
            10 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = rdi; rcx = 0x30; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { r10 } else { rax }) as u64; fl_l = rax; fl_r = rax; rax = 0x10; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rcx = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x30u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x28u64))) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rcx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { fl_l = rcx; fl_r = rdx; r8 = rcx.wrapping_add(0x8); r8 = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { r8 }) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 16; continue; }
            16 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r9 = ((r12 as u32)) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; r9 = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (r9 as u32) }) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = rdi; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = rcx.wrapping_add(0x10); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(rsi).wrapping_sub(0x10u64), (xmm0) as u128); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 19; continue; }
            19 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 27; continue; } bb = 20; continue; }
            20 => { rcx = ((r9 as u32)) as u32 as u64; r10 = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; r10 = ((r10).wrapping_add(r10)) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rdx = ((r12 as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = rcx.wrapping_add(0x10); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(r10.wrapping_mul(8)))) as u128; crate::simd_ops::wu128(rcx.wrapping_sub(0x10u64), (xmm0) as u128); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { bb = 27; continue; }
            23 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            24 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = rsi.wrapping_add(0x8); rcx = ((r12 as u32)) as u32 as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), (0x0u64) as u64); crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), (0x42c80000u32) as u32); rax = rax.wrapping_add(0x10); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0xe0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = r13.wrapping_add(0x20); bb = 30; continue; }
            29 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x48u64))) as u32 as u64; r8 = ((rdi as u32)) as u32 as u64; r11 = (crate::simd_ops::ru64(rbx)) as u64; rdi = rax.wrapping_add(0x40); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; bb = 32; continue; }
            32 => { rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r8 as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = r11; r10 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 46; continue; } bb = 33; continue; }
            33 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 32; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = ((((r8 as u32)) as i32 as i64) as u64) as u64; r10 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 35; continue; }
            35 => { rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rax = ((rdi as u32)) as u32 as u64; r15 = ((rdi as u32)) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { r10 } else { rax }) as u64; fl_l = rax; fl_r = rax; crate::simd_ops::wu8(rbp, (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 50; continue; } bb = 36; continue; }
            36 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; fl_l = ((r12).wrapping_add(0x8u64)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; rbx = ((crate::simd_ops::ru8(rbp)) as u32) as u32 as u64; bb = 37; continue; }
            37 => { rdx = (((rbx as u8)) as u32) as u32 as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(r14)) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rcx = rsi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 48; continue; } bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 48; continue; } bb = 40; continue; }
            40 => { rax = ((rcx as u32)) as u32 as u64; rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((rax as u32)) as u32); fl_l = ((rdi).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x20u64)))) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 48; continue; } bb = 41; continue; }
            41 => { r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x3cu64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), r8); crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x4cu64), r8); crate::simd_ops::wu64(rdi.wrapping_add(0x54u64), r8); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), r8); crate::simd_ops::wu32(rdi.wrapping_add(0x80u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x40u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x5cu64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x60u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(0x64u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), (0x3f800000u64) as u64); rsi = (crate::simd_ops::ru64(r14)) as u64; r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 44; continue; } bb = 42; continue; }
            42 => { rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 47; continue; } bb = 43; continue; }
            43 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 47; continue; } bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x30u64)))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 47; continue; } bb = 45; continue; }
            45 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = rdi; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu8(rax, ((rcx as u8)) as u8); xmm0 = (crate::simd_ops::ru128(r13)) as u128; rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r13.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru32(r12.wrapping_sub(0x4u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x4cu64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(r12.wrapping_sub(0x8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(r12)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r12.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x64u64), ((rdx as u32)) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rcx); rcx = r9; crate::simd_ops::wu32(rdi.wrapping_add(0x5cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), (0x0u64) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x68u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; bb = 48; continue; }
            46 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 35; continue; }
            47 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0x28u64), (1) as u32)) as u32 as u64; bb = 48; continue; }
            48 => { r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; r12 = ((r12).wrapping_add(0x10u64)) as u64; r13 = ((r13).wrapping_add(0x38u64)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 37; continue; } bb = 49; continue; }
            49 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; r12 = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; bb = 50; continue; }
            50 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x98u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(0xd8u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0xd0u64))) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rbx); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 52; continue; }
            52 => { xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x30u64))) as u128; rsp = rbp.wrapping_add(0x48); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160af00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160b590; const P1: u64 = 0x141605340; const P2: u64 = 0x14160aa40; const P3: u64 = 0x1415d7950; const P4: u64 = 0x143426fd0; const P5: u64 = 0x14160b740; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x44u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0xcu64), (0x7f7fffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x4au64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r9 = rax; r8 = rbp.wrapping_sub(0x40); rdx = rbx; rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdi); rdi = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rax.wrapping_add(0x20u64)))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x48u64))) as u16 as u64; fl_r = ((rsi as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 10; continue; } bb = 7; continue; }
            7 => { xmm3 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0xcu64))) as u32) as u128; fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P4)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm3 = 0u128; bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32) as u128; r8 = rax; rdx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x7cu64), (0xfffffffeu32) as u32); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = rbp.wrapping_sub(0x40); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), ((rsi as u32)) as u32); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160b040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143815b60; const P2: u64 = 0x143931d3c; const P3: u64 = 0x142689164; const P4: u64 = 0x14160bcd0; const P5: u64 = 0x14160b2c0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((r15 as u32)) as u32); r10 = r8; rax = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; rbx = rdx; crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((rax as u8)) as u8); rbp = rcx; rax = ((crate::simd_ops::ru8(P2)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = rsp.wrapping_add(0x70); rcx = r10; r8 = rsp.wrapping_add(0x68); rdx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r15 as u32)) as u32); rsi = ((r15 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdi = (((rdx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x20u64)))) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), r15); crate::simd_ops::wu32(rcx.wrapping_add(0x38u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x4cu64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x54u64), r15); crate::simd_ops::wu32(rcx.wrapping_add(0x5cu64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x68u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x80u64), ((r15 as u32)) as u32); bb = 11; continue; }
            11 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x54u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x54u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) < f32::from_bits(xmm6 as u32) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(xmm6 as u32) }).to_bits() as u128); xmm6 = (xmm1) as u128; bb = 13; continue; }
            13 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x28u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 15; continue; }
            15 => { xmm0 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160b200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = ((rdx as u32)) as u32 as u64; rdx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(rbx), ((rdx as u8)) as u8); rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160b2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rbx = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { r9 = (0xffffffffffffffccu64) as u64; rdx = rdi.wrapping_add(0x34); r9 = ((r9).wrapping_sub(rdi)) as u64; r10 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { r8 = rdx.wrapping_sub(0x34); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdx.wrapping_sub(0x4u64), r10); crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), r10); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r10); crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), (0x42c80000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x30u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x34u64), ((r10 as u32)) as u32); crate::simd_ops::wu64(rdx.wrapping_add(0x3cu64), r10); crate::simd_ops::wu64(rdx.wrapping_add(0x44u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rdx.wrapping_add(0x4cu64), ((r10 as u32)) as u32); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((r9.wrapping_add(rdx)) as u64) as u64; rdx = ((rdx).wrapping_add(0x88u64)) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x70u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x80u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbx, rdi); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160b460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524350; const P1: u64 = 0x141524320; const P2: u64 = 0x14183acbc; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = rdi; bb = 5; continue; }
            3 => { rcx = 0x4; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { r8 = rdx; r9 = ((rcx.wrapping_add(0xcu64)) as u32) as u32 as u64; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 21; continue; }
            7 => { r8 = 0x10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { r8 = ((rsi.wrapping_mul(8)) as u32) as u32 as u64; rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            9 => { fl_l = rdx; fl_r = (0x20u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rbx = rdi; bb = 12; continue; }
            11 => { rcx = 0x4; r8 = ((rcx.wrapping_add(0xcu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 14; continue; }
            14 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 15; continue; }
            15 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(rdx.wrapping_mul(8)), rcx); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = 0x4; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((rbp as u32)) as u32); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160b590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160b460; const P1: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; r13 = (((rdx as u8)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32 as u64; rbx = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x28u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; rcx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x30u64))) as u64; r14 = (((rcx) as i64).wrapping_mul((0x88u64) as i64) as u64) as u64; r15 = ((crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u32) as u32 as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0x20u64)))) as u64; rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x70u64))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; bb = 4; continue; }
            4 => { rcx = ((rdx as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rdi as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru64(r9.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 5; continue; }
            5 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rdi as u32) } else { (rax as u32) }) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rcx = ((((rdi as u32)) as i32 as i64) as u64) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rbp = ((rdi as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x8); rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x38u64), (1) as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; bb = 17; continue; }
            13 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 17; continue; }
            17 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rcx, rsi); bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x4cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 20; continue; }
            20 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); bb = 22; continue; }
            22 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0x50u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x44u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rbx.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); bb = 25; continue; }
            25 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x54u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((rax as u32)) as u32); bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x28u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 28; continue; }
            28 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x4cu64), ((r13 as u8)) as u8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { crate::simd_ops::wu8(rcx.wrapping_add(0x4cu64), ((r13 as u8)) as u8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160b740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930fa0; const P1: u64 = 0x1415c4d70; const P2: u64 = 0x14268da20; const P3: u64 = 0x1415d75c0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x48u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rsi = r8; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdi = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); xmm7 = (xmm3) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32) as u128; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u32) as u32 as u64; xmm6 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(3u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm3 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32) as u128; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = 0x4; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(6u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm3 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32) as u128; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = 0x7; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { r9 = rsp.wrapping_add(0x88); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), (0x0u32) as u32); r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), (0x0u32) as u32); rdx = rdi; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32) as u128; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = 0x2; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(2u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32) as u128; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = 0x3; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(4u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm3 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32) as u128; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(5u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { xmm3 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32) as u128; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = 0x6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(7u32)) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = rsp.wrapping_add(0x20); xmm3 = (xmm7) as u128; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x49u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { xmm3 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32) as u128; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = 0x9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160b950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x4; const P2: u64 = 0x141524330; const P3: u64 = 0x14164cfd0; const P4: u64 = 0x8; const P5: u64 = 0x58; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = 0xe8; rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
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
pub unsafe extern "win64" fn fn_14160ba50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d3d; const P1: u64 = 0x141591570; const P2: u64 = 0x141524330; const P3: u64 = 0x14268f070; const P4: u64 = 0x14164d790; const P5: u64 = 0x14268f0b8; const P6: u64 = 0x14268f028; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = 0x10; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (0x0u64) as u64); rax = P3; bb = 14; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((rax as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = 0x10; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (0x0u64) as u64); rax = P5; bb = 14; continue; }
            10 => { rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x160u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x18cu64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 12; continue; }
            12 => { rdx = ((rbx.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu8(rax.wrapping_add(0x1fu64), (0x0u8) as u8); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (0x0u64) as u64); rax = P6; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), (0x0u32) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0x1cu64), (0x0u16) as u16); crate::simd_ops::wu8(rbx.wrapping_add(0x1eu64), (0x0u8) as u8); bb = 14; continue; }
            14 => { rdx = rsi; crate::simd_ops::wu64(rbx, rax); rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160bb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141591570;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160bba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x13e; const P2: u64 = 0x38; const P3: u64 = 0x14164d550; const P4: u64 = 0xec; const P5: u64 = 0x143426564; const P6: u64 = 0x142673708; const P7: u64 = 0x1426736fc; const P8: u64 = 0x142673700; const P9: u64 = 0x1426736ec; const P10: u64 = 0x1426736e8; const P11: u64 = 0x1426736f0; const P12: u64 = 0x14164d400;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P6)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm3 = 0u128; bb = 7; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; xmm3 = (xmm0) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P9))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(crate::simd_ops::ru32(P11))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x30u64)))).to_bits() as u128); bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x114u64))) as u32 as u64; rcx = r9; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160bc90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x13e; const P2: u64 = 0x38; const P3: u64 = 0x14164d550;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } bb = 2; continue; }
            2 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160bcd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x30; const P2: u64 = 0x8; const P3: u64 = 0x13e;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = r9; rdi = r8; rbx = rdx; rsi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rbp = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_or_u8(rbx, (0x1u8) as u8); crate::simd_ops::wu32(r14, (0x0u32) as u32); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_or_u8(rbx, (0x4u8) as u8); fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_or_u8(rdi, (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 6; continue; }
            4 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_or_u8(rbx, (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu32(r14, (0x3f800000u32) as u32); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_or_u8(rdi, (0x8u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160bd70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18c; const P2: u64 = 0x18d;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64));
    rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_sub(1)) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if (((fl_l as u8)) <= ((fl_r as u8))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160bda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x114; const P2: u64 = 0x143815b3c; const P3: u64 = 0xec;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x0;
    if (((fl_l as u32)) == ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64;
    if !((f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160be70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x50; const P2: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu8(r8.wrapping_add(P2), (0x1u8) as u8); rax = 0x1; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); return rax; }
    rax = 0x2;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160beb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x38; const P2: u64 = 0x14164dbd0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = rcx; rcx = r8; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160bf40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x50; const P2: u64 = 0x2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu8(rdx.wrapping_add(P2), (0x0u8) as u8); }
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0x0u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160bf70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f90; const P1: u64 = 0x14153f690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = rcx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdi).wrapping_add(0x220u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 10; continue; }
            2 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(P0)) as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rdi = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdi = rsi; bb = 10; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi).wrapping_add(0x68u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rdx, ((rsi as u32)) as u32); crate::simd_ops::wu8(rdx.wrapping_add(0x4u64), (0x8u8) as u8); bb = 16; continue; }
            16 => { rsi = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rdx, rbx); bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rsi; fl_r = rsi; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160c070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143930fa0; const P2: u64 = 0x1415c5690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rsi = rdx;
    rdi = rcx;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32))) { loop { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsi; rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rbx.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; if !(((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32))) { break; } } crate::simd_ops::wu32(rdi.wrapping_add(P0), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rbx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160c0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d48; const P1: u64 = 0x8; const P2: u64 = 0x143931d50; const P3: u64 = 0x143931d60; const P4: u64 = 0x143931d68; const P5: u64 = 0x143931d58; const P6: u64 = 0x143931d6c; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = rcx; rax = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(P0, rbx); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rdi, rbx); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P2)) as u64; r8 = (crate::simd_ops::ru64(P3)) as u64; fl_l = rax; fl_r = rcx; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rcx }) as u64; crate::simd_ops::wu64(P2, rcx); fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rax, rcx); fl_l = (crate::simd_ops::lk_sub_u32(P6, (1) as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(P5, rax); bb = 11; continue; }
            9 => { rdx = rax; rcx = 0x5; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(P6, (1) as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; bb = 11; continue; }
            10 => { rdi = rax; bb = 11; continue; }
            11 => { rax = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160c1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d78; const P1: u64 = 0x70; const P2: u64 = 0x143931d70; const P3: u64 = 0x1415edf30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; rbx = rdx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rdx; rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P0, rax); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(0x70u64, rax); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = rcx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rax }) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P2, rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(P0)) as u64; bb = 2; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160c240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d58; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffed; const P3: u64 = 0x141524330; const P4: u64 = 0x143931d50; const P5: u64 = 0x143931d48; const P6: u64 = 0x143931d6c; const P7: u64 = 0x143931d40; const P8: u64 = 0xc; const P9: u64 = 0x8; const P10: u64 = 0x143931d44;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(P0)) as u64; rdi = rdx; rbx = ((rcx as u32)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rax, r8); crate::simd_ops::wu64(P0, rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(P5, r9); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rax, r9); r9 = (crate::simd_ops::ru64(P0)) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(P4, r9); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(P0, rax); crate::simd_ops::wu64(r9, (0x0u64) as u64); crate::simd_ops::lk_add_u32(P6, (1) as u32); rcx = (crate::simd_ops::ru64(P4)) as u64; fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { crate::simd_ops::wu64(rcx, rdi); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(P8), ((rbx as u8)) as u8); crate::simd_ops::lk_and_u32(rcx.wrapping_add(P8), (0xfffffeffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32); rax = 0x1; rcx = ((crate::simd_ops::ru8(P10)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (if (((fl_l as u32)) <= ((fl_r as u32))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu8(P10, ((rcx as u8)) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160c320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d40; const P1: u64 = 0x141524330; const P2: u64 = 0x143931d60; const P3: u64 = 0x143931d68; const P4: u64 = 0x143931d58; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x3000;
    crate::simd_ops::wu32(P0, (0x0u32) as u32);
    rcx = 0x5;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(P2, rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0x34; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    crate::simd_ops::wu32(P3, (0x200u32) as u32);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(P4, rax);
    loop { rcx = ((rax.wrapping_add(P5)) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rax, rcx); r8 = rax; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; rax = rcx; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    crate::simd_ops::wu64(r8, (0x0u64) as u64);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160c3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d44; const P1: u64 = 0x143931d48; const P2: u64 = 0x143931d50; const P3: u64 = 0x143931d60; const P4: u64 = 0x143931d68; const P5: u64 = 0x143931d58; const P6: u64 = 0x1415242e0; const P7: u64 = 0x143931d6c; const P8: u64 = 0x1415940e0; const P9: u64 = 0x1415ee0d0; const P10: u64 = 0x14160c7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x298u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 24; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x304u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x378u64))) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P1)) as u64; rbp = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(P1, rbp); bb = 10; continue; }
            9 => { crate::simd_ops::wu64(rsi, rbp); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(P2)) as u64; rdx = (crate::simd_ops::ru64(P3)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rsi } else { rax }) as u64; crate::simd_ops::wu64(P2, rax); fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(P5, rbx); bb = 14; continue; }
            13 => { rdx = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::lk_sub_u32(P7, (1) as u32); rdx = 0x1; rcx = rdi; rbx = rbp; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            15 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P1)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(P1, rdi); bb = 19; continue; }
            18 => { crate::simd_ops::wu64(rsi, rdi); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(P2)) as u64; rdx = (crate::simd_ops::ru64(P3)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rsi } else { rax }) as u64; crate::simd_ops::wu64(P2, rax); fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(P4)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_sub_u32(P7, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P5, rbx); rbx = rdi; bb = 25; continue; }
            22 => { rdx = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(P7, (1) as u32)) as u32 as u64; rbx = rdi; bb = 25; continue; }
            23 => { rsi = rbx; rbx = rdi; bb = 25; continue; }
            24 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 25; continue; }
            25 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 26; continue; }
            26 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(P0, (0x0u8) as u8); bb = 28; continue; }
            28 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160c570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d78; const P1: u64 = 0x143426780; const P2: u64 = 0x143427068; const P3: u64 = 0x143931d48; const P4: u64 = 0x143931d50; const P5: u64 = 0x143931d60; const P6: u64 = 0x143931d68; const P7: u64 = 0x143931d58; const P8: u64 = 0x1415242e0; const P9: u64 = 0x143931d6c; const P10: u64 = 0x143931d70; const P11: u64 = 0x1415edf30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rsi = (crate::simd_ops::ru64(P0)) as u64; r12 = 0; fl_l = 0; fl_r = 0; r15 = ((rcx as u32)) as u32 as u64; r14 = ((r12 as u32)) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rcx = 0x304; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); rbp = rsi; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r13); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); rdx = ((rcx.wrapping_sub(0x58u64)) as u32) as u32 as u64; xmm6 = ((crate::simd_ops::ru32(P1)) as u32) as u128; r13 = ((rcx.wrapping_add(0x74u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); r9 = ((rcx.wrapping_add(0x24u64)) as u32) as u32 as u64; xmm7 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm8) as u128); xmm8 = 0u128; bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = r8; rax = r8.wrapping_add(0x2ec); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = r8; rax = r8.wrapping_add(0x360); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r13 } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 4; continue; }
            4 => { r9 = (crate::simd_ops::ru64(P3)) as u64; rdi = r12; rdx = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; rax = rcx.wrapping_add(0x18); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rax }) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(P3, rbx); bb = 10; continue; }
            9 => { crate::simd_ops::wu64(rdi, rbx); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(P4)) as u64; r8 = (crate::simd_ops::ru64(P5)) as u64; fl_l = rdx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rax }) as u64; crate::simd_ops::wu64(P4, rax); fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu64(rdx, rax); crate::simd_ops::wu64(P7, rdx); bb = 14; continue; }
            13 => { rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_sub_u32(P9, (1) as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru64(P3)) as u64; bb = 16; continue; }
            15 => { rdi = rdx; bb = 16; continue; }
            16 => { rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 17; continue; }
            17 => { fl_l = rsi; fl_r = (crate::simd_ops::ru64(P0)) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(P0, rcx); bb = 20; continue; }
            19 => { crate::simd_ops::wu64(r14.wrapping_add(0x70u64), rcx); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(P10)) as u64; fl_l = rsi; fl_r = rax; rsi = rcx; rcx = rbp; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { r14 } else { rax }) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P10, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0x328; bb = 28; continue; }
            21 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 27; continue; } bb = 22; continue; }
            22 => { xmm1 = 0u128; rcx = r8.wrapping_add(0x294); fl_l = r8; fl_r = r8; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((r15) as i64) as f32).to_bits() as u128); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rcx }) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { xmm0 = (xmm6) as u128; bb = 25; continue; }
            24 => { xmm0 = (xmm7) as u128; bb = 25; continue; }
            25 => { fl_l = r8; fl_r = r8; rdx = r8.wrapping_add(0x310); rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r9 } else { rdx }) as u64; rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); rax = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); bb = 27; continue; }
            27 => { r14 = rsi; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; bb = 28; continue; }
            28 => { rdx = 0x2ac; rbp = rsi; rcx = ((rdx.wrapping_add(0x58u64)) as u32) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 29; continue; }
            29 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 30; continue; }
            30 => { rsp = ((rsp).wrapping_add(0x58u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14160c7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x143931d48; const P2: u64 = 0x1415f2b00; const P3: u64 = 0x1415f1c50; const P4: u64 = 0x143931d50; const P5: u64 = 0x143931d60; const P6: u64 = 0x143931d68; const P7: u64 = 0x143931d58; const P8: u64 = 0x1415242e0; const P9: u64 = 0x143931d6c; const P10: u64 = 0x141601820; const P11: u64 = 0x141601300; const P12: u64 = 0x1416015f0; const P13: u64 = 0x141602720; const P14: u64 = 0x141601670;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = ((P0) as u64) as u64; bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(P1)) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdi = (rbx) as u64; r14 = (r15) as u64; bb = 4; continue; }
            4 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((((rax as u32)) >> (8u32)) & 1) as u64; if ((fl_l as u32) != 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x304u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x378u64))) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x298u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3fu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { fl_l = (crate::simd_ops::lk_or_u32(rdi.wrapping_add(0x14u64), (0x100u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 14; continue; }
            14 => { r14 = (rdi) as u64; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r15 = (rbx) as u64; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 18; continue; }
            18 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 19; continue; }
            19 => { bb = 2; continue; }
            20 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; r14 = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((((rcx as u32)) >> (8u32)) & 1) as u64; if ((fl_l as u32) != 0) { bb = 33; continue; } bb = 21; continue; }
            21 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) < ((fl_r as u8))) { bb = 31; continue; } bb = 22; continue; }
            22 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 23; continue; }
            23 => { fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(P1)) as u64 as u64; rsi = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(P1, (rsi) as u64); bb = 26; continue; }
            25 => { crate::simd_ops::wu64(r15, (rsi) as u64); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(P4)) as u64; rdx = (crate::simd_ops::ru64(P5)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rax }) as u64; crate::simd_ops::wu64(P4, (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); crate::simd_ops::wu64(P7, (rbx) as u64); bb = 30; continue; }
            29 => { rdx = (rbx) as u64; rcx = (0x5u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 30; continue; }
            30 => { crate::simd_ops::lk_sub_u32(P9, (1) as u32); rbx = (rsi) as u64; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 32; continue; }
            32 => { bb = 34; continue; }
            33 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x298u64))) as u64; bb = 34; continue; }
            34 => { fl_l = ((((rcx as u32)) >> (8u32)) & 1) as u64; if ((fl_l as u32) == 0) { bb = 50; continue; } bb = 35; continue; }
            35 => { rax = (((rcx as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x5u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 43; continue; } bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x160cac0u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r12)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            37 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 43; continue; }
            38 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 39; continue; }
            39 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 43; continue; }
            40 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 43; continue; }
            41 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 43; continue; }
            42 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 43; continue; }
            43 => { fl_l = (rbx) as u64 as u64; fl_r = (crate::simd_ops::ru64(P1)) as u64 as u64; rdi = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(P1, (rdi) as u64); bb = 46; continue; }
            45 => { crate::simd_ops::wu64(r15, (rdi) as u64); bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(P4)) as u64; rdx = (crate::simd_ops::ru64(P5)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rax }) as u64; crate::simd_ops::wu64(P4, (rax) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 49; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); fl_l = (crate::simd_ops::lk_sub_u32(P9, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P7, (rbx) as u64); rbx = (rdi) as u64; bb = 2; continue; }
            49 => { rdx = (rbx) as u64; rcx = (0x5u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_sub_u32(P9, (1) as u32)) as u32 as u64; rbx = (rdi) as u64; bb = 2; continue; }
            50 => { r15 = (rbx) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 2; continue; }
            51 => { fl_l = (rdi) as u64 as u64; fl_r = (crate::simd_ops::ru64(P1)) as u64 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 53; continue; } bb = 52; continue; }
            52 => { crate::simd_ops::wu64(P1, (rax) as u64); bb = 54; continue; }
            53 => { crate::simd_ops::wu64(r14, (rax) as u64); bb = 54; continue; }
            54 => { rax = (crate::simd_ops::ru64(P4)) as u64; rdx = (crate::simd_ops::ru64(P5)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { r14 } else { rax }) as u64; crate::simd_ops::wu64(P4, (rax) as u64); fl_l = (rdi) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 57; continue; } bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); fl_l = (crate::simd_ops::lk_sub_u32(P9, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(P7, (rdi) as u64); bb = 1; continue; }
            57 => { rdx = (rdi) as u64; rcx = (0x5u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (crate::simd_ops::lk_sub_u32(P9, (1) as u32)) as u32 as u64; bb = 1; continue; }
            58 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x14160c7b0u64) { 0x20 => { bb = 1; continue; } 0x30 => { bb = 2; continue; } 0x39 => { bb = 3; continue; } 0x45 => { bb = 4; continue; } 0x4a => { bb = 5; continue; } 0x53 => { bb = 6; continue; } 0x57 => { bb = 7; continue; } 0x64 => { bb = 8; continue; } 0x6d => { bb = 9; continue; } 0x7d => { bb = 10; continue; } 0x8b => { bb = 11; continue; } 0x90 => { bb = 12; continue; } 0x95 => { bb = 13; continue; } 0x9c => { bb = 14; continue; } 0xa7 => { bb = 15; continue; } 0xac => { bb = 16; continue; } 0xb0 => { bb = 17; continue; } 0xc2 => { bb = 18; continue; } 0xc7 => { bb = 19; continue; } 0xcc => { bb = 20; continue; } 0xe2 => { bb = 21; continue; } 0xeb => { bb = 22; continue; } 0xfb => { bb = 23; continue; } 0x107 => { bb = 24; continue; } 0x110 => { bb = 25; continue; } 0x113 => { bb = 26; continue; } 0x134 => { bb = 27; continue; } 0x147 => { bb = 28; continue; } 0x15a => { bb = 29; continue; } 0x167 => { bb = 30; continue; } 0x173 => { bb = 31; continue; } 0x180 => { bb = 32; continue; } 0x182 => { bb = 33; continue; } 0x18c => { bb = 34; continue; } 0x196 => { bb = 35; continue; } 0x19e => { bb = 36; continue; } 0x1ab => { bb = 37; continue; } 0x1b5 => { bb = 38; continue; } 0x1bd => { bb = 39; continue; } 0x1c7 => { bb = 40; continue; } 0x1d1 => { bb = 41; continue; } 0x1de => { bb = 42; continue; } 0x1e6 => { bb = 43; continue; } 0x1f2 => { bb = 44; continue; } 0x1fb => { bb = 45; continue; } 0x1fe => { bb = 46; continue; } 0x21f => { bb = 47; continue; } 0x232 => { bb = 48; continue; } 0x251 => { bb = 49; continue; } 0x26c => { bb = 50; continue; } 0x277 => { bb = 51; continue; } 0x283 => { bb = 52; continue; } 0x28c => { bb = 53; continue; } 0x28f => { bb = 54; continue; } 0x2b0 => { bb = 55; continue; } 0x2c3 => { bb = 56; continue; } 0x2df => { bb = 57; continue; } 0x2f7 => { bb = 58; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}
