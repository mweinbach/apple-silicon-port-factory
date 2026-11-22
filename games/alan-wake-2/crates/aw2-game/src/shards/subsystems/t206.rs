#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bfe30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c29fc; const P2: u64 = 0x1414bb6c4; const P3: u64 = 0x1414c247c; const P4: u64 = 0x1414c1b94; const P5: u64 = 0x142646f28; const P6: u64 = 0x142646fd0; const P7: u64 = 0x142647090; const P8: u64 = 0x14266c960; const P9: u64 = 0x142647088; const P10: u64 = 0x1414c2ca4; const P11: u64 = 0x1426470d8; const P12: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 38; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rbx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdi = rdx.wrapping_add(0x1); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 42; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 29; continue; } bb = 11; continue; }
            11 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x28u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 29; continue; } bb = 12; continue; }
            12 => { rdx = rbp.wrapping_add(0x38); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x40u64), ((rcx as u32)) as u32); bb = 35; continue; }
            13 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x22u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 29; continue; } bb = 14; continue; }
            14 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; fl_k = 6; bb = 15; continue; }
            15 => { rdi = ((rdi).wrapping_add(1)) as u64; rcx = r14.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rax; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 42; continue; } bb = 16; continue; }
            16 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 37; continue; } bb = 17; continue; }
            17 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 37; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 15; continue; }
            19 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 29; continue; } bb = 20; continue; }
            20 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x38); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P8; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 42; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 36; continue; }
            24 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 29; continue; } bb = 25; continue; }
            25 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x30); bb = 26; continue; }
            26 => { r8 = rax; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            27 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x28); bb = 26; continue; }
            29 => { rax = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 41; continue; } bb = 30; continue; }
            30 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 41; continue; } bb = 31; continue; }
            31 => { rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 34; continue; }
            33 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            34 => { rcx = ((rbx as u32)) as u32 as u64; r9 = rsi; r8 = rdi; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_k = 4; bb = 36; continue; }
            36 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 42; continue; } bb = 37; continue; }
            37 => { rcx = rsi; bb = 38; continue; }
            38 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 39; continue; }
            39 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 40; continue; }
            40 => { r11 = rsp.wrapping_add(0x50); rax = rdi; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            41 => { rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 40; continue; }
            42 => { rdi = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 40; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c008c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x142646fa0; const P2: u64 = 0x14266c960; const P3: u64 = 0x142647000; const P4: u64 = 0x142646f98; const P5: u64 = 0x142646fa8; const P6: u64 = 0x1414c1bb4; const P7: u64 = 0x1414c1c1c; const P8: u64 = 0x142647010; const P9: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = r8; rcx = ((rcx).wrapping_add(0x28u64)) as u64; rbx = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x1; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 2; continue; }
            2 => { rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x2; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rbp = rdi.wrapping_add(0x38); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { r9 = P2; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x3; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 11; continue; }
            11 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x4; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; r8 = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c032c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c0fb8; const P1: u64 = 0x28; const P2: u64 = 0x1414c1108; const P3: u64 = 0x40; const P4: u64 = 0x1414c115c; const P5: u64 = 0x58; const P6: u64 = 0x142646f58; const P7: u64 = 0x60; const P8: u64 = 0x68; const P9: u64 = 0x70; const P10: u64 = 0x78; const P11: u64 = 0x142646f28; const P12: u64 = 0x1414c3534; const P13: u64 = 0x141834d30; const P14: u64 = 0x14183acc2; const P15: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = 0x20; rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x80u64), (0x0u64) as u64); rcx = rbx.wrapping_add(0x88); rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(0x20u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c03e8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c7364; const P1: u64 = 0x142647048; const P2: u64 = 0x1414bf75c; const P3: u64 = 0x1414bfd14; const P4: u64 = 0x142646fc8; const P5: u64 = 0x142647040; const P6: u64 = 0x142647038; const P7: u64 = 0x14392c0d0; const P8: u64 = 0x1414be400; const P9: u64 = 0x142647258; const P10: u64 = 0x142647028; const P11: u64 = 0x142647118; const P12: u64 = 0x1414c0f7c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rcx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rcx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rcx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rcx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 8; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 9; continue; }
            9 => { rcx = rsi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rsi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { rcx = rsi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rsi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { rcx = rsi.wrapping_add(0x68); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rsi.wrapping_add(0x68); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 15; continue; }
            15 => { rcx = rsi.wrapping_add(0x70); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rsi.wrapping_add(0x70); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { rcx = rsi.wrapping_add(0x78); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = rsi.wrapping_add(0x78); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { rax = P7; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; fl_r = rdi; rax = rbp.wrapping_add(0x9); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x98u64))) as u32 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rax }) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64; fl_r = rdi; rbx = rax.wrapping_add(0x9); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rax } else { rbx }) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x9cu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa4u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 30; continue; }
            30 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { r8 = rsi.wrapping_add(0xa8); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            32 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c0668(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c29bc; const P2: u64 = 0x1414c2a54; const P3: u64 = 0x142646f28; const P4: u64 = 0x142646fd0; const P5: u64 = 0x142647090; const P6: u64 = 0x14266c9c8; const P7: u64 = 0x142647088; const P8: u64 = 0x14266c998; const P9: u64 = 0x1414c13d0; const P10: u64 = 0x1414c2914; const P11: u64 = 0x1414c29fc; const P12: u64 = 0x1414bb794; const P13: u64 = 0x1414c286c; const P14: u64 = 0x1414c1bac; const P15: u64 = 0x1414bb72c; const P16: u64 = 0x1414c27c4; const P17: u64 = 0x1414c1ba4; const P18: u64 = 0x1414bb6c4; const P19: u64 = 0x1414c247c; const P20: u64 = 0x1414c1b9c; const P21: u64 = 0x1414c2ca4; const P22: u64 = 0x1426470d8; const P23: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = rcx; r14 = r8; rcx = r8; bb = 84; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rbx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdi = rdx.wrapping_add(0x1); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 88; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 40; continue; } bb = 7; continue; }
            7 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 75; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 16; continue; }
            16 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), rax); bb = 39; continue; }
            17 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 18; continue; }
            18 => { rdx = rbp.wrapping_sub(0x48); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 88; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x98u64), ((rax as u32)) as u32); fl_k = 4; bb = 83; continue; }
            20 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x2au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 21; continue; }
            21 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x78); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rdx = P6; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; bb = 24; continue; }
            24 => { rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 88; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 82; continue; }
            26 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x22u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 27; continue; }
            27 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x70); bb = 28; continue; }
            28 => { r8 = rax; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 81; continue; }
            29 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 30; continue; }
            30 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x68); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P8; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; bb = 24; continue; }
            33 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 34; continue; }
            34 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x60); bb = 28; continue; }
            35 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 36; continue; }
            36 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x58); bb = 28; continue; }
            37 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x41u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 38; continue; }
            38 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x90u64), rax); bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x8u64) as u64)) as u64; fl_r = fl_l; fl_k = 6; bb = 83; continue; }
            40 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x9; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 69; continue; } bb = 41; continue; }
            41 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 67; continue; } bb = 42; continue; }
            42 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 61; continue; } bb = 43; continue; }
            43 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 59; continue; } bb = 44; continue; }
            44 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 53; continue; } bb = 45; continue; }
            45 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 46; continue; }
            46 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 75; continue; } bb = 47; continue; }
            47 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x7au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), rax); bb = 50; continue; }
            50 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; r8 = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 81; continue; }
            51 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x70u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 52; continue; }
            52 => { rdx = rbp.wrapping_add(0x38); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xa4u64), ((rcx as u32)) as u32); bb = 81; continue; }
            53 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x6au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 54; continue; }
            54 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; fl_k = 6; bb = 55; continue; }
            55 => { rdi = ((rdi).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x40); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 88; continue; } bb = 56; continue; }
            56 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 83; continue; } bb = 57; continue; }
            57 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 83; continue; } bb = 58; continue; }
            58 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 55; continue; }
            59 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x60u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 60; continue; }
            60 => { rdx = rbp.wrapping_sub(0x50); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xa0u64), ((rcx as u32)) as u32); bb = 81; continue; }
            61 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x5au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 62; continue; }
            62 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; fl_k = 6; bb = 63; continue; }
            63 => { rdi = ((rdi).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x28); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 88; continue; } bb = 64; continue; }
            64 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 83; continue; } bb = 65; continue; }
            65 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 83; continue; } bb = 66; continue; }
            66 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 63; continue; }
            67 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x50u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 68; continue; }
            68 => { rdx = rbp.wrapping_sub(0x4c); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4cu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x9cu64), ((rcx as u32)) as u32); bb = 81; continue; }
            69 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 75; continue; } bb = 70; continue; }
            70 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; fl_k = 6; bb = 71; continue; }
            71 => { rdi = ((rdi).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 88; continue; } bb = 72; continue; }
            72 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 83; continue; } bb = 73; continue; }
            73 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 83; continue; } bb = 74; continue; }
            74 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 71; continue; }
            75 => { rax = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 87; continue; } bb = 76; continue; }
            76 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 87; continue; } bb = 77; continue; }
            77 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 79; continue; } bb = 78; continue; }
            78 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 80; continue; }
            79 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 80; continue; }
            80 => { rcx = ((rbx as u32)) as u32 as u64; r9 = r14; r8 = rdi; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P22), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 81; continue; }
            81 => { fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_k = 4; bb = 82; continue; }
            82 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 88; continue; } bb = 83; continue; }
            83 => { rcx = r14; bb = 84; continue; }
            84 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P23), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 85; continue; }
            85 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 86; continue; }
            86 => { r11 = rsp.wrapping_add(0x70); rax = rdi; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            87 => { rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 86; continue; }
            88 => { rdi = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 86; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c0b3c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x142646fa0; const P2: u64 = 0x14266c998; const P3: u64 = 0x142647000; const P4: u64 = 0x142646f98; const P5: u64 = 0x14266c9c8; const P6: u64 = 0x142646fa8; const P7: u64 = 0x142647020; const P8: u64 = 0x142647018; const P9: u64 = 0x1414c1bb4; const P10: u64 = 0x1414c1c1c; const P11: u64 = 0x142647010; const P12: u64 = 0x1414c1dd4; const P13: u64 = 0x1414c1e2c; const P14: u64 = 0x14392c0d0; const P15: u64 = 0x1414c4030; const P16: u64 = 0x1414c1e84; const P17: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = r8; rcx = ((rcx).wrapping_add(0x58u64)) as u64; rbx = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r12 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 2; continue; }
            2 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x2; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rbp = rdi.wrapping_add(0x68); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { r9 = P2; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x3; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { rcx = rdi.wrapping_add(0x70); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rdi.wrapping_add(0x70); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x4; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 10; continue; }
            10 => { rbp = rdi.wrapping_add(0x78); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 13; continue; }
            13 => { r9 = P5; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x5; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; r8 = rbx; rcx = 0x6; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; r8 = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 26; continue; } bb = 23; continue; }
            23 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; r8 = rbx; rcx = 0x8; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 26; continue; }
            26 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rbp = ((r12 as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 29; continue; }
            29 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 27; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x9cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 31; continue; }
            31 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x9cu64))) as u32 as u64; r8 = rbx; rcx = 0xa; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 34; continue; }
            34 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rbp = ((r12 as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 37; continue; }
            37 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0xb; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 35; continue; } bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 39; continue; }
            39 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; r8 = rbx; rcx = 0xc; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 42; continue; }
            42 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; rbp = ((r12 as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 46; continue; } bb = 43; continue; }
            43 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 45; continue; }
            45 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0xd; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 43; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xa4u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 47; continue; }
            47 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 49; continue; }
            49 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa4u64))) as u32 as u64; r8 = rbx; rcx = 0xe; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 50; continue; }
            50 => { rax = P14; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 55; continue; } bb = 51; continue; }
            51 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 55; continue; } bb = 52; continue; }
            52 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 54; continue; }
            54 => { rcx = rdi; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rsi; r8 = rbx; rcx = 0xf; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 55; continue; }
            55 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 57; continue; }
            57 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c14a8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1426470b8; const P2: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l;
    rbx = ((fl_l as u64)) as u64;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0x20; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c17b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1414c349c; const P5: u64 = 0x141834d30; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r14 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x48; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c184c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c2ca4; const P1: u64 = 0x1426470c0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; } else { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rax;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c1b7c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0xau8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c1b84(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x12u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c1b8c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1au8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c1b94(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x22u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c1b9c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x4au8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c1ba4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x5au8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c1bac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x6au8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c1bb4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c29bc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acbc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = rcx;
    r8 = 0x8;
    rcx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdx = rsp.wrapping_add(0x40);
    r8 = 0x8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rcx = rsp.wrapping_add(0x48);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c29fc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x2; const P2: u64 = 0x142647068;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rax = ((rcx.wrapping_add(P0)) as u64) as u64; rbx = rdx; rdx = rcx; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx, ((r8 as u32)) as u32); bb = 5; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbx, ((r8 as u32)) as u32); rax = ((rdx.wrapping_add(P1)) as u64) as u64; bb = 5; continue; }
            4 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c2a54(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x2; const P2: u64 = 0x142647070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rax = ((rcx.wrapping_add(P0)) as u64) as u64; rbx = rdx; rdx = rcx; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx, r8); bb = 5; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, rax); rax = ((rdx.wrapping_add(P1)) as u64) as u64; bb = 5; continue; }
            4 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c2ab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470b8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c2abc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acbc;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = rcx;
    rsi = 0x8;
    r8 = ((rsi as u32)) as u32 as u64;
    rcx = rsp.wrapping_add(0x40);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = ((rsi as u32)) as u32 as u64;
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = ((rsi as u32)) as u32 as u64;
    rdx = rsp.wrapping_add(0x40);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c2ca4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1426470b0; const P4: u64 = 0x1414c2ab0; const P5: u64 = 0xffffffffffffffe8; const P6: u64 = 0x142646fb8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414c2ca4, funcinfo_rva: 0x36bb7b4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = (rcx) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rcx = (rbx) as u64; bb = 5; continue; }
            2 => { rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { fl_l = (((rbx) & (0xfffffffffffffffcu64))) as u64 as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsi)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 7; continue; }
            7 => { fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rcx = (0x20u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414c2cf0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = 0u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm0) as u128); rcx = ((rax.wrapping_add(P2)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414c2d10); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            10 => { rdi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { bb = 13; continue; }
            12 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r9 = ((P4) as u64) as u64; rdx = (0x20u32) as u32 as u64; r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414c2d32); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P6)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rax, (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_add(P1), (xmm0) as u128); rcx = ((rax.wrapping_add(P2)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414c2d49); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rax = ((((rax as u32)) & (0x2u32))) as u32 as u64; rcx = (rdi) as u64; rcx = (((rcx) | (rax))) as u64; rcx = (((rcx) | (0x1u64))) as u64; crate::simd_ops::wu64(rsi, (rcx) as u64); crate::simd_ops::wu64(rdi, (rbx) as u64); rax = ((rdi.wrapping_add(P2)) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c2e48(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266bb20; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0xa8; const P12: u64 = 0x1426470a8; const P13: u64 = 0x1414c3ec8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P8), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P10), rax);
    rcx = ((rdi.wrapping_add(P11)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c2eb8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f0; const P1: u64 = 0x14266bb20; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58; const P12: u64 = 0x142646f60; const P13: u64 = 0x142646fc0; const P14: u64 = 0x1426470a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = P1; crate::simd_ops::wu64(rbx, rax); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P6), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P7), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P8), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P9), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P10), rdi); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; rdx = (crate::simd_ops::ru64(P12)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x60); rdx = (crate::simd_ops::ru64(P12)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x68); rdx = (crate::simd_ops::ru64(P12)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x70); rdx = (crate::simd_ops::ru64(P12)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x78); rdx = (crate::simd_ops::ru64(P12)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x88u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rdi); rcx = rbx.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c2f98(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266c788; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x3c; const P6: u64 = 0x1426470a8; const P7: u64 = 0x142646f50; const P8: u64 = 0x28; const P9: u64 = 0x142646f48; const P10: u64 = 0x30; const P11: u64 = 0xc; const P12: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P4), rax);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rdi.wrapping_add(P8)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rdi.wrapping_add(P10)) as u64) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    r8 = ((rdx.wrapping_add(P11)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3008(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266c828; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x44; const P7: u64 = 0x1426470a8; const P8: u64 = 0x142646f50; const P9: u64 = 0x28; const P10: u64 = 0x142646f48; const P11: u64 = 0x30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = P1; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx); crate::simd_ops::lk_and_u32(rdi.wrapping_add(P3), (0x0u32) as u32); crate::simd_ops::lk_and_u32(rdi.wrapping_add(P4), (0x0u32) as u32); crate::simd_ops::lk_and_u64(rdi.wrapping_add(P5), (0x0u64) as u64); rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x40u64), (0x0u32) as u32); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3094(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f0; const P1: u64 = 0x14266c828; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x142646f60; const P8: u64 = 0x142646fc0; const P9: u64 = 0x30; const P10: u64 = 0x38; const P11: u64 = 0x40; const P12: u64 = 0x1426470a8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rbx, rax);
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P2), (0x0u64) as u64);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P4), (0x0u32) as u32);
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P5), (0x0u64) as u64);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(P7)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P9)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(P7)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P10)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(P7)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P11), (0x0u32) as u32);
    rcx = rbx.wrapping_add(0x44);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266bbc0; const P2: u64 = 0x1c; const P3: u64 = 0x1426470a8; const P4: u64 = 0x142646f50; const P5: u64 = 0x10; const P6: u64 = 0x142646f48; const P7: u64 = 0x18;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(P7), (0x0u32) as u32);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c33b4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266bb20; const P1: u64 = 0x1414c3f48; const P2: u64 = 0x8; const P3: u64 = 0x1414c14a8; const P4: u64 = 0x40; const P5: u64 = 0x1414c324c; const P6: u64 = 0x28; const P7: u64 = 0x1414c322c; const P8: u64 = 0x10; const P9: u64 = 0x1414c31ac; const P10: u64 = 0x142647100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (((rdx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 5; continue; }
            3 => { rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (((rdx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3428(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266c788; const P1: u64 = 0x142646f50; const P2: u64 = 0x28; const P3: u64 = 0x142646fe0; const P4: u64 = 0x8; const P5: u64 = 0x1414c14a8; const P6: u64 = 0x10; const P7: u64 = 0x1414c31ac; const P8: u64 = 0x142647100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 5; continue; }
            3 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c349c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266c828; const P1: u64 = 0x142646f50; const P2: u64 = 0x28; const P3: u64 = 0x142646fe0; const P4: u64 = 0x30; const P5: u64 = 0x38; const P6: u64 = 0x8; const P7: u64 = 0x1414c14a8; const P8: u64 = 0x10; const P9: u64 = 0x1414c31ac; const P10: u64 = 0x142647100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P6)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 5; continue; }
            3 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c36c8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c2ca4; const P1: u64 = 0x1414c18ac; const P2: u64 = 0x142646ff0; const P3: u64 = 0x142646f28; const P4: u64 = 0x142646f50; const P5: u64 = 0x142646fd8; const P6: u64 = 0x1414c2c40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = rdx; rcx = rdx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = rax; rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = rsi.wrapping_add(0x10); rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x28); rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x40); rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x58); r9 = rbx; r8 = rax; rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x60); r9 = rbx; r8 = rax; rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x68); r9 = rbx; r8 = rax; rcx = rdi.wrapping_add(0x68); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x70); r9 = rbx; r8 = rax; rcx = rdi.wrapping_add(0x70); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x78); r9 = rbx; r8 = rax; rcx = rdi.wrapping_add(0x78); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rsi.wrapping_add(0x80); rcx = rdi.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3a18(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c1878; const P1: u64 = 0x1414c1edc; const P2: u64 = 0x1414c1f4c; const P3: u64 = 0x1414c1f68; const P4: u64 = 0x142646fc8; const P5: u64 = 0x142646f28; const P6: u64 = 0x142646f18; const P7: u64 = 0x14392c0d0; const P8: u64 = 0x1414c13d0; const P9: u64 = 0x14392bfa0; const P10: u64 = 0x1414c3e00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = rdi.wrapping_add(0x10); rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rdi.wrapping_add(0x28); rcx = rsi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rdi.wrapping_add(0x40); rcx = rsi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x58); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi.wrapping_add(0x60); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x60); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = rdi.wrapping_add(0x68); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rdi.wrapping_add(0x68); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x68); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rdi.wrapping_add(0x70); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rdi.wrapping_add(0x70); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x70); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = rdi.wrapping_add(0x78); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rdi.wrapping_add(0x78); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x78); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rax = P7; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), rax); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; rdx = P9; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; fl_l = rax; fl_r = rax; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), rax); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsi.wrapping_add(0x90u64), rax); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rsi.wrapping_add(0x98u64), ((rax as u32)) as u32); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x9cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rsi.wrapping_add(0x9cu64), ((rax as u32)) as u32); bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rsi.wrapping_add(0xa0u64), ((rax as u32)) as u32); bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xa4u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu32(rsi.wrapping_add(0xa4u64), ((rax as u32)) as u32); bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3c5c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414c1878; const P2: u64 = 0x10; const P3: u64 = 0x1414c1edc; const P4: u64 = 0x28; const P5: u64 = 0x142646fc8; const P6: u64 = 0x142646f28; const P7: u64 = 0x142646f18; const P8: u64 = 0x30; const P9: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = ((rdi.wrapping_add(P2)) as u64) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi.wrapping_add(P8), rax); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P9))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rsi.wrapping_add(P9), ((rax as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3cfc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c1878; const P1: u64 = 0x1414c1edc; const P2: u64 = 0x142646fc8; const P3: u64 = 0x142646f28; const P4: u64 = 0x142646f18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = rdi.wrapping_add(0x10); rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x28); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x30); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x38); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rsi.wrapping_add(0x40u64), ((rax as u32)) as u32); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414c1878; const P2: u64 = 0x10; const P3: u64 = 0x142646fc8; const P4: u64 = 0x142646f28; const P5: u64 = 0x142646f18; const P6: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P2)) as u64) as u64; r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu32(rsi.wrapping_add(P6), ((rax as u32)) as u32); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3ec8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646f50; const P1: u64 = 0x58; const P2: u64 = 0x142646f48; const P3: u64 = 0x60; const P4: u64 = 0x68; const P5: u64 = 0x70; const P6: u64 = 0x78; const P7: u64 = 0x80; const P8: u64 = 0x28; const P9: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = ((rbx.wrapping_add(P7)) as u64) as u64;
    r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c3f48(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646f50; const P1: u64 = 0x58; const P2: u64 = 0x142646fe0; const P3: u64 = 0x60; const P4: u64 = 0x68; const P5: u64 = 0x70; const P6: u64 = 0x78; const P7: u64 = 0x14392c0d0; const P8: u64 = 0x80; const P9: u64 = 0x1414c3534; const P10: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = P7; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = 0x20; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c4514(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646f58; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c4540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x10; const P2: u64 = 0x142647038; const P3: u64 = 0x1; const P4: u64 = 0x18; const P5: u64 = 0x142647128; const P6: u64 = 0x8; const P7: u64 = 0x20; const P8: u64 = 0x142647118; const P9: u64 = 0x1414c4874;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rdi.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax.wrapping_add(P3)) as u64) as u64; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
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
pub unsafe extern "win64" fn fn_1414c4e60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414c1878; const P2: u64 = 0x10; const P3: u64 = 0x142646fc8; const P4: u64 = 0x142646f28; const P5: u64 = 0x142646f18; const P6: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P2)) as u64) as u64; r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi.wrapping_add(P6), rax); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c644c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c8ef4; const P1: u64 = 0x8; const P2: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c6470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x142646fc8; const P2: u64 = 0x142647040; const P3: u64 = 0x10; const P4: u64 = 0x1414c6f9c; const P5: u64 = 0x142647048; const P6: u64 = 0x1414c6c10; const P7: u64 = 0x142647128; const P8: u64 = 0x142647038; const P9: u64 = 0x1; const P10: u64 = 0x8; const P11: u64 = 0x18; const P12: u64 = 0x142647118; const P13: u64 = 0x1414c4424;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; rdi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            11 => { rbx = 0x9; bb = 16; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            13 => { rbx = 0x2; bb = 16; continue; }
            14 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rbx = ((rax.wrapping_add(P9)) as u64) as u64; bb = 16; continue; }
            16 => { rcx = ((rdi.wrapping_add(P10)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = ((rdi.wrapping_add(P11)) as u64) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            18 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c653c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c8ef4; const P2: u64 = 0x142646f50; const P3: u64 = 0x142646f48; const P4: u64 = 0x142646f28; const P5: u64 = 0x142646fd0; const P6: u64 = 0x142647090; const P7: u64 = 0x1414c7f00; const P8: u64 = 0x1414c8508; const P9: u64 = 0x1414c7dbc; const P10: u64 = 0x1414c8460; const P11: u64 = 0x1414c85b0; const P12: u64 = 0x1414c2a54; const P13: u64 = 0x14266de38; const P14: u64 = 0x142647088; const P15: u64 = 0x1414c2ca4; const P16: u64 = 0x1426470d8; const P17: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rcx = r8; r14 = r8; bb = 56; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rsi = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 60; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 38; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 46; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 46; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x7u32) as u32); rdi = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            16 => { rdi = rbx.wrapping_add(0x10); bb = 17; continue; }
            17 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            18 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x32u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 46; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x6u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r8 = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            22 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x2au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 46; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x5u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r8 = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            26 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x21u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 46; continue; } bb = 27; continue; }
            27 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x4; xmm6 = (xmm0) as u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x4u32) as u32); bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x8u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm6 as u64)) as u64); fl_k = 6; bb = 55; continue; }
            30 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x18u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 46; continue; } bb = 31; continue; }
            31 => { rdx = rbp.wrapping_add(0x38); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x3; rcx = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x3u32) as u32); bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); fl_k = 4; bb = 54; continue; }
            34 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 46; continue; } bb = 35; continue; }
            35 => { rdx = rbp.wrapping_sub(0x50); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rdi = (rdi & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x2u32) as u32); bb = 37; continue; }
            37 => { crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), ((rdi as u8)) as u8); bb = 53; continue; }
            38 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 46; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x1u32) as u32); rdi = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            41 => { rdi = rbx.wrapping_add(0x10); bb = 42; continue; }
            42 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; rdi = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rdi; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rdi)) as u64; bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rdx = P13; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 60; continue; } bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 54; continue; }
            46 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 59; continue; } bb = 47; continue; }
            47 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 59; continue; } bb = 48; continue; }
            48 => { rcx = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 51; continue; }
            50 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 51; continue; }
            51 => { rcx = ((rdi as u32)) as u32 as u64; r9 = r14; r8 = rsi; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            52 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); bb = 53; continue; }
            53 => { fl_l = rax; fl_r = rax; fl_k = 4; bb = 54; continue; }
            54 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 60; continue; } bb = 55; continue; }
            55 => { rcx = r14; bb = 56; continue; }
            56 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 57; continue; }
            57 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 58; continue; }
            58 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            59 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 58; continue; }
            60 => { rsi = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 58; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c68e4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x142646f50; const P2: u64 = 0x14266de38; const P3: u64 = 0x142647000; const P4: u64 = 0x142646f98; const P5: u64 = 0x142646fa8; const P6: u64 = 0x142647140; const P7: u64 = 0x142647120; const P8: u64 = 0x142647138; const P9: u64 = 0x1414c4fac; const P10: u64 = 0x1414c8234; const P11: u64 = 0x1414c828c; const P12: u64 = 0x142646fa0; const P13: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x1; r14 = rcx.wrapping_add(0x10); rsi = r8; rbx = rdx; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x1; rbp = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 6; continue; }
            6 => { r9 = P2; r8 = 0x1; rdx = ((rbp as u32)) as u32 as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { r9 = rbx; r8 = rax; rdx = 0x1; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 10; continue; }
            10 => { rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (rdx & !0xffu64) | (((crate::simd_ops::ru8(r14)) as u8 as u64)); bb = 16; continue; }
            15 => { rdx = (rdx & !0xffu64) | ((((rbp as u8)) as u8 as u64)); bb = 16; continue; }
            16 => { r8 = rbx; rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rbp = (crate::simd_ops::ru64(r14)) as u64; bb = 22; continue; }
            22 => { r8 = rbx; rdx = rbp; rcx = 0x3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 24; continue; }
            24 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { xmm1 = ((crate::simd_ops::ru64(r14)) as u64) as u128; bb = 29; continue; }
            28 => { xmm1 = 0u128; bb = 29; continue; }
            29 => { r8 = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 31; continue; }
            31 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 33; continue; }
            33 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; r9 = rsi; r8 = rbx; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 35; continue; }
            35 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 37; continue; }
            37 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; r9 = rsi; r8 = rbx; rcx = 0x6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rbx; rdx = 0x7; r8 = rax; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 42; continue; }
            42 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c7300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646f58; const P1: u64 = 0x142646f28; const P2: u64 = 0x18; const P3: u64 = 0x1414c87a4; const P4: u64 = 0x141834d30; const P5: u64 = 0x8; const P6: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0x20; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P2), (0x0u64) as u64); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c7364(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x10; const P2: u64 = 0x142647038; const P3: u64 = 0x1; const P4: u64 = 0x14392c2b0; const P5: u64 = 0x18; const P6: u64 = 0x1414c6470; const P7: u64 = 0x142647048; const P8: u64 = 0x8; const P9: u64 = 0x20; const P10: u64 = 0x142647118; const P11: u64 = 0x1414c4874;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax.wrapping_add(P3)) as u64) as u64; bb = 2; continue; }
            2 => { rax = P4; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P8)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = ((rbx.wrapping_add(P9)) as u64) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rdx = ((rdi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = rdi; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c7400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x142646f28; const P2: u64 = 0x1414c7d50; const P3: u64 = 0x1414c83b8; const P4: u64 = 0x142646fd0; const P5: u64 = 0x142647090; const P6: u64 = 0x14266de70; const P7: u64 = 0x142647088; const P8: u64 = 0x1414c2ca4; const P9: u64 = 0x1426470d8; const P10: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = rcx; r14 = r8; rcx = r8; bb = 26; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 30; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 17; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 17; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rax); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; r8 = rbx; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            12 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 17; continue; } bb = 13; continue; }
            13 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P6; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 30; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 24; continue; }
            17 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 29; continue; } bb = 18; continue; }
            18 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 29; continue; } bb = 19; continue; }
            19 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 22; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rcx = ((rdi as u32)) as u32 as u64; r9 = r14; r8 = rbx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            23 => { fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_k = 4; bb = 24; continue; }
            24 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 30; continue; } bb = 25; continue; }
            25 => { rcx = r14; bb = 26; continue; }
            26 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 28; continue; }
            28 => { r11 = rsp.wrapping_add(0x50); rax = rbx; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            29 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 28; continue; }
            30 => { rbx = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 28; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c75bc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x14266de70; const P2: u64 = 0x142647000; const P3: u64 = 0x142646f98; const P4: u64 = 0x14392c2b0; const P5: u64 = 0x142646fa8; const P6: u64 = 0x1414c8fcc; const P7: u64 = 0x1414c81dc; const P8: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx.wrapping_add(0x10); rdi = rcx; rcx = rbp; rsi = r8; rbx = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { r9 = P1; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x1; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rax = P4; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rsi; r8 = rbx; rcx = 0x2; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 11; continue; }
            11 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c7780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c0fb8; const P1: u64 = 0x28; const P2: u64 = 0x142646f58; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x8; const P6: u64 = 0x1414c184c;
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
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P4), (0x0u32) as u32);
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c77c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c7364; const P1: u64 = 0x142647048; const P2: u64 = 0x142646fc8; const P3: u64 = 0x142647038; const P4: u64 = 0x142647028; const P5: u64 = 0x142647118; const P6: u64 = 0x1414bfbdc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rbp = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rsi as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = rbp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rcx = rbp.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = rbp.wrapping_add(0x3c); rdx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            11 => { rdx = ((rsi as u32)) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = rsi; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c78b4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c29fc; const P2: u64 = 0x1414bb6c4; const P3: u64 = 0x1414c247c; const P4: u64 = 0x1414c1b8c; const P5: u64 = 0x142646f28; const P6: u64 = 0x142646fd0; const P7: u64 = 0x142647090; const P8: u64 = 0x14266ded8; const P9: u64 = 0x142647088; const P10: u64 = 0x14266dea0; const P11: u64 = 0x1414c2ca4; const P12: u64 = 0x1426470d8; const P13: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 37; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rbx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdi = rdx.wrapping_add(0x1); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 41; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 28; continue; } bb = 10; continue; }
            10 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 28; continue; } bb = 11; continue; }
            11 => { rdx = rbp.wrapping_add(0x38); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x38u64), ((rcx as u32)) as u32); bb = 34; continue; }
            12 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 28; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; fl_k = 6; bb = 14; continue; }
            14 => { rdi = ((rdi).wrapping_add(1)) as u64; rcx = r14.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rax; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 41; continue; } bb = 15; continue; }
            15 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 36; continue; } bb = 16; continue; }
            16 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 36; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 14; continue; }
            18 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 28; continue; } bb = 19; continue; }
            19 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x30); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rdx = P8; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; bb = 22; continue; }
            22 => { rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 41; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 35; continue; }
            24 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 28; continue; } bb = 25; continue; }
            25 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x28); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P10; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; bb = 22; continue; }
            28 => { rax = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 40; continue; } bb = 29; continue; }
            29 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 40; continue; } bb = 30; continue; }
            30 => { rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 33; continue; }
            32 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rcx = ((rbx as u32)) as u32 as u64; r9 = rsi; r8 = rdi; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            34 => { fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_k = 4; bb = 35; continue; }
            35 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 41; continue; } bb = 36; continue; }
            36 => { rcx = rsi; bb = 37; continue; }
            37 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 38; continue; }
            38 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 39; continue; }
            39 => { r11 = rsp.wrapping_add(0x60); rax = rdi; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            40 => { rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 39; continue; }
            41 => { rdi = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 39; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c7b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x14266dea0; const P2: u64 = 0x142647000; const P3: u64 = 0x142646f98; const P4: u64 = 0x14266ded8; const P5: u64 = 0x142646fa8; const P6: u64 = 0x1414c1bb4; const P7: u64 = 0x1414c1c1c; const P8: u64 = 0x142647010; const P9: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx.wrapping_add(0x28); rdi = rcx; rcx = rbp; rsi = r8; rbx = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { r9 = P1; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x1; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rbp = rdi.wrapping_add(0x30); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { r9 = P4; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x2; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 11; continue; }
            11 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x3; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; r8 = rbx; rcx = 0x4; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c7f6c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1414c87a4; const P5: u64 = 0x141834d30; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r14 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x20; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c85b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acbc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = rcx;
    r8 = 0x8;
    rcx = rsp.wrapping_add(0x38);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rdx = rsp.wrapping_add(0x40);
    r8 = 0x8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rcx = rsp.wrapping_add(0x48);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64) as u128;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c85f4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266daf0; const P2: u64 = 0x18; const P3: u64 = 0x1426470a8; const P4: u64 = 0x1c;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rbx, rax);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P4), (0x0u32) as u32);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c866c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266dd70; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x3c; const P7: u64 = 0x1426470a8; const P8: u64 = 0x142646f50; const P9: u64 = 0x28; const P10: u64 = 0x142646f48; const P11: u64 = 0x30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(P4), (0x0u32) as u32);
    crate::simd_ops::lk_and_u64(rdi.wrapping_add(P5), (0x0u64) as u64);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rdi.wrapping_add(P9)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rdi.wrapping_add(P11)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x38u64), (0x0u32) as u32);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c87a4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266daf0; const P1: u64 = 0x1c; const P2: u64 = 0x1414c8ef4; const P3: u64 = 0x8; const P4: u64 = 0x1414c14a8; const P5: u64 = 0x142647100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = ((rbx.wrapping_add(P3)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 7; continue; }
            5 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rdx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c8864(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266dd70; const P1: u64 = 0x142646f50; const P2: u64 = 0x28; const P3: u64 = 0x142646fe0; const P4: u64 = 0x30; const P5: u64 = 0x8; const P6: u64 = 0x1414c14a8; const P7: u64 = 0x10; const P8: u64 = 0x1414c31ac; const P9: u64 = 0x142647100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P5)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 5; continue; }
            3 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c88e8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266dcd0; const P1: u64 = 0x142646f50; const P2: u64 = 0x10; const P3: u64 = 0x142646fe0; const P4: u64 = 0x14392c2b0; const P5: u64 = 0x18; const P6: u64 = 0x1414c87a4; const P7: u64 = 0x141834d30; const P8: u64 = 0x8; const P9: u64 = 0x1414c14a8; const P10: u64 = 0x142647100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = P4; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = 0x20; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = ((rbx.wrapping_add(P8)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 8; continue; }
            6 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c8a14(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414c2ca4; const P2: u64 = 0x1414c18ac; const P3: u64 = 0x10; const P4: u64 = 0x142646ff0; const P5: u64 = 0x142646f28; const P6: u64 = 0x142646f50; const P7: u64 = 0x28; const P8: u64 = 0x142646fd8; const P9: u64 = 0x30; const P10: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P0))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = rax; rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = ((rsi.wrapping_add(P3)) as u64) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P7)) as u64) as u64; r9 = rbx; r8 = rax; rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(P9)) as u64) as u64; r9 = rbx; r8 = rax; rcx = ((rdi.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsi.wrapping_add(P10))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P10))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rax as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(P10), ((rcx as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c8ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c1878; const P1: u64 = 0x142646fc8; const P2: u64 = 0x1414c8ef4; const P3: u64 = 0x142646f28; const P4: u64 = 0x1414c7f00; const P5: u64 = 0x143813350; const P6: u64 = 0x1414c8eb0; const P7: u64 = 0x1414c7dbc; const P8: u64 = 0x143813320; const P9: u64 = 0x1414c8cdc; const P10: u64 = 0x142646f50; const P11: u64 = 0x142646f48; const P12: u64 = 0x142646f18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdi = rdx; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 9; continue; }
            9 => { rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x7; rsi = rax; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x7u32) as u32); bb = 34; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x6u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; bb = 16; continue; }
            15 => { rdx = P5; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x5u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; bb = 22; continue; }
            21 => { rdx = P8; bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x4; xmm6 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64) as u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x4u32) as u32); bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), ((xmm6 as u64)) as u64); bb = 36; continue; }
            26 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x3; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x3u32) as u32); bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rdi); bb = 36; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x2; rdi = (rdi & !0xffu64) | (((crate::simd_ops::ru8(rdi.wrapping_add(0x10u64))) as u8 as u64)); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x2u32) as u32); bb = 31; continue; }
            31 => { crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), ((rdi as u8)) as u8); bb = 36; continue; }
            32 => { rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; fl_r = 0x1; rsi = rax; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), (0x1u32) as u32); bb = 34; continue; }
            34 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rsi; rcx = rbx.wrapping_add(0x10); r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c8d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414c1878; const P2: u64 = 0x10; const P3: u64 = 0x1414c1edc; const P4: u64 = 0x28; const P5: u64 = 0x142646fc8; const P6: u64 = 0x142646f28; const P7: u64 = 0x142646f18; const P8: u64 = 0x30; const P9: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = ((rdi.wrapping_add(P2)) as u64) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P8)) as u64) as u64; r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P9))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rsi.wrapping_add(P9), ((rax as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c8dec(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414c1878; const P2: u64 = 0x10; const P3: u64 = 0x142646fc8; const P4: u64 = 0x142646f28; const P5: u64 = 0x142646f18; const P6: u64 = 0x14392c2b0; const P7: u64 = 0x18; const P8: u64 = 0x1414c7d50; const P9: u64 = 0x143813300; const P10: u64 = 0x1414c8ae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P2)) as u64) as u64; r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = P6; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P7))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P7), rax); bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_r = (0x0u64) as u64; rdx = P9; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P7))) as u64; rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rdi.wrapping_add(P7)) } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c8ef4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x142646f28; const P2: u64 = 0x10; const P3: u64 = 0x142647130; const P4: u64 = 0x1414c8984; const P5: u64 = 0x1414c8804; const P6: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; r8 = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::lk_and_u32(rbx.wrapping_add(P0), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 8; continue; }
            8 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            9 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 11; continue; }
            11 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdx = 0x30; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c9124(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c29fc; const P2: u64 = 0x1414bb6c4; const P3: u64 = 0x1414c247c; const P4: u64 = 0x1414c1b7c; const P5: u64 = 0x1414c2ca4; const P6: u64 = 0x1426470d8; const P7: u64 = 0x142647060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 24; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x10); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 9; continue; }
            9 => { rdx = rbp.wrapping_add(0x38); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x28u64), ((rcx as u32)) as u32); bb = 22; continue; }
            10 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 11; continue; }
            11 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = r14.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; rdx = rax; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 14; continue; }
            14 => { rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 12; continue; }
            16 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 18; continue; }
            18 => { rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 21; continue; }
            20 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 23; continue; }
            23 => { rcx = rsi; bb = 24; continue; }
            24 => { rdx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 26; continue; }
            26 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            27 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 26; continue; }
            28 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 26; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c954c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1414c2ca4; const P2: u64 = 0x1414c18ac; const P3: u64 = 0x10; const P4: u64 = 0x142646ff0; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = rdx; rcx = ((rdx.wrapping_add(P0)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P0))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 5; continue; }
            4 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = rax; rcx = ((rdi.wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ca594(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c0fb8; const P1: u64 = 0x28; const P2: u64 = 0x142646f58; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x142646f28; const P6: u64 = 0x40; const P7: u64 = 0x1414c87a4; const P8: u64 = 0x141834d30; const P9: u64 = 0x48; const P10: u64 = 0x1c; const P11: u64 = 0x14183acc2; const P12: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = 0x20; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P6), (0x0u64) as u64); rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(P10)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ca624(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c7364; const P1: u64 = 0x142647048; const P2: u64 = 0x142646fc8; const P3: u64 = 0x142647038; const P4: u64 = 0x142647040; const P5: u64 = 0x14392c3e0; const P6: u64 = 0x1414c6470; const P7: u64 = 0x142647258; const P8: u64 = 0x142647028; const P9: u64 = 0x142647118; const P10: u64 = 0x1414cad80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = rsi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rsi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rcx = rsi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rsi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rax = P5; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_r = rdi; rbx = rbp.wrapping_add(0x9); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rbx }) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x54u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; fl_r = rdi; rax = rbx.wrapping_add(0x9); rcx = rsi.wrapping_add(0x8); rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rbx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rbx = rax.wrapping_add(0x5); rbx = (if (((fl_l as u32)) == ((fl_r as u32))) { rax } else { rbx }) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = rsi.wrapping_add(0x64); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            18 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ca7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414bb6c4; const P2: u64 = 0x1414c247c; const P3: u64 = 0x1414cb3d0; const P4: u64 = 0x142646f28; const P5: u64 = 0x1414c7d50; const P6: u64 = 0x1414c83b8; const P7: u64 = 0x142646fd0; const P8: u64 = 0x142647090; const P9: u64 = 0x14266ed10; const P10: u64 = 0x142647088; const P11: u64 = 0x1414c2a54; const P12: u64 = 0x1414c29bc; const P13: u64 = 0x1414c29fc; const P14: u64 = 0x1414cb954; const P15: u64 = 0x1414c2ca4; const P16: u64 = 0x1426470d8; const P17: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rsi = rcx; r14 = r8; rcx = r8; bb = 58; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rbx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdi = rdx.wrapping_add(0x1); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 62; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 36; continue; } bb = 7; continue; }
            7 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 49; continue; } bb = 13; continue; }
            13 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x32u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 14; continue; }
            14 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; fl_k = 6; bb = 15; continue; }
            15 => { rdi = ((rdi).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 62; continue; } bb = 16; continue; }
            16 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 57; continue; } bb = 17; continue; }
            17 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 57; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 15; continue; }
            19 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x2au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rax); bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; r8 = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            23 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 24; continue; }
            24 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x28); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P9; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 62; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 56; continue; }
            28 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 29; continue; }
            29 => { rdx = rbp.wrapping_sub(0x40); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 62; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); fl_k = 4; bb = 57; continue; }
            31 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 32; continue; }
            32 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x8u64) as u64)) as u64; fl_r = fl_l; fl_k = 6; bb = 57; continue; }
            34 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x38u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 35; continue; }
            35 => { rdx = rbp.wrapping_add(0x38); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x54u64), ((rcx as u32)) as u32); bb = 55; continue; }
            36 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 37; continue; }
            37 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 38; continue; }
            38 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 39; continue; }
            39 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 49; continue; } bb = 40; continue; }
            40 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x59u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 41; continue; }
            41 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rax); bb = 33; continue; }
            42 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x52u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 43; continue; }
            43 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x38); bb = 44; continue; }
            44 => { r8 = rax; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            45 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 46; continue; }
            46 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x30); bb = 44; continue; }
            47 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x45u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 49; continue; } bb = 48; continue; }
            48 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x4u64) as u64)) as u64; fl_r = fl_l; crate::simd_ops::wu32(rsi.wrapping_add(0x60u64), ((rax as u32)) as u32); fl_k = 6; bb = 57; continue; }
            49 => { rax = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 61; continue; } bb = 50; continue; }
            50 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 61; continue; } bb = 51; continue; }
            51 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 53; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 54; continue; }
            53 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            54 => { rcx = ((rbx as u32)) as u32 as u64; r9 = r14; r8 = rdi; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            55 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; fl_k = 4; bb = 56; continue; }
            56 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 62; continue; } bb = 57; continue; }
            57 => { rcx = r14; bb = 58; continue; }
            58 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 59; continue; }
            59 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 60; continue; }
            60 => { r11 = rsp.wrapping_add(0x60); rax = rdi; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            61 => { rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 60; continue; }
            62 => { rdi = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 60; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414caad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fa8; const P1: u64 = 0x142647018; const P2: u64 = 0x142647020; const P3: u64 = 0x142646fc8; const P4: u64 = 0x14266ed10; const P5: u64 = 0x142647000; const P6: u64 = 0x142646f98; const P7: u64 = 0x14392c3e0; const P8: u64 = 0x1414cc244; const P9: u64 = 0x1414c81dc; const P10: u64 = 0x1414c1bb4; const P11: u64 = 0x1414c1c1c; const P12: u64 = 0x142647010; const P13: u64 = 0x142647148; const P14: u64 = 0x142646fa0; const P15: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; rsi = r8; rbx = rdx; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r8; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; r8 = rbx; rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; r8 = rbx; rcx = 0x2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { rbp = rdi.wrapping_add(0x28); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 11; continue; }
            11 => { r9 = P4; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x3; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 12; continue; }
            12 => { rax = P7; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 16; continue; }
            16 => { rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rsi; r8 = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 20; continue; }
            20 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x6; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; r8 = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; r8 = rbx; rcx = 0x8; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 29; continue; }
            29 => { rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x9; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 31; continue; }
            31 => { rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0xa; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 37; continue; } bb = 34; continue; }
            34 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; r8 = rbx; rcx = 0xb; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 39; continue; }
            39 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cb3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x32u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cb954(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acbc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = rcx;
    r8 = 0x4;
    rcx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64;
    rdx = rsp.wrapping_add(0x40);
    r8 = 0x4;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32);
    rcx = rsp.wrapping_add(0x48);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cb994(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266ec08; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x64; const P6: u64 = 0x1426470a8; const P7: u64 = 0x142646f50; const P8: u64 = 0x28; const P9: u64 = 0x142646f48; const P10: u64 = 0x30; const P11: u64 = 0x38; const P12: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = P1; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); crate::simd_ops::wu64(rdi.wrapping_add(P4), rax); rcx = ((rdi.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rdi.wrapping_add(P8)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rdi.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rdi.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x40); rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(0x24u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cba2c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f0; const P1: u64 = 0x14266ec08; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x142646f60; const P8: u64 = 0x142646fc0; const P9: u64 = 0x30; const P10: u64 = 0x38; const P11: u64 = 0x40; const P12: u64 = 0x1426470a8;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = P1; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P2), (0x0u64) as u64); crate::simd_ops::lk_and_u32(rbx.wrapping_add(P3), (0x0u32) as u32); crate::simd_ops::lk_and_u32(rbx.wrapping_add(P4), (0x0u32) as u32); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P5), (0x0u64) as u64); rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rdx = (crate::simd_ops::ru64(P7)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rdx = (crate::simd_ops::ru64(P7)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rdx = (crate::simd_ops::ru64(P7)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P11), (0x0u64) as u64); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x48u64), (0x0u64) as u64); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x50u64), (0x0u32) as u32); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x54u64), (0x0u32) as u32); crate::simd_ops::lk_and_u64(rbx.wrapping_add(0x58u64), (0x0u64) as u64); crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x60u64), (0x0u32) as u32); rcx = rbx.wrapping_add(0x64); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cbc38(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14266ec08; const P1: u64 = 0x1414cc1d0; const P2: u64 = 0x8; const P3: u64 = 0x1414c14a8; const P4: u64 = 0x10; const P5: u64 = 0x1414c31ac; const P6: u64 = 0x142647100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (((rdx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 5; continue; }
            3 => { rax = ((rdx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (((rdx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cbe70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c1878; const P1: u64 = 0x1414c1edc; const P2: u64 = 0x142646fc8; const P3: u64 = 0x142646f28; const P4: u64 = 0x142646f18; const P5: u64 = 0x14392c3e0; const P6: u64 = 0x1414c7d50; const P7: u64 = 0x143813300; const P8: u64 = 0x1414c8ae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = rdi.wrapping_add(0x10); rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x28); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x30); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = rdi.wrapping_add(0x38); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x38); r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = P5; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rax); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; rdx = P7; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rdi.wrapping_add(0x40u64)) } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rsi.wrapping_add(0x54u64), ((rax as u32)) as u32); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rax); bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rsi.wrapping_add(0x60u64), ((rax as u32)) as u32); bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cc1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646f50; const P1: u64 = 0x28; const P2: u64 = 0x142646fe0; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x14392c3e0; const P6: u64 = 0x40; const P7: u64 = 0x1414c87a4; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P5;
    fl_l = rbx; fl_r = rax;
    if (((fl_l as u64)) == ((fl_r as u64))) { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64;
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = 0x20; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cc348(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d1c2c; const P1: u64 = 0x28; const P2: u64 = 0x5; const P3: u64 = 0x14183acc2; const P4: u64 = 0x8; const P5: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    r8 = ((rdx.wrapping_add(P2)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cc380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x1414cd3ac; const P4: u64 = 0x142647048; const P5: u64 = 0x28; const P6: u64 = 0x142647258; const P7: u64 = 0x2c; const P8: u64 = 0x30; const P9: u64 = 0x142647118; const P10: u64 = 0x1414cc6e8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P5))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(P7))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rbp.wrapping_add(0x2); rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rbx = (if (((fl_l as u8)) == ((fl_r as u8))) { rbp } else { rbx }) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r8 = ((rsi.wrapping_add(P8)) as u64) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            7 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cc42c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c2a54; const P2: u64 = 0x1414bcc14; const P3: u64 = 0x1414d3fa8; const P4: u64 = 0x1414c1b7c; const P5: u64 = 0x1414c2ca4; const P6: u64 = 0x1426470d8; const P7: u64 = 0x142647060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 28; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x10u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x18u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 10; continue; }
            10 => { rdx = rbp.wrapping_add(0x38); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_r = (0x0u64) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(r14.wrapping_add(0x2cu64), ((rcx as u8)) as u8); bb = 26; continue; }
            11 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 12; continue; }
            12 => { rdx = rbp.wrapping_sub(0x20); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x20u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 27; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = r14.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; rdx = rax; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 17; continue; }
            17 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 18; continue; }
            18 => { rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 16; continue; }
            20 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 21; continue; }
            21 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 22; continue; }
            22 => { rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 25; continue; }
            24 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 27; continue; }
            27 => { rcx = rsi; bb = 28; continue; }
            28 => { rdx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 30; continue; }
            30 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            31 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 30; continue; }
            32 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cc5d8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fa8; const P1: u64 = 0x1414c1bb4; const P2: u64 = 0x1414d31b8; const P3: u64 = 0x142647020; const P4: u64 = 0x142647140; const P5: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; rsi = r8; rbx = rdx; rdi = rcx; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 3; continue; }
            3 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; r8 = rbx; rcx = 0x2; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x2cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 11; continue; }
            11 => { rdx = (rdx & !0xffu64) | (((crate::simd_ops::ru8(rdi.wrapping_add(0x2cu64))) as u8 as u64)); r8 = rbx; rcx = 0x3; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 14; continue; }
            14 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ccef4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d6168;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0x7;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rax = rbx;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cd280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d6168;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0x8;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rax = rbx;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cd364(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d1adc; const P1: u64 = 0x28; const P2: u64 = 0x1414c0fb8; const P3: u64 = 0x40; const P4: u64 = 0x14; const P5: u64 = 0x14183acc2; const P6: u64 = 0x1414d6160; const P7: u64 = 0x8; const P8: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = ((rbx.wrapping_add(P3)) as u64) as u64;
    r8 = ((rdx.wrapping_add(P4)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P7)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cd3ac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414cfbb0; const P1: u64 = 0x142647048; const P2: u64 = 0x1414c7364; const P3: u64 = 0x142647028; const P4: u64 = 0x142647118; const P5: u64 = 0x1414cd950;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rcx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rcx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; rax = rbp.wrapping_add(0x9); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rax }) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; rbx = rax.wrapping_add(0x9); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rax } else { rbx }) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = ((rbx).wrapping_add(0x9u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = rsi.wrapping_add(0x60); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cd4b4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c29fc; const P2: u64 = 0x1414bb6c4; const P3: u64 = 0x1414c247c; const P4: u64 = 0x1414d2ee8; const P5: u64 = 0x1414c29bc; const P6: u64 = 0x1414d6160; const P7: u64 = 0x1414d193c; const P8: u64 = 0x1414d3a68; const P9: u64 = 0x1414d2ee0; const P10: u64 = 0x1414c85b0; const P11: u64 = 0x1414c2ca4; const P12: u64 = 0x1426470d8; const P13: u64 = 0x142647060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rcx = r8; r14 = r8; bb = 48; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x18u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 14; continue; }
            14 => { rdx = rbp.wrapping_add(0x38); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rcx as u32)) as u32); bb = 46; continue; }
            15 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 16; continue; }
            16 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x28); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 18; continue; }
            18 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 47; continue; } bb = 19; continue; }
            19 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 47; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 17; continue; }
            21 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x31u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 22; continue; }
            22 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x6; rbx = rax; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x64u64), (0x6u32) as u32); bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rbx); bb = 39; continue; }
            25 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x2au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 26; continue; }
            26 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 27; continue; }
            27 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 28; continue; }
            28 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 47; continue; } bb = 29; continue; }
            29 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 47; continue; } bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 27; continue; }
            31 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x21u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 32; continue; }
            32 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x4; xmm6 = (xmm0) as u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x64u64), (0x4u32) as u32); bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), ((xmm6 as u64)) as u64); bb = 39; continue; }
            35 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 36; continue; }
            36 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); bb = 39; continue; }
            37 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x11u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rax); bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x8u64) as u64)) as u64; fl_r = fl_l; bb = 47; continue; }
            40 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 51; continue; } bb = 41; continue; }
            41 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 42; continue; }
            42 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 45; continue; }
            44 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 45; continue; }
            45 => { rcx = ((rdi as u32)) as u32 as u64; r9 = r14; r8 = rbx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 46; continue; }
            46 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 47; continue; }
            47 => { rcx = r14; bb = 48; continue; }
            48 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 49; continue; }
            49 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 50; continue; }
            50 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            51 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 50; continue; }
            52 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 50; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cd754(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fa8; const P1: u64 = 0x142647018; const P2: u64 = 0x142647138; const P3: u64 = 0x1414c1bb4; const P4: u64 = 0x1414d2ef8; const P5: u64 = 0x1426471d0; const P6: u64 = 0x1414c1c1c; const P7: u64 = 0x142647010; const P8: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; rsi = r8; rbx = rdx; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; r8 = rbx; rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; r8 = rbx; rcx = 0x3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64) as u128; bb = 14; continue; }
            13 => { xmm1 = 0u128; bb = 14; continue; }
            14 => { r8 = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 15; continue; }
            15 => { rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 18; continue; }
            18 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 20; continue; }
            20 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x58u64))) as u64; bb = 25; continue; }
            24 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { r8 = rbx; rcx = 0x6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 26; continue; }
            26 => { rbp = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 29; continue; }
            29 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 27; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 31; continue; }
            31 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; r8 = rbx; rcx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 36; continue; }
            36 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cd95c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d6168;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0x9;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rax = rbx;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cda40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183ae96; const P1: u64 = 0x30; const P2: u64 = 0x14183aeba; const P3: u64 = 0x48; const P4: u64 = 0x1414d1adc; const P5: u64 = 0x60; const P6: u64 = 0x1414c0fb8; const P7: u64 = 0x78; const P8: u64 = 0x18; const P9: u64 = 0x14183acc2; const P10: u64 = 0x10; const P11: u64 = 0x90; const P12: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P5)) as u64) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; r8 = ((rdx.wrapping_add(P8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P10))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(P11)) as u64) as u64; r8 = ((rdx.wrapping_add(0x18u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { crate::simd_ops::lk_and_u32(rbx.wrapping_add(0xa8u64), (0x0u32) as u32); rcx = rbx.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cdac8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183ae90; const P1: u64 = 0x1426471d8; const P2: u64 = 0x140226ba0; const P3: u64 = 0x1401de2f0; const P4: u64 = 0x14183aeb4; const P5: u64 = 0x1414cfbb0; const P6: u64 = 0x142647048; const P7: u64 = 0x1414c7364; const P8: u64 = 0x142647028; const P9: u64 = 0x142647118; const P10: u64 = 0x1414ce28c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = rbx.wrapping_add(0x1); fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 3; continue; }
            2 => { rbp = rbx; bb = 3; continue; }
            3 => { rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x28); rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x30); crate::simd_ops::wu32(rax, ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsi.wrapping_add(0x40); rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax, ((rbx as u32)) as u32); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; rax = ((rdi.wrapping_add(rdx)) as u64) as u64; rbp = ((rbp).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; rcx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rcx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 7; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rcx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rcx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 10; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64; fl_r = (0x0u64) as u64; rax = rbp.wrapping_add(0x9); rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rax }) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; rcx = rax.wrapping_add(0x9); rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; fl_r = (0x0u64) as u64; rbx = rcx.wrapping_add(0x9); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rbx }) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = rbx.wrapping_add(0x9); rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rbx } else { rax }) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = rax.wrapping_add(0x9); rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((rbx).wrapping_add(0x9u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r8 = rsi.wrapping_add(0x14); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            18 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cdc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1426471f8; const P2: u64 = 0x1414c29bc; const P3: u64 = 0x14183ae4e; const P4: u64 = 0x1414c85b0; const P5: u64 = 0x142647200; const P6: u64 = 0x14183ae54; const P7: u64 = 0x1414c29fc; const P8: u64 = 0x1414bb6c4; const P9: u64 = 0x1414c247c; const P10: u64 = 0x1414c1b9c; const P11: u64 = 0x1414d193c; const P12: u64 = 0x1414d3a68; const P13: u64 = 0x1414d2ef0; const P14: u64 = 0x1414c2ca4; const P15: u64 = 0x1426470d8; const P16: u64 = 0x142647060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rcx; r14 = r8; rcx = r8; r15 = 0; fl_l = 0; fl_r = 0; bb = 61; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x10u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 30; continue; } bb = 7; continue; }
            7 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x32u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rsi.wrapping_add(0x18); r8 = r14; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 59; continue; }
            15 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x31u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 16; continue; }
            16 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x18); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rax); rdx = rbp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            17 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 18; continue; }
            18 => { r15 = ((((r15 as u32)) | (0x1u32))) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x90u64), ((xmm0 as u64)) as u64); bb = 25; continue; }
            19 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x21u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 20; continue; }
            20 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), rax); bb = 25; continue; }
            21 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 22; continue; }
            22 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x80u64), rax); bb = 25; continue; }
            23 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x11u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 24; continue; }
            24 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x78u64), rax); bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x38u64), (0x8u64) as u64)) as u64; fl_r = fl_l; bb = 60; continue; }
            26 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = rsi.wrapping_add(0x30); r8 = r14; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 59; continue; }
            28 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 29; continue; }
            29 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), ((xmm0 as u64)) as u64); rdx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            30 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 31; continue; }
            31 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 32; continue; }
            32 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 33; continue; }
            33 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 37; continue; } bb = 34; continue; }
            34 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 35; continue; }
            35 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x61u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 36; continue; }
            36 => { r15 = ((((r15 as u32)) | (0x4u32))) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0xa0u64), ((xmm0 as u64)) as u64); bb = 25; continue; }
            37 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x59u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 38; continue; }
            38 => { r15 = ((((r15 as u32)) | (0x2u32))) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x98u64), ((xmm0 as u64)) as u64); bb = 25; continue; }
            39 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x50u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 40; continue; }
            40 => { rdx = rbp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0xa8u64), ((rcx as u32)) as u32); bb = 59; continue; }
            41 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 42; continue; }
            42 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 43; continue; }
            43 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x60); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 44; continue; }
            44 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 60; continue; } bb = 45; continue; }
            45 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 60; continue; } bb = 46; continue; }
            46 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; bb = 43; continue; }
            47 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x42u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 48; continue; }
            48 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 49; continue; }
            49 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x48); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 50; continue; }
            50 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 60; continue; } bb = 51; continue; }
            51 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 60; continue; } bb = 52; continue; }
            52 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; bb = 49; continue; }
            53 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 64; continue; } bb = 54; continue; }
            54 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 64; continue; } bb = 55; continue; }
            55 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 58; continue; }
            57 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 58; continue; }
            58 => { rcx = ((rdi as u32)) as u32 as u64; r9 = r14; r8 = rbx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 59; continue; }
            59 => { crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 60; continue; }
            60 => { rcx = r14; bb = 61; continue; }
            61 => { rdx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 62; continue; }
            62 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; bb = 63; continue; }
            63 => { crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x10u64), ((r15 as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            64 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 63; continue; }
            65 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 63; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cdfdc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fa8; const P1: u64 = 0x142647018; const P2: u64 = 0x142647138; const P3: u64 = 0x14183ae90; const P4: u64 = 0x1414d4428; const P5: u64 = 0x14183aeb4; const P6: u64 = 0x1414d4398; const P7: u64 = 0x1414c1bb4; const P8: u64 = 0x1414d2ef8; const P9: u64 = 0x1414c1c1c; const P10: u64 = 0x142647010; const P11: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; fl_r = (0x0u64) as u64; rdi = r8; rbx = rdx; rsi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64; r8 = rbx; rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x80u64))) as u64; r8 = rbx; rcx = 0x3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64; r8 = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 15; continue; }
            15 => { xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64) as u128; r8 = rbx; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 16; continue; }
            16 => { rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { r9 = rbx; r8 = rsi.wrapping_add(0x18); rdx = 0x6; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 18; continue; }
            18 => { rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r9 = rbx; r8 = rsi.wrapping_add(0x30); rdx = 0x7; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x50u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 23; continue; }
            23 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rsi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rdi; r8 = rbx; rdx = rax; rcx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 21; continue; } bb = 24; continue; }
            24 => { rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x68u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 27; continue; }
            27 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rsi.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rdi; r8 = rbx; rdx = rax; rcx = 0x9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0xa8u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 29; continue; }
            29 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 31; continue; }
            31 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0xa8u64))) as u32 as u64; r8 = rbx; rcx = 0xa; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 35; continue; }
            35 => { xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x98u64))) as u64) as u128; r8 = rbx; rcx = 0xb; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 37; continue; }
            37 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 39; continue; }
            39 => { xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0xa0u64))) as u64) as u128; r8 = rbx; rcx = 0xc; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = rdi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 42; continue; }
            42 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ce298(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d6168;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0xa;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rax = rbx;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ce3a8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426471e8; const P1: u64 = 0x1426471d8; const P2: u64 = 0x1; const P3: u64 = 0x140226ba0; const P4: u64 = 0x20; const P5: u64 = 0x1401de2f0; const P6: u64 = 0x24; const P7: u64 = 0x1426471e0; const P8: u64 = 0x8; const P9: u64 = 0x28; const P10: u64 = 0x142647118; const P11: u64 = 0x1414beae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax.wrapping_add(P2)) as u64) as u64; rax = ((rbp as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(rbp)) as u64; crate::simd_ops::wu32(rax, ((rbx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P6))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rcx = ((rsi.wrapping_add(P8)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rsi.wrapping_add(P9)) as u64) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            6 => { rdx = ((rdi as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rax = rdi; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ce44c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1426471f0; const P2: u64 = 0x1414c2a54; const P3: u64 = 0x14183ae4e; const P4: u64 = 0x1414c2ca4; const P5: u64 = 0x1426470d8; const P6: u64 = 0x142647060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 23; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x10); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 27; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = r14.wrapping_add(0x10); r8 = rsi; rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            10 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 14; continue; } bb = 11; continue; }
            11 => { rdx = rbp.wrapping_add(0x38); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rcx = r14.wrapping_add(0x10); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rdx = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 21; continue; }
            12 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = rbp.wrapping_sub(0x18); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((((rdx as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x24u64), ((rdx as u32)) as u32); bb = 20; continue; }
            14 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 26; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 26; continue; } bb = 16; continue; }
            16 => { rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 19; continue; }
            18 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; fl_k = 4; bb = 21; continue; }
            21 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 27; continue; } bb = 22; continue; }
            22 => { rcx = rsi; bb = 23; continue; }
            23 => { rdx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 25; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            26 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 25; continue; }
            27 => { rbx = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 25; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ce5c4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0x142646fa8; const P2: u64 = 0x1426471c8; const P3: u64 = 0x20; const P4: u64 = 0x1401de2f0; const P5: u64 = 0x142647160; const P6: u64 = 0x10; const P7: u64 = 0x1414d44b8; const P8: u64 = 0x8; const P9: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0; rsi = r8; rbx = rdx; rdi = rcx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r8; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; r8 = rbx; rcx = 0x1; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rcx = ((rdi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(P5)) as u64; r8 = ((rdi.wrapping_add(P6)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = 0x2; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P8))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ce674(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d6168;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0xb;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rax = rbx;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ce758(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c0fb8; const P1: u64 = 0x30; const P2: u64 = 0x1414d1adc; const P3: u64 = 0x142646f28; const P4: u64 = 0x48; const P5: u64 = 0x1414d4eb0; const P6: u64 = 0x141834d30; const P7: u64 = 0x50; const P8: u64 = 0x58; const P9: u64 = 0x18; const P10: u64 = 0x14183acc2; const P11: u64 = 0x70; const P12: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = 0x30; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P4), (0x0u64) as u64); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = 0x30; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P7), (0x0u64) as u64); rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(P9)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_and_u64(rbx.wrapping_add(P11), (0x0u64) as u64); rcx = rbx.wrapping_add(0x78); rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rdx.wrapping_add(0x10u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx.wrapping_add(0x88); r8 = ((rdx.wrapping_add(0x10u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx.wrapping_add(0x10); r8 = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ce83c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c7364; const P1: u64 = 0x142647048; const P2: u64 = 0x1414cfbb0; const P3: u64 = 0x143813530; const P4: u64 = 0x1414ce3a8; const P5: u64 = 0x1426471e0; const P6: u64 = 0x142647028; const P7: u64 = 0x142647118; const P8: u64 = 0x1414ce28c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rcx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rcx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = P3; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 10; continue; }
            9 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; fl_r = (0x0u64) as u64; rax = rbp.wrapping_add(0x9); rdi = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rax }) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; fl_r = (0x0u64) as u64; rcx = rax.wrapping_add(0x9); rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64; fl_r = (0x0u64) as u64; rdx = rcx.wrapping_add(0x9); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rdx }) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = rdx.wrapping_add(0x9); rax = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64; fl_r = (0x0u64) as u64; rbx = rax.wrapping_add(0x9); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rax } else { rbx }) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { rax = rbx; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = rbx.wrapping_add(0x9); rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax } else { rbx }) as u64; fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = ((rbx).wrapping_add(0x9u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 19; continue; }
            19 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { r8 = rsi.wrapping_add(0x14); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            21 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ce9e4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c2a54; const P2: u64 = 0x1414c85b0; const P3: u64 = 0x1414c29bc; const P4: u64 = 0x1414bb6c4; const P5: u64 = 0x1414c247c; const P6: u64 = 0x1414c1b7c; const P7: u64 = 0x1414d193c; const P8: u64 = 0x1414d3a68; const P9: u64 = 0x1414c1ba4; const P10: u64 = 0x1414c29fc; const P11: u64 = 0x142646f28; const P12: u64 = 0x1414d1ec4; const P13: u64 = 0x1414d3c60; const P14: u64 = 0x1414c2ca4; const P15: u64 = 0x1426470d8; const P16: u64 = 0x142647060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; r14 = r8; rcx = r8; r15 = 0; fl_l = 0; fl_r = 0; bb = 68; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rbx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdi = rdx.wrapping_add(0x1); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 72; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 32; continue; } bb = 7; continue; }
            7 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 15; continue; }
            15 => { rdx = rbp.wrapping_add(0x48); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x1u32))) as u32 as u64; rcx = (0u32.wrapping_sub((rcx as u32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((((rdx as u32)) ^ ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x80u64), ((rdx as u32)) as u32); bb = 66; continue; }
            16 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 17; continue; }
            17 => { r15 = ((((r15 as u32)) | (0x1u32))) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), ((xmm0 as u64)) as u64); bb = 40; continue; }
            18 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x21u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 19; continue; }
            19 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), rax); bb = 40; continue; }
            20 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 21; continue; }
            21 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), rax); bb = 40; continue; }
            22 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x11u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 23; continue; }
            23 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), rax); bb = 40; continue; }
            24 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 26; continue; }
            26 => { rdi = ((rdi).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x18); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 72; continue; } bb = 27; continue; }
            27 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 67; continue; } bb = 28; continue; }
            28 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 67; continue; } bb = 29; continue; }
            29 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; bb = 26; continue; }
            30 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x39u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 31; continue; }
            31 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x78u64), rax); bb = 40; continue; }
            32 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 56; continue; } bb = 33; continue; }
            33 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 34; continue; }
            34 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 49; continue; } bb = 35; continue; }
            35 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 43; continue; } bb = 36; continue; }
            36 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 37; continue; }
            37 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 38; continue; }
            38 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x69u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 39; continue; }
            39 => { r15 = ((((r15 as u32)) | (0x4u32))) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x90u64), ((xmm0 as u64)) as u64); bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x38u64), (0x8u64) as u64)) as u64; fl_r = fl_l; bb = 67; continue; }
            41 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x61u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 42; continue; }
            42 => { r15 = ((((r15 as u32)) | (0x2u32))) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x88u64), ((xmm0 as u64)) as u64); bb = 40; continue; }
            43 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x5au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 44; continue; }
            44 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 45; continue; }
            45 => { rdi = ((rdi).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x30); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 72; continue; } bb = 46; continue; }
            46 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 67; continue; } bb = 47; continue; }
            47 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 67; continue; } bb = 48; continue; }
            48 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; bb = 45; continue; }
            49 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x50u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 50; continue; }
            50 => { rdx = rbp.wrapping_add(0x30); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x84u64), ((rcx as u32)) as u32); bb = 66; continue; }
            51 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 52; continue; }
            52 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x50u64), rax); bb = 54; continue; }
            54 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; bb = 55; continue; }
            55 => { r8 = rdi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 66; continue; }
            56 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x42u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 60; continue; } bb = 57; continue; }
            57 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 59; continue; } bb = 58; continue; }
            58 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); bb = 59; continue; }
            59 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; bb = 55; continue; }
            60 => { rax = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 71; continue; } bb = 61; continue; }
            61 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 71; continue; } bb = 62; continue; }
            62 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 64; continue; } bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 65; continue; }
            64 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 65; continue; }
            65 => { rcx = ((rbx as u32)) as u32 as u64; r9 = r14; r8 = rdi; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 66; continue; }
            66 => { crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 72; continue; } bb = 67; continue; }
            67 => { rcx = r14; bb = 68; continue; }
            68 => { rdx = rbp.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 69; continue; }
            69 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; bb = 70; continue; }
            70 => { crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x10u64), ((r15 as u32)) as u32); rax = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            71 => { rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 70; continue; }
            72 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 70; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ced68(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fa8; const P1: u64 = 0x1414c1bb4; const P2: u64 = 0x1414c1c1c; const P3: u64 = 0x142647018; const P4: u64 = 0x142647138; const P5: u64 = 0x1426471c8; const P6: u64 = 0x143813530; const P7: u64 = 0x1414d61b4; const P8: u64 = 0x1414d3000; const P9: u64 = 0x1414af96c; const P10: u64 = 0x142647010; const P11: u64 = 0x1414d2ef8; const P12: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r15 = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; rdi = r8; rbx = rdx; rsi = rcx; r14 = ((rbp as u32)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 3; continue; }
            3 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rsi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rdi; r8 = rbx; rdx = rax; rcx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; r8 = rbx; rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; r8 = rbx; rcx = 0x3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64; r8 = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 19; continue; }
            19 => { xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64) as u128; r8 = rbx; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x80u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x80u64))) as u32 as u64; r8 = rbx; rcx = 0x6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 25; continue; }
            25 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x78u64))) as u64; r8 = rbx; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 28; continue; }
            28 => { rax = P6; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 30; continue; }
            30 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 32; continue; }
            32 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; r9 = rdi; r8 = rbx; rcx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 34; continue; }
            33 => { fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 35; continue; }
            35 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 37; continue; }
            37 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; r9 = rdi; r8 = rbx; rcx = 0x9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 39; continue; }
            39 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 41; continue; }
            41 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x84u64))) as u32 as u64; r8 = rbx; rcx = 0xa; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 42; continue; }
            42 => { r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 46; continue; } bb = 43; continue; }
            43 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 45; continue; }
            45 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rsi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rdi; r8 = rbx; rdx = rax; rcx = 0xb; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 43; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 47; continue; }
            47 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 49; continue; }
            49 => { xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x88u64))) as u64) as u128; r8 = rbx; rcx = 0xc; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 54; continue; } bb = 51; continue; }
            51 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 53; continue; }
            53 => { xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64) as u128; r8 = rbx; rcx = 0xd; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = rdi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 56; continue; }
            56 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cf098(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d6168;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0xc;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rax = rbx;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cf17c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x14183acc2; const P2: u64 = 0x8; const P3: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cf1a8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x20; const P4: u64 = 0x142647118; const P5: u64 = 0x1414c4874;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; rbx = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rbx = 0x9; bb = 3; continue; }
            3 => { xmm1 = ((crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64) as u128; fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbx).wrapping_add(0x9u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { r8 = ((rcx.wrapping_add(P3)) as u64) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            8 => { rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cf20c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c85b0; const P2: u64 = 0x1414c2ca4; const P3: u64 = 0x1426470d8; const P4: u64 = 0x142647060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 20; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x11u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), ((xmm0 as u64)) as u64); bb = 12; continue; }
            10 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x9u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), ((xmm0 as u64)) as u64); bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x8u64) as u64)) as u64; fl_r = fl_l; bb = 19; continue; }
            13 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 14; continue; }
            14 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 15; continue; }
            15 => { rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 18; continue; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 19; continue; }
            19 => { rcx = rsi; bb = 20; continue; }
            20 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 22; continue; }
            22 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 22; continue; }
            24 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 22; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cf340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x142646fa8; const P2: u64 = 0x142647138; const P3: u64 = 0x18; const P4: u64 = 0x8; const P5: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64) as u128; rbx = r8; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rdi = rcx; xmm6 = 0u128; fl_l = ((xmm6 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64) as u128; r8 = rdx; rcx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64) as u128; fl_l = ((xmm6 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm6 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 9; continue; }
            9 => { xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64) as u128; r8 = rdx; rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P4))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; r8 = rbx; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rdx; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cf3fc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d6168;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0xd;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rax = rbx;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cf4e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d1d28; const P1: u64 = 0x28; const P2: u64 = 0x1414c0fb8; const P3: u64 = 0x40; const P4: u64 = 0x24; const P5: u64 = 0x14183acc2; const P6: u64 = 0x8; const P7: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
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
pub unsafe extern "win64" fn fn_1414cf520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414cf1a8; const P1: u64 = 0x142647048; const P2: u64 = 0x1414c7364; const P3: u64 = 0x142647028; const P4: u64 = 0x142647118; const P5: u64 = 0x1414cad80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; rcx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rcx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 5; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64) as u128; rcx = rbp.wrapping_add(0x9); rdx = 0; fl_l = 0; fl_r = 0; xmm1 = 0u128; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; fl_r = rdx; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rcx }) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_r = rdx; rax = rcx.wrapping_add(0x9); rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; fl_r = rdx; rbx = rax.wrapping_add(0x9); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rax } else { rbx }) as u64; fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbx).wrapping_add(0x9u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = rsi.wrapping_add(0x64); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            13 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = rbx; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cf63c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414c29fc; const P2: u64 = 0x1414bb6c4; const P3: u64 = 0x1414c247c; const P4: u64 = 0x1414d2ee8; const P5: u64 = 0x1414d1a74; const P6: u64 = 0x1414d42f0; const P7: u64 = 0x1414cb3d0; const P8: u64 = 0x1414c85b0; const P9: u64 = 0x1414c29bc; const P10: u64 = 0x1414c2ca4; const P11: u64 = 0x1426470d8; const P12: u64 = 0x142647060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; r14 = r8; rcx = r8; bb = 44; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 14; continue; }
            14 => { rdx = rbp.wrapping_add(0x38); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x60u64), ((rcx as u32)) as u32); bb = 42; continue; }
            15 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 16; continue; }
            16 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x28); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 18; continue; }
            18 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 43; continue; } bb = 19; continue; }
            19 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 17; continue; }
            21 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x32u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 22; continue; }
            22 => { fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 23; continue; }
            23 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 24; continue; }
            24 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 43; continue; } bb = 25; continue; }
            25 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 23; continue; }
            27 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x29u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 28; continue; }
            28 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x58u64), ((xmm0 as u64)) as u64); bb = 35; continue; }
            29 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x21u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 30; continue; }
            30 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x50u64), rax); bb = 35; continue; }
            31 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 32; continue; }
            32 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); bb = 35; continue; }
            33 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x11u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rax); bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x8u64) as u64)) as u64; fl_r = fl_l; bb = 43; continue; }
            36 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 47; continue; } bb = 37; continue; }
            37 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 47; continue; } bb = 38; continue; }
            38 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 41; continue; }
            40 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 41; continue; }
            41 => { rcx = ((rdi as u32)) as u32 as u64; r9 = r14; r8 = rbx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            42 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 43; continue; }
            43 => { rcx = r14; bb = 44; continue; }
            44 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 45; continue; }
            45 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 46; continue; }
            46 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            47 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 46; continue; }
            48 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 46; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cf89c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fa8; const P1: u64 = 0x142647018; const P2: u64 = 0x142647138; const P3: u64 = 0x1414c1bb4; const P4: u64 = 0x1414d3370; const P5: u64 = 0x1414c1c1c; const P6: u64 = 0x142647010; const P7: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_r = (0x0u64) as u64; rdi = r8; rbx = rdx; rsi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; r8 = rbx; rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; r8 = rbx; rcx = 0x3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; r8 = rbx; rcx = 0x4; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 12; continue; }
            12 => { xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64) as u128; xmm1 = 0u128; fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64; if !(f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = ((xmm0 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if (f64::from_bits(fl_l) >= f64::from_bits(fl_r)) { bb = 17; continue; } bb = 14; continue; }
            14 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 16; continue; }
            16 => { xmm1 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64) as u128; r8 = rbx; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 17; continue; }
            17 => { rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 20; continue; }
            20 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rdi; r8 = rbx; rdx = rax; rcx = 0x6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 21; continue; }
            21 => { rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 24; continue; }
            24 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rsi.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rdi; r8 = rbx; rdx = rax; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 22; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rdx = rbx; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32 as u64; r8 = rbx; rcx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = rdi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 31; continue; }
            31 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cfa84(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d6168;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0xe;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rax = rbx;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cfb68(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c0fb8; const P1: u64 = 0x28; const P2: u64 = 0x142646f58; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x1414d6158; const P6: u64 = 0x8; const P7: u64 = 0x1414c184c;
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
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P4), (0x0u64) as u64);
    rcx = rbx;
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
pub unsafe extern "win64" fn fn_1414cfbb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414c7364; const P1: u64 = 0x142647048; const P2: u64 = 0x142646fc8; const P3: u64 = 0x142647040; const P4: u64 = 0x142647118; const P5: u64 = 0x1414d00ac;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rsi = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rdx = rax.wrapping_add(0x8); fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rbp as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((rbp).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = rsi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rsi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rsi.wrapping_add(0x30); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbp).wrapping_add(rax)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; fl_r = rdi; rbx = rbp.wrapping_add(0x9); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rbx }) as u64; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbx).wrapping_add(0x9u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 10; continue; }
            10 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = rsi.wrapping_add(0x48); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            12 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cfcac(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414bb6c4; const P2: u64 = 0x1414c247c; const P3: u64 = 0x1414d2ee8; const P4: u64 = 0x1414c29bc; const P5: u64 = 0x1414d6158; const P6: u64 = 0x142646f28; const P7: u64 = 0x142646fd0; const P8: u64 = 0x142647090; const P9: u64 = 0x1414c85b0; const P10: u64 = 0x1414c2ca4; const P11: u64 = 0x1426470d8; const P12: u64 = 0x142647060;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rcx = r8; r14 = r8; bb = 42; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rbx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rdi = rdx.wrapping_add(0x1); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x18u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 13; continue; }
            13 => { fl_l = ((rdi).wrapping_sub(1)) as u64; rdi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rdi = ((rdi).wrapping_add(1)) as u64; rcx = rsi.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rdi; rdx = rax; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 15; continue; }
            15 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 41; continue; } bb = 16; continue; }
            16 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 17; continue; }
            17 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 14; continue; }
            18 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x31u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 19; continue; }
            19 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = 0x6; rbx = rax; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x4cu64), (0x6u32) as u32); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), rbx); bb = 33; continue; }
            22 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x2au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 23; continue; }
            23 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x30); bb = 24; continue; }
            24 => { r8 = rax; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 40; continue; }
            25 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x22u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 26; continue; }
            26 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x28); bb = 24; continue; }
            27 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 28; continue; }
            28 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = 0x3; xmm6 = (xmm0) as u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsi.wrapping_add(0x4cu64), (0x3u32) as u32); bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), ((xmm6 as u64)) as u64); bb = 33; continue; }
            31 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x11u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rax); bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_add(0x28u64), (0x8u64) as u64)) as u64; fl_r = fl_l; bb = 41; continue; }
            34 => { rax = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 45; continue; } bb = 35; continue; }
            35 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 45; continue; } bb = 36; continue; }
            36 => { rcx = rsi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 39; continue; }
            38 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { rcx = ((rbx as u32)) as u32 as u64; r9 = r14; r8 = rdi; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 40; continue; }
            40 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 41; continue; }
            41 => { rcx = r14; bb = 42; continue; }
            42 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 43; continue; }
            43 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 44; continue; }
            44 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            45 => { rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 44; continue; }
            46 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 44; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414cfef8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fa8; const P1: u64 = 0x142647018; const P2: u64 = 0x142647138; const P3: u64 = 0x142646fc8; const P4: u64 = 0x142646fa0; const P5: u64 = 0x1426471d0; const P6: u64 = 0x1414c1bb4; const P7: u64 = 0x1414c1c1c; const P8: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rsi = r8; rbx = rdx; rdi = rcx; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; r8 = rbx; rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm1 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64) as u128; bb = 10; continue; }
            9 => { xmm1 = 0u128; bb = 10; continue; }
            10 => { r8 = rbx; rcx = 0x3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 11; continue; }
            11 => { rcx = rdi.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rdi.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rbx; rdx = 0x4; r8 = rax; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 13; continue; }
            13 => { rcx = rdi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rdi.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rbx; rdx = 0x5; r8 = rax; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; bb = 21; continue; }
            20 => { rdx = rbp; bb = 21; continue; }
            21 => { r8 = rbx; rcx = 0x6; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 22; continue; }
            22 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rdx = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 25; continue; }
            25 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x7; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 23; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 28; continue; }
            28 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d00b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1414d6168;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0xf;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rax = rbx;
    crate::simd_ops::wu128(rbx, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d0d80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646f58; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x1414d6050; const P4: u64 = 0x8; const P5: u64 = 0x1414c184c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P4)) as u64) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d0dc4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x142647038; const P2: u64 = 0x1414cd000; const P3: u64 = 0x1414ccc04; const P4: u64 = 0x1414cc804; const P5: u64 = 0x1414cc380; const P6: u64 = 0x1414d1690; const P7: u64 = 0x142647048; const P8: u64 = 0x142647118; const P9: u64 = 0x1414cc6e8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rbx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax.wrapping_add(0x1); bb = 2; continue; }
            2 => { rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdi.wrapping_add(0x18); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 4; continue; }
            4 => { rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi.wrapping_add(0x20); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rbx).wrapping_add(rax)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 17; continue; }
            17 => { rcx = rdi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = rdi.wrapping_add(0x30); rdx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            19 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d0ee8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x1414d6050; const P2: u64 = 0x142646f28; const P3: u64 = 0x1414d2300; const P4: u64 = 0x1414d41a0; const P5: u64 = 0x1414d1de8; const P6: u64 = 0x1414d3b10; const P7: u64 = 0x1414d1f9c; const P8: u64 = 0x1414d3db0; const P9: u64 = 0x1414d2294; const P10: u64 = 0x1414d40f8; const P11: u64 = 0x1414d1f30; const P12: u64 = 0x1414d3d08; const P13: u64 = 0x142646fd0; const P14: u64 = 0x142647090; const P15: u64 = 0x142670b60; const P16: u64 = 0x142647088; const P17: u64 = 0x142670b28; const P18: u64 = 0x142670af8; const P19: u64 = 0x1414c2ca4; const P20: u64 = 0x1426470d8; const P21: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0; let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rdi = rcx; r14 = r8; rcx = r8; bb = 57; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rbx = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rsi = rdx.wrapping_add(0x1); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rsi)) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = rdx.wrapping_add(0x2); bb = 5; continue; }
            4 => { r8 = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rsi); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 61; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 44; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 9; continue; }
            9 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 10; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 12; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 48; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x5au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0xb; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), (0xbu32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            18 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x52u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), (0xau32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            22 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x4au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), (0x9u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            26 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), (0x7u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            30 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x2au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x34u64), (0x5u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            34 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 35; continue; }
            35 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x20); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rdx = P15; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x40u64))) as u128; bb = 38; continue; }
            38 => { rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 61; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; fl_k = 1; bb = 55; continue; }
            40 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 41; continue; }
            41 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x18); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 43; continue; }
            43 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); rdx = P17; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x30u64))) as u128; bb = 38; continue; }
            44 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 48; continue; } bb = 45; continue; }
            45 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = r14; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 47; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 47; continue; }
            47 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P18; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; bb = 38; continue; }
            48 => { rax = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 60; continue; } bb = 49; continue; }
            49 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 60; continue; } bb = 50; continue; }
            50 => { rcx = rdi.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 53; continue; }
            52 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            53 => { rcx = ((rbx as u32)) as u32 as u64; r9 = r14; r8 = rsi; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P20), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            54 => { fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_k = 4; bb = 55; continue; }
            55 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 61; continue; } bb = 56; continue; }
            56 => { rcx = r14; bb = 57; continue; }
            57 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P21), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 58; continue; }
            58 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 59; continue; }
            59 => { r11 = rsp.wrapping_add(0x70); rax = rsi; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            60 => { rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 59; continue; }
            61 => { rsi = 0; fl_l = 0; fl_r = 0; fl_k = 3; bb = 59; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d12c4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fc8; const P1: u64 = 0x142670af8; const P2: u64 = 0x142647000; const P3: u64 = 0x142646f98; const P4: u64 = 0x142670b28; const P5: u64 = 0x142670b60; const P6: u64 = 0x142646fa8; const P7: u64 = 0x1414d61ac; const P8: u64 = 0x1414d3058; const P9: u64 = 0x1414d3268; const P10: u64 = 0x1414d30b0; const P11: u64 = 0x1414d2f50; const P12: u64 = 0x1414d32c0; const P13: u64 = 0x142647110;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx.wrapping_add(0x10); rsi = rcx; rcx = rbp; rdi = r8; rbx = rdx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { r9 = P1; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x1; r8 = rax; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rbp = rsi.wrapping_add(0x18); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { r9 = P4; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x2; r8 = rax; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { rbp = rsi.wrapping_add(0x20); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax)) as u64; bb = 11; continue; }
            11 => { r9 = P5; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x3; r8 = rax; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rdi; r8 = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 17; continue; }
            17 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 19; continue; }
            19 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rdi; r8 = rbx; rcx = 0x7; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x9; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rdi; r8 = rbx; rcx = 0x9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0xa; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 25; continue; }
            25 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 27; continue; }
            27 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rdi; r8 = rbx; rcx = 0xa; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0xb; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 29; continue; }
            29 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 31; continue; }
            31 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rdi; r8 = rbx; rcx = 0xb; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r8 = rdi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 34; continue; }
            34 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d1e54(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141834d64; const P1: u64 = 0x1414d46e4; const P2: u64 = 0x14266f250; const P3: u64 = 0x8; const P4: u64 = 0x142646fb8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1414d1e54, funcinfo_rva: 0x36bb9d4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (0x98u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414d1e67); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414d1e7e); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rbx, (rax) as u64); bb = 4; continue; }
            3 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (0x98u32) as u32 as u64; r8 = ((r9.wrapping_add(P3)) as u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414d1ea5); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8(({ let t = (crate::simd_ops::ru64(P4)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdx = (rbx) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1414d1eb1); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; bb = 6; continue; }
            6 => { rax = (rbx) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d2444(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1414d4d04; const P5: u64 = 0x141834d30; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r14 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x50; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d24d8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1414d4df8; const P5: u64 = 0x141834d30; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r14 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x98; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d2600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1414d504c; const P5: u64 = 0x141834d30; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r14 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x38; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d2ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x2au8) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            1 => { crate::lift_ops::trap(0x80000003, 0x1414d2ee7, rcx, 0); return rax; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x3au8) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d2ee8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x3au8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d2ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x42u8) as u8 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d44b8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142646fa8; const P1: u64 = 0x142647158; const P2: u64 = 0x14183aea2; const P3: u64 = 0x14183ae90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = ((r9 as u32)) as u32 as u64; rbp = r8; rsi = ((rdx as u32)) as u32 as u64; rdi = rcx; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { r9 = rax; r8 = ((rbx as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rbx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rsi = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r14 = 0x80; rbp = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rsi = ((rsi).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x1); rcx = rax; fl_l = rax; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx, ((rcx as u8)) as u8); rbx = rdx; bb = 12; continue; }
            7 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | ((r14 as u8)))) as u8 as u64)); rcx = ((rcx).wrapping_shr(7u32)) as u64; crate::simd_ops::wu8(rbx, ((rax as u8)) as u8); fl_l = rcx; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = ((rbx).wrapping_add(0x2u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 11; continue; }
            9 => { rax = (rax & !0xffu64) | ((((rcx as u8)) as u8 as u64)); rcx = ((rcx).wrapping_shr(7u32)) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | ((r14 as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdx, ((rax as u8)) as u8); rdx = ((rdx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rbx = rdx.wrapping_add(0x1); bb = 11; continue; }
            11 => { crate::simd_ops::wu8(rdx, ((rcx as u8)) as u8); bb = 12; continue; }
            12 => { fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d45a8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266f430; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x48; const P7: u64 = 0x1426470a8; const P8: u64 = 0x142646f50; const P9: u64 = 0x28; const P10: u64 = 0x142646f48; const P11: u64 = 0x30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(P4), (0x0u32) as u32);
    crate::simd_ops::lk_and_u64(rdi.wrapping_add(P5), (0x0u64) as u64);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rdi.wrapping_add(P9)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rdi.wrapping_add(P11)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u64(rdi.wrapping_add(0x38u64), (0x0u64) as u64);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(0x4cu64), (0x0u32) as u32);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d4628(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f0; const P1: u64 = 0x14266f430; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20; const P6: u64 = 0x28; const P7: u64 = 0x142646f60; const P8: u64 = 0x142646fc0; const P9: u64 = 0x30; const P10: u64 = 0x38; const P11: u64 = 0x48; const P12: u64 = 0x1426470a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rbx, rax);
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P2), (0x0u64) as u64);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P4), (0x0u32) as u32);
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P5), (0x0u64) as u64);
    rcx = ((rbx.wrapping_add(P6)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(P7)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P9)) as u64) as u64;
    rdx = (crate::simd_ops::ru64(P7)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P10), (0x0u64) as u64);
    rcx = ((rbx.wrapping_add(P11)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((rax as u32)) as u32);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d46e4(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266f250; const P2: u64 = 0x14; const P3: u64 = 0x10; const P4: u64 = 0x1426470a8; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x48; const P8: u64 = 0x28; const P9: u64 = 0x30; const P10: u64 = 0x50; const P11: u64 = 0x38; const P12: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(P6), rax);
    rcx = ((rdi.wrapping_add(P7)) as u64) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P8), rax);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P9), rbx);
    r8 = ((rax.wrapping_add(P10)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rax);
    rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d474c(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426470f8; const P1: u64 = 0x14266f1b0; const P2: u64 = 0x10; const P3: u64 = 0x14183ae84; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x1426470a8; const P7: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rdx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32);
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(P7), (0x0u32) as u32);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}
