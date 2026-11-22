#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b4650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14183af0c; const P2: u64 = 0x1412dd770; const P3: u64 = 0x14266b220; const P4: u64 = 0x1411f4730; const P5: u64 = 0x142645c20; const P6: u64 = 0x1402034f0; const P7: u64 = 0x14148aef4; const P8: u64 = 0x14266b290; const P9: u64 = 0x1426467f0; const P10: u64 = 0x141834d30; const P11: u64 = 0x1434278a0; const P12: u64 = 0x140200450; const P13: u64 = 0x1412dd710; const P14: u64 = 0x142645ac0; const P15: u64 = 0x140200850; const P16: u64 = 0x1401e3410; const P17: u64 = 0x1401de2f0; const P18: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 59>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0xc0); rsp = ((rsp).wrapping_sub(0x1c0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rax); rsi = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) >= 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 10; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdx = 0x1; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P3; rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x90); rcx = rbp.wrapping_sub(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x90); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x90u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0x23; r8 = P8; rdx = ((r9.wrapping_sub(0x22u64)) as u32) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414b478d, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(P11)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x90u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1f0u64))) as u64; rsp = ((rsp).wrapping_add(0x1c0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { rbx = rcx.wrapping_add(0x8); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::lk_add_u64(rdi.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rax, ((rdi as u8)) as u8); bb = 10; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414b4825, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b4828(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414ab11c; const P1: u64 = 0x141834d64; const P2: u64 = 0x1414b43d0; const P3: u64 = 0x140200070;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rsi = r8; r15 = rdx; r14 = rcx; rax = (crate::simd_ops::ru64(r8)) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 1; continue; }
            1 => { r12 = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbp, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 4; continue; }
            4 => { xmm6 = ((crate::simd_ops::ru64(rax)) as u64) as u128; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) - f64::from_bits(crate::simd_ops::ru64(rdi))).to_bits() as u128); rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(r12)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = r12; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x108u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(r12)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = r12; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rsp.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rcx = 0x28; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (rdx & !0xffu64) | (((crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64)); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r15, rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm6 as u64)) as u64); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) != ((fl_r as u8))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), ((rcx as u8)) as u8); rax = r15; r11 = rsp.wrapping_add(0x120); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414b4956, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b4958(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414ab11c; const P1: u64 = 0x141834d64; const P2: u64 = 0x1414b4418; const P3: u64 = 0x140200070;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rdi = r8; rsi = rdx; r14 = rcx; rax = (crate::simd_ops::ru64(r8)) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r11.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x20); rcx = rdi; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rax)) as u64; rdi = ((rdi).wrapping_sub(crate::simd_ops::ru64(rbx))) as u64; rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x108u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rsp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = 0x28; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (rdx & !0xffu64) | (((crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64)); rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsi, rax); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x0u8) as u8); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); rax = rsi; r11 = rsp.wrapping_add(0x110); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414b4a68, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b4a6c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414ab11c; const P1: u64 = 0x141834d64; const P2: u64 = 0x1414b43d0; const P3: u64 = 0x140200070;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rsi = r8; r15 = rdx; r14 = rcx; rax = (crate::simd_ops::ru64(r8)) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 1; continue; }
            1 => { r12 = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbp, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 4; continue; }
            4 => { xmm6 = ((crate::simd_ops::ru64(rax)) as u64) as u128; xmm6 = (xmm6 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm6 as u64) + f64::from_bits(crate::simd_ops::ru64(rdi))).to_bits() as u128); rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(r12)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = r12; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x108u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(r12)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = r12; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rsp.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rcx = 0x28; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (rdx & !0xffu64) | (((crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64)); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r15, rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm6 as u64)) as u64); rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; rcx = (if (((fl_l as u8)) != ((fl_r as u8))) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), ((rcx as u8)) as u8); rax = r15; r11 = rsp.wrapping_add(0x120); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414b4b9a, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b4b9c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414ab11c; const P1: u64 = 0x141834d64; const P2: u64 = 0x1414b4418; const P3: u64 = 0x140200070;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rbx = r8; rsi = rdx; r14 = rcx; rax = (crate::simd_ops::ru64(r8)) as u64; rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r11.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(rbp, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rax)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (((crate::simd_ops::ru8(rsp.wrapping_add(0x108u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rsp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rcx = 0x28; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = (rdx & !0xffu64) | (((crate::simd_ops::ru8(r14.wrapping_add(0x20u64))) as u8 as u64)); rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsi, rax); fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu8(rax.wrapping_add(0x18u64), (0x0u8) as u8); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rax = rsi; r11 = rsp.wrapping_add(0x110); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            17 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414b4cac, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b4cb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x140200850; const P2: u64 = 0x10; const P3: u64 = 0x70; const P4: u64 = 0x20; const P5: u64 = 0x1401e3410; const P6: u64 = 0x1401de2f0;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdi = rcx;
    rcx = rbx;
    rsi = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P2))) as u128;
    rcx = rsp.wrapping_add(0x30);
    crate::simd_ops::wu8(rsi.wrapping_add(P3), (0x0u8) as u8);
    xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu128(rsi, (xmm0) as u128);
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8);
    crate::simd_ops::wu64(rsi.wrapping_add(P2), ((xmm1 as u64)) as u64);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = rbx;
    rdi = (rdi & !0xffu64) | ((((rax as u8)) as u8 as u64));
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu8(rax, ((rdi as u8)) as u8);
    rax = rsi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b4d18(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414af9bc; const P1: u64 = 0x14149c840; const P2: u64 = 0x1414b10fc; const P3: u64 = 0x1414aa66c; const P4: u64 = 0x1414aca18;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b4d18, funcinfo_rva: 0x36ba9a4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x37u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r12 = (r8) as u64; rsi = (rdx) as u64; r15 = (rcx) as u64; rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4d42); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (rax) as u64; r9 = (crate::simd_ops::ru64(r15)) as u64; rdi = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rax) as u64); r8 = ((rbp.wrapping_add(0x67u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4d5f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (r12) as u64; rdx = ((rbp.wrapping_add(0x67u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4d81); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (r14) as u64; r8 = (rax) as u64; rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4d90); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0xf8u64))) as u64; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4da6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r9 = (rbx) as u64; r8 = (rax) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4dbc); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (r12) as u64; rdx = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4dcf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (r14) as u64; r8 = (rax) as u64; rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4dde); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4df5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); r11 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b4e18(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414af9bc; const P1: u64 = 0x14149c840; const P2: u64 = 0x1414b10fc; const P3: u64 = 0x1414aa66c; const P4: u64 = 0x1414aca18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b4e18, funcinfo_rva: 0x36ba9dc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x37u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r15 = (r8) as u64; rsi = (rdx) as u64; rdi = (rcx) as u64; rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4e3c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (rax) as u64; r9 = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (rax) as u64); r8 = ((rbp.wrapping_add(0x67u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4e59); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (r15) as u64; rdx = ((rbp.wrapping_add(0x67u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4e7b); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (r14) as u64; r8 = (rax) as u64; rdx = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4e8a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0xf8u64))) as u64; rcx = ((rbp.wrapping_sub(0x31u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4ea0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r9 = (rbx) as u64; r8 = (rax) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rbp.wrapping_add(0x7fu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4eb6); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rdi)) as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = (r15) as u64; rdx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4ed3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r9 = (r14) as u64; r8 = (rax) as u64; rdx = (rsi) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4ee2); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = (0x1u32) as u32 as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4ef8); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b4f18(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x1414b59e8; const P2: u64 = 0x1414ab11c; const P3: u64 = 0x1414a8960; const P4: u64 = 0x1414a8554; const P5: u64 = 0x1414b5dc0; const P6: u64 = 0x1414b5354; const P7: u64 = 0x141834d30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b4f18, funcinfo_rva: 0x36baa18, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x138, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rax.wrapping_sub(0x38u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x130u64)) as u64; rbx = (r8) as u64; rdi = (rdx) as u64; rsi = (rcx) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rcx = (0x68u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4f4d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); xmm0 = 0u128; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x1u8) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x58u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4f92); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (rcx) as u64); rcx = (((crate::simd_ops::ru8(rax.wrapping_add(0x70u64))) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; r8 = (rax) as u64; rdx = ((rbp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4fad); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (((crate::simd_ops::ru8(rbp.wrapping_add(0x20u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rbp.wrapping_sub(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4fcd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { rax = ((rsp.wrapping_add(0x20u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b4fef); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rbx, (rbx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rdx = (rdi) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b500d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b502d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x88u64), (0x90u64) as u64)) as u64; fl_r = fl_l; bb = 9; continue; }
            8 => { r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = (rax) as u64; rcx = ((rbx.wrapping_add(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b504e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rax = (((crate::simd_ops::ru8(rbp.wrapping_sub(0x58u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (rax) as u64 as u64; fl_r = (0x1u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5070); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5088); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x68u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5097); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); r11 = ((rsp.wrapping_add(0x130u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b50b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d64; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b50b8, funcinfo_rva: 0x36ba970, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (r9) as u64;
    rsi = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rcx = (0x28u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414b50e4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rbx = (rax) as u64;
    crate::simd_ops::wu64(rsi.wrapping_add(P0), (rax) as u64);
    rcx = (crate::simd_ops::ru64(rdi)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P2), (rdx) as u64);
    crate::simd_ops::lk_and_u64(rax.wrapping_add(P3), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rax.wrapping_add(P4), (0x0u64) as u64);
    rcx = (0x20u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414b5109); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rax, (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P0), (rax) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64);
    rax = (rsi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b512c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140208240; const P1: u64 = 0x1414ab11c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx)) as u64; rsi = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdi; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x18u64))) as u128; rdx = rbx.wrapping_add(0x48); rcx = rdi.wrapping_add(0x48); crate::simd_ops::wu128(rdi.wrapping_add(0x18u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x28u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x38u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rax); rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rbx.wrapping_add(0x68u64))) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x68u64), ((rax as u8)) as u8); bb = 7; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rdi.wrapping_add(0x70u64))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rdi.wrapping_add(0x70u64), (0xffu8) as u8); r8 = (crate::simd_ops::ru64(rsi)) as u64; r9 = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; crate::simd_ops::lk_and_u64(r8.wrapping_add(0x10u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u64(r8.wrapping_add(0x8u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::lk_and_u64(r8, (0x0u64) as u64); crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(r9.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r9.wrapping_add(0x10u64), rdx); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(r9.wrapping_add(0x18u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(0x28u64))) as u128; crate::simd_ops::wu128(r9.wrapping_add(0x28u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r8.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(r9.wrapping_add(0x38u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x58u64))) as u64; crate::simd_ops::lk_and_u64(r8.wrapping_add(0x58u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x50u64))) as u64; crate::simd_ops::lk_and_u64(r8.wrapping_add(0x50u64), (0x0u64) as u64); fl_l = (crate::simd_ops::lk_and_u64(r8.wrapping_add(0x48u64), (0x0u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(r9.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(r9.wrapping_add(0x50u64), rcx); crate::simd_ops::wu64(r9.wrapping_add(0x58u64), rdx); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(r9.wrapping_add(0x60u64), rax); rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(r8.wrapping_add(0x68u64))) as u8 as u64)); crate::simd_ops::wu8(r9.wrapping_add(0x68u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x70u64), (0x1u8) as u8); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x10; const P2: u64 = 0x1414ab11c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rsi = rcx; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P1), (xmm1) as u128); bb = 7; continue; }
            2 => { rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P1))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm1) as u128); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x2u8) as u8); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b52d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x10; const P2: u64 = 0x1414ab11c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rsi = rcx; rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P1), ((xmm1 as u64)) as u64); bb = 7; continue; }
            2 => { fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P1), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x0u8) as u8); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5354(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401eae50; const P1: u64 = 0x141834d64; const P2: u64 = 0x1414b5dc0; const P3: u64 = 0x1414a8f90; const P4: u64 = 0x1414b596c; const P5: u64 = 0x1414b6070; const P6: u64 = 0x1414937f0; const P7: u64 = 0x1414b5500; const P8: u64 = 0x1401e6f50; const P9: u64 = 0x1414b5506;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b5354, funcinfo_rva: 0x36ba934, handler: 1, shape: 2, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = (rdx) as u64; rsi = (rcx) as u64; r9 = (rdx) as u64; r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rcx))) as u64; r10 = (0xe38e38e38e38e39u64) as u64; rax = (r10) as u64; let prod = (rax as i64 as i128).wrapping_mul((r9) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; r14 = (rdx) as u64; r14 = ((((r14) as i64).wrapping_shr(3u32) as u64)) as u64; rax = (r14) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; r14 = ((r14).wrapping_add(rax)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64; rax = (r10) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rbx = (0x1c71c71c71c71c7u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { r13 = ((rdx.wrapping_add(0x1u64)) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rsi))) as u64; rax = (r10) as u64; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (rbx) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); rcx = (0xfffffffffffffff0u64) as u64; bb = 5; continue; }
            3 => { rax = ((rcx.wrapping_add(rdx)) as u64) as u64; rcx = (r13) as u64; fl_l = (rax) as u64 as u64; fl_r = (r13) as u64 as u64; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 14; continue; } bb = 4; continue; }
            4 => { rbx = (rcx) as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); fl_l = (rcx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5436); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 9; continue; }
            6 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b544f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rbx) as u64); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); r14 = ((r14.wrapping_add(r14.wrapping_mul(8))) as u64) as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(rdi)) as u64; r15 = ((r14.wrapping_add(0x90u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r15) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (r15) as u64); rdx = (r8) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b548e); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414b548e, (P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414b5354) }; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (r14) as u64); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r9 = (rsi) as u64; r8 = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (r12) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b54ad); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414b54ad, (P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414b5354) }; bb = 12; continue; }
            11 => { rdx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b54b7); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414b54b7, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414b5354) }; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (rdi) as u64); r9 = (rsi) as u64; r8 = (r15) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b54d1); crate::simd_ops::replace_rax_in(rax); rax = match eh.call(0x1414b54d1, (P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8) { Ok(v) => v, Err(l) => l.unknown(0x1414b5354) }; bb = 12; continue; }
            12 => { r9 = (rbx) as u64; r8 = (r13) as u64; rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b54e3); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (r14) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            13 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b54fb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P7, rcx, rsp); return rax; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5501); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5718(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14148b98c; const P1: u64 = 0x1430782b0; const P2: u64 = 0x1414b50b8; const P3: u64 = 0x1414930e0; const P4: u64 = 0x1414931ac; const P5: u64 = 0x14149bce4; const P6: u64 = 0x141834d30; const P7: u64 = 0x14307e950; const P8: u64 = 0x141833c0f; const P9: u64 = 0x1414b5968;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b5718, funcinfo_rva: 0x36ba8c4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x78, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (r8) as u64; rdi = (rdx) as u64; rbp = (rcx) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b573a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (rax) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rcx = (((rcx) & (rax))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r15 = ((rbp.wrapping_add(0x8u64)) as u64) as u64; rsi = (crate::simd_ops::ru64(r15)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rbx = (rsi) as u64; bb = 2; continue; }
            2 => { rax = (0x666666666666666u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (r14) as u64); rax = ((rsp.wrapping_add(0x80u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rsp.wrapping_add(0x90u64)) as u64) as u64; r8 = ((P1) as u64) as u64; rdx = (r15) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b57a4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = ((rdx).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = 0u128; if ((((fl_l as u64)) as i64) < 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 14; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; bb = 8; continue; }
            6 => { fl_l = (rbx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 8; continue; }
            8 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rax = (rbx) as u64; rbx = (rsi) as u64; bb = 10; continue; }
            10 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x0u8) as u8); bb = 37; continue; }
            12 => { rsi = (rbx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 10; continue; }
            13 => { rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; xmm1 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 17; continue; }
            16 => { rax = (rcx) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 17; continue; }
            17 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 18; continue; }
            18 => { rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5849); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5854); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rdx = (((rdx) & (r12))) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r15)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 22; continue; }
            20 => { rbx = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; bb = 22; continue; }
            22 => { fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); bb = 25; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 28; continue; }
            26 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); bb = 25; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 28; continue; }
            28 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x10u64), (1) as u64); crate::simd_ops::wu64(rax, (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu64(r8, (rax) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rcx = (((rcx) & (r12))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = (r9) as u64 as u64; fl_r = (crate::simd_ops::ru64(r15)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), (rax) as u64); bb = 33; continue; }
            30 => { fl_l = (r9) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)), (rax) as u64); bb = 34; continue; }
            32 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64), (rax) as u64); bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rcx = ((rbx.wrapping_add(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5919); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x20u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5927); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rdx = (0x28u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b593b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 37; continue; }
            37 => { rax = (rdi) as u64; r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            38 => { rcx = ((P7) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5963); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b59e8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x1414ab11c; const P2: u64 = 0x1414b5250; const P3: u64 = 0x1414b512c; const P4: u64 = 0x1414b52d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P0))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rdi.wrapping_add(P0), (0xffu8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x3u8) as u8); bb = 18; continue; }
            10 => { rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            11 => { rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            12 => { rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rax = (((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as i8 as i64) as u64) as u64; fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = rax; fl_r = (0x1u64) as u64; rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = rax; fl_r = (0x1u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { crate::simd_ops::wu8(rbx.wrapping_add(P0), (0xffu8) as u8); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5ab8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x28; const P5: u64 = 0x38; const P6: u64 = 0x48; const P7: u64 = 0x58; const P8: u64 = 0x50; const P9: u64 = 0x60; const P10: u64 = 0x68;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = 0; fl_l = 0; fl_r = 0; r9 = r8; r10 = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rax.wrapping_add(P0), (0x3u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8.wrapping_add(P1))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm1) as u128); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x2u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { r8 = (crate::simd_ops::ru64(rdx)) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(r9.wrapping_add(P1), r11); crate::simd_ops::wu64(r9.wrapping_add(P2), r11); crate::simd_ops::wu64(r9, r11); crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(P2), rcx); crate::simd_ops::wu64(r8.wrapping_add(P1), rdx); xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(P3))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r9.wrapping_add(P4))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P4), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r9.wrapping_add(P5))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P5), (xmm0) as u128); rax = (crate::simd_ops::ru64(r9.wrapping_add(P6))) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(P7))) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P8))) as u64; crate::simd_ops::wu64(r9.wrapping_add(P7), r11); crate::simd_ops::wu64(r9.wrapping_add(P8), r11); crate::simd_ops::wu64(r9.wrapping_add(P6), r11); crate::simd_ops::wu64(r8.wrapping_add(P6), rax); crate::simd_ops::wu64(r8.wrapping_add(P8), rcx); crate::simd_ops::wu64(r8.wrapping_add(P7), rdx); rax = (crate::simd_ops::ru64(r9.wrapping_add(P9))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P9), rax); rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(r9.wrapping_add(P10))) as u8 as u64)); crate::simd_ops::wu8(r8.wrapping_add(P10), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), (0x1u8) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rax = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(P1), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu8(rax.wrapping_add(P0), ((r11 as u8)) as u8); bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14148b98c; const P1: u64 = 0x141834d64; const P2: u64 = 0x1414930e0; const P3: u64 = 0x1414931ac; const P4: u64 = 0x14307e950; const P5: u64 = 0x141833c0f; const P6: u64 = 0x1414b5dbd;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b5ba0, funcinfo_rva: 0x36ba904, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r15 = (r8) as u64; rsi = (rdx) as u64; r14 = (rcx) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5bc8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r13 = (rax) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rcx = (((rcx) & (rax))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; r12 = ((r14.wrapping_add(0x8u64)) as u64) as u64; rbp = (crate::simd_ops::ru64(r12)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rbx = (rbp) as u64; bb = 2; continue; }
            2 => { rax = (0x666666666666666u64) as u64; fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rcx = (0x28u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5c17); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rcx = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rcx) as u64); rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u64(r15.wrapping_add(0x8u64), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rcx) as u64); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), (rax) as u64); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_l = ((rdx).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = 0u128; if ((((fl_l as u64)) as i64) < 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 14; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; bb = 8; continue; }
            6 => { fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 8; continue; }
            8 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rbx)) as u64; rax = (rbx) as u64; rbx = (rbp) as u64; bb = 10; continue; }
            10 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsi, (rax) as u64); crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), (0x0u8) as u8); bb = 34; continue; }
            12 => { rbp = (rbx) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 10; continue; }
            13 => { rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; xmm1 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 17; continue; }
            16 => { rax = (rcx) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 17; continue; }
            17 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 18; continue; }
            18 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5ce3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5cee); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rcx = (((rcx) & (r13))) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r12)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; bb = 22; continue; }
            20 => { rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 22; continue; }
            22 => { fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rax)) as u64; bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); bb = 25; continue; }
            25 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(r14.wrapping_add(0x10u64), (1) as u64); crate::simd_ops::wu64(rdi, (rbp) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu64(rdx, (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rdi) as u64); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rax = (((rax) & (r13))) as u64; rax = ((rax).wrapping_add(rax)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (r8) as u64 as u64; fl_r = (crate::simd_ops::ru64(r12)) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (rdi) as u64); bb = 32; continue; }
            28 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); bb = 25; continue; }
            29 => { fl_l = (r8) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (rdi) as u64); bb = 33; continue; }
            31 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64), (rdi) as u64); bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rsi, (rdi) as u64); crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), (0x1u8) as u8); bb = 34; continue; }
            34 => { rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { rcx = ((P4) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5db8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P6, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d64; const P2: u64 = 0x18; const P3: u64 = 0x10; const P4: u64 = 0x70; const P5: u64 = 0x1414b5ab8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64); rdi = rcx; crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64); rbx = rdx; rcx = 0x68; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = ((rbx.wrapping_add(P2)) as u64) as u64; rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(P0), rax); crate::simd_ops::wu64(rax.wrapping_add(P3), rax); crate::simd_ops::wu16(rax.wrapping_add(P2), (0x101u16) as u16); crate::simd_ops::wu64(rdi, rax); rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rax = ((rdi.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx); crate::simd_ops::wu8(rax.wrapping_add(P4), (0xffu8) as u8); rcx = (((crate::simd_ops::ru8(r8.wrapping_add(P4))) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5e48(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414aa66c; const P2: u64 = 0x68; const P3: u64 = 0x70; const P4: u64 = 0x1414aa284; const P5: u64 = 0xb0; const P6: u64 = 0x1414aa214; const P7: u64 = 0xf0; const P8: u64 = 0xf8; const P9: u64 = 0x1414ab1c4;
    let (mut rax, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b5e48, funcinfo_rva: 0x36ba7dc, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rdi = (r9) as u64; rbx = ((r8 as u32)) as u32 as u64; rbp = (rdx) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5e79); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu32(rsi.wrapping_add(P2), ((rbx as u32)) as u32); rcx = ((rsi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5e9a); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = ((rsi.wrapping_add(P5)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5ebb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu8(rsi.wrapping_add(P7), (0x0u8) as u8); crate::simd_ops::wu64(rsi.wrapping_add(P8), (rdi) as u64); rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b5ed1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (rsi) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5ef8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x1414aae64; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x40; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0x28; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5f4c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x14149bce4; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = 0x20; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x28; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5f9c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x1414ab11c; const P2: u64 = 0x8; const P3: u64 = 0x1414a8960; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = (((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as i8 as i64) as u64) as u64;
    fl_l = ((rax).wrapping_add(0x1u64)) as u64; fl_r = fl_l;
    rax = ((fl_l as u64)) as u64;
    if (((fl_l as u64)) == 0) { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbx; rcx = rbx; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0x68; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = rax; fl_r = (0x1u64) as u64;
    rax = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbx; rcx = rbx; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0x68; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    fl_l = rax; fl_r = (0x1u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    r8 = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = rbx;
    rcx = rbx;
    r8 = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = 0x68;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b5fec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x8; const P2: u64 = 0x1414aa858; const P3: u64 = 0x80; const P4: u64 = 0x1414b19c8; const P5: u64 = 0x1414ab1c4;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = rsp;
    crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx);
    crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0xd0u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (((rax) ^ (rsp))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0xc8u64), rax);
    rsi = rdx;
    rdi = rcx;
    crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdx);
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rcx = rsp.wrapping_add(0x28);
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = (crate::simd_ops::ru64(rdi)) as u64;
    rax = crate::simd_ops::call_stk(rbx, rax, rcx, rdx, r8, r9, rsp);
    rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64));
    rcx = ((rsi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsi;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rax = (rax & !0xffu64) | ((((rbx as u8)) as u8 as u64));
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64;
    fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    r11 = rsp.wrapping_add(0xd0);
    rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64;
    rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64;
    rsp = r11;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b6070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414a8694; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x141834d30; const P5: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = r9; rcx = (crate::simd_ops::ru64(rcx)) as u64; r14 = r8; rdi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (0xe38e38e38e38e39u64) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; r8 = ((r8).wrapping_sub(rcx)) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rdx = ((rdx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx, rdi); rax = ((r14.wrapping_add(r14.wrapping_mul(8))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdi)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rax = ((rsi.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdi)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414b6135, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b6138(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x140200850; const P2: u64 = 0x1414b5718; const P3: u64 = 0x14149b924; const P4: u64 = 0x14148d294; const P5: u64 = 0x1401e3410; const P6: u64 = 0x1401de2f0; const P7: u64 = 0x141834d64; const P8: u64 = 0x1414aa364; const P9: u64 = 0x1414b4d18; const P10: u64 = 0x1414b5508; const P11: u64 = 0x1414b4e18; const P12: u64 = 0x1414aae64; const P13: u64 = 0x141834d30; const P14: u64 = 0x1434278a0; const P15: u64 = 0x140203100; const P16: u64 = 0x1414b5ba0; const P17: u64 = 0x1414b4f18; const P18: u64 = 0x1414aa8d4; const P19: u64 = 0x1414b5fec; const P20: u64 = 0x1414b19c8; const P21: u64 = 0x1414ab1c4; const P22: u64 = 0x14149bce4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 81>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x150); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x140u64), rax); r12 = r9; rbx = r8; r13 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x1b8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x1c0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); r15 = rcx.wrapping_add(0xf0); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x60u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(rdi)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); r8 = rsp.wrapping_add(0x58); rdx = rbp.wrapping_sub(0x28); rcx = r13.wrapping_add(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; rcx = ((rcx).wrapping_add(0x18u64)) as u64; r8 = r14; rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rdi).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = r13.wrapping_add(0x70); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x78u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rbx = ((((rbx as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 49; continue; }
            10 => { rcx = 0x20; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), rsi); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x20u64), (0x0u64) as u64); rcx = 0x40; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x38); rdx = rsp.wrapping_add(0x4c); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            12 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rdi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 14; continue; }
            14 => { rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), r13); r14 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r14 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(r14)) as u64; bb = 17; continue; }
            15 => { rdx = rbx.wrapping_add(0x18); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = rbp.wrapping_sub(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 17; continue; }
            17 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 20; continue; }
            20 => { r14 = r13.wrapping_add(0xb0); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x68); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r13.wrapping_add(0xb8u64))) as u64; rcx = r14; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 21; continue; }
            21 => { r8 = rsp.wrapping_add(0x30); rdx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::lk_and_u64(rcx.wrapping_add(0x18u64), (0x0u64) as u64); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 22; continue; }
            22 => { rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r13); rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 25; continue; }
            23 => { rdx = rbx.wrapping_add(0x18); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = rbp.wrapping_sub(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 25; continue; }
            25 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 26; continue; }
            26 => { bb = 28; continue; }
            27 => { r8 = rsp.wrapping_add(0x30); rdx = rbp.wrapping_sub(0x80); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rax)) as u64; r12 = (crate::simd_ops::ru64(r12.wrapping_add(0x20u64))) as u64; bb = 28; continue; }
            28 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); r8 = rsp.wrapping_add(0x30); rdx = rbp.wrapping_sub(0x38); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rax)) as u64; r14 = ((r14).wrapping_add(0x18u64)) as u64; rax = rbp.wrapping_sub(0x18); fl_l = r14; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(r14, rdi); rdi = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rdi.wrapping_add(0x40u64)) as u32) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1b0u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x8u64), rbx); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x40; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x40; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { r14 = r13.wrapping_add(0xb0); bb = 36; continue; }
            36 => { r8 = rsp.wrapping_add(0x30); rdx = rbp.wrapping_sub(0x8); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0xa0u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P14)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xb0u64), (xmm1) as u128); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_add(0xa0u64), ((r14 as u8)) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0xc0u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0xd0u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0xc0u64), ((r14 as u8)) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0xe0u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0xf0u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0xe0u64), ((r14 as u8)) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x110u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x120u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x130u64), r14); rdx = r13; rcx = rbp.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r13.wrapping_add(0x20); rcx = rbp.wrapping_add(0xc0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r13.wrapping_add(0x40); rcx = rbp.wrapping_add(0xe0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x100u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x64u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x108u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x110u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x118u64), rbx); rax = rbp.wrapping_add(0xa0); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 41; continue; }
            37 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rbx); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x1b0u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rbx); r8 = rbp.wrapping_sub(0x70); rdx = rbp.wrapping_sub(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 38; continue; }
            38 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x40; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            39 => { rdx = rbx.wrapping_add(0x18); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 41; continue; }
            41 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 42; continue; }
            42 => { rdx = rbp.wrapping_add(0xa0); rcx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (r14 & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = rbp.wrapping_add(0x120); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x40; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 44; continue; }
            44 => { rdx = rsi; rcx = rbp.wrapping_sub(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x20; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rax, ((rbx as u8)) as u8); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 45; continue; }
            45 => { rbx = ((((rbx as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 48; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 48; continue; }
            48 => { rax = (rax & !0xffu64) | ((((r14 as u8)) as u8 as u64)); bb = 49; continue; }
            49 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x2a8u64))) as u64; rsp = ((rsp).wrapping_add(0x250u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b66ac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfffffffffffffff8; const P1: u64 = 0x141834d30; const P2: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = rdx; rdx = ((r8.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = rcx; bb = 3; continue; }
            3 => { rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414b66ee, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b66f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0xfffffffffffffff0; const P4: u64 = 0x141834d64; const P5: u64 = 0x18; const P6: u64 = 0x1414a8554;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b66f0, funcinfo_rva: 0x36b9be8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    rax = (rsp) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P0), (rbx) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P1), (rsi) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = (r8) as u64;
    rbx = (rdx) as u64;
    rsi = (rcx) as u64;
    crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P2), (rcx) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P3), (rcx) as u64);
    rcx = (0x68u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414b671f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rax, (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P0), (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P1), (rax) as u64);
    crate::simd_ops::wu16(rax.wrapping_add(P5), (0x101u16) as u16);
    crate::simd_ops::wu64(rsi, (rax) as u64);
    rdx = (rbx) as u64;
    rcx = (rsi) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414b673e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    crate::simd_ops::wu64(rsi.wrapping_add(P5), (rax) as u64);
    rax = (rsi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b6760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d64; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1414a8554;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b6760, funcinfo_rva: 0x36bac48, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (r8) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rcx, (rdx) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64); rcx = (0x30u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b678c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rsi.wrapping_add(P0), (rax) as u64); rbx = ((rax.wrapping_add(P2)) as u64) as u64; crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); rcx = (0x68u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b67b1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(P0), (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rax) as u64); crate::simd_ops::wu16(rax.wrapping_add(P3), (0x101u16) as u16); crate::simd_ops::wu64(rbx, (rax) as u64); rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b67d0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P3), (rax) as u64); rax = (rsi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b67f4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d64; const P2: u64 = 0x10; const P3: u64 = 0x18;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::lk_and_u64(rcx, (0x0u64) as u64);
    rsi = rcx;
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rdi = r8;
    rcx = 0x68;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax, rax);
    crate::simd_ops::wu64(rax.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rax.wrapping_add(P2), rax);
    crate::simd_ops::wu16(rax.wrapping_add(P3), (0x101u16) as u16);
    crate::simd_ops::wu64(rsi, rax);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    crate::simd_ops::wu64(rsi, rcx);
    crate::simd_ops::wu64(rbx, rax);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rsi.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx);
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rsi.wrapping_add(P3), rax);
    rax = rsi;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b6868(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x14183acda; const P3: u64 = 0x40; const P4: u64 = 0x20; const P5: u64 = 0x1414b6bf8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = (0x10u64) as u64; rbx = rdx; rdi = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x10u64) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 4; continue; }
            4 => { rsi = 0; fl_l = 0; fl_r = 0; fl_l = r8; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (((crate::simd_ops::ru8(rdi.wrapping_add(P3))) as i8 as i64) as u64) as u64; r8 = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = (((crate::simd_ops::ru8(r8.wrapping_add(P4))) as i8 as i64) as u64) as u64; rdi = ((rdi).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | ((((rsi as u8)) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b68ec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414b70f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    r8 = rdx;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rax;
    if (((fl_l as u64)) != ((fl_r as u64))) { rax = (rax & !0xffu64) | ((((rbx as u8)) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = (r9 & !0xffu64) | ((((rbx as u8)) as u8 as u64));
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = (crate::simd_ops::ru64(r8)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); }
    rax = (rax & !0xffu64) | ((((rbx as u8)) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b6924(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0x18; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 6; continue; }
            6 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b6af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x1414b68ec;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (((rax) & (r9))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = ((rax).wrapping_add(rax)) as u64; rsi = r8; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx, rdx); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P2), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 8; continue; }
            3 => { rbp = (crate::simd_ops::ru64(rbp.wrapping_add(rax.wrapping_mul(8)))) as u64; bb = 6; continue; }
            4 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; bb = 6; continue; }
            6 => { rdx = ((rdi.wrapping_add(P3)) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi); bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { crate::simd_ops::wu64(rbx, rdi); bb = 2; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b6b78(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1414a8960; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::lk_and_u64(rax, (0x0u64) as u64);
    rsi = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rsi; fl_r = rsi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rdi = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rsi.wrapping_add(P1)) as u64) as u64; r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbx; rcx = rbx; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0x68; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0x30; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rdi; fl_l = rdi; fl_r = rdi; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b6bf8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acda; const P1: u64 = 0x1414b6be4; const P2: u64 = 0x1413aa6a0; const P3: u64 = 0x1414b7084;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (r8) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x8u64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 28; continue; } bb = 1; continue; }
            1 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 2; continue; }
            2 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 3; continue; }
            3 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 4; continue; }
            4 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 5; continue; }
            5 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 6; continue; }
            6 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 7; continue; }
            7 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 8; continue; }
            8 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 54; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_k = 1; bb = 52; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { r9 = (crate::simd_ops::ru64(r8)) as u64; bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 15; continue; }
            15 => { fl_l = (r8) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 54; continue; } bb = 16; continue; }
            16 => { rdx = (r9) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; fl_k = 4; bb = 17; continue; }
            17 => { if (match fl_k { 3 => (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()), 4 => ((((fl_l as u32)) & ((fl_r as u32))) != 0), _ => false }) { bb = 54; continue; } bb = 18; continue; }
            18 => { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 54; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdx)) as u64; xmm0 = ((crate::simd_ops::ru64(rax)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { bb = 54; continue; } bb = 20; continue; }
            20 => { fl_k = 3; bb = 17; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru64(rcx)) as u64 as u64; fl_r = (rax) as u64 as u64; fl_k = 1; bb = 24; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; fl_k = 0; bb = 24; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(r8)) as u8 as u64)); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; fl_k = 2; bb = 24; continue; }
            24 => { rax = (rax & !0xffu64) | (((if (match fl_k { 0 => (((fl_l as u32)) == ((fl_r as u32))), 1 => (((fl_l as u64)) == ((fl_r as u64))), 2 => (((fl_l as u8)) == ((fl_r as u8))), _ => false }) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 55; continue; }
            25 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 55; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(rax)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rcx = (((rcx) ^ (rax))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0xfffffffffffffffcu64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { fl_k = 5; bb = 50; continue; } bb = 27; continue; }
            27 => { r8 = (crate::simd_ops::ru64(r8)) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            28 => { fl_l = (rcx) as u64 as u64; fl_r = (0x9u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 49; continue; } bb = 29; continue; }
            29 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 30; continue; }
            30 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 31; continue; }
            31 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 32; continue; }
            32 => { fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(rax)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0x1u64) as u64 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { fl_k = 1; bb = 50; continue; } bb = 35; continue; }
            35 => { r8 = (crate::simd_ops::ru64(r8)) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            36 => { rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rcx = (((rcx) ^ (rax))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0xfffffffffffffff8u64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { fl_k = 5; bb = 50; continue; } bb = 37; continue; }
            37 => { r8 = (crate::simd_ops::ru64(r8)) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            38 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(rax)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rcx = (((rcx) ^ (rax))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0xffffffffffffffe0u64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 54; continue; } bb = 39; continue; }
            39 => { r9 = (r9 & !0xffu64) | ((((rbx as u8)) as u8 as u64)); rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rbx & !0xffu64) | ((((rax as u8)) as u8 as u64)); bb = 54; continue; }
            40 => { rax = (crate::simd_ops::ru64(rdx)) as u64; r8 = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (r10) as u64; rax = ((rax).wrapping_sub(rdx)) as u64; rcx = (((rcx) ^ (rax))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0xfffffffffffffff8u64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { fl_k = 5; bb = 50; continue; } bb = 41; continue; }
            41 => { fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { fl_k = 1; bb = 18; continue; } bb = 42; continue; }
            42 => { fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 43; continue; }
            43 => { xmm0 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(rdx))) as u64; if (f64::from_bits(fl_l).is_nan() || f64::from_bits(fl_r).is_nan()) { fl_k = 3; bb = 50; continue; } bb = 44; continue; }
            44 => { if (f64::from_bits(fl_l) != f64::from_bits(fl_r) && !f64::from_bits(fl_l).is_nan() && !f64::from_bits(fl_r).is_nan()) { fl_k = 3; bb = 50; continue; } bb = 45; continue; }
            45 => { rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r10) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { fl_k = 1; bb = 18; continue; } bb = 46; continue; }
            46 => { bb = 43; continue; }
            47 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(rax)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rcx = (((rcx) ^ (rax))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0xfffffffffffffff8u64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { fl_k = 5; bb = 50; continue; } bb = 48; continue; }
            48 => { r8 = (crate::simd_ops::ru64(r8)) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            49 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; r10 = (crate::simd_ops::ru64(rax)) as u64; rax = (rdx) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rcx = (((rcx) ^ (rax))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (0xfffffffffffffffcu64) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } fl_k = 5; bb = 50; continue; }
            50 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 54; continue; }
            51 => { rcx = (r10) as u64; fl_k = 5; bb = 52; continue; }
            52 => { r8 = (crate::simd_ops::ru64(r8)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 53; continue; }
            53 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbx = (rbx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 54; continue; }
            54 => { rax = (rax & !0xffu64) | ((((rbx as u8)) as u8 as u64)); bb = 55; continue; }
            55 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b6ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414af9bc; const P1: u64 = 0x1414b6af0; const P2: u64 = 0x1414b6760; const P3: u64 = 0x1414930e0; const P4: u64 = 0x1414b7d8c; const P5: u64 = 0x1414a8960; const P6: u64 = 0x141834d30; const P7: u64 = 0x14307e950; const P8: u64 = 0x141833c0f; const P9: u64 = 0x1414b7080;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b6ea0, funcinfo_rva: 0x36babd4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = (r8) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; rcx = (r8) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b6ec5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rax) as u64; r9 = (rax) as u64; r8 = (rsi) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b6edb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rdi, (rcx) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x0u8) as u8); bb = 20; continue; }
            2 => { rax = (0x555555555555555u64) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 3; continue; }
            3 => { r8 = (rsi) as u64; rdx = ((rbx.wrapping_add(0x8u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b6f16); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = ((rdx).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = 0u128; if ((((fl_l as u64)) as i64) < 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 6; continue; }
            5 => { rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; xmm1 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 9; continue; }
            8 => { rax = (rcx) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 9; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b6f81); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b6f8c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = ((r8).wrapping_add(0x10u64)) as u64; r9 = (rbp) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b6fa5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r9 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (1) as u64); crate::simd_ops::wu64(rcx, (r8) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (r9) as u64); crate::simd_ops::wu64(r9, (rcx) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rcx) as u64); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rax = (((rax) & (rbp))) as u64; rax = ((rax).wrapping_add(rax)) as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (r10) as u64 as u64; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)), (rcx) as u64); bb = 16; continue; }
            13 => { fl_l = (r10) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)), (rcx) as u64); bb = 17; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64 as u64; fl_r = (r9) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64), (rcx) as u64); bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rdi, (rcx) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 18; continue; }
            18 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = ((rbx.wrapping_add(0x10u64)) as u64) as u64; rcx = ((rbx.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b702f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x68u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b703d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = (0x30u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7051); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            20 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            21 => { rcx = ((P7) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b707b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7084(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x14183acda;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rsi = rdx; rdi = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rdi; fl_r = rbx; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_r = (0x10u64) as u64; rdx = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(rbx).wrapping_add(P0))) as u64; fl_r = (0x10u64) as u64; rcx = ((rbx.wrapping_add(rdi)) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(rbx).wrapping_add(P1))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 6; continue; }
            6 => { fl_l = r8; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; rax = ((rbx.wrapping_add(rdi)) as u64) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 10; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b70f8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414b6868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rsi = rdx; rbx = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 1; continue; }
            1 => { rdx = rdi.wrapping_add(0x20); rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 6; continue; }
            4 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rbx = rax; bb = 10; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = rax; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; bb = 14; continue; }
            12 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rdi = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rdi = rax; bb = 18; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdi = rax; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 19; continue; }
            19 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            21 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 20; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7244(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x28; const P2: u64 = 0xfffffffffffffff8; const P3: u64 = 0x141834d30; const P4: u64 = 0x20; const P5: u64 = 0x8; const P6: u64 = 0x1414b6b78; const P7: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = r8; bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P4), (0x0u64) as u64); fl_l = (crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64)) as u64; fl_r = fl_l; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = 0x30; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414b72bc, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b72c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1414a8960; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = 0x68; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x30; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7318(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414b7244; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = 0x50; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7344(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414b6b78; const P1: u64 = 0x141834d30;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = 0x30;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7368(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x18; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7384(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1414b6b78; const P3: u64 = 0x20; const P4: u64 = 0x18; const P5: u64 = 0x14148d224;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rbx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rax, rax); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P1), rax); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b741c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x141833c2d; const P5: u64 = 0x1414b6960; const P6: u64 = 0x141833c49; const P7: u64 = 0x141834d30; const P8: u64 = 0x1418348a3; const P9: u64 = 0x1414b74d4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b741c, funcinfo_rva: 0x36baa4c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (r9) as u64; rbp = (r8) as u64; r14 = (rdx) as u64; rsi = (rcx) as u64; rcx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b743f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax, (r14) as u64); crate::simd_ops::wu64(rax.wrapping_add(P1), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (rbx) as u64); bb = 3; continue; }
            2 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rdi) as u64); rbx = ((rsi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b746c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x50u64), (0x0u64) as u64); crate::simd_ops::wu64(rdx, (rdi) as u64); fl_l = (crate::simd_ops::lk_add_u64(rsi.wrapping_add(P1), (0x8u64) as u64)) as u64; fl_r = fl_l; bb = 7; continue; }
            6 => { r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7497); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 7; continue; }
            7 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b74a0); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b74b5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b74cf); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P9, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b74d8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d64; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1414b66f0; const P5: u64 = 0x1414b6ea0; const P6: u64 = 0x1414a8960; const P7: u64 = 0x141834d30;
    let (mut rax, mut r11, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x38u64), (0x0u64) as u64); rdi = 0x68; rcx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(P0), rax); crate::simd_ops::wu64(rax.wrapping_add(P2), rax); crate::simd_ops::wu16(rax.wrapping_add(P3), (0x101u16) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); r8 = rbp.wrapping_add(0x18); rdx = rbp.wrapping_sub(0x40); rcx = rbp.wrapping_sub(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbp.wrapping_sub(0x28); rdx = rbp.wrapping_sub(0x50); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdx = rbp.wrapping_sub(0x28); rcx = rbp.wrapping_sub(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rdi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdx = rbp.wrapping_sub(0x40); rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rdi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x8; const P2: u64 = 0x1414b66f0; const P3: u64 = 0x1414aa95c; const P4: u64 = 0x1414b67f4; const P5: u64 = 0x1414b6ea0; const P6: u64 = 0x1414a8960; const P7: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r9)) as u64; rdx = rbp.wrapping_sub(0x40); rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbp.wrapping_add(0x18); rdx = rbp.wrapping_sub(0x40); rcx = rbp.wrapping_sub(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            2 => { rdx = r8; rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; r8 = rbp.wrapping_add(0x18); rcx = rbp.wrapping_sub(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { r8 = rbp.wrapping_sub(0x28); rdx = rbp.wrapping_sub(0x50); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rdx = rbp.wrapping_sub(0x28); rcx = rbp.wrapping_sub(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = 0x68; rdx = ((rbx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rdx = rbp.wrapping_sub(0x40); rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rbx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d64; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1414b66f0; const P5: u64 = 0x1414b6ea0; const P6: u64 = 0x1414a8960; const P7: u64 = 0x141834d30;
    let (mut rax, mut r11, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::lk_and_u64(rbp.wrapping_sub(0x38u64), (0x0u64) as u64); rdi = 0x68; rcx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(P0), rax); crate::simd_ops::wu64(rax.wrapping_add(P2), rax); crate::simd_ops::wu16(rax.wrapping_add(P3), (0x101u16) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); r8 = rbp.wrapping_add(0x18); rdx = rbp.wrapping_sub(0x40); rcx = rbp.wrapping_sub(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = rbp.wrapping_sub(0x28); rdx = rbp.wrapping_sub(0x50); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdx = rbp.wrapping_sub(0x28); rcx = rbp.wrapping_sub(0x28); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rdx = rbp.wrapping_sub(0x40); rcx = rbp.wrapping_sub(0x40); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7708(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x8; const P2: u64 = 0x1414b66f0; const P3: u64 = 0x1414aa95c; const P4: u64 = 0x1414b67f4; const P5: u64 = 0x1414b6ea0; const P6: u64 = 0x1414a8960; const P7: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r9)) as u64; rdx = rbp.wrapping_sub(0x40); rcx = r9; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = rbp.wrapping_add(0x18); rdx = rbp.wrapping_sub(0x40); rcx = rbp.wrapping_sub(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            2 => { rdx = r8; rcx = rbp.wrapping_sub(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r8 = rbp.wrapping_add(0x18); rcx = rbp.wrapping_sub(0x28); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { r8 = rbp.wrapping_sub(0x28); rdx = rbp.wrapping_sub(0x50); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rdx = rbp.wrapping_sub(0x28); rcx = rbp.wrapping_sub(0x28); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = 0x68; rdx = ((rbx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; rdx = rbp.wrapping_sub(0x40); rcx = rbp.wrapping_sub(0x40); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b77c8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x141833c2d; const P2: u64 = 0x14148aee4; const P3: u64 = 0x1414b81c4; const P4: u64 = 0x141834d64; const P5: u64 = 0x14266b350; const P6: u64 = 0x1414b71d4; const P7: u64 = 0x14266b410; const P8: u64 = 0x14266b370; const P9: u64 = 0x14266b438; const P10: u64 = 0x14183af0c; const P11: u64 = 0x1412dd770; const P12: u64 = 0x14266b388; const P13: u64 = 0x1411f4730; const P14: u64 = 0x14266ac70; const P15: u64 = 0x1402034f0; const P16: u64 = 0x14148aef4; const P17: u64 = 0x14266b3c0; const P18: u64 = 0x1426467f0; const P19: u64 = 0x1414b7b3a; const P20: u64 = 0x141834d30; const P21: u64 = 0x1434278a0; const P22: u64 = 0x140200450; const P23: u64 = 0x1412dd710; const P24: u64 = 0x142645ac0; const P25: u64 = 0x141833c49; const P26: u64 = 0x1418348a3; const P27: u64 = 0x1414b7bbc;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 77>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b77c8, funcinfo_rva: 0x36baa98, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x268, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x168u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x240u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x130u64), (rax) as u64); rbx = (rdx) as u64; rdi = (rcx) as u64; r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rcx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7811); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 38; continue; } bb = 1; continue; }
            1 => { r14 = (crate::simd_ops::ru64(rdi)) as u64; r13 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; bb = 27; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7831); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rax.wrapping_add(0x64u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7840); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r12 = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 3; continue; }
            3 => { rcx = (0x50u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7856); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); rcx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x48u64), (0x0u32) as u32); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7892); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), (r15) as u64); bb = 7; continue; }
            6 => { rsi = (r15) as u64; bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rsi) as u64); rcx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b78a9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = 0u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rsi) as u64); bb = 10; continue; }
            9 => { rdi = (r15) as u64; bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rdi) as u64); rax = (crate::simd_ops::ru64(r14)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm0) as u128); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rdi) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), (0x1u8) as u8); rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b791a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(r12)) as u64; rdx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; r8 = (rbx) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b792b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            13 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), (rsi) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); rcx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rcx) as u64); crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); r8 = ((rsp.wrapping_add(0x31u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b795f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), (r15) as u64); bb = 16; continue; }
            15 => { rsi = (r15) as u64; bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rsi) as u64); rcx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7976); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { xmm0 = 0u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rdi, (rax) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rsi) as u64); bb = 19; continue; }
            18 => { rdi = (r15) as u64; bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (rsi) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rdi) as u64); rax = (crate::simd_ops::ru64(r14)) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rdi) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x80u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b79e6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((r8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(r12)) as u64; rdx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; r8 = (rbx) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b79f7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 22; continue; }
            22 => { fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rsi = ((((rsi as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7a14); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7a27); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 26; continue; }
            26 => { fl_l = ((r14).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; r15 = (0x0u32) as u32 as u64; bb = 27; continue; }
            27 => { fl_l = (r14) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 28; continue; }
            28 => { bb = 37; continue; }
            29 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7a42); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 37; continue; } bb = 30; continue; }
            30 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7a51); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 31; continue; }
            31 => { rdx = (0x1u32) as u32 as u64; rcx = ((rbp.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7a6b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P12) as u64) as u64; rcx = ((rbp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7a7c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P14) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7a8b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_and_u32(rsp.wrapping_add(0x34u64), (0x0u32) as u32); r8 = ((rsp.wrapping_add(0x32u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x34u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7aad); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_add(0x110u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7abe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp.wrapping_add(0x110u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x128u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x110u64)) } else { rax }) as u64; rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0x38u32) as u32 as u64; r8 = ((P17) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7afa); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rbx) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x128u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 32; continue; }
            32 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x110u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7b34); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P18)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P19, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7b3b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 36; continue; }
            36 => { xmm0 = (crate::simd_ops::ru128(P21)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x120u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x110u64), ((r15 as u8)) as u8); rcx = ((rbp.wrapping_sub(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7b5b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0xa8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7b68); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0xa8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7b74); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P24)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7b80); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P25) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x130u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0x240u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            38 => { rcx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7bb7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P26) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P27, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141833c2d; const P1: u64 = 0x141834d30; const P2: u64 = 0x1414b7f74; const P3: u64 = 0x141833c49; const P4: u64 = 0x1418348a3;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rcx; r15 = r9; rcx = ((rcx).wrapping_add(0x18u64)) as u64; r12 = r8; r13 = rdx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(r14)) as u64; bb = 6; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 8; continue; }
            8 => { rsi = rdi.wrapping_add(0x8); bb = 17; continue; }
            9 => { rax = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::lk_and_u64(rsi, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rdi, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = 0x18; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { fl_l = ((rsi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 18; continue; }
            18 => { r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x50); r8 = rdi; rcx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = r14.wrapping_add(0x18); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            19 => { rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414b7ca9, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7cac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d30; const P2: u64 = 0x10; const P3: u64 = 0xfffffffffffffff8; const P4: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r14 = r9; r15 = r8; rsi = rdx; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; bb = 5; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = 0x18; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = ((rdx).wrapping_sub(rcx)) as u64; rdx = (((rdx) & (0xfffffffffffffff8u64))) as u64; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = ((rcx).wrapping_sub(r8)) as u64; rax = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = r8; bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = ((rsi.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); rax = ((rsi.wrapping_add(r14.wrapping_mul(8))) as u64) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414b7d5f, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1414b7244; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x50; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7d8c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141492e20; const P1: u64 = 0x1414af9bc; const P2: u64 = 0x1414b68ec; const P3: u64 = 0x14307e970; const P4: u64 = 0x141833c0f; const P5: u64 = 0x1414b7f2c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b7d8c, funcinfo_rva: 0x36bab70, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (rcx) as u64; rax = (0xfffffffffffffffu64) as u64; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rbx = (0x1u32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(((rcx as u32) & 63))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_sub(0x1u64)) as u64) as u64; rax = (((rax) | (rbx))) as u64; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_shl(((rcx as u32) & 63))) as u64; rbp = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r15 = ((rsi.wrapping_add(0x18u64)) as u64) as u64; rdx = ((rbx.wrapping_add(rbx)) as u64) as u64; r8 = (rbp) as u64; rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7deb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((rbx.wrapping_sub(0x1u64)) as u64) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rsi) as u64); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; rdi = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; r12 = ((rbx.wrapping_add(0x10u64)) as u64) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7e1e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = (rax) as u64; r14 = (((r14) & (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))))) as u64; r14 = ((r14).wrapping_add(r14)) as u64; r15 = (crate::simd_ops::ru64(r15)) as u64; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(r14.wrapping_mul(8)))) as u64 as u64; fl_r = (rbp) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r15.wrapping_add(r14.wrapping_mul(8)), (rbx) as u64); crate::simd_ops::wu64(r15.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x8u64), (rbx) as u64); bb = 13; continue; }
            4 => { rsi = (crate::simd_ops::ru64(r15.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = ((rsi.wrapping_add(0x10u64)) as u64) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7e50); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, (rdi) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, (r8) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rcx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r15.wrapping_add(r14.wrapping_mul(8)).wrapping_add(0x8u64), (rbx) as u64); bb = 12; continue; }
            8 => { r13 = (rsi) as u64; bb = 10; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r13 = (rsi) as u64; rdx = ((rsi.wrapping_add(0x10u64)) as u64) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7e9c); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(r14.wrapping_mul(8)))) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, (rdi) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, (r13) as u64); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu64(r13.wrapping_add(0x8u64), (rcx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(r15.wrapping_add(r14.wrapping_mul(8)), (rbx) as u64); bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 13; continue; }
            13 => { rbx = (rdi) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rbp) as u64 as u64; r15 = ((rsi.wrapping_add(0x18u64)) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            15 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, (rdi) as u64); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, (r8) as u64); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, (rbx) as u64); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), (rcx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rdx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (rax) as u64); bb = 12; continue; }
            16 => { rcx = ((P3) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b7f27); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); crate::lift_ops::trap(0x80000003, P5, rcx, rsp); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b7f74(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r9; rdi = r8; r14 = rdx; rbp = rcx; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { r15 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = r8; bb = 6; continue; }
            2 => { fl_l = rsi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsi, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = 0x18; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rbx = rsi; fl_l = rsi; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = 0x18; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rsi); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r14; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(r14, rdi); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b460; const P2: u64 = 0x1414aa66c; const P3: u64 = 0x1414ab1c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b8020, funcinfo_rva: 0x36bac84, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rbx = (r9) as u64; rsi = (r8) as u64; rbp = (rdx) as u64; rdi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r8) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rbx) as u64); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); rcx = ((rcx).wrapping_add(0x8u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8066); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::lk_and_u64(rsi, (0x0u64) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x78u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x80u64), (0x0u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), (rax) as u64); rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b80a5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b80bb); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rsi = ((((rsi as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b80db); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rdx)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b80ee); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 8; continue; }
            8 => { rax = (rdi) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b810c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x70; const P4: u64 = 0x1414ab1c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b81ac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x1414b741c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b81cc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x1414b7bc0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b81e4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x8; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8214(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x14266b4f8; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x14266b570; const P5: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b8214, funcinfo_rva: 0x36bad00, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (rcx) as u64; rcx = (0x8u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b822b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); bb = 3; continue; }
            2 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rdi, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P2), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rbx) as u64); rcx = (0x18u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8259); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rax, (xmm0) as u128); rcx = (0x1u32) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P2), ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P3), ((rcx as u32)) as u32); rcx = ((P4) as u64) as u64; crate::simd_ops::wu64(rax, (rcx) as u64); crate::simd_ops::wu64(rax.wrapping_add(P5), (rbx) as u64); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi, (rbx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(P2), (rax) as u64); rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b829c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266b478; const P1: u64 = 0x140200910; const P2: u64 = 0x141834d64; const P3: u64 = 0x140200590; const P4: u64 = 0x1402019a0; const P5: u64 = 0x14266b548; const P6: u64 = 0x140203460;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r9 = rdx; rbx = rcx; rcx = P0; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rcx); fl_l = (((rax) | (0xffffffffffffffffu64))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 1; continue; }
            1 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x9); rdx = rbp.wrapping_add(0x7); rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x17u64))) as u8 as u64; fl_r = (0x5u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xfu64))) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); bb = 12; continue; }
            6 => { rcx = 0x38; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rbp.wrapping_add(0x1fu64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rax); crate::simd_ops::wu16(rbp.wrapping_add(0x37u64), ((rax as u16)) as u16); rcx = rbp.wrapping_add(0x3f); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x1f); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; bb = 9; continue; }
            8 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { crate::simd_ops::lk_and_u64(rbx, (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x8u64), (0x0u64) as u64); rcx = 0x18; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), (0x1u32) as u32); rcx = P5; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdi); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 12; continue; }
            12 => { rcx = rbp.wrapping_add(0x7); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b83b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14183acda; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x14266b488;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (((rax) ^ (rsp))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    xmm0 = 0u128;
    rdx = rsp.wrapping_add(0x20);
    r8 = 0x10;
    rdi = rcx;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rax = (rax & !0xffu64) | ((((rbx as u8)) as u8 as u64)); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    r8 = ((rbx.wrapping_add(P3)) as u32) as u32 as u64;
    rdx = P4;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); }
    rax = (rax & !0xffu64) | ((((rbx as u8)) as u8 as u64));
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rcx = (((rcx) ^ (rsp))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8428(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8444(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140200850; const P1: u64 = 0x1414a7d84; const P2: u64 = 0x141834d64; const P3: u64 = 0x1414aa364; const P4: u64 = 0x1414aae64; const P5: u64 = 0x141834d30; const P6: u64 = 0x1401e3410; const P7: u64 = 0x1401de2f0; const P8: u64 = 0x1414b6138;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = r9; r12 = r8; r13 = rcx; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); rbx = rcx.wrapping_add(0x248); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = r13.wrapping_add(0x78); rdx = r15; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0x40; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); r8 = rbp.wrapping_add(0x40); rdx = rbp.wrapping_add(0x50); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            2 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(r15)) as u64; crate::simd_ops::wu64(r15, r14); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x40; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rbp.wrapping_add(0x40u64), (0x0u8) as u8); rcx = rbp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (rsi & !0xffu64) | ((((rax as u8)) as u8 as u64)); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rax, ((rsi as u8)) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; xmm1 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); rcx = r13.wrapping_add(0x148); rax = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rdi; r8 = rbp.wrapping_sub(0x10); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8568(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414aa66c; const P2: u64 = 0x1414aabd4; const P3: u64 = 0x14266b5b0; const P4: u64 = 0x14183af0c; const P5: u64 = 0x1412dd770; const P6: u64 = 0x14266b660; const P7: u64 = 0x1411f4730; const P8: u64 = 0x14266ac70; const P9: u64 = 0x14266ac58; const P10: u64 = 0x1402034f0; const P11: u64 = 0x14148aef4; const P12: u64 = 0x14266b6b0; const P13: u64 = 0x1426467f0; const P14: u64 = 0x1414b8704; const P15: u64 = 0x141834d30; const P16: u64 = 0x1434278a0; const P17: u64 = 0x140200450; const P18: u64 = 0x1412dd710; const P19: u64 = 0x142645ac0; const P20: u64 = 0x1414ab1c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 69>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b8568, funcinfo_rva: 0x36ba10c, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x228, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x100u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x200u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xf8u64), (rax) as u64); r14 = (r8) as u64; r15 = (rdx) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r8) as u64); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::lk_and_u64(r8, (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b85c9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = (rax) as u64; rcx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b85db); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b85f6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8605); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = (0x1u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8620); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P6) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8631); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8640); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P9) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b864f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8676); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8687); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64)) } else { rax }) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0x16u32) as u32 as u64; r8 = ((P12) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b86c4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rbx) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b86fe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8705); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(P16)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xe8u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0xd8u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8726); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8730); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8739); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P19)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8743); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8759); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (rsi) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf8u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x248u64))) as u64; rsp = ((rsp).wrapping_add(0x200u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8788(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414aa66c; const P2: u64 = 0x1414aabd4; const P3: u64 = 0x14266b640; const P4: u64 = 0x14183af0c; const P5: u64 = 0x1412dd770; const P6: u64 = 0x14266b7b0; const P7: u64 = 0x1411f4730; const P8: u64 = 0x14266ac70; const P9: u64 = 0x14266ac58; const P10: u64 = 0x1402034f0; const P11: u64 = 0x14148aef4; const P12: u64 = 0x14266b6b0; const P13: u64 = 0x1426467f0; const P14: u64 = 0x1414b8924; const P15: u64 = 0x141834d30; const P16: u64 = 0x1434278a0; const P17: u64 = 0x140200450; const P18: u64 = 0x1412dd710; const P19: u64 = 0x142645ac0; const P20: u64 = 0x1414ab1c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 69>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b8788, funcinfo_rva: 0x36ba10c, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x228, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x100u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x200u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xf8u64), (rax) as u64); r14 = (r8) as u64; r15 = (rdx) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r8) as u64); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::lk_and_u64(r8, (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b87e9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = (rax) as u64; rcx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b87fb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8816); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8825); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = (0x1u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8840); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P6) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8851); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8860); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P9) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b886f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8896); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b88a7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64)) } else { rax }) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0xb5u32) as u32 as u64; r8 = ((P12) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b88e4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rbx) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b891e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8925); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(P16)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xe8u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0xd8u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8946); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8950); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8959); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P19)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8963); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8979); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (rsi) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf8u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x248u64))) as u64; rsp = ((rsp).wrapping_add(0x200u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b89a8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414aa66c; const P2: u64 = 0x1414aabd4; const P3: u64 = 0x14266b610; const P4: u64 = 0x14183af0c; const P5: u64 = 0x1412dd770; const P6: u64 = 0x14266b750; const P7: u64 = 0x1411f4730; const P8: u64 = 0x14266ac70; const P9: u64 = 0x14266ac58; const P10: u64 = 0x1402034f0; const P11: u64 = 0x14148aef4; const P12: u64 = 0x14266b6b0; const P13: u64 = 0x1426467f0; const P14: u64 = 0x1414b8b44; const P15: u64 = 0x141834d30; const P16: u64 = 0x1434278a0; const P17: u64 = 0x140200450; const P18: u64 = 0x1412dd710; const P19: u64 = 0x142645ac0; const P20: u64 = 0x1414ab1c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 69>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b89a8, funcinfo_rva: 0x36ba10c, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x228, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x100u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x200u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xf8u64), (rax) as u64); r14 = (r8) as u64; r15 = (rdx) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r8) as u64); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::lk_and_u64(r8, (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8a09); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = (rax) as u64; rcx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8a1b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8a36); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8a45); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = (0x1u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8a60); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P6) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8a71); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8a80); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P9) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8a8f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8ab6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8ac7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64)) } else { rax }) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0x83u32) as u32 as u64; r8 = ((P12) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8b04); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rbx) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8b3e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8b45); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(P16)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xe8u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0xd8u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8b66); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8b70); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8b79); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P19)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8b83); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8b99); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (rsi) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf8u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x248u64))) as u64; rsp = ((rsp).wrapping_add(0x200u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8bc8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414aa66c; const P2: u64 = 0x1414aabd4; const P3: u64 = 0x14266b5e0; const P4: u64 = 0x14183af0c; const P5: u64 = 0x1412dd770; const P6: u64 = 0x14266b6f0; const P7: u64 = 0x1411f4730; const P8: u64 = 0x14266ac70; const P9: u64 = 0x14266ac58; const P10: u64 = 0x1402034f0; const P11: u64 = 0x14148aef4; const P12: u64 = 0x14266b6b0; const P13: u64 = 0x1426467f0; const P14: u64 = 0x1414b8d64; const P15: u64 = 0x141834d30; const P16: u64 = 0x1434278a0; const P17: u64 = 0x140200450; const P18: u64 = 0x1412dd710; const P19: u64 = 0x142645ac0; const P20: u64 = 0x1414ab1c4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 69>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b8bc8, funcinfo_rva: 0x36ba10c, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x228, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x100u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x200u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xf8u64), (rax) as u64); r14 = (r8) as u64; r15 = (rdx) as u64; rsi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), (rdx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (r8) as u64); rax = ((rsp.wrapping_add(0x38u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); rax = (crate::simd_ops::ru64(r8)) as u64; crate::simd_ops::lk_and_u64(r8, (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8c29); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rdx = (rax) as u64; rcx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8c3b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsi, (rax) as u64); fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8c56); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 9; continue; } bb = 2; continue; }
            2 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8c65); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = (0x1u32) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8c80); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((P6) as u64) as u64; rcx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8c91); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P8) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8ca0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; rdx = ((P9) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8caf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8cd6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x18u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8ce7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64)) } else { rax }) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (0x4cu32) as u32 as u64; r8 = ((P12) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8d24); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((rbx) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8d5e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P14, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8d65); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(P16)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xe8u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0xd8u64), (0x0u8) as u8); rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8d86); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8d90); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8d99); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P19)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rcx = (r15) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8da3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(r14)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rdx = (0x1u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b8db9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (rsi) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf8u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x248u64))) as u64; rsp = ((rsp).wrapping_add(0x200u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8de8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8ed8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = 0u128;
    r8 = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8f38(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = rsp.wrapping_add(0x20);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8f94(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r9 = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P1), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = ((((rax as u32)) | (0xffffffffu32))) as u32 as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b8ff4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x142646648; const P2: u64 = 0x14183af0c; const P3: u64 = 0x1412dd770; const P4: u64 = 0x14266b870; const P5: u64 = 0x1411f4730; const P6: u64 = 0x14148b13c; const P7: u64 = 0x1402034f0; const P8: u64 = 0x14148aef4; const P9: u64 = 0x14266b6b0; const P10: u64 = 0x1426467f0; const P11: u64 = 0x141834d30; const P12: u64 = 0x1434278a0; const P13: u64 = 0x140200450; const P14: u64 = 0x1412dd710; const P15: u64 = 0x142645ac0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 61>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rax.wrapping_sub(0xe8); rsp = ((rsp).wrapping_sub(0x1d0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rax); rdi = r8; xmm6 = (xmm1) as u128; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; r14 = 0x1; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm1 as u64)) as u64); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm6 as u64)) as u64); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rdi; xmm1 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P4; rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x90); rcx = rbp.wrapping_sub(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x90); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x90u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0xd6; r8 = P9; rdx = ((r14 as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414b9181, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(P12)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x90u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x1d0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b91e4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x142646648; const P2: u64 = 0x14183af0c; const P3: u64 = 0x1412dd770; const P4: u64 = 0x14266b800; const P5: u64 = 0x1411f4730; const P6: u64 = 0x14148b13c; const P7: u64 = 0x1402034f0; const P8: u64 = 0x14148aef4; const P9: u64 = 0x14266b6b0; const P10: u64 = 0x1426467f0; const P11: u64 = 0x141834d30; const P12: u64 = 0x1434278a0; const P13: u64 = 0x140200450; const P14: u64 = 0x1412dd710; const P15: u64 = 0x142645ac0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 63>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0xf8); rsp = ((rsp).wrapping_sub(0x1d0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rax); rsi = r9; rdi = r8; xmm6 = (xmm1) as u128; rbx = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; r15 = 0x1; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm1 as u64)) as u64); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm6 as u64)) as u64); rcx = rsp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = rsi; r8 = rdi; xmm1 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { rdx = ((r15 as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x70); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = P4; rcx = rbp.wrapping_sub(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; rdx = rbx.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); r8 = rsp.wrapping_add(0x30); rdx = rsp.wrapping_add(0x38); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x90); rcx = rbp.wrapping_sub(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbp.wrapping_add(0x90); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_r = (0x10u64) as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x90u64)) } else { rax }) as u64; rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = 0xc6; r8 = P9; rdx = ((r15 as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rbx, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa8u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414b9375, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(P12)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x90u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xb0u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x1c0u64))) as u128; rsp = ((rsp).wrapping_add(0x1d0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b93d8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141834d64; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x140205190;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b93d8, funcinfo_rva: 0x36bb118, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (r8) as u64;
    rdi = (rcx) as u64;
    crate::simd_ops::wu64(rcx, (rdx) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rcx = (0x30u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414b93ff); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    crate::simd_ops::wu64(rdi.wrapping_add(P0), (rax) as u64);
    rcx = ((rax.wrapping_add(P2)) as u64) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx, (xmm0) as u128);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P2), (0x0u64) as u64);
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P3), (0x0u64) as u64);
    r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64 as u64; fl_r = (0x10u64) as u64 as u64;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rbx = (crate::simd_ops::ru64(rbx)) as u64; }
    rdx = (rbx) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414b942d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rdi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b9690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140204cc0; const P1: u64 = 0x1414a89cc; const P2: u64 = 0x1414b93d8; const P3: u64 = 0x1414930e0; const P4: u64 = 0x1414bb200; const P5: u64 = 0x1402038e0; const P6: u64 = 0x141834d30; const P7: u64 = 0x14307e950; const P8: u64 = 0x141833c0f; const P9: u64 = 0x1414b9868;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b9690, funcinfo_rva: 0x36bb0e8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x48, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (r8) as u64; rdi = (rdx) as u64; rsi = (rcx) as u64; rcx = (r8) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r8)) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b96c3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbp = (rax) as u64; r9 = (rax) as u64; r8 = (rbx) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b96d9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi, (rcx) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x0u8) as u8); bb = 23; continue; }
            4 => { rax = (0x555555555555555u64) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 5; continue; }
            5 => { r8 = (rbx) as u64; rdx = ((rsi.wrapping_add(0x8u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9714); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = ((rdx).wrapping_add(0x1u64)) as u64 as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = 0u128; if ((((fl_l as u64)) as i64) < 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); bb = 8; continue; }
            7 => { rcx = (rdx) as u64; rcx = ((rcx).wrapping_shr(1u32)) as u64; rax = (rdx) as u64; rax = ((((rax as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rcx) | (rax))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; xmm1 = 0u128; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); bb = 11; continue; }
            10 => { rax = (rcx) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; fl_l = (((rax) | (rcx))) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 11; continue; }
            11 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b977f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b978a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r8 = ((rbx.wrapping_add(0x10u64)) as u64) as u64; r9 = (rbp) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b97a3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); bb = 14; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 14; continue; }
            14 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::lk_add_u64(rsi.wrapping_add(0x10u64), (1) as u64); crate::simd_ops::wu64(rbx, (rdx) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (r8) as u64); crate::simd_ops::wu64(r8, (rbx) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (rbx) as u64); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rax = (((rax) & (rbp))) as u64; rax = ((rax).wrapping_add(rax)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = (r9) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (rbx) as u64); bb = 19; continue; }
            16 => { fl_l = (r9) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)), (rbx) as u64); bb = 20; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64), (rbx) as u64); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rdi, (rbx) as u64); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9825); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = (0x30u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9839); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 23; continue; }
            23 => { rax = (rdi) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rcx = ((P7) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9863); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b9944(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1402038e0; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = 0x30; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b998c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14183b638; const P2: u64 = 0x1414b986c; const P3: u64 = 0x1414bb464; const P4: u64 = 0x1402050d0; const P5: u64 = 0x140204cc0; const P6: u64 = 0x1414a89cc; const P7: u64 = 0x1414b9690; const P8: u64 = 0x1414b94d8; const P9: u64 = 0x14149f678; const P10: u64 = 0x141497b54; const P11: u64 = 0x1414b9444; const P12: u64 = 0x141498664; const P13: u64 = 0x141834d30; const P14: u64 = 0x1426467f0; const P15: u64 = 0x1414b98dc; const P16: u64 = 0x1414b9c41;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 55>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b998c, funcinfo_rva: 0x36bb020, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x1b8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rbp = ((rsp.wrapping_sub(0xa0u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x1a0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), (rax) as u64); rax = (rdx) as u64; rbx = (rcx) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm0) as u128); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (0xfu64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), ((rdi as u8)) as u8); rdx = ((rbp.wrapping_add(0x10u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b99da); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (0x1000u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu16(rsp.wrapping_add(0x24u64), (0x3d2cu16) as u16); rax = ((rbp.wrapping_add(0x10u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_add(0x10u64)) } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x24u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x70u64), ((rax as u16)) as u16); crate::simd_ops::wu8(rsp.wrapping_add(0x72u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rdi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rdi) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); r8 = ((rsp.wrapping_add(0x28u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x24u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9a5e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { r9 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9a78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x20u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); crate::simd_ops::wu64(rbp, (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (rdi) as u64); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9aa9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64)) } else { rcx }) as u64; rdx = (crate::simd_ops::ru64(rbp)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9ac1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (rax) as u64; r8 = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9ad5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64 as u64; fl_r = (rdi) as u64 as u64; rcx = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rax.wrapping_add(0x8u64)) } else { rcx }) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { r8 = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9af9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9b0a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9b21); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (rdi) as u64); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = ((rbp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9b43); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rbp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9b55); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rbp.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9b66); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9b70); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9ba4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rdi) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (0xfu64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x30u64), ((rdi as u8)) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 12; continue; }
            12 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9beb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            15 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9bf5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P14)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 16; continue; }
            16 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9bfc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P14)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rcx = ((rbp.wrapping_sub(0x80u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9c07); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9c3b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P14)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P16, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9c42); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; rsp = ((rsp).wrapping_add(0x1a0u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b9c6c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d1e0; const P1: u64 = 0x14392bf08; const P2: u64 = 0x1438130a8; const P3: u64 = 0x143813090; const P4: u64 = 0x1438130a0; const P5: u64 = 0x8; const P6: u64 = 0x141834a78; const P7: u64 = 0x142643db4; const P8: u64 = 0x141834d18; const P9: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = rcx; rdx = (crate::simd_ops::ru32(P0)) as u32 as u64; rcx = 0x74; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = (0x10u64) as u64; rax = P3; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(P3) } else { rax }) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = P1; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = P7; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = P1; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b9d30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x60; const P2: u64 = 0x141834d64; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1414b998c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b9d30, funcinfo_rva: 0x36bb0b8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rcx) as u64);
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (r8) as u64;
    rdi = (rdx) as u64;
    rsi = (rcx) as u64;
    rax = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx, (rax) as u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (rax) as u64);
    rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414b9d5d); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    crate::simd_ops::wu64(rax, (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P0), (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P3), (rax) as u64);
    crate::simd_ops::wu16(rax.wrapping_add(P4), (0x101u16) as u16);
    crate::simd_ops::wu64(rsi, (rax) as u64);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rdx = (rbx) as u64;
    rcx = (rsi) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414b9d84); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rdx = (rdi) as u64;
    rcx = (rsi) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414b9d8f); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8);
    rax = (rsi) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b9da8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b970; const P3: u64 = 0x14266ba10; const P4: u64 = 0x14266ba00; const P5: u64 = 0x1434278a0; const P6: u64 = 0x14183b638; const P7: u64 = 0x14183acbc; const P8: u64 = 0x14266b8e0; const P9: u64 = 0x140571d70; const P10: u64 = 0x1402050d0; const P11: u64 = 0x1426467f0; const P12: u64 = 0x1414b9f0b; const P13: u64 = 0x141834d30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b9da8, funcinfo_rva: 0x36bae10, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xd8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), (rcx) as u64); rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x544e49u32) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), (0x455f504cu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x15u64), (0x4f50444eu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), (0x544e49u32) as u32); xmm0 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x1fu64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x2fu64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x3fu64), ((rdi as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x49u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P5)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x49u64), ((rdi as u8)) as u8); rdx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9e4f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); r8 = (0x20u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9e73); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            2 => { rcx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9e82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (0xbu32) as u32 as u64; rdx = ((P8) as u64) as u64; rcx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9e9c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_sub(0x49u64)) as u64) as u64; bb = 1; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (rdi) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (rdi) as u64); rax = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; fl_l = (((r8) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdx = ((rbp.wrapping_add(0x1fu64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9ecd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9f05); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9f0c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x47u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = ((rsp.wrapping_add(0xd0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414b9f38(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b930; const P3: u64 = 0x14266ba10; const P4: u64 = 0x14266b8f0; const P5: u64 = 0x1434278a0; const P6: u64 = 0x14183b638; const P7: u64 = 0x14183acbc; const P8: u64 = 0x14266b8d0; const P9: u64 = 0x140571d70; const P10: u64 = 0x1402050d0; const P11: u64 = 0x1426467f0; const P12: u64 = 0x1414ba098; const P13: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414b9f38, funcinfo_rva: 0x36badd8, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm1) as u128); crate::simd_ops::wu16(rbp.wrapping_add(0x27u64), (0x544eu16) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x29u64), (0x0u8) as u8); crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), (0x455f504cu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x5u64), (0x4f50444eu32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0x544e49u32) as u32); xmm0 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x2fu64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x3fu64), (xmm1) as u128); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P5)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x39u64), (0x0u8) as u8); rdx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9fd7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414b9ffd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            2 => { rcx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba00c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = (0xau32) as u32 as u64; rdx = ((P8) as u64) as u64; rcx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba026); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; bb = 1; continue; }
            4 => { xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); rax = ((rbp.wrapping_add(0x2fu64)) as u64) as u64; fl_l = (((r8) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rdx = ((rbp.wrapping_add(0x2fu64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba05a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba092); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P11)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P12, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba099); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            11 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba0c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b900; const P3: u64 = 0x1414b9d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rsp;
    rsp = ((rsp).wrapping_sub(0x80u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (((rax) ^ (rsp))) as u64;
    crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax);
    xmm0 = (crate::simd_ops::ru128(P1)) as u128;
    r8 = rbp.wrapping_sub(0x50);
    xmm1 = (crate::simd_ops::ru128(P2)) as u128;
    rdx = rbp.wrapping_sub(0x30);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rcx);
    crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu8(rbp.wrapping_sub(0xeu64), ((rax as u8)) as u8);
    rbx = rcx;
    crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128);
    crate::simd_ops::wu8(rbp.wrapping_sub(0x36u64), ((rax as u8)) as u8);
    crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128);
    crate::simd_ops::wu16(rbp.wrapping_sub(0x10u64), (0x5352u16) as u16);
    crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), (0x485f504cu32) as u32);
    crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), (0x45444145u32) as u32);
    crate::simd_ops::wu16(rbp.wrapping_sub(0x38u64), (0x5352u16) as u16);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64;
    rcx = (((rcx) ^ (rsp))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64;
    rsp = ((rsp).wrapping_add(0x80u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba154(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b940; const P3: u64 = 0x1434278a0; const P4: u64 = 0x1414bb1c8; const P5: u64 = 0x14183acbc; const P6: u64 = 0x1426467f0; const P7: u64 = 0x1414ba264; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414ba154, funcinfo_rva: 0x36bae68, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x41434946u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x4bu64), (0x4554u16) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x4du64), (0x0u8) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x435f504cu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x49545245u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x41434946u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x23u64), (0x4554u16) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x25u64), (0x0u8) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x19u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba1ee); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba214); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba25e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba265); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba28c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b940; const P3: u64 = 0x14266b9a0; const P4: u64 = 0x1434278a0; const P5: u64 = 0x1414bb1c8; const P6: u64 = 0x14183acbc; const P7: u64 = 0x1426467f0; const P8: u64 = 0x1414ba3a1; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414ba28c, funcinfo_rva: 0x36baec0, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x41434946u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x535f4554u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x4e495254u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x23u64), (0x47u16) as u16); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x4e495254u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x4bu64), (0x47u16) as u16); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba32b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba351); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba39b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba3a2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba3c8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b950; const P3: u64 = 0x14266b920; const P4: u64 = 0x1434278a0; const P5: u64 = 0x1414bb1c8; const P6: u64 = 0x14183acbc; const P7: u64 = 0x1426467f0; const P8: u64 = 0x1414ba4dd; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414ba3c8, funcinfo_rva: 0x36baec0, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x45435f54u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x46495452u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x54414349u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x23u64), (0x45u16) as u16); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x54414349u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x4bu64), (0x45u16) as u16); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba467); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba48d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba4d7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba4de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba504(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b950; const P3: u64 = 0x14266b9d0; const P4: u64 = 0x14266b920; const P5: u64 = 0x1434278a0; const P6: u64 = 0x1414bb1c8; const P7: u64 = 0x14183acbc; const P8: u64 = 0x1426467f0; const P9: u64 = 0x1414ba62a; const P10: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414ba504, funcinfo_rva: 0x36bafc8, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x474e4952u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x4bu64), (0x0u8) as u8); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), (0x54414349u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xbu64), (0x54535f45u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), (0x474e4952u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x13u64), (0x0u8) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P5)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x39u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba5b4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba5da); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba624); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba62b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b950; const P3: u64 = 0x1434278a0; const P4: u64 = 0x1414bb1c8; const P5: u64 = 0x14183acbc; const P6: u64 = 0x1426467f0; const P7: u64 = 0x1414ba758; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414ba650, funcinfo_rva: 0x36baf18, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x454b5f54u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x4bu64), (0x59u16) as u16); crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x435f504cu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x4e45494cu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x454b5f54u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x23u64), (0x59u16) as u16); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x19u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba6e2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba708); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba752); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba759); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b950; const P3: u64 = 0x14266b9e0; const P4: u64 = 0x1434278a0; const P5: u64 = 0x1414bb1c8; const P6: u64 = 0x14183acbc; const P7: u64 = 0x1426467f0; const P8: u64 = 0x1414ba891; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414ba780, funcinfo_rva: 0x36baf70, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x454b5f54u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x54535f59u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x474e4952u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x23u64), (0x0u8) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x474e4952u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x4bu64), (0x0u8) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba81b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba841); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba88b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414ba892); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba8b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b960; const P3: u64 = 0x1414b9cf8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); xmm0 = (crate::simd_ops::ru128(P1)) as u128; rdx = rbp.wrapping_sub(0x50); xmm1 = (crate::simd_ops::ru128(P2)) as u128; rbx = rcx; r8 = rcx; crate::simd_ops::wu16(rbp.wrapping_sub(0x10u64), (0x5455u16) as u16); rcx = rbp.wrapping_sub(0x30); crate::simd_ops::wu8(rbp.wrapping_sub(0xeu64), (0x0u8) as u8); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), (0x545f504cu32) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), (0x4f454d49u32) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); crate::simd_ops::wu16(rbp.wrapping_sub(0x38u64), (0x5455u16) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x36u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = rbx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx, (0x5f5e100u64) as u64); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b980; const P3: u64 = 0x1414b9d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbp);
    rbp = rsp;
    rsp = ((rsp).wrapping_sub(0x80u64)) as u64;
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (((rax) ^ (rsp))) as u64;
    crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax);
    xmm0 = (crate::simd_ops::ru128(P1)) as u128;
    r8 = rbp.wrapping_sub(0x50);
    xmm1 = (crate::simd_ops::ru128(P2)) as u128;
    rdx = rbp.wrapping_sub(0x30);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rcx);
    crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128);
    crate::simd_ops::wu8(rbp.wrapping_sub(0x36u64), ((rax as u8)) as u8);
    rbx = rcx;
    crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128);
    crate::simd_ops::wu16(rbp.wrapping_sub(0x10u64), (0x53u16) as u16);
    crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128);
    crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), (0x485f504cu32) as u32);
    crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), (0x45444145u32) as u32);
    crate::simd_ops::wu16(rbp.wrapping_sub(0x38u64), (0x5352u16) as u16);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = rbx;
    rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64;
    rcx = (((rcx) ^ (rsp))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64;
    rsp = ((rsp).wrapping_add(0x80u64)) as u64;
    rbp = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ba9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b910; const P3: u64 = 0x1434278a0; const P4: u64 = 0x1414bb1c8; const P5: u64 = 0x14183acbc; const P6: u64 = 0x1426467f0; const P7: u64 = 0x1414baaec; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414ba9e0, funcinfo_rva: 0x36bae3c, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x54414349u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x4bu64), (0x45u16) as u16); crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x435f504cu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x49545245u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x41434946u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x23u64), (0x4554u16) as u16); crate::simd_ops::wu8(rbp.wrapping_add(0x25u64), (0x0u8) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x19u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414baa76); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414baa9c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414baae6); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414baaed); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bab14(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b910; const P3: u64 = 0x14266b9a0; const P4: u64 = 0x1434278a0; const P5: u64 = 0x1414bb1c8; const P6: u64 = 0x14183acbc; const P7: u64 = 0x1426467f0; const P8: u64 = 0x1414bac27; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414bab14, funcinfo_rva: 0x36bae94, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x54414349u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x54535f45u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x474e4952u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x23u64), (0x0u8) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x4e495254u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x4bu64), (0x47u16) as u16); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414babb1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414babd7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bac21); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bac28); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bac50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b9b0; const P3: u64 = 0x14266b920; const P4: u64 = 0x1434278a0; const P5: u64 = 0x1414bb1c8; const P6: u64 = 0x14183acbc; const P7: u64 = 0x1426467f0; const P8: u64 = 0x1414bad63; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414bac50, funcinfo_rva: 0x36bae94, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x5245435fu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x49464954u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x45544143u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x23u64), (0x0u8) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x54414349u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x4bu64), (0x45u16) as u16); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414baced); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bad13); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bad5d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bad64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bad8c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b9b0; const P3: u64 = 0x14266b990; const P4: u64 = 0x14266b920; const P5: u64 = 0x1434278a0; const P6: u64 = 0x1414bb1c8; const P7: u64 = 0x14183acbc; const P8: u64 = 0x1426467f0; const P9: u64 = 0x1414baeae; const P10: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414bad8c, funcinfo_rva: 0x36baf9c, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x474e49u32) as u32); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), (0x54414349u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xbu64), (0x54535f45u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), (0x474e4952u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x13u64), (0x0u8) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P5)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x39u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x17u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bae38); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bae5e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414baea8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414baeaf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; rsp = ((rsp).wrapping_add(0xc0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414baed4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b9b0; const P3: u64 = 0x1434278a0; const P4: u64 = 0x1414bb1c8; const P5: u64 = 0x14183acbc; const P6: u64 = 0x1426467f0; const P7: u64 = 0x1414bafda; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414baed4, funcinfo_rva: 0x36baeec, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xa8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x59454b5fu32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x4bu64), (0x0u8) as u8); crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x435f504cu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x4e45494cu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x454b5f54u32) as u32); crate::simd_ops::wu16(rbp.wrapping_add(0x23u64), (0x59u16) as u16); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x19u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x7u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414baf64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414baf8a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bafd4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P7, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bafdb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b9b0; const P3: u64 = 0x14266b9e0; const P4: u64 = 0x1434278a0; const P5: u64 = 0x1414bb1c8; const P6: u64 = 0x14183acbc; const P7: u64 = 0x1426467f0; const P8: u64 = 0x1414bb10d; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414bb000, funcinfo_rva: 0x36baf44, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rbp = ((rsp.wrapping_sub(0x57u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4fu64), (rax) as u64); rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x59454b5fu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1bu64), (0x5254535fu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x474e49u32) as u32); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x47u64), (0x474e4952u32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x4bu64), (0x0u8) as u8); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x0u8) as u8); r8 = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rdx = ((rbp.wrapping_add(0x27u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x9u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bb097); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x18u64), (0x0u64) as u64); r8 = (0x20u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bb0bd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            2 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), (0xfu64) as u64); crate::simd_ops::wu8(rbx, (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bb107); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P8, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414bb10e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rax = (rbx) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x4fu64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb134(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14266b9c0; const P2: u64 = 0x14266b9f0; const P3: u64 = 0x1414b9cf8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); xmm0 = (crate::simd_ops::ru128(P1)) as u128; rdx = rbp.wrapping_sub(0x50); xmm1 = (crate::simd_ops::ru128(P2)) as u128; rbx = rcx; r8 = rcx; crate::simd_ops::wu16(rbp.wrapping_sub(0x10u64), (0x54u16) as u16); rcx = rbp.wrapping_sub(0x30); crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), (0x545f504cu32) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), (0x4f454d49u32) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); crate::simd_ops::wu16(rbp.wrapping_sub(0x38u64), (0x5455u16) as u16); crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x36u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = rbx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx, (0x5f5e100u64) as u64); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x8u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141492e20; const P1: u64 = 0x140204cc0; const P2: u64 = 0x14183acda; const P3: u64 = 0x14307e970; const P4: u64 = 0x141833c0f;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (0xfffffffffffffffu64) as u64; r13 = rcx; let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rbx = 0x1; rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(((rcx as u32) & 63))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 28; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rax = rdx.wrapping_sub(0x1); rax = (((rax) | (rbx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); let bit_in: u64 = rax; if bit_in != 0 { rcx = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rsi = r13.wrapping_add(0x18); r8 = rbp; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_shl(((rcx as u32) & 63))) as u64; rcx = rsi; rdx = ((rbx.wrapping_add(rbx)) as u64) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r13.wrapping_add(0x38u64), rbx); rax = rbx.wrapping_sub(0x1); crate::simd_ops::wu64(r13.wrapping_add(0x30u64), rax); rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 2; continue; }
            2 => { rbx = rdi; bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; r14 = rbx.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; rcx = r14; r12 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(r14)) as u64; bb = 5; continue; }
            5 => { rdx = r12; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsi)) as u64; r15 = rax; r15 = (((r15) & (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r15 = ((r15).wrapping_add(r15)) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(r15.wrapping_mul(8)))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbp.wrapping_add(r15.wrapping_mul(8)), rbx); crate::simd_ops::wu64(rbp.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x8u64), rbx); bb = 26; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = rsi.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; rcx = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r14)) as u64; bb = 11; continue; }
            11 => { fl_l = r12; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { r8 = r12; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r8; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rdi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, r8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbp.wrapping_add(r15.wrapping_mul(8)).wrapping_add(0x8u64), rbx); bb = 25; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(r15.wrapping_mul(8)))) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdx = rsi.wrapping_add(0x10); fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; rcx = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(r14)) as u64; bb = 21; continue; }
            21 => { fl_l = r12; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 22; continue; }
            22 => { r8 = r12; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 23; continue; }
            23 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rdi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, r8); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); bb = 25; continue; }
            24 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rdx, rdi); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx, rsi); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rdx); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(r15.wrapping_mul(8)), rbx); bb = 25; continue; }
            25 => { rsi = r13.wrapping_add(0x18); bb = 26; continue; }
            26 => { rbx = rdi; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            28 => { rcx = P3; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414bb3fd, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1402038e0; const P3: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rsi = rcx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::lk_and_u64(rax, (0x0u64) as u64);
    rdi = (crate::simd_ops::ru64(rdx)) as u64;
    fl_l = rdi; fl_r = rdi;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0x30; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    loop { rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x30; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rbx; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rcx = (crate::simd_ops::ru64(rsi)) as u64;
    rdx = 0x30;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb464(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acd4; const P1: u64 = 0x1414b9c6c; const P2: u64 = 0x1401de590;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r15 = rdx; r14 = rcx; crate::simd_ops::wu8(rdx, (0x1u8) as u8); r13 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (((rbp) | (0xffffffffffffffffu64))) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = r13; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 2; continue; }
            2 => { r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(r14)) as u64; r8 = r13; r8 = ((r8).wrapping_sub(rdi)) as u64; rdx = (((crate::simd_ops::ru8(r14.wrapping_add(0x10u64))) as i8 as i32) as u32) as u32 as u64; rcx = ((rsi.wrapping_add(rdi)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_sub(rsi)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            6 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 8; continue; }
            7 => { rbx = r13.wrapping_sub(0x1); bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rsi = rbx; rcx = ((xmm0 as u64)) as u64; bb = 11; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdi))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdi).wrapping_add(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { bb = 15; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rsi))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rsi = ((rsi).wrapping_sub(1)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 30; continue; } bb = 16; continue; }
            16 => { rsi = ((rsi).wrapping_sub(rdi)) as u64; rsi = ((rsi).wrapping_add(1)) as u64; rax = ((rax).wrapping_sub(rdi)) as u64; fl_l = rax; fl_r = rsi; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rsi }) as u64; rdi = ((rdi).wrapping_add(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = (((r12 as u8)) as u32) as u32 as u64; rbx = ((rbx).wrapping_sub(rax)) as u64; rbx = ((rbx).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rbx); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x12u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu8(r15, (0x1u8) as u8); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu128(rax, (xmm0) as u128); rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rax)) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu128(rax, (xmm0) as u128); bb = 20; continue; }
            20 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 29; continue; }
            21 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rdx = (((crate::simd_ops::ru8(r14.wrapping_add(0x11u64))) as i8 as i32) as u32) as u32 as u64; r8 = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rdi)) as u64; fl_l = rcx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = rcx; fl_l = rsi; fl_r = rcx; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { rax }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu128(rax, (xmm0) as u128); rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 25; continue; }
            25 => { rsi = ((rsi).wrapping_sub(rcx)) as u64; rax = ((rdi.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); fl_l = rsi; fl_r = rbp; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { rsi } else { rbp }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu128(rax, (xmm0) as u128); bb = 27; continue; }
            26 => { crate::simd_ops::wu8(r15, (0x0u8) as u8); bb = 27; continue; }
            27 => { rcx = rbx.wrapping_add(0x2); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rcx); bb = 20; continue; }
            28 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            30 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414bb640, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb644(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1414bb6c4; const P2: u64 = 0x1414be04c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414bb644, funcinfo_rva: 0x35c2d9c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    rsp = rsp.wrapping_sub(8);
    core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    rbx = (rdx) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm1 as u64)) as u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(rax)) as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rdx = (0u32) as u32 as u64;
    fl_l = 0; fl_r = 0;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414bb66e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    xmm0 = (crate::simd_ops::ru128(rbx)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64;
    rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64;
    rcx = (rax) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x1414bb689); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rbx = core::ptr::read_unaligned(rsp as *const u64);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb698(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffd8; const P1: u64 = 0xffffffffffffffe8; const P2: u64 = 0x1414bb7fc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = rsp;
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(r8)) as u128;
    r8 = ((rax.wrapping_add(P0)) as u64) as u64;
    xmm1 = (crate::simd_ops::ru128(rdx)) as u128;
    rdx = ((rax.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu128(rax.wrapping_add(P0), (xmm0) as u128);
    crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm1) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb7fc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14310a214; const P2: u64 = 0x140571d70; const P3: u64 = 0x1402050d0; const P4: u64 = 0x1426467f0; const P5: u64 = 0x141834d30; const P6: u64 = 0x14129dff0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); r14 = r8; rsi = rdx; rdi = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = 0x1; rdx = P1; rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            2 => { crate::simd_ops::wu8(rax, (0x0u8) as u8); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x30); fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_r = (0x10u64) as u64; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64)) } else { rdx }) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = 0x1000; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rax = rcx; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414bb8b9, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { r8 = (r8 & !0xffu64) | (((0x3du8) as u8 as u64)); rdx = 0x1; rcx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x30); fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_r = (0x10u64) as u64; rdx = (if (((fl_l as u64)) >= ((fl_r as u64))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64)) } else { rdx }) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rax = rcx; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414bb93f, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414bb72c; const P1: u64 = 0x8; const P2: u64 = 0x1414bc230; const P3: u64 = 0x30; const P4: u64 = 0x1414bb644;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = r9; rbx = r8; rdi = rdx; rcx = ((rcx).wrapping_add(0x30u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P3), rbx); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bb9ec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414bb794; const P2: u64 = 0x1414bc230; const P3: u64 = 0x1414bc47c; const P4: u64 = 0x1434278a0; const P5: u64 = 0x1414bb698; const P6: u64 = 0x1414bbba0; const P7: u64 = 0x142646f28; const P8: u64 = 0x142646f20; const P9: u64 = 0x1426467f0; const P10: u64 = 0x141834d30; const P11: u64 = 0x1414bb644;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rax); rsi = r8; rdi = rdx; rcx = ((rcx).wrapping_add(0x48u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); r8 = 0x10; rdx = rdi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi.wrapping_add(0x10); r8 = 0x8; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x17u64), (0x0u8) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x49u64), (0x1u8) as u8); rax = rbp.wrapping_add(0x17); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rax = rbp.wrapping_sub(0x49); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rax = rbp.wrapping_sub(0x21); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); rax = P5; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x9u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x7); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx.wrapping_add(0x38); r9 = rax; r8 = rbp.wrapping_add(0x17); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2fu64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414bbb08, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(P4)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x17u64), (0x0u8) as u8); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rbx = ((((rbx as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = rbp.wrapping_sub(0x41); crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); rax = rbp.wrapping_sub(0x29); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rbp.wrapping_sub(0x39); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rcx; rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rdx)) as u64; r14 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx).wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); fl_l = (((rax) | (0xffffffffffffffffu64))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); fl_l = (((rax) | (0xffffffffffffffffu64))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x50); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(r14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdi = ((rdi).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 9; continue; }
            9 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 9; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbc60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0xc0; const P3: u64 = 0x140203100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    crate::simd_ops::lk_and_u64(rdx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), (0xfu64) as u64);
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((rdx).wrapping_add(0x40u64)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c866c; const P1: u64 = 0x14266dd70; const P2: u64 = 0xc0; const P3: u64 = 0x142646f28; const P4: u64 = 0x28; const P5: u64 = 0x142646f18; const P6: u64 = 0x30; const P7: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = P1; crate::simd_ops::wu64(rdi, rax); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; r9 = rax; r8 = rsi; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; r9 = rax; r8 = ((rbx.wrapping_add(P7)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbd28(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0xb8; const P3: u64 = 0x1414d6e40; const P4: u64 = 0x140203100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rdx, (xmm0) as u128);
    crate::simd_ops::lk_and_u64(rdx.wrapping_add(P0), (0x0u64) as u64);
    crate::simd_ops::wu64(rdx.wrapping_add(P1), (0xfu64) as u64);
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbd6c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c9470; const P1: u64 = 0x14266e0e0; const P2: u64 = 0xb8; const P3: u64 = 0x1414be0a4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbdb4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414bb6c4; const P1: u64 = 0x1414be04c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rbx)) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    r8 = rdi;
    rdx = rsp.wrapping_add(0x20);
    rcx = rax;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbdf4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x98;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((r8).wrapping_add(rdx)) as u64; fl_r = fl_l;
    r8 = ((fl_l as u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbe08(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414bc540; const P2: u64 = 0x1414bc3b8; const P3: u64 = 0x14266b488; const P4: u64 = 0x14183acda; const P5: u64 = 0x1414bc2f4; const P6: u64 = 0x1434278a0; const P7: u64 = 0x1414bb698; const P8: u64 = 0x1414bbba0; const P9: u64 = 0x142646f28; const P10: u64 = 0x142646f20; const P11: u64 = 0x1426467f0; const P12: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rax); rbx = rdx; rdi = rcx.wrapping_add(0x8); r8 = 0x10; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x10); rsi = 0x8; r8 = ((rsi as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rsi as u32)) as u32 as u64; rdx = P3; rcx = rbp.wrapping_add(0x77); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((rsi as u32)) as u32 as u64; rdx = rbp.wrapping_add(0x77); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(P6)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm1) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x17u64), (0x0u8) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), (0x1u8) as u8); rax = rbp.wrapping_add(0x17); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rax = rbp.wrapping_sub(0x29); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rax = rbp.wrapping_sub(0x21); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x9u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); rdx = rbp.wrapping_add(0x7); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x68); r9 = rax; r8 = rbp.wrapping_add(0x17); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2fu64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414bbf3b, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(P6)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_add(0x17u64), (0x0u8) as u8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rdi = ((((rdi as u32)) | (0xffffffffu32))) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x8u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0xcu64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = (((rax as u32)).wrapping_add((rdi as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x37u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbfac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414bc16c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rdx = (crate::simd_ops::ru64(rdx)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bbfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0xa0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = ((rdx.wrapping_add(P0)) as u32) as u32 as u64; bb = 10; continue; }
            6 => { rax = 0x4; bb = 10; continue; }
            7 => { rax = 0x3; bb = 10; continue; }
            8 => { rax = 0x2; bb = 10; continue; }
            9 => { rax = 0x1; bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bc018(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x80; const P2: u64 = 0x142646f28; const P3: u64 = 0x1414c13d0; const P4: u64 = 0x18; const P5: u64 = 0x1414bc0a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = r8; rdi = ((rdx as u32)) as u32 as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rdi as u32)) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bc874(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1414bc6d4; const P2: u64 = 0x1414bc7a4; const P3: u64 = 0x1414bc80c; const P4: u64 = 0x142646f28; const P5: u64 = 0x1414c36c8; const P6: u64 = 0x1414c3698; const P7: u64 = 0x1414bbca0; const P8: u64 = 0x1414c7e28; const P9: u64 = 0x1414c8a14; const P10: u64 = 0x1414c89e4; const P11: u64 = 0x1414c8864; const P12: u64 = 0x1414bbc60; const P13: u64 = 0x142646f18; const P14: u64 = 0x141834d30; const P15: u64 = 0x1414bbd6c; const P16: u64 = 0x1414c93a8; const P17: u64 = 0x1414c954c; const P18: u64 = 0x1414c951c; const P19: u64 = 0x1414c94bc; const P20: u64 = 0x1414bbd28; const P21: u64 = 0x1414c33b4; const P22: u64 = 0x1426467f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rax); rsi = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 2; continue; }
            2 => { rax = rdx.wrapping_add(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); bb = 3; continue; }
            3 => { r15 = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::lk_and_u64(rsi, (0x0u64) as u64); rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); rcx = rax.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r13 = rax; r12 = r15.wrapping_add(0x8); rcx = rax.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rcx = r12; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rdi; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = r12; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            6 => { rdx = r12; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdx = rbp.wrapping_sub(0x29); rcx = r15; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r14 = rax; rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = r13; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdi = rax; crate::simd_ops::wu64(r13.wrapping_add(0x30u64), rax); bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            12 => { rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = rbp.wrapping_sub(0x29); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_sub(0x29); rcx = r15; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = r13; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rax; r8 = rbx; rdx = 0; fl_l = 0; fl_r = 0; rcx = r13.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 31; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rdx = rbp.wrapping_sub(0x29); rcx = r15; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); r14 = rax; r13 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x30u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = r13; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rdi = rax; crate::simd_ops::wu64(r13.wrapping_add(0x30u64), rax); bb = 19; continue; }
            19 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P17, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            22 => { rdx = r14; rcx = rdi; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rcx = rbp.wrapping_sub(0x29); rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); rdx = rbp.wrapping_sub(0x29); rcx = r15; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rcx = r13; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = r13.wrapping_add(0x28); r9 = rax; r8 = rbx; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; fl_l = rdx; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x29u64))) as u64; rax = rcx; fl_l = rdx; fl_r = (0x1000u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = rax; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 32; continue; } bb = 26; continue; }
            26 => { rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = r12; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); rdx = 0xc8; rcx = r15; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rsi = ((rsi).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x39u64))) as u64; bb = 3; continue; }
            31 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P22), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x1414bcb13, rcx, rsp); return rax; }
            32 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P22), rax), rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x17u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 34; continue; }
            34 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bcd54(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x1414bd370; const P2: u64 = 0x1414bd368; const P3: u64 = 0x1414bcd4c; const P4: u64 = 0x8; const P5: u64 = 0x14266ba28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::lk_and_u64(rcx.wrapping_add(P0), (0x0u64) as u64);
    rdi = P1;
    rbx = rcx;
    rcx = rsp.wrapping_add(0x40);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rdi = P2; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); rax = P5; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rbx); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bcdac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414bcc14; const P1: u64 = 0x1414d6160; const P2: u64 = 0x1414bb6c4; const P3: u64 = 0x1414bdff4; const P4: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r13 = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); r14 = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 1; continue; }
            1 => { rax = rdx.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); rbp = r14.wrapping_add(0x18); bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu64(rax.wrapping_add(0x40u64), r13); crate::simd_ops::wu64(rax.wrapping_add(0x48u64), rbx); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 3; continue; }
            3 => { xmm1 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x64u64), (0x6u32) as u32); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rax); bb = 11; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x64u64), (0x4u32) as u32); bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), ((xmm0 as u64)) as u64); bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(r14)) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 22; continue; }
            12 => { rax = rbx.wrapping_add(0x20); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = rbx.wrapping_add(0x40); rdx = rsp.wrapping_add(0x30); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 18; continue; }
            16 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 19; continue; }
            19 => { rbx = rax; bb = 22; continue; }
            20 => { rbx = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 22; continue; }
            22 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 23; continue; }
            23 => { rdi = 0x90; r14 = ((r14).wrapping_add(rdi)) as u64; rbp = ((rbp).wrapping_add(rdi)) as u64; fl_l = r14; fl_r = r15; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414bcf51, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bcf54(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414bd3c8; const P1: u64 = 0x1414bcb44; const P2: u64 = 0x1414aa508; const P3: u64 = 0x14183ae54; const P4: u64 = 0x14183ae4e; const P5: u64 = 0x1414bb6c4; const P6: u64 = 0x1414bdff4; const P7: u64 = 0x1414ab11c; const P8: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rdi = rdx; rbx = rcx; rcx = ((rcx).wrapping_add(0x68u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); r15 = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), r15); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rax); r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; r12 = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 1; continue; }
            1 => { r13 = rdi.wrapping_add(0x10); bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu64(rax.wrapping_add(0x78u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), rax); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x88u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 3; continue; }
            3 => { rdx = r14.wrapping_add(0x18); rcx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_sub(0x39u64))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as i64) as f64).to_bits() as u128); bb = 7; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x41u64))) as u64) as u128; bb = 7; continue; }
            7 => { rdx = rsi.wrapping_add(0x10); rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; rcx = ((((rcx as u32)) | (0x1u32))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x90u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), rax); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xfu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_sub(0x29u64))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as i64) as f64).to_bits() as u128); bb = 12; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 11; continue; }
            11 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64) as u128; bb = 12; continue; }
            12 => { rax = rdx; rcx = ((((rcx as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x98u64), ((xmm0 as u64)) as u64); rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rbp.wrapping_sub(0x19u64))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as i64) as f64).to_bits() as u128); bb = 16; continue; }
            14 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 15; continue; }
            15 => { xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64) as u128; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::lk_or_u32(rdx, (0x4u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rsi.wrapping_add(0xa0u64), ((xmm0 as u64)) as u64); bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; bb = 19; continue; }
            18 => { xmm0 = ((crate::simd_ops::ru64(rbx)) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), ((xmm0 as u64)) as u64); rdx = rbp.wrapping_add(0x6f); rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 20; continue; }
            20 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; bb = 22; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6fu64), rax); rdx = rbp.wrapping_add(0x6f); rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(r14)) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 34; continue; }
            24 => { rax = rbx.wrapping_add(0x20); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x69u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm0) as u128); r8 = rbx.wrapping_add(0x40); rdx = rbp.wrapping_add(0x17); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 32; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 30; continue; }
            28 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 31; continue; }
            31 => { rbx = rax; bb = 34; continue; }
            32 => { rbx = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 34; continue; }
            34 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 35; continue; }
            35 => { rcx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((r14).wrapping_add(0x90u64)) as u64; fl_l = r14; fl_r = r12; r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 36; continue; }
            36 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            37 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414bd1a1, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bd1a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414bd3c8; const P1: u64 = 0x1414bcc14; const P2: u64 = 0x1414d6160; const P3: u64 = 0x1414bb6c4; const P4: u64 = 0x1414bdff4; const P5: u64 = 0x140200070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = rdx; rbx = rcx; rcx = ((rcx).wrapping_add(0x68u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x2cu64), (0x1u8) as u8); r12 = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r12); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rax); r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; r15 = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 1; continue; }
            1 => { r13 = rdi.wrapping_add(0x10); rbp = r14.wrapping_add(0x18); bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu64(rax.wrapping_add(0x40u64), r12); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); rdx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x70u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 3; continue; }
            3 => { xmm1 = (crate::simd_ops::ru128(rbp)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x64u64), (0x6u32) as u32); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rax); bb = 11; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x64u64), (0x4u32) as u32); bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), ((xmm0 as u64)) as u64); bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(r14)) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 12; continue; }
            12 => { rax = rbx.wrapping_add(0x20); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = rbx.wrapping_add(0x40); rdx = rsp.wrapping_add(0x30); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; bb = 18; continue; }
            16 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 19; continue; }
            19 => { rbx = rax; bb = 22; continue; }
            20 => { rbx = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rbx = rcx; rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 22; continue; }
            22 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 23; continue; }
            23 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 24; continue; }
            24 => { rbx = 0x90; r14 = ((r14).wrapping_add(rbx)) as u64; rbp = ((rbp).wrapping_add(rbx)) as u64; fl_l = r14; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x1414bd366, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bd370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            7 => { rax = 0x2; return rax; }
            8 => { rax = 0x1; return rax; }
            9 => { rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = 0x2d; fl_l = ((((rax as u32)) >> ((((rdx as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = 0x3; return rax; }
            12 => { rax = 0x1; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4; rcx = ((rax.wrapping_add(P0)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 14; continue; }
            14 => { return rax; }
            15 => { rax = 0x2; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x1; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); bb = 17; continue; }
            17 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bd39c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = 0x2d; fl_l = ((((rax as u32)) >> ((((rdx as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0x3; return rax; }
            3 => { rax = 0x1; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4; rcx = ((rax.wrapping_add(P0)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bd3c8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0x2;
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x1; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bd3dc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646f28; const P1: u64 = 0x142646f18; const P2: u64 = 0x1414bd39c; const P3: u64 = 0x1414d6050; const P4: u64 = 0x1414d2294; const P5: u64 = 0x1414bd1a4; const P6: u64 = 0x1414d1f30; const P7: u64 = 0x1414bcdac; const P8: u64 = 0x1414d1f9c; const P9: u64 = 0x1414bcf54;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; rcx = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); r9 = rax; r8 = rsi; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r8 = rsi.wrapping_add(0x20); rcx = rbx.wrapping_add(0x18); r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r8 = rsi.wrapping_add(0x40); rcx = rbx.wrapping_add(0x20); r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), (0x7u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), (0x5u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x34u64), (0x9u32) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bd518(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1414bcc7c; const P2: u64 = 0x1414bd548;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = ((rdx.wrapping_add(P0)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bd548(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646f28; const P1: u64 = 0x1414c93a8; const P2: u64 = 0x1414be0a4; const P3: u64 = 0x1414bcce4; const P4: u64 = 0x1414c7e28; const P5: u64 = 0x142646f18; const P6: u64 = 0x1414bcbac; const P7: u64 = 0x1414bd3dc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rsi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); bb = 2; continue; }
            2 => { rdx = rbx; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; bb = 10; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = rbp.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rax); bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x28); r9 = rax; r8 = rbx; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x30); r9 = rax; r8 = rbx.wrapping_add(0x20); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; bb = 8; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = r14.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx).wrapping_add(0x98u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { fl_l = ((rsi).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { fl_l = rsi; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bd680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bd704(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c8ef4; const P1: u64 = 0x142646f50; const P2: u64 = 0x142646f48; const P3: u64 = 0x142646f28; const P4: u64 = 0x142646f18; const P5: u64 = 0x1414b1ba8; const P6: u64 = 0x1414c7dbc; const P7: u64 = 0x1414bd69c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 88; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rdi = rdx; rbx = rcx; rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rdx.wrapping_add(0x20u64))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdi = (rdi & !0xffu64) | (((crate::simd_ops::ru8(rdx)) as u8 as u64)); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x2u32) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), ((rdi as u8)) as u8); bb = 87; continue; }
            5 => { r15 = 0x1; rsi = ((r15.wrapping_add(0x2u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; bb = 12; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru32(rdx)) as u32 as u64; bb = 12; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rsi as u32)) as u32); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); bb = 87; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 16; continue; }
            16 => { xmm6 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x4u32) as u32); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm6 as u64)) as u64); bb = 87; continue; }
            19 => { r12 = 0x5; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; r8 = rdi; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 87; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 24; continue; }
            24 => { rbp = (crate::simd_ops::ru64(rdx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            25 => { fl_l = rbp; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = rsi; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 87; continue; } bb = 27; continue; }
            27 => { rcx = rsi; rax = ((r15 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r14 = (r14 & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), (0x2u32) as u32); bb = 31; continue; }
            31 => { crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((r14 as u8)) as u8); fl_l = rsi; fl_r = (0x1fu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { fl_l = ((rsi).wrapping_add(r15)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 25; continue; }
            33 => { rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 25; continue; }
            34 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 43; continue; } bb = 35; continue; }
            35 => { r14 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; bb = 41; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r15 = (((crate::simd_ops::ru32(rdi)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rsi as u32)) as u32); bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r15); fl_l = ((rdi).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 41; continue; }
            41 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 42; continue; }
            42 => { bb = 87; continue; }
            43 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 52; continue; } bb = 44; continue; }
            44 => { r14 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; bb = 50; continue; }
            45 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r15 = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rsi as u32)) as u32); bb = 49; continue; }
            49 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r15); fl_l = ((rdi).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 50; continue; }
            50 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 51; continue; }
            51 => { bb = 87; continue; }
            52 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 61; continue; } bb = 53; continue; }
            53 => { r14 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; bb = 59; continue; }
            54 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 56; continue; } bb = 55; continue; }
            55 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 56; continue; }
            56 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r15 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rsi as u32)) as u32); bb = 58; continue; }
            58 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r15); fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 59; continue; }
            59 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 54; continue; } bb = 60; continue; }
            60 => { bb = 87; continue; }
            61 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 70; continue; } bb = 62; continue; }
            62 => { r14 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; bb = 68; continue; }
            63 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 65; continue; } bb = 64; continue; }
            64 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r15 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 67; continue; } bb = 66; continue; }
            66 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rsi as u32)) as u32); bb = 67; continue; }
            67 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r15); fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 68; continue; }
            68 => { fl_l = rdi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 63; continue; } bb = 69; continue; }
            69 => { bb = 87; continue; }
            70 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 79; continue; } bb = 71; continue; }
            71 => { rbp = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; bb = 77; continue; }
            72 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 74; continue; }
            74 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; xmm6 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 76; continue; } bb = 75; continue; }
            75 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), (0x4u32) as u32); bb = 76; continue; }
            76 => { crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), ((xmm6 as u64)) as u64); fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 77; continue; }
            77 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 72; continue; } bb = 78; continue; }
            78 => { bb = 87; continue; }
            79 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 87; continue; } bb = 80; continue; }
            80 => { rbp = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; bb = 86; continue; }
            81 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 83; continue; } bb = 82; continue; }
            82 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 83; continue; }
            83 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 85; continue; } bb = 84; continue; }
            84 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 85; continue; }
            85 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; r8 = rdi; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rdi).wrapping_add(0x20u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 86; continue; }
            86 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 81; continue; } bb = 87; continue; }
            87 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 88; continue; }
            88 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bdb84(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c8ef4; const P1: u64 = 0x1414be138; const P2: u64 = 0x1414be230; const P3: u64 = 0x142646f28; const P4: u64 = 0x1414c7dbc; const P5: u64 = 0x1414bd69c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 93; continue; } bb = 1; continue; }
            1 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rbx = rcx; rax = (rax & !0xffu64) | (((crate::simd_ops::ru8(rdx.wrapping_add(0x10u64))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdi = (rdi & !0xffu64) | (((crate::simd_ops::ru8(rdx)) as u8 as u64)); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x2u32) as u32); bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), ((rdi as u8)) as u8); bb = 92; continue; }
            5 => { rdi = 0x3; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rsi = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; bb = 12; continue; }
            7 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru32(rdx)) as u32 as u64; bb = 12; continue; }
            10 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xdu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rdi as u32)) as u32); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rsi); bb = 92; continue; }
            15 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 16; continue; }
            16 => { xmm6 = ((crate::simd_ops::ru64(rdx)) as u64) as u128; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x4u32) as u32); bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm6 as u64)) as u64); bb = 92; continue; }
            19 => { r12 = 0x5; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 92; continue; }
            21 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 92; continue; }
            23 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xfu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; r14 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((r14).wrapping_add(rsi)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 30; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r15 = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rdi as u32)) as u32); bb = 29; continue; }
            29 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r15); fl_l = ((rsi).wrapping_add(1)) as u64; rsi = ((fl_l as u64)) as u64; bb = 30; continue; }
            30 => { fl_l = rsi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 31; continue; }
            31 => { bb = 92; continue; }
            32 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 41; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = ((rbp).wrapping_add(rdi)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 39; continue; }
            34 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; r14 = (r14 & !0xffu64) | (((crate::simd_ops::ru8(rdi)) as u8 as u64)); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), (0x2u32) as u32); bb = 38; continue; }
            38 => { crate::simd_ops::wu8(rsi.wrapping_add(0x10u64), ((r14 as u8)) as u8); fl_l = ((rdi).wrapping_add(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 39; continue; }
            39 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 40; continue; }
            40 => { bb = 92; continue; }
            41 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 50; continue; } bb = 42; continue; }
            42 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r14 = ((rsi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; bb = 48; continue; }
            43 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 45; continue; }
            45 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r15 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rdi as u32)) as u32); bb = 47; continue; }
            47 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r15); fl_l = ((rsi).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 48; continue; }
            48 => { fl_l = rsi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 43; continue; } bb = 49; continue; }
            49 => { bb = 92; continue; }
            50 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 59; continue; } bb = 51; continue; }
            51 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r14 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 57; continue; }
            52 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r15 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 56; continue; } bb = 55; continue; }
            55 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rdi as u32)) as u32); bb = 56; continue; }
            56 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r15); fl_l = ((rsi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 57; continue; }
            57 => { fl_l = rsi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 52; continue; } bb = 58; continue; }
            58 => { bb = 92; continue; }
            59 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 68; continue; } bb = 60; continue; }
            60 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r14 = ((rsi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; bb = 66; continue; }
            61 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 63; continue; } bb = 62; continue; }
            62 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 63; continue; }
            63 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r15 = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 65; continue; } bb = 64; continue; }
            64 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rdi as u32)) as u32); bb = 65; continue; }
            65 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r15); fl_l = ((rsi).wrapping_add(0x4u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 66; continue; }
            66 => { fl_l = rsi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 61; continue; } bb = 67; continue; }
            67 => { bb = 92; continue; }
            68 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xeu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 77; continue; } bb = 69; continue; }
            69 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r14 = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 75; continue; }
            70 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 72; continue; } bb = 71; continue; }
            71 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 72; continue; }
            72 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; r15 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((rdi as u32)) as u32); bb = 74; continue; }
            74 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r15); fl_l = ((rsi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 75; continue; }
            75 => { fl_l = rsi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 70; continue; } bb = 76; continue; }
            76 => { bb = 92; continue; }
            77 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xbu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 86; continue; } bb = 78; continue; }
            78 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbp = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 84; continue; }
            79 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 81; continue; } bb = 80; continue; }
            80 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 81; continue; }
            81 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; xmm6 = ((crate::simd_ops::ru64(rdi)) as u64) as u128; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 83; continue; } bb = 82; continue; }
            82 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), (0x4u32) as u32); bb = 83; continue; }
            83 => { crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), ((xmm6 as u64)) as u64); fl_l = ((rdi).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 84; continue; }
            84 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 79; continue; } bb = 85; continue; }
            85 => { bb = 92; continue; }
            86 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcu8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 92; continue; } bb = 87; continue; }
            87 => { rsi = (crate::simd_ops::ru64(rdx)) as u64; rdi = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; fl_l = ((rdi).wrapping_add(rsi)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 91; continue; }
            88 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 90; continue; } bb = 89; continue; }
            89 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((r12 as u32)) as u32); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 90; continue; }
            90 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rsi).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 91; continue; }
            91 => { fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 88; continue; } bb = 92; continue; }
            92 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 93; continue; }
            93 => { return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414be0a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414b1850; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x28; const P4: u64 = 0x20; const P5: u64 = 0x1414bb6c4; const P6: u64 = 0x30; const P7: u64 = 0x1414bdff4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rcx; rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 5; continue; }
            2 => { rax = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = ((rbx.wrapping_add(P6)) as u64) as u64; rdx = rsp.wrapping_add(0x30); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); bb = 7; continue; }
            7 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414be138(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x10; const P2: u64 = 0x1c; const P3: u64 = 0x1414c8ef4; const P4: u64 = 0x142646f50; const P5: u64 = 0x142646f48; const P6: u64 = 0x142646f28; const P7: u64 = 0x1402050d0; const P8: u64 = 0x142646f20; const P9: u64 = 0xfffffffffffffff8; const P10: u64 = 0x1426467f0; const P11: u64 = 0x1414be20b; const P12: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414be138, funcinfo_rva: 0x36bb3b0, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); rdi = (rdx) as u64; rbx = (rcx) as u64; rsi = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be166); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x1u32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be172); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be17e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be187); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm1) as u128); fl_l = (((r8) | (0xffffffffffffffffu64))) as u64 as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(r8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = (rdi) as u64; rcx = ((rsp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be1b7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (rbx) as u64; r8 = ((rsp.wrapping_add(0x28u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be1ca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be205); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be20c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414be230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x10; const P2: u64 = 0x1c; const P3: u64 = 0x1414c8ef4; const P4: u64 = 0x142646f50; const P5: u64 = 0x142646f48; const P6: u64 = 0x142646f28; const P7: u64 = 0x1402050d0; const P8: u64 = 0x142646f20; const P9: u64 = 0xfffffffffffffff8; const P10: u64 = 0x1426467f0; const P11: u64 = 0x1414be2f8; const P12: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414be230, funcinfo_rva: 0x36bb3e4, handler: 2, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); rbp = (r8) as u64; rsi = (rdx) as u64; rbx = (rcx) as u64; rdi = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be25e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x1u32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be26a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rax) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be276); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 2; continue; }
            2 => { rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be27f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); r8 = (rbp) as u64; rdx = (rsi) as u64; rcx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be2a4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (rbx) as u64; r8 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be2b7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (rcx) as u64; fl_l = (rdx) as u64 as u64; fl_r = (0x1000u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(0x27u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (0x1fu64) as u64 as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be2f2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::lift_ops::trap(0x80000003, P11, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414be2f9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414be3d4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646f58; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P1), (0x0u32) as u32);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414be400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x10; const P2: u64 = 0x142647038; const P3: u64 = 0x1; const P4: u64 = 0x18; const P5: u64 = 0x142647258; const P6: u64 = 0x8; const P7: u64 = 0x1c; const P8: u64 = 0x142647118; const P9: u64 = 0x1414be73c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax.wrapping_add(P3)) as u64) as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(P6)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rdi.wrapping_add(P7)) as u64) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            6 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414be484(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c2a54; const P2: u64 = 0x142646f28; const P3: u64 = 0x142646fd0; const P4: u64 = 0x142647090; const P5: u64 = 0x14266c9f0; const P6: u64 = 0x142647088; const P7: u64 = 0x1414c2ca4; const P8: u64 = 0x1426470d8; const P9: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 24; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 28; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 16; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x18u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 16; continue; } bb = 9; continue; }
            9 => { rdx = rbp.wrapping_add(0x38); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 28; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x18u64), ((rax as u32)) as u32); fl_k = 4; bb = 23; continue; }
            11 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P5; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 28; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 22; continue; }
            16 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 27; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 27; continue; } bb = 18; continue; }
            18 => { rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 21; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; fl_k = 4; bb = 22; continue; }
            22 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 28; continue; } bb = 23; continue; }
            23 => { rcx = rsi; bb = 24; continue; }
            24 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 26; continue; }
            26 => { r11 = rsp.wrapping_add(0x50); rax = rbx; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 26; continue; }
            28 => { rbx = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 26; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414be634(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x14266c9f0; const P2: u64 = 0x142647000; const P3: u64 = 0x142646f98; const P4: u64 = 0x142646fa8; const P5: u64 = 0x142647020; const P6: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx.wrapping_add(0x10); rdi = rcx; rcx = rbp; rsi = r8; rbx = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { r9 = P1; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x2; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = rbx; rcx = 0x3; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bf71c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c0fb8; const P1: u64 = 0x28; const P2: u64 = 0x142646f58; const P3: u64 = 0x30; const P4: u64 = 0xc; const P5: u64 = 0x14183acc2; const P6: u64 = 0x8; const P7: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bf75c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x1414c7364; const P4: u64 = 0x142647048; const P5: u64 = 0x28; const P6: u64 = 0x142646fc8; const P7: u64 = 0x10; const P8: u64 = 0x142647038; const P9: u64 = 0x30; const P10: u64 = 0x38; const P11: u64 = 0x142647028; const P12: u64 = 0x142647118; const P13: u64 = 0x1414bfbdc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P9))) as u64; fl_r = (0x0u64) as u64; rbx = rbp.wrapping_add(0x9); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P10))) as u32 as u64; rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rbx }) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = rsi.wrapping_add(0x3c); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bf834(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c29fc; const P2: u64 = 0x1414bb6c4; const P3: u64 = 0x1414c247c; const P4: u64 = 0x1414c1b8c; const P5: u64 = 0x142646f28; const P6: u64 = 0x142646fd0; const P7: u64 = 0x142647090; const P8: u64 = 0x14266c930; const P9: u64 = 0x142647088; const P10: u64 = 0x1414c29bc; const P11: u64 = 0x1414c2ca4; const P12: u64 = 0x1426470d8; const P13: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 34; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 38; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 25; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 25; continue; } bb = 11; continue; }
            11 => { rdx = rbp.wrapping_add(0x38); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x38u64), ((rcx as u32)) as u32); bb = 31; continue; }
            12 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 25; continue; } bb = 13; continue; }
            13 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; fl_k = 6; bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = r14.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rax; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 38; continue; } bb = 15; continue; }
            15 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 33; continue; } bb = 16; continue; }
            16 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 33; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 14; continue; }
            18 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 25; continue; } bb = 19; continue; }
            19 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x28); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P8; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 38; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 32; continue; }
            23 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x8u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rax); fl_k = 6; bb = 33; continue; }
            25 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 37; continue; } bb = 26; continue; }
            26 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 37; continue; } bb = 27; continue; }
            27 => { rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 30; continue; }
            29 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_k = 4; bb = 32; continue; }
            32 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 38; continue; } bb = 33; continue; }
            33 => { rcx = rsi; bb = 34; continue; }
            34 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 36; continue; }
            36 => { r11 = rsp.wrapping_add(0x50); rax = rbx; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            37 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 36; continue; }
            38 => { rbx = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 36; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bfa60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fa8; const P1: u64 = 0x142647018; const P2: u64 = 0x142646fc8; const P3: u64 = 0x14266c930; const P4: u64 = 0x142647000; const P5: u64 = 0x142646f98; const P6: u64 = 0x1414c1bb4; const P7: u64 = 0x1414c1c1c; const P8: u64 = 0x142647010; const P9: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; rsi = r8; rbx = rdx; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r8; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; r8 = rbx; rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rbp = rdi.wrapping_add(0x28); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { r9 = P3; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x2; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 11; continue; }
            11 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x3; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; r8 = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bfccc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c0fb8; const P1: u64 = 0x28; const P2: u64 = 0x142646f58; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0x8; const P7: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P5), (0x0u32) as u32);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bfd14(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c7364; const P1: u64 = 0x142647048; const P2: u64 = 0x142646fc8; const P3: u64 = 0x142647040; const P4: u64 = 0x142647038; const P5: u64 = 0x142647028; const P6: u64 = 0x142647118; const P7: u64 = 0x1414c023c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rbp = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rsi as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = rbp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { rcx = rbp.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = rbp.wrapping_add(0x44); rdx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { rdx = ((rsi as u32)) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = rsi; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
