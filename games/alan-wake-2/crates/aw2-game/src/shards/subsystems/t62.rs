#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_14048f490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x40; const P3: u64 = 0x8; const P4: u64 = 0x141a00a20;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = 0; fl_l = 0; fl_r = 0; r14 = r8; rbx = rdx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { let y_addr: u64 = rbx.wrapping_add(P1); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(P2); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); r8 = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x70); xmm0 = (ymm1_hi) as u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = rsp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); xmm1 = (ymm1_hi) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (((xmm1 >> 64) as u64)) as u64); xmm0 = (ymm2_hi) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(P3))) as u128; ymm0_hi = 0; xmm1 = (ymm2_hi) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (((xmm1 >> 64) as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (((xmm2 >> 64) as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), ((xmm1 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0xd0u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0xb0u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404913f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = rdx; r11 = (crate::simd_ops::ru64(rcx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rbx = ((r11.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0xffffffff; rdi = (rdi & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(r11.wrapping_add(rcx.wrapping_mul(8)))) as u32 as u64; r8 = ((r11.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r8)) as u64; rax = (crate::simd_ops::ru64(r10)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_shr(32u32)) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 7; continue; }
            7 => { r8 = rbx; bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = r8; fl_r = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404917f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1))) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140491a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140215e80; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1401de830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (crate::simd_ops::ru64(r9)) as u64; r15 = r9; rbp = ((rbp).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; r12 = r8; rbp = ((((rbp) as i64).wrapping_shr(4u32) as u64)) as u64; rbx = rdx; rsi = rcx; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); rdi = rcx.wrapping_add(0x8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); fl_l = rbp; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r13 = (((rbp as u8)) as u32) as u32 as u64; bb = 5; continue; }
            4 => { r13 = (r13 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; r10 = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rax = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r8 = ((r8).wrapping_add(r8)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; r9 = ((rcx.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r9); fl_l = rbp; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 6; continue; }
            6 => { rax = rbx; rax = ((rax).wrapping_sub(rdx)) as u64; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = ((r10 as u32)) as u32 as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rax = ((rax).wrapping_add(r10)) as u64; fl_l = rax; fl_r = r9; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { r9 } else { rax }) as u64; rdx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r15)) as u64; r14 = rax; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdx = (crate::simd_ops::ru64(r12)) as u64; r8 = r15; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rdx = ((rdx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu128(r8, (xmm0) as u128); r8 = r8.wrapping_add(0x10); fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdi = rsi.wrapping_add(0x8); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = r14; fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((r8).wrapping_shl(4u32)) as u64; r8 = ((fl_l as u64)) as u64; bb = 13; continue; }
            12 => { r8 = rbx; r8 = ((r8).wrapping_sub(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = ((r15.wrapping_add(rbp)) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rdx = rbx; r8 = ((r8).wrapping_sub(rbx)) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_l = ((rcx).wrapping_add(r14)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsi, r14); bb = 24; continue; }
            14 => { r14 = ((rdx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rcx = rbp; r8 = r14; rcx = ((rcx).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_sub(rbx)) as u64; rax = r8; rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 19; continue; } bb = 15; continue; }
            15 => { r8 = rcx; rdx = r14; rdx = ((rdx).wrapping_sub(rcx)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (((rbp) as i64).wrapping_mul((0xfffffffffffffff0u64) as i64) as u64) as u64; rdx = rbx; r8 = ((r8).wrapping_sub(rbx)) as u64; r8 = ((r8).wrapping_add(r14)) as u64; r14 = ((r14).wrapping_sub(r8)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru64(r12)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rcx = ((rcx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu128(rbx, (xmm0) as u128); rbx = rbx.wrapping_add(0x10); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { bb = 22; continue; }
            19 => { rcx = ((rcx).wrapping_add(rbx)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r15)) as u64; rcx = (crate::simd_ops::ru64(r12)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rcx = ((rcx).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu128(rbx, (xmm0) as u128); rbx = rbx.wrapping_add(0x10); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rdi = rsi.wrapping_add(0x8); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 25; continue; }
            25 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140491c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1))) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140492ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e6d50; const P1: u64 = 0x1404937d0; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x140213060; const P6: u64 = 0x14314dec0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rdx; rdi = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x3cu64))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rdx = P1; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdx); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x38u64), (1) as u32); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rax = rdx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rsi = 0x8; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rcx }) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rbp = 0x10000; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x38u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rbx = ((rdx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rax); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r8 = 0x10; rdx = rbp; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rdx = rsp.wrapping_add(0x30); rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rbp); rax = rcx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rsi = ((rsi).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rsi }) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = rbp; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = (0x38u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rbx = ((rcx.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rax); bb = 10; continue; }
            10 => { rax = P6; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x19u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x19u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x1au64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x1au64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rcx as u32)) as u32); rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x30u64), ((rcx as u8)) as u8); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x40u64), (0x38u64) as u64); crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x48u64), (0xffffffffffffffc8u64) as u64); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x20u64)))) as u64; rbp = ((rbp).wrapping_sub(crate::simd_ops::ru64(rdi.wrapping_add(0x48u64)))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), rbp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140492c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e6d50; const P1: u64 = 0x140493800; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x140213060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rdx; rbx = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x3cu64))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = P1; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdx); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x38u64), (1) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = rdx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rdi = 0x8; rcx = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rcx }) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rsi = 0x10000; r15 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x20u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx).wrapping_add(rcx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rax); if (((fl_l as u64)) != 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r8 = 0x10; rdx = rsi; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = r15; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rdx = rsp.wrapping_add(0x30); rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rsi); rax = rcx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rdi = ((rdi).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rdi }) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = rsi; rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = rax; fl_r = (0x20u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rcx.wrapping_add(rdi)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rax); bb = 11; continue; }
            10 => { rdx = r15; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdx, r15); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), r15); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rdx, rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r14, r15); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r15); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), r15); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), r15); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x40u64), (0x20u64) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x48u64), (0xffffffffffffffe0u64) as u64); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x20u64)))) as u64; rsi = ((rsi).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x48u64)))) as u64; crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), rsi); rax = rdx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140492e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404946f0; const P1: u64 = 0x143ca594c; const P2: u64 = 0x1420af120; const P3: u64 = 0x1404945b0; const P4: u64 = 0x143ca5958; const P5: u64 = 0x140215750; const P6: u64 = 0x140494470; const P7: u64 = 0x143ca5950; const P8: u64 = 0x140494380; const P9: u64 = 0x143ca5ae8; const P10: u64 = 0x143c8d910; const P11: u64 = 0x140215610; const P12: u64 = 0x1420a28e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; r14 = r9; rsi = r8; r15 = rcx; rcx = rbp.wrapping_add(0x1f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x49); rdx = rsi; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r15.wrapping_add(0xa8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r15.wrapping_add(0xc8u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r15.wrapping_add(0xe8u64), ((xmm0 as u64)) as u64); rcx = rbp.wrapping_add(0x1f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x49); rdx = rsi; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = r15.wrapping_add(0xf0); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x40u64), ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rbp.wrapping_add(0x1f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x49); rdx = rsi; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x40u64), ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rbp.wrapping_add(0x1f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x49); rdx = rsi; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x40u64), ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rbx = ((crate::simd_ops::ru16(P10)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x44u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = r15.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(r15.wrapping_add(0x40u64), (1) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r15); rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140493110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x10088; const P2: u64 = 0x1404931f0; const P3: u64 = 0x140493480; const P4: u64 = 0x1404932e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x160u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; rbx = (((rdx as u16)) as u32) as u32 as u64; r14 = ((rsi.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rcx; rcx = rsp.wrapping_add(0x110); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; rcx = rsp.wrapping_add(0x30); r9 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = (((rbx as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0xa0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0xc0u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; ymm1_hi = 0; let y_addr: u64 = rsp.wrapping_add(0xe0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rsp.wrapping_add(0x100u64), (xmm1) as u128); r8 = rsp.wrapping_add(0x110); rdx = rsp.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { r11 = rsp.wrapping_add(0x160); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404932e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141a04990;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut ymm2_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x50); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rdi = r8; rbx = rdx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { let y_addr: u64 = rbx.wrapping_add(0x20u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbx.wrapping_add(0x40u64); xmm2 = crate::simd_ops::ru128(y_addr); ymm2_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm3 = (crate::simd_ops::ru128(rbx.wrapping_add(0x60u64))) as u128; xmm0 = (ymm1_hi) as u128; ymm0_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x10u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; r9 = rbp.wrapping_sub(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); xmm1 = (ymm1_hi) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (((xmm1 >> 64) as u64)) as u64); xmm0 = (ymm2_hi) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x8u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; xmm1 = (ymm2_hi) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (((xmm1 >> 64) as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x38u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = rbp.wrapping_sub(0x50); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64; rdx = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), ((xmm1 as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rcx = rbp.wrapping_sub(0x10); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), ((xmm1 as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64) as u128; ymm1_hi = 0; let y_addr: u64 = rbp.wrapping_sub(0x10u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (((xmm2 >> 64) as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), ((xmm1 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r14); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_add(0x10u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r11 = rsp.wrapping_add(0x150); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140493960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffd8; const P4: u64 = 0xffffffffffffffc8; const P5: u64 = 0x20; const P6: u64 = 0x1404917f0; const P7: u64 = 0x140491c90;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_add(P3), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_add(P4), (xmm7) as u128); r15 = r8; rbp = rdx; r14 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm6 = 0u128; xmm7 = 0u128; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rdi = (crate::simd_ops::ru64(r15)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm7 as u64)) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140493b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404931f0; const P1: u64 = 0x1404935a0; const P2: u64 = 0x141a04990;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 67>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x100); rsp = ((rsp).wrapping_sub(0x200u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = rcx; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r8 = ((r8).wrapping_add(r8)) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x138u64), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 1; continue; }
            1 => { rdx = rcx; rcx = rbp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x138u64))) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r8).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r8); rax = ((r9.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); fl_l = r9; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x220u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x1f8u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x1f0u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x1e8u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x1e0u64), r15); bb = 3; continue; }
            3 => { r9 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x128u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 6; continue; }
            6 => { rcx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x128u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            8 => { rdx = ((r8.wrapping_add(rax)) as u64) as u64; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rcx).wrapping_add(r8)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); bb = 13; continue; }
            13 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((r13).wrapping_add(r8)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = ((r12).wrapping_add(r8)) as u64; fl_r = fl_l; r12 = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((r15).wrapping_add(r8)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((r14).wrapping_add(r8)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { fl_l = ((rsi).wrapping_add(r8)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rdi).wrapping_add(r8)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = ((rbx).wrapping_add(r8)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 27; continue; }
            27 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_r = r9; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 30; continue; } bb = 28; continue; }
            28 => { bb = 29; continue; }
            29 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x38u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x50u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp, ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x68u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdx); rdx = rbp.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rcx); rcx = rbp.wrapping_add(0x80); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), r8); r8 = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r9); r9 = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r13); let y_addr: u64 = rsp.wrapping_add(0x70u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x80u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rsi); let y_addr: u64 = rbp.wrapping_sub(0x70u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rbx); let y_addr: u64 = rbp.wrapping_sub(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0xc0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_add(0xa0u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r9 = ((r9).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x128u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9); fl_l = r9; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 29; continue; } bb = 30; continue; }
            30 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 31; continue; }
            31 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x138u64))) as u64; r9 = ((r9).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x138u64), r9); fl_l = r9; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 32; continue; }
            32 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e8u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1f0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1f8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x220u64))) as u64; bb = 33; continue; }
            33 => { rsp = ((rsp).wrapping_add(0x200u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404940d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142407c80; const P1: u64 = 0x143ca5aa8; const P2: u64 = 0x142489ee0; const P3: u64 = 0x1430782b0; const P4: u64 = 0x140491c70; const P5: u64 = 0x140491c90;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm7) as u128); r13 = rcx; rax = (((r8 as u16)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_add(r8)) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; rbp = (crate::simd_ops::ru32(rdi.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax as u32)) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x188u64) as i64) as u64) as u64; rsi = ((rsi).wrapping_add(rbx)) as u64; r13 = (crate::simd_ops::ru64(r13)) as u64; rbp = ((rdi.wrapping_add(rbp.wrapping_mul(2))) as u64) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { xmm6 = 0u128; xmm7 = 0u128; bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(r13.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rcx = r11; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(r13).wrapping_add(0x180acu64))) as u32 as u64; r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r13).wrapping_add(0x18098u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rax = ((r12.wrapping_add(rbx)) as u64) as u64; fl_l = rbx; fl_r = rbx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm7 as u64)) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x0u8) as u8); r8 = rsp.wrapping_add(0x60); rdx = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x30); rcx = rsi.wrapping_add(0x580f0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 14; continue; }
            14 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404946f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143ca58a8; const P1: u64 = 0x143ca5700; const P2: u64 = 0x143c8f8dc; const P3: u64 = 0x143c8fb0c; const P4: u64 = 0x143ca5aa8; const P5: u64 = 0x143c8d120; const P6: u64 = 0x143c917cc; const P7: u64 = 0x143c8f0d0; const P8: u64 = 0x143c8d160; const P9: u64 = 0x143c8f4d0; const P10: u64 = 0x143c9eac0; const P11: u64 = 0x143ca5aac; const P12: u64 = 0x143c8d694; const P13: u64 = 0x143c8f4d4; const P14: u64 = 0x143ca5720; const P15: u64 = 0x143ca5740; const P16: u64 = 0x143ca5760; const P17: u64 = 0x143ca5780; const P18: u64 = 0x143ca5790; const P19: u64 = 0x141834a0c; const P20: u64 = 0x141834a78;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rbx = rcx; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 5; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx, (0xcu64) as u64); rax = P1; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rdi); rax = rbx; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { bb = 3; continue; }
            3 => { crate::simd_ops::wu16(rcx, (0xffffu16) as u16); rcx = rcx.wrapping_add(0xc); fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x41u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x3du64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x25u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x1du64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0xdu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x5u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P8)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0xbu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x13u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P10)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P11)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x23u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x2bu64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P12)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), ((rdi as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x37u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P13)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), ((rdi as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x3bu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x43u64), (0x2u32) as u32); let y_addr: u64 = rbp.wrapping_sub(0x49u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P1; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P14; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P15; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_add(0x17u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P16; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x37u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(P17, (xmm0) as u128); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x47u64))) as u32 as u64; crate::simd_ops::wu32(P18, ((rax as u32)) as u32); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            5 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 6; continue; }
            6 => { rcx = rbp.wrapping_sub(0x49); rax = 0xc; bb = 3; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140494da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f2bc0; const P1: u64 = 0x1401dec00; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1425371a0; const P4: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 19; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((((rax as u32)) & (0x3fffffffu32))) as u32 as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 3; continue; }
            3 => { rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rbp = rax; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rcx as u32)) as u32 as u64; rcx = rax; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = 0x40000000; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = r9.wrapping_sub(0x1); rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); if (((fl_l as u32)) != 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { rcx = rdx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { fl_l = ((((rsi as u32)) & (0x3fffffffu32))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rsi = rdi.wrapping_add(0x8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((rax as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = rdi.wrapping_add(0x8); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))))) as u32 as u64; rcx = ((((rcx as u32)) & (0x3fffffffu32))) as u32 as u64; rcx = ((((rcx as u32)) ^ (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rcx as u32)) as u32); rdx = ((rcx as u32)) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))))) as u32 as u64; rdx = ((((rdx as u32)) & (0x40000000u32))) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rcx as u32)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdx as u32)) as u32); rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))))) as u32 as u64; fl_l = ((((rdx as u32)) >> (31u32)) & 1) as u64; rdx = (((rdx as u32)) & !((1u32) << (31u32))) as u32 as u64; rdx = ((((rdx as u32)) ^ (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))))) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rsi, ((rax as u32)) as u32); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 21; continue; }
            21 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404950e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0xffffffffffffffe8; const P4: u64 = 0x142535070; const P5: u64 = 0x1426456c8; const P6: u64 = 0x1426453a0; const P7: u64 = 0x21; const P8: u64 = 0x14314f7a0; const P9: u64 = 0x25; const P10: u64 = 0x27; const P11: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rcx = 0x6348; rdi = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(P2), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); bb = 5; continue; }
            3 => { rdx = 0x28; r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rdi; bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rdx.wrapping_add(P7), ((rdi as u32)) as u32); rax = P8; crate::simd_ops::wu16(rdx.wrapping_add(P9), ((rdi as u16)) as u16); crate::simd_ops::wu8(rdx.wrapping_add(P10), ((rdi as u8)) as u8); crate::simd_ops::wu64(rdx.wrapping_add(P11), rdi); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rdi); crate::simd_ops::wu64(rdx, rax); rax = rbx; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x1cu64), (0x1u32) as u32); crate::simd_ops::wu8(rdx.wrapping_add(0x20u64), ((rdi as u8)) as u8); crate::simd_ops::wu64(rbx, rdx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140495930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14314f768; const P1: u64 = 0x1401de830; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x28; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140496a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x10088; const P2: u64 = 0x143ca5b90; const P3: u64 = 0x20; const P4: u64 = 0x580e0; const P5: u64 = 0x8; const P6: u64 = 0x143c8fd28; const P7: u64 = 0x143c8d08c; const P8: u64 = 0x140496c90; const P9: u64 = 0x140496ba0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rdx as u16)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { r11 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r10)) as u64; r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r10); crate::simd_ops::wu16(rsp.wrapping_add(0x40u64), ((rdx as u16)) as u16); rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r11); rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(r8.wrapping_mul(2)))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(P5)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; ymm1_hi = 0; rax = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(rax.wrapping_mul(8)))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x40u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); r8 = rsp.wrapping_add(0x80); let y_addr: u64 = rax; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x60u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rdx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140496ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x141a06560;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = 0; fl_l = 0; fl_r = 0; rbx = r8; rsi = rdx; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { let y_addr: u64 = rsi.wrapping_add(P1); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); r9 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x50); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rsp.wrapping_add(0x70); let y_addr: u64 = rsp.wrapping_add(0x50u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm0 = (ymm1_hi) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (((xmm1 >> 64) as u64)) as u64); xmm1 = (ymm1_hi) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (((xmm1 >> 64) as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64) as u128; ymm1_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P3))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404971e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143ca5f00; const P1: u64 = 0x5e308; const P2: u64 = 0x143ca5b44; const P3: u64 = 0x20; const P4: u64 = 0x8; const P5: u64 = 0x143c8fd28; const P6: u64 = 0x580e0; const P7: u64 = 0x10; const P8: u64 = 0x18; const P9: u64 = 0x141a068d0; const P10: u64 = 0x140497a00;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P1)))) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8); rdx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P4))) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); rcx = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P6))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), r8); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x68u64), (xmm1) as u128); xmm2 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), ((xmm2 as u64)) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rbx); crate::simd_ops::wu8(r11.wrapping_sub(0x30u64), (0x0u8) as u8); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { bb = 4; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm2 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rdx = rsp.wrapping_add(0x40); rcx = rsp.wrapping_add(0x90); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140497490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143ca5b90; const P1: u64 = 0x143c8d08c; const P2: u64 = 0x143c8fd28; const P3: u64 = 0x140496c90; const P4: u64 = 0x141a06560;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbp = r11.wrapping_sub(0x48); rsp = ((rsp).wrapping_sub(0x138u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; r9 = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rbx); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rsi); rsi = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); r14 = ((rbx.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), r10); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r10); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); rcx = ((crate::simd_ops::ru16(rcx.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(0x580e0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rsi.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r13 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(r10.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r12 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu16(rbp.wrapping_sub(0x80u64), ((rax as u16)) as u16); rdx = rbp.wrapping_sub(0x40); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r13); rcx = rbp.wrapping_sub(0x80); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r10); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = 0; fl_l = 0; fl_r = 0; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x60u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x58u64))) as u64; rdx = rbp.wrapping_sub(0x40); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rbp.wrapping_sub(0x20); crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); let y_addr: u64 = rsp.wrapping_add(0x48u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r15); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x68u64))) as u128; let y_addr: u64 = rbp.wrapping_sub(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rbp, (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((r15).wrapping_add(1)) as u64; fl_l = r15; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 11; continue; }
            11 => { r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rbx); fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 13; continue; }
            13 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; bb = 15; continue; }
            15 => { rsp = ((rsp).wrapping_add(0x138u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404978b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x142442de0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rsi = ((rax.wrapping_add(rax)) as u32) as u32 as u64; rax = 0x10; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rsi as u32) }) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            5 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 12; continue; }
            6 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rax = rbx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { r8 = 0x10; rcx = r10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = rax; bb = 12; continue; }
            12 => { rbp = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = 0xdeadd00d; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rbp as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { r8 = rbx; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rcx, rbp); crate::simd_ops::wu64(r8, rax); r8 = r8.wrapping_add(0x8); rcx = rcx.wrapping_add(0x8); fl_l = (((rdx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 17; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rsi as u32)) as u32); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140497a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de830; const P1: u64 = 0x142442de0; const P2: u64 = 0x1425371a0; const P3: u64 = 0x142533200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = 0; fl_l = 0; fl_r = 0; r14 = rbp.wrapping_add(0x20); rsi = rbp.wrapping_add(0x18); fl_l = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(1)) as u64; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r14 = rbp.wrapping_add(0x20); bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = r9.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = rbp.wrapping_add(0x18); r14 = rbp.wrapping_add(0x20); bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { r14 = rbp.wrapping_add(0x20); bb = 14; continue; }
            13 => { r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsi, (0x0u64) as u64); crate::simd_ops::wu32(r14, (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404984e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143ca5c54; const P1: u64 = 0x143ca5e28; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x141834a78; const P6: u64 = 0x143c8d694; const P7: u64 = 0x143ca5cf4; const P8: u64 = 0x143ca5f00; const P9: u64 = 0x143c8ff58; const P10: u64 = 0x143ca5e48; const P11: u64 = 0x143ca5e58; const P12: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx, (0x4u64) as u64); rax = P1; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdi as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdi as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x3cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rdi as u32)) as u32); rax = ((crate::simd_ops::ru16(P8)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rdi as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x2u32) as u32); rax = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x3u32) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x54u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rdi as u32)) as u32); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P1; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(P10, (xmm1) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(P11, ((rax as u32)) as u32); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426467c8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x140498980, rcx, rsp); return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426459f8; const P2: u64 = 0x142645840;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rbx = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rcx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1404989d1, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404990d0; const P1: u64 = 0x1403c8050; const P2: u64 = 0x140498ed0; const P3: u64 = 0x1401f33c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x14u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x28u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x38u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x58u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x68u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x6cu64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x70u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x74u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x78u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x7cu64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x80u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x84u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x88u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8cu64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x90u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x98u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0xa8u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0xb8u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0xb9u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0xbau64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0xbbu64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0xbcu64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0xbdu64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0xc0u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x108u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x1402467f0; const P2: u64 = 0x140498f90; const P3: u64 = 0x140499130;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0xc0u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0xa8u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x98u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x58u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x38u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x28u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498ce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401df3f0; const P1: u64 = 0x28; const P2: u64 = 0x29; const P3: u64 = 0x30; const P4: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rdx = rax;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P1), ((rcx as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), ((rcx as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(rax.wrapping_add(P4), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14034c370; const P1: u64 = 0x123; const P2: u64 = 0x128;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8);
    rdx = rsp.wrapping_add(0x20);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (((rax as u8)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64;
    rax = ((rcx as u32)) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x21u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P1), ((rcx as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x1u8) as u8);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f9f90;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rdx = rax;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = ((rax).wrapping_add(0x40u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = ((rax).wrapping_add(0x40u64)) as u64;
    rdx = rax;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x1424b4fb0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdx = 0x8; rcx = 0x138; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0x138; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 2; continue; }
            1 => { bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xa8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128;
    crate::simd_ops::wu32(rax.wrapping_add(P0), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140498f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de410; const P1: u64 = 0x8; const P2: u64 = 0x14049e5c0; const P3: u64 = 0x142442de0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049c0a0; const P1: u64 = 0x19; const P2: u64 = 0x1402174f0; const P3: u64 = 0x14313f720; const P4: u64 = 0x142645890;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdx = rsp.wrapping_add(0x50); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P1))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); bb = 4; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = P3; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_add(0x68u64)) as u64; rsp = ((rsp).wrapping_add(0x78u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049c880;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x19; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P1))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsp, rax); rax = (crate::simd_ops::ru64(rsp)) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P1))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 1; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 9; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P1))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax); bb = 6; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rax, rcx); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e7c70; const P1: u64 = 0x1401e7d10; const P2: u64 = 0x1404992e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_add(0x8u64)) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e7cd0; const P1: u64 = 0x1401e7d10; const P2: u64 = 0x1404992e0;
    let (mut rax, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rsp.wrapping_add(0x20);
    rdi = rax;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    rcx = 0x18;
    while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); }
    r8 = rsp.wrapping_add(0x20);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rsi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404992e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e7cc0; const P1: u64 = 0x1401e7c60; const P2: u64 = 0x14040a8d0; const P3: u64 = 0x20; const P4: u64 = 0x14040aa30; const P5: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x22); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x20u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = ((crate::simd_ops::ru16(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x38); rdi = rax; rsi = rcx; rcx = 0x28; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rdx = rsp.wrapping_add(0x60); rcx = rsp.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rcx = rsp.wrapping_add(0x90); rdi = ((rax.wrapping_add(P5)) as u64) as u64; rsi = rcx; rcx = 0x28; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0xc8u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404994c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049cd20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = rsp.wrapping_add(0x20);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((0x10u64) as i64) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404995b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049d5f0; const P1: u64 = 0x14049d540; const P2: u64 = 0x14049d5b0; const P3: u64 = 0x8; const P4: u64 = 0x14049d860; const P5: u64 = 0x14049d510;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; rdx = rax; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x10u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401eeda0; const P1: u64 = 0x1403220b0; const P2: u64 = 0x1424823d0; const P3: u64 = 0x1401eb1d0; const P4: u64 = 0x1401e7140;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140499670, funcinfo_rva: 0x35dd8e4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140499690); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404996b7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404996d2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404996eb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404996f6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1423fdfc0; const P1: u64 = 0x14026e520; const P2: u64 = 0x14026e560; const P3: u64 = 0x1401e39a0;
    let (mut rax, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x108u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x40); rdi = rax; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0xc0); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rcx = rsp.wrapping_add(0xe0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x60); rdi = rcx; rsi = rax; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0x50); rcx = rsp.wrapping_add(0x40); rdi = rax; rsi = rcx; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0x70); rcx = rsp.wrapping_add(0x50); rdi = rax; rsi = rcx; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0x80); rcx = rsp.wrapping_add(0x60); rdi = rax; rsi = rcx; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0x90); rcx = rsp.wrapping_add(0x70); rdi = rax; rsi = rcx; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x90); r8 = 0x7ff; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = rsp.wrapping_add(0xa0); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0xd0); rdi = rcx; rsi = rax; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0xb0); rcx = rsp.wrapping_add(0xa0); rdi = rax; rsi = rcx; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu8(rax, (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x108u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142481220; const P1: u64 = 0x1424823d0; const P2: u64 = 0x1401eb1d0; const P3: u64 = 0x1401e7140;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140499870, funcinfo_rva: 0x35dd920, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x14049988d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (((rax as u8)) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404998a5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404998be); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404998c9); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404998e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142480ac0; const P1: u64 = 0x1424823d0; const P2: u64 = 0x1401eb1d0; const P3: u64 = 0x1401e7140;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1404998e0, funcinfo_rva: 0x35dd920, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1404998fd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (((rax as u8)) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140499915); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049992e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140499939); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; }
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140499f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e5530; const P1: u64 = 0x14028e350; const P2: u64 = 0x14028e000; const P3: u64 = 0x14307d640; const P4: u64 = 0x14049db70; const P5: u64 = 0x1401f3ce0; const P6: u64 = 0x1402c0a50; const P7: u64 = 0x1401e5510; const P8: u64 = 0x1401f3e20; const P9: u64 = 0x1401e54e0; const P10: u64 = 0x141a08a80; const P11: u64 = 0x1402c0a60; const P12: u64 = 0x1401f4970; const P13: u64 = 0x1401de2f0; const P14: u64 = 0x140498d60; const P15: u64 = 0x1401f3c00;
    let (mut rax, mut fl_l, mut fl_r, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = 0x2; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdx = P3; rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32 as u64; rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = 0x4; r8 = rax; rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (0x0u64) as u64); rax = rsp.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            4 => { bb = 9; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            7 => { bb = 9; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            9 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            11 => { bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049a360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430782b0; const P1: u64 = 0x1401de2f0; const P2: u64 = 0x14021a690; const P3: u64 = 0x1401e4930;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu8(rsp.wrapping_add(0x8u64), ((rcx as u8)) as u8); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdx = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x2c); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((rax as u8)) as u8); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x21u64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x22u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x23u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x23u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x29u64), ((rax as u8)) as u8); rdx = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x2d); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2eu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2fu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x25u64), ((rax as u8)) as u8); rcx = rsp.wrapping_add(0x31); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x24u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x32u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x25u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x26u64), ((rax as u8)) as u8); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x26u64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x27u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x27u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2au64), ((rax as u8)) as u8); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x29u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x2au64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x2bu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x2bu64))) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049a4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430782b0; const P1: u64 = 0x1401de2f0; const P2: u64 = 0x14021a690; const P3: u64 = 0x14045cf90; const P4: u64 = 0x1401e4930;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu8(rsp.wrapping_add(0x8u64), ((rcx as u8)) as u8); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdx = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x2f); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((rax as u8)) as u8); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x21u64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x22u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x22u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x23u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x23u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2cu64), ((rax as u8)) as u8); rdx = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x27u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x32u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x25u64), ((rax as u8)) as u8); rcx = rsp.wrapping_add(0x33); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x24u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x26u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x34u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x25u64))) as u32) as u32 as u64; r8 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x26u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x27u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x29u64), ((rax as u8)) as u8); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x29u64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x2au64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x2au64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2bu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x2bu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x2du64), ((rax as u8)) as u8); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x2cu64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x2du64))) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu8(rsp.wrapping_add(0x2eu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x2eu64))) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049a5e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e0f20; const P1: u64 = 0x1401e3b60; const P2: u64 = 0x142308640; const P3: u64 = 0x143078880; const P4: u64 = 0x14020c4d0; const P5: u64 = 0x1430790e8; const P6: u64 = 0x14049f0f0; const P7: u64 = 0x1401de2e0; const P8: u64 = 0x1401e19b0; const P9: u64 = 0x1401e0f10;
    let (mut rax, mut fl_l, mut fl_r, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x1c8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = 0x1; rax = (((rax) as i64).wrapping_mul((0x8u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_r = (0xffffc0u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_add(0x168u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffe00u32))) as u32 as u64; rax = ((((rax as u32)) | (0xa3u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; rax = ((((rax as u32)) & (0x7fffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(9u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1ffu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); bb = 4; continue; }
            2 => { bb = 4; continue; }
            3 => { rax = rsp.wrapping_add(0x20); rdi = rax; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x1; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0x21); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); rax = 0x1; rax = (((rax) as i64).wrapping_mul((0x8u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = P3; rcx = rsp.wrapping_add(0x150); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); rax = rsp.wrapping_add(0x140); rdi = rax; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r8 = rsp.wrapping_add(0x140); rdx = P5; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_add(0x168u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x14; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffe00u32))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; rax = ((((rax as u32)) & (0x7fffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(9u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1ffu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rax, (0xa3u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (0x14u64) as u64); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); bb = 4; continue; }
            4 => { rax = rsp.wrapping_add(0x160); rdi = rax; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0xf8); rcx = rsp.wrapping_add(0x160); rdi = rax; rsi = rcx; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e0u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e8u64))) as u64; rcx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x4u64), ((rcx as u8)) as u8); rax = rsp.wrapping_add(0x180); rcx = rsp.wrapping_add(0xf8); rdi = rax; rsi = rcx; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = rsp.wrapping_add(0x1a0); rcx = rsp.wrapping_add(0x180); rdi = rax; rsi = rcx; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0x120); rcx = rsp.wrapping_add(0x1a0); rdi = rax; rsi = rcx; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = rcx; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32 as u64; rax = ((((rax as u32)) & (0x7fffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(9u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1ffu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_add(0x168u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(0x198u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_add(0x1a0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rax); rdx = 0x1; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x1c8u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049aa50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e3b60; const P1: u64 = 0x1401e66e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), (0x1u8) as u8);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (((rax) as i64).wrapping_mul((0x3e8u64) as i64) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x28);
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049aba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e0f20; const P1: u64 = 0x1401e3b60; const P2: u64 = 0x142308640; const P3: u64 = 0x143078880; const P4: u64 = 0x14020c4d0; const P5: u64 = 0x1430790e8; const P6: u64 = 0x14049f0f0; const P7: u64 = 0x1401de2e0; const P8: u64 = 0x1401e19b0; const P9: u64 = 0x1401e0f10;
    let (mut rax, mut fl_l, mut fl_r, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x1c8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = 0x1; rax = (((rax) as i64).wrapping_mul((0x8u64) as i64) as u64) as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_r = (0xffffc0u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_add(0x168u64)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffe00u32))) as u32 as u64; rax = ((((rax as u32)) | (0x94u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; rax = ((((rax as u32)) & (0x7fffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(9u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1ffu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); bb = 4; continue; }
            2 => { bb = 4; continue; }
            3 => { rax = rsp.wrapping_add(0x20); rdi = rax; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x1; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0x21); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); rax = 0x1; rax = (((rax) as i64).wrapping_mul((0x8u64) as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = P3; rcx = rsp.wrapping_add(0x150); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); rax = rsp.wrapping_add(0x140); rdi = rax; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; r8 = rsp.wrapping_add(0x140); rdx = P5; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rax).wrapping_add(0x168u64)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = 0x14; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((((rax as u32)) & (0xfffffe00u32))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = 0x1; rax = ((((rax as u32)) & (0x7fffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(9u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1ffu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rax, (0x94u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (0x14u64) as u64); rax = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); bb = 4; continue; }
            4 => { rax = rsp.wrapping_add(0x160); rdi = rax; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0xf8); rcx = rsp.wrapping_add(0x160); rdi = rax; rsi = rcx; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rcx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e0u64))) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1e8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); rax = rsp.wrapping_add(0x180); rcx = rsp.wrapping_add(0xf8); rdi = rax; rsi = rcx; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = rsp.wrapping_add(0x1a0); rcx = rsp.wrapping_add(0x180); rdi = rax; rsi = rcx; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = rsp.wrapping_add(0x120); rcx = rsp.wrapping_add(0x1a0); rdi = rax; rsi = rcx; rcx = 0x20; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; rax = rcx; rax = ((((rax) as i64).wrapping_shr(3u32) as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32 as u64; rax = ((((rax as u32)) & (0x7fffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(9u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1ffu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_add(0x168u64)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rax); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x8u64), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rax = ((rax).wrapping_add(0x30u64)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(0x198u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_add(0x1a0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rax); rdx = 0x1; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x1c8u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049b010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e5530; const P1: u64 = 0x14028e350; const P2: u64 = 0x14028e000; const P3: u64 = 0x14307d640; const P4: u64 = 0x14049deb0; const P5: u64 = 0x141a08d90; const P6: u64 = 0x1402c0a50; const P7: u64 = 0x1401e5510; const P8: u64 = 0x1401f9d70; const P9: u64 = 0x141a08c70; const P10: u64 = 0x1402c0a60; const P11: u64 = 0x1401de2f0; const P12: u64 = 0x141a0a330;
    let (mut rax, mut fl_l, mut fl_r, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = 0x2; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdx = P3; rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32 as u64; rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = 0x4; r8 = rax; rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x0u64) as u64); rax = rsp.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            4 => { bb = 9; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            7 => { bb = 9; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            9 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x0u32) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            11 => { bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r8 = rax; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049b380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e5530; const P1: u64 = 0x14028e350; const P2: u64 = 0x14028e000; const P3: u64 = 0x14307d640; const P4: u64 = 0x14049dfa0; const P5: u64 = 0x1401f3ce0; const P6: u64 = 0x1402c0a50; const P7: u64 = 0x1401e5510; const P8: u64 = 0x1401f3e20; const P9: u64 = 0x1401e54e0; const P10: u64 = 0x141a08ed0; const P11: u64 = 0x1402c0a60; const P12: u64 = 0x1401f4970; const P13: u64 = 0x1401de2f0; const P14: u64 = 0x1401f3c00;
    let (mut rax, mut fl_l, mut fl_r, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = 0x2; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdx = P3; rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32 as u64; rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = 0x4; r8 = rax; rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (0x0u64) as u64); rax = rsp.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            4 => { bb = 9; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            7 => { bb = 9; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            9 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            11 => { bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049b6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e5530; const P1: u64 = 0x14028e350; const P2: u64 = 0x14028e000; const P3: u64 = 0x14307d640; const P4: u64 = 0x14049e060; const P5: u64 = 0x1401f3ce0; const P6: u64 = 0x1402c0a50; const P7: u64 = 0x1401e5510; const P8: u64 = 0x1401f3e20; const P9: u64 = 0x1401e54e0; const P10: u64 = 0x141a090b0; const P11: u64 = 0x1402c0a60; const P12: u64 = 0x1401f4970; const P13: u64 = 0x1401de2f0; const P14: u64 = 0x141a0a0f0; const P15: u64 = 0x1401f3c00;
    let (mut rax, mut fl_l, mut fl_r, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14049b6b0, funcinfo_rva: 0x35dda84, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r9) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); rdx = (0x2u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b6ef); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b706); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b723); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdx = ((P3) as u64) as u64; rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b745); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_add(0xb8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32 as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b784); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b7d8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (0x4u32) as u32 as u64; r8 = (rax) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b7f0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (0x0u64) as u64); rax = ((rsp.wrapping_add(0x28u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (rax) as u64); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b853); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b868); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b873); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            4 => { bb = 9; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b897); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (rax) as u64); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b8c0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b8d2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b8e7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b8f2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            7 => { bb = 9; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x10u32) as u32 as u64; while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u8, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b91f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b934); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b93f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            9 => { rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b956); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b960); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 11; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x10u32) as u32 as u64; while rcx != 0 { core::ptr::write_unaligned(rdi as *mut u8, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b980); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            11 => { bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b9a7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rax) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b9b7); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049b9c4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049b9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402029d0; const P1: u64 = 0x1401e88f0; const P2: u64 = 0x1403eaab0; const P3: u64 = 0x1403b3270; const P4: u64 = 0x1403b31f0; const P5: u64 = 0x1403b31e0; const P6: u64 = 0x1403eaa30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x24); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rsp.wrapping_add(0x28); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            1 => { rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049bac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e88f0; const P1: u64 = 0x1401de410; const P2: u64 = 0x140499180; const P3: u64 = 0x1424b4cb0;
    let (mut rax, mut fl_l, mut fl_r, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x20); rax = rsp.wrapping_add(0x20); rdi = rax; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x1; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } bb = 2; continue; }
            1 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); bb = 5; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x21u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(0x1cu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049bc30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e8770; const P1: u64 = 0x1403eaa40; const P2: u64 = 0x1403c83c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdx = rsp.wrapping_add(0x24); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); bb = 2; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = rsp.wrapping_add(0x28); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = rsp.wrapping_add(0x28); rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049bce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e8770; const P1: u64 = 0x14049d090; const P2: u64 = 0x1401f33c0; const P3: u64 = 0x1424b4da0; const P4: u64 = 0x14049e150; const P5: u64 = 0x1401de3c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x98u64)) as u64; rdx = rsp.wrapping_add(0x24); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); bb = 2; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x28); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rsp.wrapping_add(0x28); r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x98u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049bda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049e1e0; const P1: u64 = 0x1402174f0; const P2: u64 = 0x14049d430; const P3: u64 = 0x140200130; const P4: u64 = 0x1401de2f0; const P5: u64 = 0x1430782b0; const P6: u64 = 0x14049e340; const P7: u64 = 0x14049cc20; const P8: u64 = 0x14049cda0;
    let (mut rax, mut fl_l, mut fl_r, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x148u64)) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; rdx = rsp.wrapping_add(0xe8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(0x1cu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x1u32) as u32); bb = 4; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x0u32) as u32); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x8u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; bb = 9; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rcx = rsp.wrapping_add(0x32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = P5; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = rsp.wrapping_add(0x118); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rax); rcx = rsp.wrapping_add(0x118); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x130); rcx = rsp.wrapping_add(0xe8); rdi = rax; rsi = rcx; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdx = rsp.wrapping_add(0x130); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x31u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x8u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x148u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049c7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049c0a0; const P1: u64 = 0x19; const P2: u64 = 0x1402174f0; const P3: u64 = 0x1401de2f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdx = rsp.wrapping_add(0x50); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P1))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); bb = 4; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 7; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049c880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de410; const P1: u64 = 0x8; const P2: u64 = 0x19; const P3: u64 = 0x10; const P4: u64 = 0x14049ea50; const P5: u64 = 0x14049f530; const P6: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P2))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049c930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x18; const P3: u64 = 0x19;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = 0x1; rax = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rdx = 0x8; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P2), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P3), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x78u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049ca20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x18; const P3: u64 = 0x19;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = 0x1; rax = (((rax) as i64).wrapping_mul((0xa8u64) as i64) as u64) as u64; rdx = 0x8; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P2), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P3), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x78u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049cb10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401de3c0; const P2: u64 = 0x141834d30; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(0x48u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = 0x88; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049cc20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401de3c0; const P2: u64 = 0x141834d30; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x1cu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = 0x4c; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049cd20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rax);
    rax = rsp;
    rdi = rax;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x1;
    while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); }
    rax = ((crate::simd_ops::ru8(rsp)) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x8u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x10;
    while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64;
    crate::simd_ops::wu64(rax, (0x0u64) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P0), (0x0u32) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P1), (0x0u32) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049cda0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049d350; const P1: u64 = 0x14049d270;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 24; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rcx); bb = 5; continue; }
            5 => { bb = 8; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax, rcx); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 16; continue; }
            13 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { bb = 22; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 22; continue; }
            19 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { bb = 9; continue; }
            23 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 24; continue; }
            24 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x14049e5c0; const P2: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x68u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402174f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1401e19b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P0), (0x1u8) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P0), ((rcx as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = ((rax).wrapping_add(0xcu64)) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64;
    rax = ((((rax as u32)) | (0x1u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x19; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x18u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P1))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); bb = 7; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); rsp = ((rsp).wrapping_add(0x18u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x19; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x18u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); rax = (crate::simd_ops::ru64(rsp)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P1))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); bb = 7; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 7; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P2), rcx); rsp = ((rsp).wrapping_add(0x18u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e6f40; const P1: u64 = 0x1401e40e0; const P2: u64 = 0x1401de6d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x68;
    let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = rsp.wrapping_add(0x30);
    rcx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e6f40; const P1: u64 = 0x1401e40e0; const P2: u64 = 0x1401de6d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0xa8;
    let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = rsp.wrapping_add(0x30);
    rcx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d4f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e19b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rax).wrapping_add(0xcu64)) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049ed60; const P1: u64 = 0x14049ead0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = rsp.wrapping_add(0x20);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049eea0; const P1: u64 = 0x14049ef10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((0x10u64) as i64) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((0x10u64) as i64) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu32(rsp, (0x1u32) as u32); } else { crate::simd_ops::wu32(rsp, (0x0u32) as u32); }
    rax = ((crate::simd_ops::ru8(rsp)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049edb0; const P1: u64 = 0x14049f0a0; const P2: u64 = 0x14049ee30; const P3: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0xe8u64)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x10u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu32(rax, (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = ((rax).wrapping_add(0xcu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu32(rax, (0x0u32) as u32); rsp = ((rsp).wrapping_add(0xe8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de2e0;
    let (mut rax, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rcx = 0x10;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    rcx = 0x10;
    while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); }
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    rsi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049d990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de2f0; const P1: u64 = 0x14034c370; const P2: u64 = 0x122; const P3: u64 = 0x1401f3c00;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x0u32) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8);
    rdx = rsp.wrapping_add(0x20);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x21u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), ((rcx as u8)) as u8);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64;
    rax = ((((rax as u32)) | (0x1u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049dae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de2f0; const P1: u64 = 0x14034c370; const P2: u64 = 0x123; const P3: u64 = 0x128; const P4: u64 = 0x1401f3c00;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x0u32) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8);
    rdx = rsp.wrapping_add(0x20);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x21u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), ((rcx as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P3), (0x1u8) as u8);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64;
    rax = ((((rax as u32)) | (0x1u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049dc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e6c90; const P1: u64 = 0x1401f3ba0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x10u64), ((rdx as u8)) as u8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049dc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430782b0; const P1: u64 = 0x14049dc40; const P2: u64 = 0x28; const P3: u64 = 0x8; const P4: u64 = 0x30; const P5: u64 = 0x10;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu8(rsp.wrapping_add(0x18u64), ((r8 as u8)) as u8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    r8 = rax;
    rdx = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P3), rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P5), rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049dce0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049f150; const P1: u64 = 0x14049f1f0; const P2: u64 = 0x1426467d0; const P3: u64 = 0x8; const P4: u64 = 0x142645840; const P5: u64 = 0x141a08240;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14049dce0, funcinfo_rva: 0x35ddc80, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049dd03); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049dd46); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P3), (0x0u32) as u32); rcx = (0x6u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049ddb2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 3; continue; }
            3 => { rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14049ddbe); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsp = ((rsp).wrapping_add(0x78u64)) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049deb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x1402d7460; const P3: u64 = 0x14314fa78; const P4: u64 = 0x141a075c0; const P5: u64 = 0x14049ebe0; const P6: u64 = 0x14314fa90;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = 0x8; rcx = 0xc0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0xc0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = P3; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_add(0xb0u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = P6; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 2; continue; }
            1 => { bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049dfa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x1402d7460; const P3: u64 = 0x14314fa60; const P4: u64 = 0x1401f3d70; const P5: u64 = 0x14314fa48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = 0x8; rcx = 0x20; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0x20; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = P3; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = P5; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 2; continue; }
            1 => { bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049e060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x1402d7470; const P3: u64 = 0x14314f9e0; const P4: u64 = 0x141a076f0; const P5: u64 = 0x1401f3d70; const P6: u64 = 0x14314faa8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = 0x8; rcx = 0xc8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0xc8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = P3; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_add(0xb8u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = P6; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 2; continue; }
            1 => { bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049e150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049f270; const P1: u64 = 0x1401de410; const P2: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64;
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    rdx = rsp.wrapping_add(0x40);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x48u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), ((rcx as u8)) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049e1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x19; const P3: u64 = 0x1402174f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P0), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P1), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P2))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rax).wrapping_add(0x1cu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P0), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P0), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P1), rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 5; continue; }
            5 => { bb = 1; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049e300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de2f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049e340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x1401de2f0; const P3: u64 = 0x1404a0110;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0xd8u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); rax = 0x1; rax = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rdx = 0x8; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x1cu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = 0x4c; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = rsp.wrapping_add(0x22); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = rsp.wrapping_add(0xc0); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x21u64), ((rax as u8)) as u8); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x21u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x19u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rsp = ((rsp).wrapping_add(0xd8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049e5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x19; const P1: u64 = 0x10; const P2: u64 = 0x14049e5c0; const P3: u64 = 0x1401de3c0; const P4: u64 = 0x141834d30; const P5: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P0))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = ((rax).wrapping_add(0x1cu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0x4c; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049ea50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x19; const P1: u64 = 0x10; const P2: u64 = 0x14049ea50; const P3: u64 = 0x14049f530;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(P0))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049ead0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    rax = ((rax).wrapping_sub(0x10u64)) as u64;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax);
    rax = rsp.wrapping_add(0x8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049eb20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14028d0a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049eb60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e88b0; const P1: u64 = 0x1401fc770;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = ((rax).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = ((rax).wrapping_add(0x8u64)) as u64;
    rdx = rax;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = ((rax).wrapping_add(0x30u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = ((rax).wrapping_add(0x30u64)) as u64;
    rdx = rax;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049ebe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049ec50; const P1: u64 = 0x8; const P2: u64 = 0x9;
    let (mut rax, mut fl_l, mut fl_r, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = rsp.wrapping_add(0x20);
    rdi = rax;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rcx = 0x1;
    while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); }
    rax = rsp.wrapping_add(0x21);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x0u32) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P1), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P2), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049ec50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x14049ecb0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rdx = 0x8; rcx = 0x38; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0x38; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 2; continue; }
            1 => { bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049ecb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049ed30; const P1: u64 = 0x1401f3bb0; const P2: u64 = 0x1401f3ba0; const P3: u64 = 0x1401f3e50; const P4: u64 = 0x1401e19b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = ((rax).wrapping_add(0xcu64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = ((rax).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = ((rax).wrapping_add(0x18u64)) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = ((rax).wrapping_add(0xcu64)) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049ed30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de2f0; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P1), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049ed60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140499560;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rsp.wrapping_add(0x38);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049edb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64;
    rax = (((rax) as i64).wrapping_mul((0x10u64) as i64) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_r = rax;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu32(rsp, (0x1u32) as u32); } else { crate::simd_ops::wu32(rsp, (0x0u32) as u32); }
    rax = ((crate::simd_ops::ru8(rsp)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049eea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049ee30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = 0x2;
    let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128);
    if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; }
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = ((rcx).wrapping_add(rax)) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_r = rax;
    if !((((fl_l as u64)) >= ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); } else { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049ef10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049d540; const P1: u64 = 0x142442e70; const P2: u64 = 0x14049f720; const P3: u64 = 0x14049d680;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (((crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as i64).wrapping_mul((0x10u64) as i64) as u64) as u64; rdx = 0x8; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax) as i64).wrapping_mul((0x10u64) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = ((rax).wrapping_add(0xcu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xc8u64))) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0xb8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f0a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049f6f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; bb = 2; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = ((rax).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401e3e40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rdx = 0x1; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x1401de2f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x0u32) as u32);
    rcx = 0x1;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); } else { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); }
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64;
    rax = ((((rax as u32)) | (0x1u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f1f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141a0ad60; const P1: u64 = 0x142645a68; const P2: u64 = 0x141a08240;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = rsp.wrapping_add(0x28);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rcx = rsp.wrapping_add(0x28);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14049e1e0; const P1: u64 = 0x1402174f0; const P2: u64 = 0x14049d430; const P3: u64 = 0x140200130; const P4: u64 = 0x14049f7a0; const P5: u64 = 0x14049cc20; const P6: u64 = 0x14049cda0;
    let (mut rax, mut fl_l, mut fl_r, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x148u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = rsp.wrapping_add(0x120); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0xd0); rdi = rcx; rsi = rax; rcx = 0x18; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (((crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as i8 as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(0x1cu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x1u32) as u32); bb = 4; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x0u32) as u32); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x8u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; bb = 9; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x178u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = rsp.wrapping_add(0xf8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), rax); rcx = rsp.wrapping_add(0xf8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x110); rcx = rsp.wrapping_add(0xd0); rdi = rax; rsi = rcx; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, crate::simd_ops::ru8(rsi)); rdi = rdi.wrapping_add(1); rsi = rsi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rdx = rsp.wrapping_add(0x110); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x31u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(0x8u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x148u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x141834d30; const P2: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = ((rax).wrapping_add(0x20u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = ((rax).wrapping_add(0x48u64)) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rax = ((((rax as u32)) & (0x1u32))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = 0x88; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f5a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0x14049fbc0; const P3: u64 = 0x1401fa0a0; const P4: u64 = 0x1401e54e0; const P5: u64 = 0x14028d0a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(P1), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; rdx = rsp.wrapping_add(0x60); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x20u64), (0xfffffffeu32) as u32); rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); bb = 2; continue; }
            2 => { rsp = ((rsp).wrapping_add(0x78u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14028d0a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rax);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a0010;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = ((rcx).wrapping_sub(rax)) as u64;
    rax = rcx;
    rax = ((((rax) as i64).wrapping_shr(4u32) as u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049f7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442e70; const P1: u64 = 0x1401de2e0; const P2: u64 = 0x1401f3380;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = 0x1; rax = (((rax) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rdx = 0x8; rcx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x1cu64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = 0x4c; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x48u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x19u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsp = ((rsp).wrapping_add(0xa8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049fb80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x9; const P1: u64 = 0x1404a0630;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P0), (0x1u8) as u8);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049fbc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e5530; const P1: u64 = 0x14028e350; const P2: u64 = 0x14028e000; const P3: u64 = 0x14307d640; const P4: u64 = 0x1404a01d0; const P5: u64 = 0x1401f3ce0; const P6: u64 = 0x1402c0a50; const P7: u64 = 0x1401e5510; const P8: u64 = 0x1401f3e20; const P9: u64 = 0x1401e54e0; const P10: u64 = 0x14049ffa0; const P11: u64 = 0x1402c0a60; const P12: u64 = 0x1401f4970; const P13: u64 = 0x1404a0270;
    let (mut rax, mut fl_l, mut fl_r, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rdx = 0x2; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdx = P3; rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32 as u64; rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = 0x4; r8 = rax; rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (0x0u64) as u64); rax = rsp.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            4 => { bb = 9; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = rsp.wrapping_add(0x38); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            7 => { bb = 9; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rax = ((((rax as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            9 => { rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = 0x10; while rcx != 0 { crate::simd_ops::wu8(rdi, rax as u8); rdi = rdi.wrapping_add(1); rcx = rcx.wrapping_sub(1); } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            11 => { bb = 13; continue; }
            12 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((((rax as u32)) | (0x1u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; bb = 13; continue; }
            13 => { rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14049fee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de2f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx, rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401f3d00; const P1: u64 = 0x14028d0a0; const P2: u64 = 0x14028dfe0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = ((rax).wrapping_add(0x20u64)) as u64;
    rcx = rax;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = ((rax).wrapping_add(0x8u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); }
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1402d7470; const P1: u64 = 0x14314f9c8; const P2: u64 = 0x8; const P3: u64 = 0x9; const P4: u64 = 0x1401f3d70;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = P1; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(P2), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(P3), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rax).wrapping_add(0x20u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0440(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rcx = ((rcx).wrapping_sub(rax)) as u64;
    rax = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = ((rcx).wrapping_add(rax)) as u64;
    rax = rcx;
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a0800; const P1: u64 = 0x1404a08c0; const P2: u64 = 0x1401de410; const P3: u64 = 0x1401f4df0; const P4: u64 = 0x1401de2f0; const P5: u64 = 0x1401f3bc0; const P6: u64 = 0x14028e000; const P7: u64 = 0x1401e19b0; const P8: u64 = 0x1402c0a60; const P9: u64 = 0x14028d0a0; const P10: u64 = 0x14049ffa0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x88u64)) as u64; rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); r8 = 0x4; rdx = 0x1; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (((rax) & (0x2u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); bb = 3; continue; }
            2 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = ((rax).wrapping_add(0xcu64)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x88u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    rax = (((rax) & (0x2u64))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu8(rsp, (0x1u8) as u8); } else { crate::simd_ops::wu8(rsp, (0x0u8) as u8); }
    rax = ((crate::simd_ops::ru8(rsp)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a07c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a08c0; const P1: u64 = 0x1401de410;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1401de2e0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = rax; rcx = 0x4; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = rax; rcx = 0x4; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(rsp)) as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rax, rcx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64;
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a08c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1404a0920; const P2: u64 = 0x1401de2e0; const P3: u64 = 0x1404a0800;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P0))) as u32) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); } else { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = 0x8; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x1u8) as u8); }
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1401de2e0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 8; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = rax; rcx = 0x4; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 8; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = rax; rcx = 0x4; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 8; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8dd18; const P1: u64 = 0x140226200; const P2: u64 = 0x14021fa50; const P3: u64 = 0x143c8de80; const P4: u64 = 0x1402215b0; const P5: u64 = 0x143c8ce68; const P6: u64 = 0x143c95d90; const P7: u64 = 0x14037d830; const P8: u64 = 0x1403706b0; const P9: u64 = 0x143c964d8; const P10: u64 = 0x143c96db8; const P11: u64 = 0x1401de410; const P12: u64 = 0x140389050; const P13: u64 = 0x143c96b10; const P14: u64 = 0x143c9eb98; const P15: u64 = 0x14037da40; const P16: u64 = 0x14040f930; const P17: u64 = 0x143c9d958; const P18: u64 = 0x141e60f70; const P19: u64 = 0x14021cff0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 51>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1404a0a30, funcinfo_rva: 0x35de258, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x198, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x80u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x180u64)) as u64; rdi = (r8) as u64; rsi = (rdx) as u64; rbx = (rcx) as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((rax as u32)) as u32); r15 = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r15) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rax) as u64); rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), ((rax as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0ab1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0ad0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x34u64), ((rax as u32)) as u32); r15 = ((P7) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r15) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xc0u64))) as u64; let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); rax = (crate::simd_ops::ru32(P9)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0b3a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0b57); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6cu64), ((rax as u32)) as u32); r15 = ((P11) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), (r15) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x68u64), (0x1u8) as u8); rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (rax) as u64); rax = (crate::simd_ops::ru32(P13)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), ((rax as u32)) as u32); rdx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0bad); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0bca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x60u64), (xmm0) as u128); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(P14)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rax as u32)) as u32); r15 = ((P15) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), (r15) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rcx as u32)) as u32); rax = ((P16) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), (rax) as u64); rax = (crate::simd_ops::ru32(P17)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0c2c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0c49); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm0) as u128); bb = 16; continue; }
            16 => { r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r8 = (rdi) as u64; rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0c78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0c83); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; r11 = ((rsp.wrapping_add(0x180u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a0cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8dd18; const P1: u64 = 0x140226200; const P2: u64 = 0x14021fa50; const P3: u64 = 0x143c8de80; const P4: u64 = 0x1402215b0; const P5: u64 = 0x143c8ce68; const P6: u64 = 0x143c95d90; const P7: u64 = 0x14037d830; const P8: u64 = 0x1403706b0; const P9: u64 = 0x143c964d8; const P10: u64 = 0x143c96db8; const P11: u64 = 0x1401de410; const P12: u64 = 0x140389050; const P13: u64 = 0x143c96b10; const P14: u64 = 0x143c9eb98; const P15: u64 = 0x14037da40; const P16: u64 = 0x14040f930; const P17: u64 = 0x143c9d958; const P18: u64 = 0x143c95f60; const P19: u64 = 0x140370740; const P20: u64 = 0x143c94b54; const P21: u64 = 0x141e60f70; const P22: u64 = 0x14021cff0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 61>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1404a0cb0, funcinfo_rva: 0x35de298, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x1e8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0xd0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1d0u64)) as u64; rdi = (r8) as u64; rsi = (rdx) as u64; rbx = (rcx) as u64; xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (r14) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((rax as u32)) as u32); r15 = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (r15) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rax) as u64); rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), ((rax as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0d34); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0d53); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x34u64), ((rax as u32)) as u32); r15 = ((P7) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (r15) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x110u64))) as u64; let y_addr: u64 = rax; xmm0 = core::ptr::read_unaligned(y_addr as *const u128); ymm0_hi = core::ptr::read_unaligned(y_addr.wrapping_add(16) as *const u128); let y_addr: u64 = rbp.wrapping_sub(0x70u64); core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); rax = (crate::simd_ops::ru32(P9)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0dbd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0dda); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xbcu64), ((rax as u32)) as u32); r15 = ((P11) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), (r15) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0xb8u64), (0x1u8) as u8); rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), (rax) as u64); rax = (crate::simd_ops::ru32(P13)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), ((rax as u32)) as u32); rdx = ((rbp.wrapping_add(0x80u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0e42); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_add(0x80u64)) as u64) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0e65); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0xb0u64), (xmm0) as u128); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(P14)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rax as u32)) as u32); r15 = ((P15) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), (r15) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x18u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x120u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), ((rcx as u32)) as u32); rax = ((P16) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), (rax) as u64); rax = (crate::simd_ops::ru32(P17)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0eca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0ee7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm0) as u128); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(P18)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x6cu64), ((rax as u32)) as u32); r15 = ((P15) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), (r15) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x68u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x128u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x30u64), ((rcx as u32)) as u32); rax = ((P19) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (rax) as u64); rax = (crate::simd_ops::ru32(P20)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), ((rax as u32)) as u32); rdx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0f49); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0f66); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x60u64), (xmm0) as u128); bb = 20; continue; }
            20 => { r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r8 = (rdi) as u64; rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0f95); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a0fa0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rbx) as u64; r11 = ((rsp.wrapping_add(0x1d0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a1430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143ca60bc; const P1: u64 = 0x143ca6190; const P2: u64 = 0x1404a1f90; const P3: u64 = 0x143c8d510; const P4: u64 = 0x14020afa0; const P5: u64 = 0x1420a6b70; const P6: u64 = 0x141834a78; const P7: u64 = 0x143150716; const P8: u64 = 0x142646960; const P9: u64 = 0x143ca61b0; const P10: u64 = 0x143ca61d0; const P11: u64 = 0x143ca61d4; const P12: u64 = 0x143ca61d6; const P13: u64 = 0x141834a0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; r15 = rdx; r14 = rcx; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 1; continue; }
            1 => { rsi = P1; rbx = (0xffffffffffffffffu64) as u64; bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(rbx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r9 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0xa0); rcx = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rbp = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rbx = rcx.wrapping_add(0x10); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rsi = rdx; rsi = ((((rsi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rsi; rax = ((rax).wrapping_shr(63u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsi.wrapping_add(0x1); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x30u64), (xmm0) as u128); rbx = ((crate::simd_ops::ru16(rsp.wrapping_add(0xa0u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0xeu64), ((rbx as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0xcu64), ((rbp as u16)) as u16); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x18u64), rsi); rcx = (crate::simd_ops::ru64(r14)) as u64; r9 = 0x2; r8 = ((crate::simd_ops::ru16(r14.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = (((rsi as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r14.wrapping_add(0x8u64))) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r15, rax); crate::simd_ops::wu16(r15.wrapping_add(0x8u64), ((rbx as u16)) as u16); crate::simd_ops::wu16(r15.wrapping_add(0xau64), ((rsi as u16)) as u16); crate::simd_ops::wu128(r15.wrapping_add(0x10u64), (xmm0) as u128); rax = r15; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 7; continue; }
            7 => { r8 = 0x46; rdx = P7; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rsp.wrapping_add(0x76u64), (0x0u8) as u8); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P1; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P9; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(P10, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x74u64))) as u32) as u32 as u64; crate::simd_ops::wu16(P11, ((rax as u16)) as u16); crate::simd_ops::wu8(P12, (0x0u8) as u8); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a1a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143ca63a8; const P1: u64 = 0x143ca6268; const P2: u64 = 0x1411e9f50; const P3: u64 = 0x1404a4310; const P4: u64 = 0x143ca6188; const P5: u64 = 0x1420af120; const P6: u64 = 0x143c8df88; const P7: u64 = 0x140215610; const P8: u64 = 0x1420a28e0; const P9: u64 = 0x1404a3b10; const P10: u64 = 0x1404a3260; const P11: u64 = 0x143c8d510; const P12: u64 = 0x14020afa0; const P13: u64 = 0x1420a6b70; const P14: u64 = 0x141834a78; const P15: u64 = 0x143150246; const P16: u64 = 0x142646960; const P17: u64 = 0x143ca6288; const P18: u64 = 0x143ca6298; const P19: u64 = 0x143ca629c; const P20: u64 = 0x143ca629e; const P21: u64 = 0x141834a0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; r12 = rdx; r15 = rcx; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 1; continue; }
            1 => { rbp = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rdi = (0xffffffffffffffffu64) as u64; rax = rdi; bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(r15)) as u64; r14 = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; rcx = r14.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu16(rax, ((rbx as u16)) as u16); bb = 4; continue; }
            4 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rbp))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); rbp = (crate::simd_ops::ru64(r14)) as u64; r14 = ((r14).wrapping_add(0x48u64)) as u64; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rsp.wrapping_add(0x40); rdx = rbp; rcx = rsp.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x98u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0xa8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rsp.wrapping_add(0xb8u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsi.wrapping_add(0xc8u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsi.wrapping_add(0xe8u64), ((xmm0 as u64)) as u64); rbx = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x50u64), (1) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x113u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P9; crate::simd_ops::wu64(rsi.wrapping_add(0x120u64), rax); rax = P10; crate::simd_ops::wu64(rsi.wrapping_add(0x128u64), rax); crate::simd_ops::wu8(rsi.wrapping_add(0x115u64), (0x1u8) as u8); rbx = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r14 = ((crate::simd_ops::ru16(P11)) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbp = rdx; rbp = ((((rbp) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbp; rax = ((rax).wrapping_shr(63u32)) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = rbp.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0xeu64), ((rbx as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0xcu64), ((r14 as u16)) as u16); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x18u64), rbp); rcx = (crate::simd_ops::ru64(r15)) as u64; r9 = 0x2; r8 = ((crate::simd_ops::ru16(r15.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r15.wrapping_add(0x8u64))) as u128; ymm0_hi = 0; rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(r12, rax); crate::simd_ops::wu16(r12.wrapping_add(0x8u64), ((rbx as u16)) as u16); crate::simd_ops::wu16(r12.wrapping_add(0xau64), ((rbp as u16)) as u16); crate::simd_ops::wu128(r12.wrapping_add(0x10u64), (xmm0) as u128); rax = r12; r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 11; continue; }
            11 => { r8 = 0x36; rdx = P15; rcx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rsp.wrapping_add(0x96u64), (0x0u8) as u8); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P1; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; ymm1_hi = 0; crate::simd_ops::wu128(P17, (xmm1) as u128); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; crate::simd_ops::wu32(P18, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x94u64))) as u32) as u32 as u64; crate::simd_ops::wu16(P19, ((rax as u16)) as u16); crate::simd_ops::wu8(P20, (0x0u8) as u8); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a1d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411e9f50; const P1: u64 = 0x1404a3e30; const P2: u64 = 0x143ca6220; const P3: u64 = 0x1420af120; const P4: u64 = 0x143c8df88; const P5: u64 = 0x140215610; const P6: u64 = 0x143ca61dc; const P7: u64 = 0x143ca637c; const P8: u64 = 0x1420a25b0; const P9: u64 = 0x1420a28e0; const P10: u64 = 0x1404a3650; const P11: u64 = 0x1404a2a40; const P12: u64 = 0x141834a78; const P13: u64 = 0x143c9c018; const P14: u64 = 0x143ca6378; const P15: u64 = 0x143ca6380; const P16: u64 = 0x141834a0c;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); r14 = rdx; rsi = rcx; xmm6 = (crate::simd_ops::ru128(r9)) as u128; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; crate::simd_ops::wu16(rax, ((rbx as u16)) as u16); rax = ((xmm6 as u64)) as u64; rdx = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rdx); r15 = (crate::simd_ops::ru64(rsi)) as u64; r12 = rsi.wrapping_add(0x48); rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x69u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x69); rdx = r15; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(0xa8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rdi.wrapping_add(0xc8u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdi.wrapping_add(0xe8u64), ((xmm0 as u64)) as u64); rbx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rdi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x50u64), (1) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x113u64), (0x1u8) as u8); rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), (0x1u64) as u64); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x69u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rdi); rdx = rbp.wrapping_sub(0x49); rcx = rbp.wrapping_sub(0x69); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rdi); rcx = rbp.wrapping_sub(0x69); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P10; crate::simd_ops::wu64(rdi.wrapping_add(0x120u64), rax); rax = P11; crate::simd_ops::wu64(rdi.wrapping_add(0x128u64), rax); crate::simd_ops::wu8(rdi.wrapping_add(0x115u64), (0x1u8) as u8); rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; crate::simd_ops::wu16(r14, ((rax as u16)) as u16); rax = r14; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rcx = P6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P6)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru16(P13)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x67u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x65u64), rax); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(P14, ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x61u64))) as u32 as u64; crate::simd_ops::wu32(P15, ((rax as u32)) as u32); rcx = P6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a2340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e6d50; const P1: u64 = 0x14021fad0; const P2: u64 = 0x142535070; const P3: u64 = 0x1426456c8; const P4: u64 = 0x1426453a0; const P5: u64 = 0x140213060; const P6: u64 = 0x143151e80; const P7: u64 = 0x1401e84a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rdx; rbx = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x3cu64))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_mul(8)) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rcx).wrapping_add(0x30u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = P1; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdx); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x38u64), (1) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = rdx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rsi = 0x8; rcx = ((rsi as u32)) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rcx }) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; rbp = 0x10000; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = ((rdx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rax); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { r9 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r8 = 0x10; rdx = rbp; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rdx = rsp.wrapping_add(0x30); rcx = rbx.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rcx); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rbp); rax = rcx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rsi = ((rsi).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rsi }) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = rbp; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = (0x40u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = ((rcx.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rax); bb = 11; continue; }
            10 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { rax = P6; crate::simd_ops::wu64(rdi, rax); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r14.wrapping_add(0x1cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((rax as u32)) as u32); rcx = rdi.wrapping_add(0x20); crate::simd_ops::wu64(rcx, (0x0u64) as u64); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x14u64), ((rax as u8)) as u8); rdx = r14.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((rcx as u32)) as u32); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x40u64), (0x40u64) as u64); crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x48u64), (0xffffffffffffffc0u64) as u64); rbp = ((rbp).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x48u64)))) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x8u64), rbp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a2510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a4200; const P1: u64 = 0x143ca60c0; const P2: u64 = 0x1420af120; const P3: u64 = 0x1404a4060; const P4: u64 = 0x143ca60c4; const P5: u64 = 0x140215750; const P6: u64 = 0x1404a3f20; const P7: u64 = 0x143ca6184; const P8: u64 = 0x14036fad0; const P9: u64 = 0x143c8d08c; const P10: u64 = 0x140215610; const P11: u64 = 0x1420a28e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; r15 = r9; rsi = r8; r14 = rcx; rcx = rbp.wrapping_add(0x1f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rdx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x49); rdx = rsi; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(0xa8u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r14.wrapping_add(0xc8u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(r14.wrapping_add(0xe8u64), ((xmm0 as u64)) as u64); rcx = rbp.wrapping_add(0x1f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x49); rdx = rsi; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = r14.wrapping_add(0xf0); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x40u64), ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rcx = rbp.wrapping_add(0x1f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; let y_addr: u64 = rax; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_sub(0x49u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r9 = ((crate::simd_ops::ru16(r9.wrapping_add(rcx.wrapping_mul(2)))) as u32) as u32 as u64; r8 = rbp.wrapping_sub(0x49); rdx = rsi; rcx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(3u32)) as u64; rax = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x40u64), ((xmm0 as u64)) as u64); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r14); rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x44u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = r14.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(r14.wrapping_add(0x40u64), (1) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), r15); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r14); rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0xc0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a2750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x10088; const P2: u64 = 0x1404a2820; const P3: u64 = 0x1404a2de0; const P4: u64 = 0x1404a2910;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; rbx = (((rdx as u16)) as u32) as u32 as u64; r14 = ((rsi.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rbx.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rcx; rcx = rsp.wrapping_add(0xb0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r14.wrapping_add(P0))) as u64; rcx = rsp.wrapping_add(0x30); r9 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rdx = (((rbx as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; let y_addr: u64 = rsp.wrapping_add(0x70u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); r8 = rsp.wrapping_add(0xb0); rdx = rsp.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { r11 = rsp.wrapping_add(0x100); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a2910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0x28; const P5: u64 = 0x48; const P6: u64 = 0x38; const P7: u64 = 0x141a10d60;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbx = r8; rdi = rdx; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(P1))) as u128; xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r9 = rsp.wrapping_add(0x30); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P3))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); r8 = rsp.wrapping_add(0x80); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rdx = rsp.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rcx = rsp.wrapping_add(0xc0); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P1))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), ((xmm1 as u64)) as u64); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P6))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), ((xmm1 as u64)) as u64); let y_addr: u64 = rsp.wrapping_add(0xc0u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { r11 = rsp.wrapping_add(0xf0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a2a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x10088; const P2: u64 = 0x143c8df88; const P3: u64 = 0x580e0; const P4: u64 = 0x1404a2ce0; const P5: u64 = 0x141a100e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (((rdx as u16)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(P1))) as u32 as u64; bb = 3; continue; }
            2 => { rax = ((rdi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r9 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rax.wrapping_mul(8)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r12; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 9; continue; }
            9 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a2ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x143084e28; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = 0;
    fl_l = 0; fl_r = 0;
    let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rcx.wrapping_add(P0), rax);
    rax = (tmp_xchg) as u64;
    rcx = P1;
    crate::simd_ops::wu64(rbx, rcx);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x40; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a33a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a2820; const P1: u64 = 0x143c8dbd0; const P2: u64 = 0x142489ee0; const P3: u64 = 0x143c8d508; const P4: u64 = 0x143c8e658; const P5: u64 = 0x141a10d60;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 51>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x78); rsp = ((rsp).wrapping_sub(0x178u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = rcx; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = ((rbx).wrapping_add(rbx)) as u64; rsi = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), rbx); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { rdx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x170u64), r13); rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = (crate::simd_ops::ru64(rdi)) as u64; rsi = ((rbx.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; r13 = ((r13).wrapping_add(0x40u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x1a0u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(r13.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbp.wrapping_add(0xa8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r12 = (crate::simd_ops::ru64(rax.wrapping_add(r13).wrapping_add(0x18058u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(r13).wrapping_add(0x1806cu64))) as u32 as u64; rcx = r12; rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r13))) as u64; rdi = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = 0xc03; rdx = r15; rcx = r12; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(r13))) as u64; r12 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r12 = 0; fl_l = 0; fl_r = 0; fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r8 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = r15; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(r13))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 16; continue; }
            15 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { fl_l = rdi; fl_r = rdi; rax = ((r14.wrapping_add(rdi)) as u64) as u64; rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rdi }) as u64; fl_l = r12; fl_r = r12; rax = ((r14.wrapping_add(r12)) as u64) as u64; r12 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r12 }) as u64; fl_l = r15; fl_r = r15; rax = ((r14.wrapping_add(r15)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_r = r14; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; bb = 18; continue; }
            18 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x18u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), ((xmm0 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x18u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); r9 = rbp.wrapping_sub(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); r8 = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); rdx = rbp.wrapping_sub(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); rcx = rbp.wrapping_add(0x30); let y_addr: u64 = rsp.wrapping_add(0x50u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64) as u128; let y_addr: u64 = rbp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 20; continue; }
            20 => { rbx = ((rbx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), rbx); fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 21; continue; }
            21 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a0u64))) as u64; bb = 22; continue; }
            22 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; bb = 23; continue; }
            23 => { rsp = ((rsp).wrapping_add(0x178u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a3650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8df88; const P1: u64 = 0x143ca6370; const P2: u64 = 0x142489ee0; const P3: u64 = 0x143c8e660; const P4: u64 = 0x141a100e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (((r8 as u16)) as u32) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rdi = ((rdi).wrapping_add(rdi)) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rbp); rbp = ((rdi.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r14); r14 = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsi = (crate::simd_ops::ru64(r14.wrapping_add(0x580e0u64))) as u64; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r13); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); bb = 3; continue; }
            3 => { r11 = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; r12 = (crate::simd_ops::ru64(r14.wrapping_add(r11.wrapping_mul(8)).wrapping_add(0x88u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x10088u64))) as u32 as u64; bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { r8 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = r11; rax = ((rax).wrapping_shl(5u32)) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(r14).wrapping_add(0x180acu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r14).wrapping_add(0x18098u64))) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r11.wrapping_add(0xc03); fl_l = ((rax).wrapping_shl(5u32)) as u64; rax = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r14).wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 10; continue; }
            9 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rax = 0xc03; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rdx = r15; r11 = ((r11).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((r11).wrapping_shl(5u32)) as u64; r11 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(r14).wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; bb = 13; continue; }
            12 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; rax = ((rbx.wrapping_add(r12)) as u64) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rbx }) as u64; fl_l = r15; fl_r = r15; rax = ((r15.wrapping_add(r12)) as u64) as u64; r15 = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { r15 }) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); let y_addr: u64 = rsp.wrapping_add(0x30u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(1)) as u64; fl_l = r14; fl_r = r13; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 18; continue; }
            18 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 19; continue; }
            19 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 20; continue; }
            20 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 21; continue; }
            21 => { rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a4800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143151e80; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x14; const P7: u64 = 0x1401e84a0; const P8: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rdi = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x20u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((rax as u32)) as u32);
    rcx = ((rcx).wrapping_add(0x20u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rax as u8)) as u8);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a4870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426bf8; const P1: u64 = 0x14; const P2: u64 = 0x28; const P3: u64 = 0x24; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x2c; const P7: u64 = 0x143426d6c; const P8: u64 = 0xc; const P9: u64 = 0x8; const P10: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); xmm6 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P2)))).to_bits() as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm10) as u128); xmm10 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); crate::simd_ops::wu128(rsp, (xmm11) as u128); xmm11 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; xmm7 = (xmm11 & !0xffff_ffffu128) | ((f32::from_bits(xmm11 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) - f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm3 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P6)))).to_bits() as u128); xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm1 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm10 & !0xffff_ffffu128) | ((f32::from_bits(xmm10 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm5 = (xmm9 & !0xffff_ffffu128) | ((f32::from_bits(xmm9 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm2 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm4 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P8))) as u64) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rdx, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rdx.wrapping_add(P9), ((rax as u32)) as u32); rax = rdx; xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32) as u128; xmm6 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm11 = (crate::simd_ops::ru128(rsp)) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P10)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32) as u128; xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P9)))).to_bits() as u128); crate::simd_ops::wu32(rdx, ((xmm3 as u32)) as u32); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P10)))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu32(rdx.wrapping_add(P10), ((xmm3 as u32)) as u32); xmm3 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P9)))).to_bits() as u128); crate::simd_ops::wu32(rdx.wrapping_add(P9), ((xmm3 as u32)) as u32); rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a4980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rdi = rdx; rsi = rcx; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); r9 = (crate::simd_ops::ru64(rdx)) as u64; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu64(rdx, r9); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0xb0u64)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); fl_l = (crate::simd_ops::ru8(r9)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r9)); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; r9 = ((r9).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rdi, r9); rax = (((rcx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(r9)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); bb = 4; continue; }
            4 => { xmm6 = (crate::simd_ops::ru128(r8)) as u128; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { crate::simd_ops::lk_sub_u64(rsi.wrapping_add(0x10u64), (1) as u64); r10 = (crate::simd_ops::ru64(rsi)) as u64; rax = ((xmm6 as u64)) as u64; r9 = rax; r9 = ((r9).wrapping_sub(r10)) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = crate::simd_ops::vpsignb(xmm0, xmm0); rax = (crate::simd_ops::vpmovmskb(xmm1) as u32) as u32 as u64; r11 = r9.wrapping_sub(0x10); rcx = r11; rcx = (((rcx) & (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))))) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r10))) as u128; xmm1 = crate::simd_ops::vpsignb(xmm0, xmm0); rcx = (crate::simd_ops::vpmovmskb(xmm1) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r8 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; let bit_in: u32 = (rcx as u32); if bit_in != 0 { rdx = (((31u32.wrapping_sub(bit_in.leading_zeros())) as u32)) as u32 as u64; } fl_l = bit_in as u64; if (((fl_l as u32)) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0x1f; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 11; continue; }
            10 => { rax = 0x20; bb = 11; continue; }
            11 => { rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 14; continue; }
            13 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rdx = 0xfffffffe; rax = 0xffffff80; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu8(r10.wrapping_add(r9), ((rdx as u8)) as u8); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rcx = rax; rcx = (((rcx) & (r11))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((rdx as u8)) as u8); rax = (((r8 as u8)) as u32) as u32 as u64; crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x28u64), (rax) as u64); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a4ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a7040;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); r9 = (crate::simd_ops::ru64(rdx)) as u64; rbx = rdx; r9 = ((r9).wrapping_add(1)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); crate::simd_ops::wu64(rdx, r9); rdi = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = ((rdx).wrapping_add(0x110u64)) as u64; fl_l = (crate::simd_ops::ru8(r9)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 4; continue; } bb = 1; continue; }
            1 => { xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r9)); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; r9 = ((r9).wrapping_add(rax)) as u64; rax = (((rax) as i64).wrapping_mul((0x110u64) as i64) as u64) as u64; crate::simd_ops::wu64(rbx, r9); rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(r9)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rdx); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; xmm6 = (crate::simd_ops::ru128(r8)) as u128; rcx = ((rcx).wrapping_add(0x10u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u64(rdi.wrapping_add(0x10u64), (1) as u64); r10 = (crate::simd_ops::ru64(rdi)) as u64; rax = ((xmm6 as u64)) as u64; r9 = rax; r9 = ((r9).wrapping_sub(r10)) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = crate::simd_ops::vpsignb(xmm0, xmm0); r11 = r9.wrapping_sub(0x10); rcx = r11; rcx = (((rcx) & (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))))) as u64; rax = (crate::simd_ops::vpmovmskb(xmm1) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r10))) as u128; xmm1 = crate::simd_ops::vpsignb(xmm0, xmm0); rcx = (crate::simd_ops::vpmovmskb(xmm1) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r8 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = (((rcx as u32)).wrapping_shl(16u32)) as u32 as u64; let bit_in: u32 = (rcx as u32); if bit_in != 0 { rdx = (((31u32.wrapping_sub(bit_in.leading_zeros())) as u32)) as u32 as u64; } fl_l = bit_in as u64; if (((fl_l as u32)) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x1f; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            8 => { rax = 0x20; bb = 9; continue; }
            9 => { rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = 0xffffff80; rdx = 0xfffffffe; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; crate::simd_ops::wu8(r10.wrapping_add(r9), ((rdx as u8)) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (r11))) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((rdx as u8)) as u8); rax = (((r8 as u8)) as u32) as u32 as u64; crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x28u64), (rax) as u64); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a4c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142442de0; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x1430782a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 1; continue; }
            1 => { r14 = 0; fl_l = 0; fl_r = 0; rdi = ((r14 as u32)) as u32 as u64; rbp = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rbp).wrapping_add(0x10u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = ((rdi).wrapping_add(1)) as u64; rbp = ((rbp).wrapping_add(0xb0u64)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { rcx = r9.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            11 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = P3; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r14); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a4d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a7040; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200; const P3: u64 = 0x1430782a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; rbx = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = 0; fl_l = 0; fl_r = 0; rdi = ((r14 as u32)) as u32 as u64; rbp = ((r14 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rax))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdi = ((rdi).wrapping_add(1)) as u64; rbp = ((rbp).wrapping_add(0x110u64)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = r9.wrapping_sub(0x1); rax = r9; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rbx.wrapping_add(0x8); bb = 13; continue; }
            10 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rsi = rbx.wrapping_add(0x8); bb = 13; continue; }
            12 => { r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsi, r14); rax = P3; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r14); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx, rax); bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a4e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x1c; const P7: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 1; continue; }
            1 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 3; continue; }
            3 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 5; continue; }
            5 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 7; continue; }
            7 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 9; continue; }
            9 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 11; continue; }
            11 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 12; continue; }
            12 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P5))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P5))) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 13; continue; }
            13 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 15; continue; }
            15 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 16; continue; }
            16 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P7))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P7))) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 17; continue; }
            17 => { if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a4f00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc2; const P5: u64 = 0x1404a70d0; const P6: u64 = 0x1425371a0; const P7: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; r14 = rcx; r15 = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r12 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdi = rdx.wrapping_add(0x20); rdi = (((rdi) & (0xfffffffffffffff0u64))) as u64; rdx = (((rdx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(0xfu64)) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rdx = (((rdx) & (0xfffffffffffffff0u64))) as u64; r10 = 0x6348; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; r13 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            4 => { rax = (0x7fffffffffffffffu64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = r13; bb = 12; continue; }
            6 => { rax = rdx.wrapping_sub(0x10); fl_l = rax; fl_r = (0x3f0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx.wrapping_add(0x7); rax = ((rax).wrapping_shr(3u32)) as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rax = rcx; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rax); bb = 12; continue; }
            10 => { r8 = 0x10; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = rax; bb = 12; continue; }
            12 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = 0xdeadd00d; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r13; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r14, rcx); rax = ((rdi.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rax); r8 = rbx.wrapping_add(0x10); rdx = 0xffffff80; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu8(rbx.wrapping_add(rax), (0xffu8) as u8); rax = rbx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = rbx; rcx = ((rcx).wrapping_sub(rax)) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r14.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rbx); rsi = r13; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 15; continue; }
            15 => { rdi = rbp.wrapping_add(0x1); rax = (0xcbf29ce484222325u64) as u64; rbp = (0x100000001b3u64) as u64; rdx = (0xde5fb9d2630458e9u64) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(rsi))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 29; continue; } bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_sub(0x1u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbp) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbp) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbp) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbp) as i64) as u64) as u64; rax = rdx; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r11 = ((rdx.wrapping_add(rax)) as u64) as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = r11; rcx = ((rcx).wrapping_shr(7u32)) as u64; rdx = r13; rcx = (((rcx) & (r9))) as u64; r10 = (crate::simd_ops::ru64(r14)) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r10))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rcx.wrapping_add(r10))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r8 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; r8 = ((r8).wrapping_add(rcx)) as u64; r8 = (((r8) & (r9))) as u64; r11 = (r11 & !0xffu64) | ((((((r11 as u8)) & (0x7fu8))) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(r10), ((r11 as u8)) as u8); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rcx = r8.wrapping_sub(0x10); rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r11 as u8)) as u8); rcx = (((r8) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_sub(0x1u64))) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xfu64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0xfu64), r13); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); rcx = ((rcx).wrapping_add(0x20u64)) as u64; rdx = rdi.wrapping_add(0x1f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xfu64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = rbx.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(0x108u64)) as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            25 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            26 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rdx = rcx; r9 = rbx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rax = (0xcbf29ce484222325u64) as u64; rdx = (0xde5fb9d2630458e9u64) as u64; bb = 29; continue; }
            29 => { rsi = ((rsi).wrapping_add(1)) as u64; rdi = ((rdi).wrapping_add(0xb0u64)) as u64; fl_l = rsi; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 30; continue; }
            30 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 31; continue; }
            31 => { rcx = r15.wrapping_sub(0x1); rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = r15; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = rcx.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r15, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r15); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 38; continue; } bb = 34; continue; }
            34 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            35 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 37; continue; }
            36 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 37; continue; }
            37 => { r9 = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a52b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x142535070; const P2: u64 = 0x1426456c8; const P3: u64 = 0x1426453a0; const P4: u64 = 0x14183acc2; const P5: u64 = 0x1404a7180; const P6: u64 = 0x1404a7040; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rsi = rdx.wrapping_add(0x20); r14 = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rsi = (((rsi) & (0xfffffffffffffff0u64))) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rdi = rcx; rcx = (crate::simd_ops::ru64(rax)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rdx = (((rdx) as i64).wrapping_mul((0x110u64) as i64) as u64) as u64; r10 = 0x6348; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = ((rdx) & ((rdx).wrapping_sub(1))) as u64; rax = ((fl_l as u64)) as u64; r10 = (crate::simd_ops::ru64(r10.wrapping_add(rcx))) as u64; if (((fl_l as u64)) != 0) { bb = 4; continue; } bb = 1; continue; }
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
            14 => { rax = ((rsi.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rdi, rcx); r8 = rbx.wrapping_add(0x10); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); rdx = 0xffffff80; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rbx; crate::simd_ops::wu8(rbx.wrapping_add(rax), (0xffu8) as u8); rax = rbx; rax = ((rax).wrapping_shr(3u32)) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbx); rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; rbx = r12; crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rcx); fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r13); rsi = rbp.wrapping_add(0x1); r13 = (0x100000001b3u64) as u64; rax = (0xcbf29ce484222325u64) as u64; rdx = (0xde5fb9d2630458e9u64) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rbx))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 21; continue; } bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_sub(0x1u64))) as u32) as u32 as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; r10 = (crate::simd_ops::ru64(rdi)) as u64; rcx = (((rcx) as i64).wrapping_mul((r13) as i64) as u64) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r13) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x2u64))) as u32) as u32 as u64; rcx = (((rcx) as i64).wrapping_mul((r13) as i64) as u64) as u64; rcx = (((rcx) ^ (rax))) as u64; rax = rdx; rcx = (((rcx) as i64).wrapping_mul((r13) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r11 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = r12; rcx = r11; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r10.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r9))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r10.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 20; continue; }
            20 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r8 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; r11 = (r11 & !0xffu64) | ((((((r11 as u8)) & (0x7fu8))) as u8 as u64)); r8 = ((r8).wrapping_add(rcx)) as u64; r8 = (((r8) & (r9))) as u64; rcx = (((r8) as i64).wrapping_mul((0x110u64) as i64) as u64) as u64; crate::simd_ops::wu8(r10.wrapping_add(r8), ((r11 as u8)) as u8); rdx = r8.wrapping_sub(0x10); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rdx = (((rdx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu8(rdx.wrapping_add(rax).wrapping_add(0x1u64), ((r11 as u8)) as u8); rdx = rsi.wrapping_add(0xf); rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x8u64)))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x1u64))) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x10u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0xf); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (0xde5fb9d2630458e9u64) as u64; rax = (0xcbf29ce484222325u64) as u64; bb = 21; continue; }
            21 => { rbx = ((rbx).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(0x110u64)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 22; continue; }
            22 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 23; continue; }
            23 => { rcx = r14.wrapping_sub(0x1); rax = r14; rcx = (((rcx) & (0xfffffffffe000000u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rdx = rcx.wrapping_add(0x108); rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x100u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r14, rax); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r14); if (((fl_l as u32)) != 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            27 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            28 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { r9 = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a55c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 4>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rsi = rcx; rcx = (0x100000001b3u64) as u64; rax = (0xcbf29ce484222325u64) as u64; r9 = (((r9) ^ (rax))) as u64; r15 = r8; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r14 = rdx; rbp = (crate::simd_ops::ru64(rsi)) as u64; rdi = ((r12 as u32)) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x2u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x3u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = (0xde5fb9d2630458e9u64) as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((r9) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rcx; r8 = ((r8).wrapping_shr(7u32)) as u64; r8 = (((r8) & (rbx))) as u64; fl_l = ((((rcx as u8)) & (0x7fu8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; xmm2 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpbroadcastb((xmm2 as u8)); bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rbp))) as u128; xmm0 = crate::simd_ops::vpcmpeqb(xmm1, xmm2); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r11 = (crate::simd_ops::ru32(r15)) as u32 as u64; bb = 3; continue; }
            3 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rcx = (((rcx) & (rbx))) as u64; r9 = (((rcx) as i64).wrapping_mul((0x110u64) as i64) as u64) as u64; r9 = ((r9).wrapping_add(r10)) as u64; fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (((rax as u32)) & (((rax as u32)).wrapping_sub(1))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { xmm0 = crate::simd_ops::vpsignb(xmm1, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; r8 = ((r8).wrapping_add(rdi)) as u64; fl_l = (((r8) & (rbx))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 1; continue; }
            7 => { rdx = ((rcx.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(r14, rdx); bb = 9; continue; }
            8 => { rax = ((rbx.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(r14, rax); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a56f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1430d3588; const P4: u64 = 0x143427660; const P5: u64 = 0x1424758f0; const P6: u64 = 0x143bc5fe0; const P7: u64 = 0x14136e370; const P8: u64 = 0x140382a50; const P9: u64 = 0x140382ee0; const P10: u64 = 0x140383060; const P11: u64 = 0x140382d30; const P12: u64 = 0x1403831d0; const P13: u64 = 0x140383a80;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1404a56f0, funcinfo_rva: 0x35de6f0, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); r15 = (r9) as u64; rsi = (r8) as u64; r14 = (rcx) as u64; r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0x80u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a5757); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a5764); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a5772); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((r13 as u32)) as u32 as u64; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rax) as u64); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), (rbx) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r12 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; xmm7 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; xmm6 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((xmm0 as u32)) as u32); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); xmm2 = (xmm1) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (0x3f800000u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((r13 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x3cu64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), (((xmm1 >> 32) as u32) as u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((xmm2 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x54u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x44u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), (((xmm1 >> 96) as u32) as u32) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), (r13) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), (r13) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), (0xffffffffu32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x54u64), (0x0u8) as u8); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), (0x7f7fffffu64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((r13 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), (0x1u8) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a583c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a583c, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a56f0) }; rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x50u64))) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a584e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a584e, (P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a56f0) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(0x50u64), ((rax as u32)) as u32); rcx = ((rbx.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a585a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a585a, (P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a56f0) }; rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a585f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a585f, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a56f0) }; rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru64(r15)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rax as u32)) as u32); rdx = (r15) as u64; rcx = ((rbx.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a587c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a587c, (P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a56f0) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x64u64), ((xmm6 as u32)) as u32); rdx = ((rsp.wrapping_add(0xb8u64)) as u64) as u64; rcx = ((rbx.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a589b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a589b, (P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a56f0) }; rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru64(rsi)) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0xcu64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x40u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x24u64))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(0x44u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((rax as u32)) as u32); rdx = (rsi) as u64; rcx = ((rbx.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a58e6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a58e6, (P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a56f0) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rsp.wrapping_add(0xb8u64), ((r12 as u8)) as u8); rdx = ((rsp.wrapping_add(0xb8u64)) as u64) as u64; rcx = ((rbx.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a58ff); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a58ff, (P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a56f0) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((xmm7 as u32)) as u32); rdx = ((rsp.wrapping_add(0xb8u64)) as u64) as u64; rcx = ((rbx.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a5919); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a5919, (P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a56f0) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14, (rbx) as u64); rax = (r14) as u64; r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a5950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 4>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rsi = rcx; rcx = (0x100000001b3u64) as u64; rax = (0xcbf29ce484222325u64) as u64; r9 = (((r9) ^ (rax))) as u64; r15 = r8; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x1u64))) as u32) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r14 = rdx; rbp = (crate::simd_ops::ru64(rsi)) as u64; rdi = ((r12 as u32)) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x2u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(r8.wrapping_add(0x3u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = (0xde5fb9d2630458e9u64) as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((r9) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rcx = ((rdx.wrapping_add(rax)) as u64) as u64; r8 = rcx; r8 = ((r8).wrapping_shr(7u32)) as u64; r8 = (((r8) & (rbx))) as u64; fl_l = ((((rcx as u8)) & (0x7fu8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rax = ((((rcx as u8)) as i8 as i32) as u32) as u32 as u64; xmm2 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpbroadcastb((xmm2 as u8)); bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(rbp))) as u128; xmm0 = crate::simd_ops::vpcmpeqb(xmm1, xmm2); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r10 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r11 = (crate::simd_ops::ru32(r15)) as u32 as u64; bb = 3; continue; }
            3 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = ((rcx).wrapping_add(r8)) as u64; rcx = (((rcx) & (rbx))) as u64; r9 = (((rcx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r9 = ((r9).wrapping_add(r10)) as u64; fl_l = (crate::simd_ops::ru32(r9)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (((rax as u32)) & (((rax as u32)).wrapping_sub(1))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { xmm0 = crate::simd_ops::vpsignb(xmm1, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(0x10u64)) as u64; r8 = ((r8).wrapping_add(rdi)) as u64; fl_l = (((r8) & (rbx))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 1; continue; }
            7 => { rdx = ((rcx.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r9); crate::simd_ops::wu64(r14, rdx); bb = 9; continue; }
            8 => { rax = ((rbx.wrapping_add(rbp)) as u64) as u64; crate::simd_ops::wu64(r14, rax); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a5a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142537280; const P1: u64 = 0x1426456c8; const P2: u64 = 0x1426453a0; const P3: u64 = 0x1430ab3b8; const P4: u64 = 0x143427660; const P5: u64 = 0x1418cb430;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0; let mut xmm13: u128 = 0; let mut xmm14: u128 = 0; let mut xmm15: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 61>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1404a5a80, funcinfo_rva: 0x35de75c, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x1e8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x1b0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x88u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x98u64), (xmm11) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xa8u64), (xmm12) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xb8u64), (xmm13) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xc8u64), (xmm14) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0xd8u64), (xmm15) as u128); r13 = (r9) as u64; r12 = (rcx) as u64; rcx = (0x6348u32) as u32 as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x88u64))) as u64; r15 = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_l = (r15) as u64 as u64; fl_r = (r15) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0x80u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a5b17); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a5b24); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = (0xdeadd00du32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a5b32); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), (rax) as u64); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x208u64), (r15) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x298u64))) as u64; r14 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x290u64))) as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x288u64))) as u64; xmm8 = ((crate::simd_ops::ru64(rax)) as u64) as u128; r11 = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; xmm7 = ((crate::simd_ops::ru64(rax.wrapping_add(0xcu64))) as u64) as u128; r10 = (crate::simd_ops::ru32(rax.wrapping_add(0x14u64))) as u32 as u64; xmm6 = ((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64) as u128; r9 = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; xmm5 = ((crate::simd_ops::ru64(rax.wrapping_add(0x24u64))) as u64) as u128; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x2cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x278u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x1f8u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x268u64))) as u64; rdi = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x260u64))) as u64; rbx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x258u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x298u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x250u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x290u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x248u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x288u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x240u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x278u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x238u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x268u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x230u64))) as u64; xmm15 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x228u64))) as u64; xmm14 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x220u64))) as u64; xmm13 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x218u64))) as u64; xmm12 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x210u64))) as u64; xmm9 = ((crate::simd_ops::ru32(rax)) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(r13)) as u32) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x200u64))) as u64; xmm11 = ((crate::simd_ops::ru32(rax)) as u32) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x280u64))) as u64; xmm4 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x270u64))) as u64; xmm3 = ((crate::simd_ops::ru64(rax)) as u64) as u128; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(r15, (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0xecu64), ((xmm0 as u32)) as u32); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xf0u64), (xmm1) as u128); xmm2 = (xmm1) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x100u64), (xmm1) as u128); crate::simd_ops::wu64(r15.wrapping_add(0x20u64), (0x3f800000u64) as u64); r13 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r15.wrapping_add(0x28u64), ((r13 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xecu64))) as u64) as u128; crate::simd_ops::wu64(r15.wrapping_add(0x2cu64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r15.wrapping_add(0x34u64), (((xmm1 >> 32) as u32) as u32) as u32); crate::simd_ops::wu64(r15.wrapping_add(0x38u64), (((xmm1 >> 64) as u64)) as u64); crate::simd_ops::wu32(r15.wrapping_add(0x40u64), ((xmm2 as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x104u64))) as u64) as u128; crate::simd_ops::wu64(r15.wrapping_add(0x44u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(r15.wrapping_add(0x4cu64), (((xmm1 >> 96) as u32) as u32) as u32); crate::simd_ops::wu8(r15.wrapping_add(0x10u64), ((r13 as u8)) as u8); crate::simd_ops::wu32(r15.wrapping_add(0x50u64), (0xffffffffu32) as u32); crate::simd_ops::wu8(r15.wrapping_add(0x54u64), ((r13 as u8)) as u8); crate::simd_ops::wu32(r15.wrapping_add(0x58u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(r15.wrapping_add(0x5cu64), (0x3f800000u32) as u32); crate::simd_ops::wu64(r15.wrapping_add(0x60u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(r15.wrapping_add(0x68u64), (0x3fc90fdbu64) as u64); crate::simd_ops::wu32(r15.wrapping_add(0x70u64), (0x3dcccccdu32) as u32); crate::simd_ops::wu64(r15.wrapping_add(0x74u64), (0x7f7fffffu64) as u64); crate::simd_ops::wu32(r15.wrapping_add(0x7cu64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), ((xmm8 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0xe8u64), ((r11 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0xecu64), ((xmm7 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0xf4u64), ((r10 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), ((xmm6 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x100u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x104u64), ((xmm5 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x10cu64), ((r8 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), ((xmm4 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), ((xmm3 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), ((rsi as u32)) as u32); rax = ((rsp.wrapping_add(0xe0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (rax) as u64); rax = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rax) as u64); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x1f8u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm0 as u32)) as u32); rax = ((rsp.wrapping_add(0xd0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), ((rdi as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rbx as u8)) as u8); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x298u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x290u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x288u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x278u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x268u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm15 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm13 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm12 as u32)) as u32); xmm3 = (xmm9) as u128; xmm2 = (xmm10) as u128; xmm1 = (xmm11) as u128; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1404a5e7c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = match eh.call_xmm(0x1404a5e7c, (P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]) { Ok(v) => v, Err(l) => l.unknown(0x1404a5a80) }; rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r12, (r15) as u64); rax = (r12) as u64; r11 = ((rsp.wrapping_add(0x1b0u64)) as u64) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x70u64))) as u128; xmm13 = (crate::simd_ops::ru128(r11.wrapping_sub(0x80u64))) as u128; xmm14 = (crate::simd_ops::ru128(r11.wrapping_sub(0x90u64))) as u128; xmm15 = (crate::simd_ops::ru128(r11.wrapping_sub(0xa0u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a6100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x1404a70d0; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x100; const P5: u64 = 0x108; const P6: u64 = 0x4; const P7: u64 = 0xe; const P8: u64 = 0x18; const P9: u64 = 0x1425371a0; const P10: u64 = 0x142533200;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu64(rdx.wrapping_add(P0), rax); rcx = ((rdx.wrapping_add(P1)) as u64) as u64; rdx = ((r8.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = ((rbx.wrapping_add(P3)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(P4))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = rbx; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(P5)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P6))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rcx.wrapping_add(P1), rbx); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P8), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rdx = rcx; r9 = rbx; rcx = r10; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a61c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411e9f50; const P1: u64 = 0x1404a6360; const P2: u64 = 0x143c8d010; const P3: u64 = 0x140215610; const P4: u64 = 0x143c8d08c; const P5: u64 = 0x143c8d910; const P6: u64 = 0x1420a28e0; const P7: u64 = 0x1404a6540; const P8: u64 = 0x1401de410;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); r15 = rdx; rdi = rcx; xmm6 = (crate::simd_ops::ru128(r9)) as u128; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu16(rax, ((rbx as u16)) as u16); rax = ((xmm6 as u64)) as u64; r8 = (0xffffffffffffffffu64) as u64; bb = 1; continue; }
            1 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r8); rbp = (crate::simd_ops::ru64(rdi)) as u64; r12 = rdi.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x50u64), (1) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x113u64), (0x1u8) as u8); rbx = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x44u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x40u64), (1) as u32); rbx = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rax = ((rdx.wrapping_add(rdx)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(rcx.wrapping_mul(2)), ((rbx as u16)) as u16); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x50u64), (1) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0x113u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P7; crate::simd_ops::wu64(rsi.wrapping_add(0x120u64), rax); rax = P8; crate::simd_ops::wu64(rsi.wrapping_add(0x128u64), rax); crate::simd_ops::wu8(rsi.wrapping_add(0x115u64), (0x0u8) as u8); rax = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; crate::simd_ops::wu16(r15, ((rax as u16)) as u16); rax = r15; r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a6360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c87ec4; const P1: u64 = 0x143c87e70; const P2: u64 = 0x143c87eac; const P3: u64 = 0x1420a25b0; const P4: u64 = 0x141834a78; const P5: u64 = 0x143c8d120; const P6: u64 = 0x143c8f6d8; const P7: u64 = 0x143c914d0; const P8: u64 = 0x143c8ec20; const P9: u64 = 0x143c8d694; const P10: u64 = 0x143c8d08c; const P11: u64 = 0x143c8d910; const P12: u64 = 0x143c8f63c; const P13: u64 = 0x143c87e90; const P14: u64 = 0x143c87eb0; const P15: u64 = 0x143c87ec0; const P16: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = rcx; rcx = 0x74; rax = ((crate::exe::tls_pointer() as u64)) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), (0x5u64) as u64); rax = P1; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), (0x3u64) as u64); rax = P2; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x49u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rbp.wrapping_add(0x37u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbx)) as u128; ymm1_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), ((xmm0 as u64)) as u64); rdx = rbp.wrapping_add(0x37); rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x49u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), (0x2u32) as u32); rcx = 0; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x49u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x1du64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x15u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x49u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru16(P8)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x49u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x5u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x3u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru16(P9)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x49u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), (0x1u32) as u32); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), (0x1u32) as u32); rax = ((crate::simd_ops::ru16(P10)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x67u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x13u64), rax); rax = ((crate::simd_ops::ru16(P11)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x67u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1bu64), rax); rax = ((crate::simd_ops::ru16(P12)) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_add(0x67u64), ((rax as u16)) as u16); crate::simd_ops::wu32(rbp.wrapping_add(0x6bu64), (0x2u32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x23u64), rax); let y_addr: u64 = rbp.wrapping_sub(0x29u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P1; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rbp.wrapping_sub(0x9u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = P13; crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x17u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(P14, (xmm0) as u128); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x27u64))) as u32 as u64; crate::simd_ops::wu32(P15, ((rax as u32)) as u32); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a6540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d910; const P1: u64 = 0x580e0; const P2: u64 = 0x143c8d08c; const P3: u64 = 0x143c8d010; const P4: u64 = 0x141a12590;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    r9 = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64;
    return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a6570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a6900;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rcx)) as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r10 = r9; r15 = r9; r10 = ((r10).wrapping_shr(7u32)) as u64; r14 = r8; r10 = (((r10) & (r11))) as u64; rdi = rdx; fl_l = ((((rax as u8)) & (0x7fu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rsi = rcx; rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; rbx = ((r12 as u32)) as u32 as u64; xmm2 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpbroadcastb((xmm2 as u8)); bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(r10.wrapping_add(rbp))) as u128; xmm0 = crate::simd_ops::vpcmpeqb(xmm1, xmm2); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru32(r14)) as u32 as u64; bb = 3; continue; }
            3 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) & (r11))) as u64; rdx = (((rcx) as i64).wrapping_mul((0x110u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(r8))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (((rax as u32)) & (((rax as u32)).wrapping_sub(1))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { xmm0 = crate::simd_ops::vpsignb(xmm1, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; fl_l = (((r10) & (r11))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 1; continue; }
            7 => { crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), ((r12 as u8)) as u8); bb = 9; continue; }
            8 => { rdx = r15; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x1u8) as u8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rdi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a6650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a67a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r11 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rcx)) as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r10 = r9; r15 = r9; r10 = ((r10).wrapping_shr(7u32)) as u64; r14 = r8; r10 = (((r10) & (r11))) as u64; rdi = rdx; fl_l = ((((rax as u8)) & (0x7fu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); rsi = rcx; rax = ((((rax as u8)) as i8 as i32) as u32) as u32 as u64; rbx = ((r12 as u32)) as u32 as u64; xmm2 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vpbroadcastb((xmm2 as u8)); bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(r10.wrapping_add(rbp))) as u128; xmm0 = crate::simd_ops::vpcmpeqb(xmm1, xmm2); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r9 = (crate::simd_ops::ru32(r14)) as u32 as u64; bb = 3; continue; }
            3 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = ((rcx).wrapping_add(r10)) as u64; rcx = (((rcx) & (r11))) as u64; rdx = (((rcx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(r8))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (((rax as u32)) & (((rax as u32)).wrapping_sub(1))) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { xmm0 = crate::simd_ops::vpsignb(xmm1, xmm1); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x10u64)) as u64; r10 = ((r10).wrapping_add(rbx)) as u64; fl_l = (((r10) & (r11))) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; bb = 1; continue; }
            7 => { crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), ((r12 as u8)) as u8); bb = 9; continue; }
            8 => { rdx = r15; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x1u8) as u8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rdi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a6a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434286b0; const P1: u64 = 0x143428770; const P2: u64 = 0x1404a6100;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rcx = rdx; rax = rdx.wrapping_add(0x1); rax = ((rax).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm2 = (crate::simd_ops::ru128(P0)) as u128; xmm3 = (crate::simd_ops::ru128(P1)) as u128; bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpshufb(xmm2, crate::simd_ops::ru128(rcx)); xmm1 = (xmm0) | (xmm3); crate::simd_ops::wu128(rcx, (xmm1) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r8.wrapping_add(rdx), (0xffu8) as u8); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = ((r15 as u32)) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rbp); r9 = (0xcbf29ce484222325u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x110u64), rsi); rbp = r15.wrapping_sub(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x118u64), r12); rsi = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), r13); r12 = (0x100000001b3u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), r14); r13 = (0xde5fb9d2630458e9u64) as u64; bb = 5; continue; }
            5 => { r11 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(r11.wrapping_add(rdi))) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; r14 = ((r11.wrapping_add(rdi)) as u64) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rdx = (((rdx) ^ (r9))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = r13; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r10 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = r15; rcx = r10; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (r8))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r11.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r8))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r11.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r9 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rdx = r10; rax = rdi; r9 = ((r9).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_shr(7u32)) as u64; rdx = (((rdx) & (r8))) as u64; r9 = (((r9) & (r8))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rcx = r9; rcx = ((rcx).wrapping_sub(rdx)) as u64; r10 = (r10 & !0xffu64) | ((((((r10 as u8)) & (0x7fu8))) as u8 as u64)); rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) & (r8))) as u64; fl_l = rcx; fl_r = (0xfffffffffffffff0u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(r14, ((r10 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rbp))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r10 as u8)) as u8); bb = 13; continue; }
            10 => { rax = ((r11.wrapping_add(r9)) as u64) as u64; r14 = (((r9) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; rcx = r9.wrapping_sub(0x10); crate::simd_ops::wu8(rax, ((r10 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (((rcx) & (rax))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r10 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((rax.wrapping_add(rsi)) as u64) as u64; rdx = ((r14.wrapping_add(rax)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(rax), (0x80u8) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rbp))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), (0x80u8) as u8); bb = 13; continue; }
            12 => { rcx = (((rcx) & (rax))) as u64; rdx = rsp.wrapping_add(0x20); rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r10 as u8)) as u8); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((r8).wrapping_add(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = ((r14.wrapping_add(rax)) as u64) as u64; rdx = ((rax.wrapping_add(rsi)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = rsp.wrapping_add(0x20); rdx = ((rdx).wrapping_add(r14)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_sub(1)) as u64; rsi = ((rsi).wrapping_sub(0xb0u64)) as u64; fl_l = ((rbp).wrapping_sub(1)) as u64; rbp = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { r9 = (0xcbf29ce484222325u64) as u64; bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(0xb0u64)) as u64; rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; bb = 16; continue; }
            16 => { rax = r8; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r8); rsp = ((rsp).wrapping_add(0xe0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a6d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434286b0; const P1: u64 = 0x143428770; const P2: u64 = 0x1404a7180; const P3: u64 = 0x1404a7040;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rcx = rdx; rax = rdx.wrapping_add(0x1); rax = ((rax).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm2 = (crate::simd_ops::ru128(P0)) as u128; xmm3 = (crate::simd_ops::ru128(P1)) as u128; bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpshufb(xmm2, crate::simd_ops::ru128(rcx)); xmm1 = (xmm0) | (xmm3); crate::simd_ops::wu128(rcx, (xmm1) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(r8.wrapping_add(rdx), (0xffu8) as u8); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = ((rbp as u32)) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x168u64), rsi); r10 = (0xcbf29ce484222325u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x170u64), r12); rsi = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x178u64), r13); r12 = (0x100000001b3u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x138u64), r14); r13 = (0xde5fb9d2630458e9u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x130u64), r15); r14 = ((rbp as u32)) as u32 as u64; bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(rdi))) as u8 as u64; fl_r = (0xfeu8) as u8 as u64; r15 = ((r9.wrapping_add(rdi)) as u64) as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x1u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rdx = (((rdx) ^ (r10))) as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = r13; rdx = (((rdx) as i64).wrapping_mul((r12) as i64) as u64) as u64; let prod = (rax as u128).wrapping_mul((rdx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r11 = ((rdx.wrapping_add(rax)) as u64) as u64; rdx = rbp; rcx = r11; rcx = ((rcx).wrapping_shr(7u32)) as u64; rcx = (((rcx) & (r8))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r9.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; rcx = (((rcx) & (r8))) as u64; xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(r9.wrapping_add(rcx))); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { let bit_in: u32 = (rax as u32); if bit_in != 0 { r10 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rdx = r11; rax = rdi; r10 = ((r10).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_shr(7u32)) as u64; rdx = (((rdx) & (r8))) as u64; r10 = (((r10) & (r8))) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rcx = r10; rcx = ((rcx).wrapping_sub(rdx)) as u64; r11 = (r11 & !0xffu64) | ((((((r11 as u8)) & (0x7fu8))) as u8 as u64)); rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) & (r8))) as u64; fl_l = rcx; fl_r = (0xfffffffffffffff0u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu8(r15, ((r11 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rsi))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r11 as u8)) as u8); bb = 14; continue; }
            11 => { rax = ((r9.wrapping_add(r10)) as u64) as u64; r15 = (((r10) as i64).wrapping_mul((0x110u64) as i64) as u64) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x80u8) as u8 as u64; rdx = r10.wrapping_sub(0x10); crate::simd_ops::wu8(rax, ((r11 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rax; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (((rcx) & (rdx))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r11 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(r14)) as u64) as u64; rdx = ((rax.wrapping_add(r15)) as u64) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; r10 = rcx.wrapping_add(0x10); crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rcx = rdx.wrapping_add(0x10); rdx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rdi.wrapping_add(rax), (0x80u8) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = rax; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = (((rcx) & (rsi))) as u64; fl_l = ((rcx).wrapping_add(rax)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), (0x80u8) as u8); bb = 14; continue; }
            13 => { rcx = (((rcx) & (rdx))) as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), ((r11 as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(r14))) as u32 as u64; r10 = ((rcx.wrapping_add(r14)) as u64) as u64; rdx = r10.wrapping_add(0x10); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r10.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(r15)) as u64) as u64; rdx = ((rax.wrapping_add(r14)) as u64) as u64; rax = (crate::simd_ops::ru32(rcx)) as u32 as u64; r10 = rcx.wrapping_add(0x10); crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rcx = rdx.wrapping_add(0x10); rdx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = ((rcx).wrapping_add(r15)) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x10u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_sub(1)) as u64; rsi = ((rsi).wrapping_sub(1)) as u64; fl_l = r14; fl_r = (0x110u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 14; continue; }
            14 => { r10 = (0xcbf29ce484222325u64) as u64; bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x110u64)) as u64; fl_l = rdi; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 16; continue; }
            16 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x178u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; bb = 17; continue; }
            17 => { rax = r8; rax = ((rax).wrapping_shr(3u32)) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r8); rsp = ((rsp).wrapping_add(0x140u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a7040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf8; const P1: u64 = 0x143a08450; const P2: u64 = 0x1404c98f0; const P3: u64 = 0x1424dee90; const P4: u64 = 0x142442de0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rax = 0x3fff; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (0x3fffffffffffc000u64) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(P1)) as u64; rcx = ((rcx).wrapping_add(0xf8u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P0), (0x0u64) as u64); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a70d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x2c; const P6: u64 = 0x34; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x44; const P10: u64 = 0x4c; const P11: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P3), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P5), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P7), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P8), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P9), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P10), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P11))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x5cu64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x5cu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x64u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x70u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x74u64))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(0x74u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x7cu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x80u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x80u64), ((rax as u8)) as u8); rax = rcx; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a7180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x38; const P3: u64 = 0x3c; const P4: u64 = 0x1404a70d0; const P5: u64 = 0xd0; const P6: u64 = 0xf0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = rdx;
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    r9 = rcx;
    crate::simd_ops::wu64(rcx, rax);
    let y_addr: u64 = rdx.wrapping_add(P0); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rcx.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm1) as u128);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x40u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rcx = ((rcx).wrapping_add(0x40u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    let y_addr: u64 = r8.wrapping_add(P5); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r9.wrapping_add(P5); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(P6))) as u128;
    crate::simd_ops::wu128(r9.wrapping_add(P6), (xmm1) as u128);
    rax = r9;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a7200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x2c; const P5: u64 = 0x24; const P6: u64 = 0x30; const P7: u64 = 0x34; const P8: u64 = 0x141fdbab0; const P9: u64 = 0xe0; const P10: u64 = 0xd0; const P11: u64 = 0xd8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x3f800000u32) as u32);
    crate::simd_ops::wu64(rcx, r9);
    r8 = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r9);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r9);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((r9 as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), (0x3f800000u64) as u64);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x3f800000u32) as u32);
    rcx = ((rcx).wrapping_add(0x40u64)) as u64;
    rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp);
    rax = r8;
    crate::simd_ops::wu16(r8.wrapping_add(P9), ((r9 as u16)) as u16);
    crate::simd_ops::wu64(r8.wrapping_add(P10), r9);
    crate::simd_ops::wu32(r8.wrapping_add(P11), ((r9 as u32)) as u32);
    crate::simd_ops::wu32(r8.wrapping_add(0xdcu64), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(r8.wrapping_add(0xe4u64), ((r9 as u32)) as u32);
    crate::simd_ops::wu32(r8.wrapping_add(0xe8u64), (0xbf800000u32) as u32);
    crate::simd_ops::wu32(r8.wrapping_add(0xecu64), (0xbf800000u32) as u32);
    crate::simd_ops::wu8(r8.wrapping_add(0xf0u64), ((r9 as u8)) as u8);
    crate::simd_ops::wu64(r8.wrapping_add(0xf8u64), r9);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a72a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404a7880; const P1: u64 = 0x1404a78e0; const P2: u64 = 0x14240aa00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rcx; rbx = (crate::simd_ops::ru64(rdx)) as u64; r14 = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r14 = ((r14).wrapping_add(rbx)) as u64; rsi = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 3; continue; } bb = 1; continue; }
            1 => { xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); bb = 2; continue; }
            2 => { xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rbx)); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r15 as u32)) as u32); let bit_in: u32 = (rax as u32); if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(rcx)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 2; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; r8 = rsi; rdx = rsp.wrapping_add(0x20); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x8u64)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::lk_and_u32(rdi, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rdi, (0x4u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(1)) as u64; rsi = ((rsi).wrapping_add(0x30u64)) as u64; rdx = rsi; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm1 = crate::simd_ops::vpcmpeqd(xmm1, xmm1); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r15 as u32)) as u32); xmm0 = crate::simd_ops::vpcmpgtb(xmm1, crate::simd_ops::ru128(rbx)); rax = (crate::simd_ops::vpmovmskb(xmm0) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rax = ((rcx as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rsi = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(rdx)) as u64; rdx = rsi; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if ((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8)) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a7410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a28bb8; const P1: u64 = 0x19; const P2: u64 = 0x20; const P3: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r11 = (crate::simd_ops::ru64(P0)) as u64; rbx = 0x3fff; r9 = r11; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r10 = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; fl_l = ((((r10 as u16)) & ((rbx as u16)))) as u16 as u64; fl_r = fl_l; r10 = (r10 & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); fl_k = 3; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru16(rax.wrapping_add(P2))) as u32) as u32 as u64; rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & ((rbx as u16)))) as u16 as u64)); fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = rcx; fl_k = 1; bb = 4; continue; }
            4 => { rcx = (rcx & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u16)) < ((fl_r as u16))), 1 => (((fl_l as u64)) < ((fl_r as u64))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; bb = 7; continue; }
            6 => { r9 = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 2; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 13; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru16(r9.wrapping_add(P2))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((rbx as u16)))) as u16 as u64)); rcx = (rcx & !0xffffu64) | ((((((rcx as u16)) & ((rbx as u16)))) as u16 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { fl_k = 0; bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = rax; fl_k = 1; bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u16)) < ((fl_r as u16))), 1 => (((fl_l as u64)) < ((fl_r as u64))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { fl_k = 4; bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx, r9); rax = rdx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = rdx; crate::simd_ops::wu64(rdx, r11); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1404a74b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143427910; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = (crate::simd_ops::ru128(P0)) as u128;
    xmm1 = crate::simd_ops::vpaddq(xmm1, crate::simd_ops::ru128(rcx));
    crate::simd_ops::wu128(rcx, (xmm1) as u128);
    r8 = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64;
    if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { rax = rcx; return rax; }
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    xmm2 = crate::simd_ops::vpcmpeqd(xmm2, xmm2);
    loop { xmm1 = crate::simd_ops::vpcmpgtb(xmm2, crate::simd_ops::ru128(r8)); rax = (crate::simd_ops::vpmovmskb(xmm1) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; let bit_in: u32 = (rax as u32); if bit_in != 0 { rax = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; r8 = ((r8).wrapping_add(rax)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rcx, r8); crate::simd_ops::wu64(rcx.wrapping_add(P1), r9); fl_l = (crate::simd_ops::ru8(r8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if !(((((fl_l as u8)) as i8) < (((fl_r as u8)) as i8))) { break; } }
    rax = rcx;
    return rax;
}
