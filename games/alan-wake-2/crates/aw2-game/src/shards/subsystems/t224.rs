#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b6090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d98; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x1415b76b0; const P4: u64 = 0x1415b85e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rbx); r15 = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rsi); rcx = P0; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rdi); r13 = 0; fl_l = 0; fl_r = 0; rdi = rdx; r12 = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = r15.wrapping_add(0xa8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_shl(3u32)) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (0xffffffffffffff0u64) as u64; bb = 3; continue; }
            3 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rsp = ((rsp).wrapping_sub(rcx)) as u64; r12 = rsp.wrapping_add(0x20); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r11 = 0; fl_l = 0; fl_r = 0; r10 = rsi; rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 9; continue; }
            8 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 9; continue; }
            9 => { r14 = r12; bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(r14, rcx); r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r14 = ((r14).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rbp, r10); r8 = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r11); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdi = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; bb = 13; continue; }
            13 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), (0x0u64) as u64); rdx = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((r8 as u32)) as u32); bb = 16; continue; }
            16 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); bb = 19; continue; }
            18 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r9.wrapping_mul(8)), rbx); bb = 19; continue; }
            19 => { xmm1 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; xmm2 = (crate::simd_ops::ru128(rbp)) as u128; xmm0 = (xmm1) as u128; rcx = ((xmm1 as u64)) as u64; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); r10 = ((xmm2 as u64)) as u64; r11 = ((xmm0 as u64)) as u64; xmm0 = (xmm2) as u128; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); r9 = ((xmm0 as u32)) as u32 as u64; bb = 20; continue; }
            20 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; bb = 22; continue; }
            22 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((r13.wrapping_sub(0x1u64)) as u32) as u32 as u64; rbx = ((((rax as u32)) as i32 as i64) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(r12.wrapping_add(rbx.wrapping_mul(8)))) as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((((fl_l as u64)).wrapping_sub((fl_r as u64))) as i64) >= 0) { bb = 23; continue; } bb = 24; continue; }
            24 => { r8 = 0x1; rdx = rdi; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; rsp = rbp.wrapping_add(0x20); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            25 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; r11 = rcx; rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 26; continue; }
            26 => { rdx = (crate::simd_ops::ru32(r10.wrapping_add(0x8u64))) as u32 as u64; bb = 27; continue; }
            27 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(r10)) as u64; r11 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 29; continue; }
            29 => { bb = 10; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b6290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1415b16f0; const P2: u64 = 0x1415bcd90; const P3: u64 = 0x1415bcaa0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rsi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rsi = ((rsi).wrapping_add(0x108u64)) as u64; r15 = rcx; rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); bb = 7; continue; }
            7 => { r14 = rdi; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 8; continue; }
            8 => { rbp = rdi.wrapping_add(0x14); crate::simd_ops::lk_add_u32(rbp, (1) as u32); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((r12 as u32)) as u32); rcx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbp, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; bb = 13; continue; }
            13 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbp, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_add_u32(rsi, (1) as u32)) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if (((fl_l as u32)) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((r12 as u32)) as u32); bb = 22; continue; }
            22 => { rcx = rsi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x38u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            23 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 24; continue; }
            24 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32 as u64; bb = 25; continue; }
            25 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 27; continue; }
            27 => { bb = 7; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b6400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r15 = r8; r14 = rdx; rsi = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, r10); crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), ((r10 as u8)) as u8); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rax = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rdx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); bb = 4; continue; }
            4 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rdx))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r15.wrapping_add(rax))) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { bb = 10; continue; }
            9 => { rax = ((r10 as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rdi = ((rax as u32)) as u32 as u64; rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); r13 = ((rdx as u32)) as u32 as u64; fl_l = r13; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 17; continue; } bb = 11; continue; }
            11 => { rcx = ((r13 as u32)) as u32 as u64; rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_sub(rdi)) as u64; r8 = ((r10 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(r14)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 12; continue; }
            12 => { r9 = r15; fl_l = r9; fl_r = rcx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rcx))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((r8 as u32)) as u32 as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { bb = 18; continue; }
            16 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            17 => { r10 = rdi; bb = 18; continue; }
            18 => { r12 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r12 = ((r12).wrapping_add(r10)) as u64; rdx = r12; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { r8 = r13; rdx = r14; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = ((rbx.wrapping_add(r13)) as u64) as u64; r8 = rdi; rdx = r15; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { crate::simd_ops::wu8(rbx.wrapping_add(r12).wrapping_sub(0x1u64), (0x0u8) as u8); rax = 0x1; crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), (0x1u8) as u8); bb = 22; continue; }
            22 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            23 => { rax = 0x34; bb = 22; continue; }
            24 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b6550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x142645718; const P3: u64 = 0x1415b5fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r15 = r8; rdi = rdx; rsi = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsi, r13); crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), ((r13 as u8)) as u8); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rbx = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); r8 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); bb = 4; continue; }
            4 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(r8.wrapping_mul(2)))) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = rbx; bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r15.wrapping_add(rcx.wrapping_mul(2)))) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 7; continue; } bb = 8; continue; }
            8 => { bb = 10; continue; }
            9 => { rcx = ((r13 as u32)) as u32 as u64; bb = 10; continue; }
            10 => { r10 = ((rcx as u32)) as u32 as u64; r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); rax = ((r8 as u32)) as u32 as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 16; continue; } bb = 11; continue; }
            11 => { rax = ((rax).wrapping_sub(r10)) as u64; r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r9 = ((r13 as u32)) as u32 as u64; rdx = ((rdi.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { r11 = r15; fl_l = r11; fl_r = rdx; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru16(r11.wrapping_add(rdx))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rdx)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 23; continue; } bb = 14; continue; }
            14 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rax = ((r9 as u32)) as u32 as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { rcx = ((r13 as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rbp = ((rcx as u32)) as u32 as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rbp = (if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { r13 } else { rbp }) as u64; rbp = ((rbp).wrapping_add(rcx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r12 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 17; continue; }
            17 => { rcx = ((rbp.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = rbx; bb = 18; continue; }
            18 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 18; continue; } bb = 19; continue; }
            19 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 21; continue; } bb = 22; continue; }
            22 => { bb = 25; continue; }
            23 => { r14 = (r14 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 16; continue; }
            24 => { rbx = ((rcx as u32)) as u32 as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r8 = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbp as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rcx = 0xfde9; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r12.wrapping_add(rcx), ((r13 as u8)) as u8); r8 = ((r12.wrapping_add(rcx)) as u64) as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = ((rbp as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { crate::simd_ops::wu8(r12.wrapping_add(rbp).wrapping_sub(0x1u64), ((r13 as u8)) as u8); rax = 0x1; crate::simd_ops::wu64(rsi, r12); crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), (0x1u8) as u8); bb = 28; continue; }
            28 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            29 => { rax = 0x34; bb = 28; continue; }
            30 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b66f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158e930; const P1: u64 = 0x14183acbc; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1415b16f0; const P4: u64 = 0x14158eaa0; const P5: u64 = 0x1415b1830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = rcx; rcx = 0xffff; r12 = r8; r13 = ((rdx as u32)) as u32 as u64; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(r10)) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = 0x40001; bb = 5; continue; }
            2 => { rcx = 0x3040; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = 0x130001; bb = 5; continue; }
            4 => { rcx = 0x3041; rdi = 0x1; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; rdx = 0x140001; rdi = (if (((fl_l as u16)) == ((fl_r as u16))) { (rdx as u32) } else { (rdi as u32) }) as u32 as u64; bb = 5; continue; }
            5 => { rcx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((r15.wrapping_add(0x34u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rdx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); rcx = rbp; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; rdx = ((rsi.wrapping_add(rbp)) as u64) as u64; rax = 0xfffe; r14 = rdx.wrapping_sub(0x8); crate::simd_ops::wu16(rcx.wrapping_add(rbp), ((rax as u16)) as u16); rax = 0x10; crate::simd_ops::wu16(rcx.wrapping_add(rbp).wrapping_add(0xeu64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x90u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(rbp).wrapping_add(0xcu64), ((rax as u16)) as u16); rcx = ((rsi as u32)) as u32 as u64; fl_l = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(r14)) as u64; r8 = 0x4; r8 = (((r8 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(r14, (0x4b4e554au32) as u32); rcx = rdx.wrapping_add(0x8); crate::simd_ops::wu32(r14.wrapping_add(0x4u64), ((r8 as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = (((rsi as u32)).wrapping_add(0x8u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(crate::simd_ops::ru32(r14.wrapping_add(0x4u64)))) as u32 as u64; r14 = ((rsi as u32)) as u32 as u64; r14 = ((r14).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = ((r14).wrapping_add(rbp)) as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r14, rbx); bb = 9; continue; }
            9 => { rcx = rsp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(r15)) as u64; rax = r14.wrapping_add(0x4); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r13 = (((r13 as u32)).wrapping_sub((rsi as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; r9 = r12; rdx = ((rsi as u32)) as u32 as u64; r8 = ((r13 as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = r15; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = r15; rcx = ((rdi as u32)) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b6870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1415b77a0; const P2: u64 = 0x141613c30; const P3: u64 = 0x1415bb740; const P4: u64 = 0x1415bb080; const P5: u64 = 0x1415b85e0; const P6: u64 = 0x143931d98; const P7: u64 = 0x1426452b0; const P8: u64 = 0x1416138e0; const P9: u64 = 0x1416139f0; const P10: u64 = 0x1426452a8; const P11: u64 = 0x1433dd4f4; const P12: u64 = 0x1415144a0; const P13: u64 = 0x1415b8830; const P14: u64 = 0x1415b9670; const P15: u64 = 0x1415bc910; const P16: u64 = 0x1415242e0; const P17: u64 = 0x141613870; const P18: u64 = 0x1415b8fa0; const P19: u64 = 0x1415b6290; const P20: u64 = 0x1415b6090; const P21: u64 = 0x1415b86a0; const P22: u64 = 0x1415bb3f0; const P23: u64 = 0x1415b70e0; const P24: u64 = 0x1415bccb0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rbp = ((r11.wrapping_sub(0x48u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x128u64)) as u64; rax = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; rbx = (rdx) as u64; rsi = (rcx) as u64; rdi = (0x1u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xbu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 80; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_add(0x8u64), (r12) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (r13) as u64); r13 = ((P0) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (r14) as u64); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), (r15) as u64); rax = (crate::simd_ops::ru32(r13.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15b7050u64))) as u32 as u64; fl_l = ((rax).wrapping_add(r13)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; j_tgt = (rax) as u64; bb = 0xffff_fffe; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; r8 = ((rbp.wrapping_add(0x58u64)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r14 as u8)) as u8); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), (r14) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r14 as u8)) as u8); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x45u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r15 = ((rdi as u32)) as u32 as u64; rdi = ((rax as u32)) as u32 as u64; bb = 11; continue; }
            4 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; crate::simd_ops::lk_or_u8(r8.wrapping_add(0x68u64), ((rdi as u8)) as u8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rsi.wrapping_add(0xa8u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rdi) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            10 => { rdi = ((r15 as u32)) as u32 as u64; bb = 11; continue; }
            11 => { r8 = ((rdi as u32)) as u32 as u64; rdx = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((r15 as u32)) as u32 as u64; bb = 79; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rcx = ((P6) as u64) as u64; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; xmm0 = (xmm1) as u128; rax = ((xmm1 as u32)) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); rcx = ((xmm0 as u64)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; xmm0 = (xmm1) as u128; xmm1 = crate::simd_ops::vpsrldq(xmm1, 0x4u8); r14 = (if (((fl_l as u32)) == ((fl_r as u32))) { rcx } else { r14 }) as u64; xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); r15 = ((xmm0 as u32)) as u32 as u64; rcx = ((rsi.wrapping_add(0xa8u64)) as u64) as u64; rdi = ((xmm1 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdi as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); rcx = ((rsi.wrapping_add(0xa8u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = (rbx) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = (rsi) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; bb = 79; continue; }
            15 => { rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (0x2u32) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; r8 = ((rdi as u32)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 79; continue; }
            16 => { rcx = ((P6) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; rdi = (0x36u32) as u32 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { r9 = ((r15 as u32)) as u32 as u64; r8 = ((P11) as u64) as u64; rdx = ((rdi.wrapping_sub(0x2au64)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { r8 = ((rdi as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 79; continue; }
            19 => { xmm2 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm3 = (crate::simd_ops::ru128(rdx)) as u128; xmm4 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; xmm5 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu8(rcx.wrapping_add(0xe8u64), ((rdi as u8)) as u8); xmm0 = (xmm2) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm3) as u128); xmm0 = crate::simd_ops::vpsrldq(xmm0, 0x8u8); r12 = ((xmm0 as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm3) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm5) as u128); crate::simd_ops::wu128(rbp, (xmm4) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm4) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm2) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm2) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm1) as u128); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 20; continue; }
            20 => { rbx = ((xmm1 as u64)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm3) as u128); r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm5) as u128); rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm4) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm2) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm1) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 21; continue; }
            21 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 22; continue; }
            22 => { rdx = ((rbx as u32)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 32; continue; }
            23 => { rbx = ((r14 as u32)) as u32 as u64; r13 = ((xmm1 as u64)) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { rax = ((rbx as u32)) as u32 as u64; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = (rsi) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm3) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm5) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm4) as u128); r15 = ((rax.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm2) as u128); r15 = ((r15).wrapping_add(r13)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm1) as u128); r8 = (crate::simd_ops::ru32(r15)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 26; continue; }
            26 => { xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; xmm2 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm3 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x10u64))) as u128; xmm4 = (crate::simd_ops::ru128(rbp)) as u128; xmm5 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 27; continue; }
            27 => { rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 28; continue; }
            28 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 29; continue; }
            29 => { rbx = (r13) as u64; r15 = ((r12 as u32)) as u32 as u64; bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx.wrapping_add(0x4u64)) as u64) as u64; fl_l = (r15) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 31; continue; }
            31 => { rdx = (r13) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (r14) as u64; r13 = ((P0) as u64) as u64; bb = 32; continue; }
            32 => { rax = (((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as i32 as i64) as u64) as u64; r8 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xe8u64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rsi.wrapping_add(0xe0u64), ((r14 as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xbu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 41; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15b7080u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r13)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            34 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 35; continue; }
            35 => { bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru32(r15.wrapping_sub(0x4u64))) as u32 as u64; r15 = ((r15.wrapping_sub(0x4u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (((rbx as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 36; continue; } bb = 37; continue; }
            37 => { bb = 31; continue; }
            38 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r14 = (if (((fl_l as u32)) == ((fl_r as u32))) { r12 } else { r14 }) as u64; bb = 55; continue; }
            39 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; r8 = (if (((fl_l as u32)) == ((fl_r as u32))) { (rbx as u32) } else { (r8 as u32) }) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; r14 = (if (((fl_l as u32)) == ((fl_r as u32))) { r12 } else { r14 }) as u64; bb = 55; continue; }
            40 => { r8 = ((r12 as u32)) as u32 as u64; bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; r14 = (if (((fl_l as u32)) == ((fl_r as u32))) { r12 } else { r14 }) as u64; bb = 55; continue; }
            42 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; r15 = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rdx = ((rbx as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; bb = 49; continue; }
            44 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rbx = ((r14 as u32)) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 48; continue; } bb = 45; continue; }
            45 => { bb = 46; continue; }
            46 => { rax = ((rbx as u32)) as u32 as u64; rcx = (rsi) as u64; rdx = (crate::simd_ops::ru32(r12.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 47; continue; }
            47 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 46; continue; } bb = 48; continue; }
            48 => { rdx = (r12) as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (r14) as u64; bb = 49; continue; }
            49 => { rax = (((crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as i32 as i64) as u64) as u64; r8 = ((r14 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xbu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 54; continue; } bb = 50; continue; }
            50 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15b70b0u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r13)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            51 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; bb = 54; continue; }
            52 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; r8 = (if (((fl_l as u32)) == ((fl_r as u32))) { (rbx as u32) } else { (r8 as u32) }) as u32 as u64; bb = 54; continue; }
            53 => { r8 = ((r15 as u32)) as u32 as u64; bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; r14 = (if (((fl_l as u32)) == ((fl_r as u32))) { r15 } else { r14 }) as u64; bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rcx = ((rsi.wrapping_add(0xf0u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r9 = (r14) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0xecu64), ((rdi as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 79; continue; }
            56 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; bb = 79; continue; }
            57 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rdi as u32)) as u32 as u64; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 79; continue; }
            58 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = ((rax as u32)) as u32 as u64; bb = 79; continue; }
            59 => { xmm2 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r9 = (((rdi as u8)) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; r14 = (r14 & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); r8 = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm2) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm2) as u128); xmm2 = crate::simd_ops::vpsrldq(xmm2, 0xcu8); crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rax = ((xmm2 as u32)) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm1) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rax as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rsi) as u64; rdi = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 79; continue; }
            60 => { xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(0xa8u64)) as u64; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (r14) as u64); xmm2 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm1) as u128); rax = ((xmm2 as u64)) as u64; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; rax = ((rax).wrapping_shr(32u32)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64) as u128; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm2) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 61; continue; }
            61 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 63; continue; } bb = 62; continue; }
            62 => { fl_l = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x70u64), ((rcx as u32)) as u32); bb = 63; continue; }
            63 => { rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 64; continue; }
            64 => { r8 = ((rdi as u32)) as u32 as u64; rdx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 79; continue; }
            65 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rcx = ((rcx).wrapping_add(0x68u64)) as u64; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P7)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 78; continue; } bb = 66; continue; }
            66 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 70; continue; } bb = 67; continue; }
            67 => { bb = 68; continue; }
            68 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 76; continue; } bb = 69; continue; }
            69 => { r14 = (crate::simd_ops::ru64(r14)) as u64; fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 68; continue; } bb = 70; continue; }
            70 => { r14 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 71; continue; }
            71 => { fl_l = (r14) as u64 as u64; fl_r = (r14) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 72; continue; }
            72 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 74; continue; }
            74 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = (0x0u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 77; continue; } bb = 75; continue; }
            75 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; rcx = ((rsi.wrapping_add(0x90u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 78; continue; }
            76 => { fl_l = ((r14).wrapping_add(0x10u64)) as u64 as u64; fl_r = fl_l; r14 = ((fl_l as u64)) as u64; bb = 71; continue; }
            77 => { rdi = (0x2u32) as u32 as u64; bb = 78; continue; }
            78 => { r8 = ((rdi as u32)) as u32 as u64; rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsi.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 79; continue; }
            79 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x168u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; bb = 80; continue; }
            80 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x128u64)) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1415b6870u64) { 0x29 => { bb = 1; continue; } 0x4d => { bb = 2; continue; } 0xa2 => { bb = 3; continue; } 0xa9 => { bb = 4; continue; } 0xae => { bb = 5; continue; } 0xbc => { bb = 6; continue; } 0xc0 => { bb = 7; continue; } 0xe6 => { bb = 8; continue; } 0xef => { bb = 9; continue; } 0x101 => { bb = 10; continue; } 0x104 => { bb = 11; continue; } 0x11a => { bb = 12; continue; } 0x1be => { bb = 13; continue; } 0x1c4 => { bb = 14; continue; } 0x20d => { bb = 15; continue; } 0x234 => { bb = 16; continue; } 0x24e => { bb = 17; continue; } 0x265 => { bb = 18; continue; } 0x27a => { bb = 19; continue; } 0x2ce => { bb = 20; continue; } 0x2fe => { bb = 21; continue; } 0x311 => { bb = 22; continue; } 0x320 => { bb = 23; continue; } 0x32d => { bb = 24; continue; } 0x330 => { bb = 25; continue; } 0x36b => { bb = 26; continue; } 0x387 => { bb = 27; continue; } 0x396 => { bb = 28; continue; } 0x39b => { bb = 29; continue; } 0x3a1 => { bb = 30; continue; } 0x3b5 => { bb = 31; continue; } 0x3c9 => { bb = 32; continue; } 0x3e4 => { bb = 33; continue; } 0x3f1 => { bb = 34; continue; } 0x3f5 => { bb = 35; continue; } 0x400 => { bb = 36; continue; } 0x415 => { bb = 37; continue; } 0x417 => { bb = 38; continue; } 0x42a => { bb = 39; continue; } 0x440 => { bb = 40; continue; } 0x443 => { bb = 41; continue; } 0x451 => { bb = 42; continue; } 0x47f => { bb = 43; continue; } 0x48e => { bb = 44; continue; } 0x49a => { bb = 45; continue; } 0x4a0 => { bb = 46; continue; } 0x4b5 => { bb = 47; continue; } 0x4bc => { bb = 48; continue; } 0x4c9 => { bb = 49; continue; } 0x4d6 => { bb = 50; continue; } 0x4e3 => { bb = 51; continue; } 0x4ea => { bb = 52; continue; } 0x4f4 => { bb = 53; continue; } 0x4f7 => { bb = 54; continue; } 0x500 => { bb = 55; continue; } 0x52d => { bb = 56; continue; } 0x564 => { bb = 57; continue; } 0x5a4 => { bb = 58; continue; } 0x5db => { bb = 59; continue; } 0x668 => { bb = 60; continue; } 0x6c5 => { bb = 61; continue; } 0x6cc => { bb = 62; continue; } 0x6d1 => { bb = 63; continue; } 0x6db => { bb = 64; continue; } 0x6ef => { bb = 65; continue; } 0x72e => { bb = 66; continue; } 0x74a => { bb = 67; continue; } 0x750 => { bb = 68; continue; } 0x756 => { bb = 69; continue; } 0x75e => { bb = 70; continue; } 0x761 => { bb = 71; continue; } 0x766 => { bb = 72; continue; } 0x76c => { bb = 73; continue; } 0x774 => { bb = 74; continue; } 0x77b => { bb = 75; continue; } 0x78d => { bb = 76; continue; } 0x793 => { bb = 77; continue; } 0x798 => { bb = 78; continue; } 0x7b2 => { bb = 79; continue; } 0x7d2 => { bb = 80; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b70e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x1415242e0; const P4: u64 = 0x18; const P5: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rdx = rbx; rsi = (0x2aaaaaaaaaaaaaabu64) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((r8).wrapping_sub(rbx)) as u64; rax = rsi; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rbx; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P0), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; bb = 7; continue; }
            6 => { fl_l = ((rbx).wrapping_add(0x18u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rax = ((r8 as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((rax as u32)) as u32 as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b71a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439318b8; const P1: u64 = 0x1439318bc; const P2: u64 = 0x1439318c4;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r9, ((r10 as u32)) as u32); rdi = r9; rsi = (((r8 as u8)) as u32) as u32 as u64; rbx = rcx; crate::simd_ops::wu32(r11, ((r10 as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x30; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 1; continue; }
            1 => { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(0x18u64))) as u128; crate::simd_ops::wu128(rsp, (xmm0) as u128); r8 = (crate::simd_ops::ru32(rsp)) as u32 as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm1) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = ((((r9 as u32)) ^ (crate::simd_ops::ru32(P1)))) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((((rax as u16)) ^ (crate::simd_ops::ru16(P2)))) as u16 as u64; fl_r = fl_l; rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); bb = 4; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0xcu64))) as u32) as u32 as u64; bb = 4; continue; }
            4 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = 0x10; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x87; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (((rax as u16)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = rbx; let d = (rcx) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0x59; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { crate::simd_ops::wu32(rdi, ((r9 as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x8f; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((r10).wrapping_shr(32u32)) as u64; r10 = ((fl_l as u64)) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu32(r11, ((r10 as u32)) as u32); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x1f; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b72a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x2; const P2: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((r8 as u32)) as u32 as u64;
    fl_l = ((((rdx as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l;
    rdx = ((fl_l as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = 0xc; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; fl_l = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; r8 = ((fl_l as u32)) as u32 as u64; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = 0x303f;
    rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64;
    fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64;
    if (((fl_l as u16)) <= ((fl_r as u16))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rdx = 0x3042;
    rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_sub((rdx as u16))) as u16 as u64));
    rdx = 0xcfbc;
    fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64;
    if !((((fl_l as u16)) <= ((fl_r as u16)))) { rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b72f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x198; const P1: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rax, ((rdx as u32)) as u32); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b7320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415b70e0; const P2: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rdi = r8; rcx = ((rcx).wrapping_add(0x68u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x98u64))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rsi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { r8 = r11; bb = 6; continue; }
            6 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::lk_add_u32(r8.wrapping_add(0x10u64), (1) as u32); r9 = r11; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r10 = 0xffffffff; rax = (crate::simd_ops::ru64(r8)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x30u64))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 12; continue; }
            12 => { bb = 15; continue; }
            13 => { r8 = rcx.wrapping_add(0x10); bb = 6; continue; }
            14 => { rcx = r11; bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xdu64))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rdi, rcx); r9 = rax; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rcx as u32)) as u32); r10 = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 19; continue; }
            19 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_add_u16(r9.wrapping_add(0xeu64), (1) as u16)) as u16 as u64; bb = 23; continue; }
            21 => { crate::simd_ops::wu64(rdi, r11); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r11); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r11 as u32)) as u32); fl_l = (crate::simd_ops::lk_add_u32(r8.wrapping_add(0x10u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = r8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x60u64), (1) as u32)) as u32 as u64; bb = 26; continue; }
            25 => { crate::simd_ops::wu64(rdi, r11); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r11); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r11 as u32)) as u32); bb = 26; continue; }
            26 => { rcx = rbp.wrapping_add(0x68); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b7470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141613980; const P1: u64 = 0x8; const P2: u64 = 0x1415b1220;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0xa8u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = 0x34; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b74b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0; const P3: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = rcx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; rbp = ((rdx as u32)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; r15 = ((rcx as u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_shr(1u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r15 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rdx as u32) } else { (r15 as u32) }) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r14 = ((r15 as u32)) as u32 as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { bb = 20; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { r10 = (0xfffffffffffffff4u64) as u64; rax = rdi.wrapping_add(0xc); fl_l = r10; fl_r = rdi; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; r9 = rsi; bb = 9; continue; }
            9 => { r8 = rax.wrapping_sub(0xc); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r8, r12); crate::simd_ops::wu64(rax.wrapping_sub(0x4u64), r12); crate::simd_ops::wu64(rax.wrapping_add(0x4u64), r12); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((r10.wrapping_add(rax)) as u64) as u64; rax = ((rax).wrapping_add(0x18u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(rcx))) as u128; crate::simd_ops::wu128(r8, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(rcx).wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rbx, rdi); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r15 as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rbp.wrapping_mul(2)) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; rsi = ((rcx.wrapping_mul(8)) as u64) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (((rax as u32)).wrapping_sub((rbp as u32))) as u32 as u64; rcx = rdi.wrapping_add(0x18); rdx = rdi; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r12); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), r12); bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax).wrapping_add(rsi)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 20; continue; }
            20 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b7630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bb740; const P1: u64 = 0x40; const P2: u64 = 0x48; const P3: u64 = 0x30; const P4: u64 = 0x1403cd310; const P5: u64 = 0x143930f08; const P6: u64 = 0x1415aa7b0; const P7: u64 = 0x4; const P8: u64 = 0xb4; const P9: u64 = 0x1415aa630;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = rdx;
    rdi = r9;
    rbx = r8;
    rsi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rsi.wrapping_add(P1), rbx);
    crate::simd_ops::wu64(rsi.wrapping_add(P2), rdi);
    crate::simd_ops::wu8(rsi.wrapping_add(P3), (0x1u8) as u8);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P5)) as u64;
    rdx = 0x1a;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rax.wrapping_add(P7), rsi);
    rax = (crate::simd_ops::ru64(P5)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P8), (1) as u32);
    rcx = (crate::simd_ops::ru64(P5)) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b76b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645290; const P1: u64 = 0xf0; const P2: u64 = 0x141613850; const P3: u64 = 0x1415bb740; const P4: u64 = 0x141580020; const P5: u64 = 0x30; const P6: u64 = 0x40; const P7: u64 = 0x48; const P8: u64 = 0x1403cd310; const P9: u64 = 0x143930f08; const P10: u64 = 0x1415aa7b0; const P11: u64 = 0x4; const P12: u64 = 0x1415aa630; const P13: u64 = 0x1426455b0; const P14: u64 = 0x142645748;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rbx = rcx; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu8(rdi.wrapping_add(P5), (0x1u8) as u8); crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); rax = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P9)) as u64; rdx = 0x1a; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(P11), rdi); rax = (crate::simd_ops::ru64(P9)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32); rcx = (crate::simd_ops::ru64(P9)) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = 0xffffffff; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b77a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d98; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1416138e0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x141524330; const P5: u64 = 0x1415b1290; const P6: u64 = 0x1415b12e0; const P7: u64 = 0x1415b14e0; const P8: u64 = 0x1415b13b0; const P9: u64 = 0x1415b9730; const P10: u64 = 0x1415b0d60; const P11: u64 = 0x1415b9950; const P12: u64 = 0x1415ba5b0; const P13: u64 = 0x1415ba030; const P14: u64 = 0x1415b9180; const P15: u64 = 0x1415b1500; const P16: u64 = 0x1415b1130; const P17: u64 = 0x1415b1260; const P18: u64 = 0x1415b7ef0; const P19: u64 = 0x141524320; const P20: u64 = 0x1415b0ff0; const P21: u64 = 0x143930f48; const P22: u64 = 0x1415fcfa0; const P23: u64 = 0x1415242e0; const P24: u64 = 0x1415ba410; const P25: u64 = 0x1415b98a0; const P26: u64 = 0x1415b8560; const P27: u64 = 0x1415b0d10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0xf); rsp = ((rsp).wrapping_sub(0x98u64)) as u64; rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r15 = rcx; rcx = P0; crate::simd_ops::wu32(rbp.wrapping_sub(0x4du64), ((rbx as u32)) as u32); r12 = ((r9 as u32)) as u32 as u64; r13 = r8; rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x35u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; bb = 3; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rcx = r15.wrapping_add(0xa8); rdx = rbp.wrapping_sub(0x49); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x45; bb = 115; continue; }
            6 => { rcx = P0; rbx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { r15 = 0; fl_l = 0; fl_r = 0; r12 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r15 = 0x1; rbx = 0; fl_l = 0; fl_r = 0; rsi = (((r15 as u8)) as u32) as u32 as u64; bb = 11; continue; }
            9 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x77u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((rbx as u32)) as u32 as u64; r12 = ((rbx as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rdx = 0x78; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; bb = 15; continue; }
            14 => { rax = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x28u64), rax); crate::simd_ops::wu8(rcx.wrapping_add(0x30u64), ((rax as u8)) as u8); crate::simd_ops::wu64(rcx.wrapping_add(0x34u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), rax); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x68u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); rax = (rax & !0xffu64) | ((((((rax as u8)) | ((rsi as u8)))) as u8 as u64)); fl_l = ((((rax as u8)) & (0xf1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x68u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x6cu64), (0x10u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x70u64), ((r12 as u32)) as u32); bb = 17; continue; }
            16 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x57u64))) as u64; fl_l = rcx; fl_r = rcx; r12 = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; rbx = 0x34; rax = 0x1; crate::simd_ops::wu64(r13, rcx); rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; bb = 20; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x77u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu64(r13, rsi); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x70u64), (1) as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rsi = r15.wrapping_add(0x8); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((r14.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u32)) <= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_sub(0x51u64), ((rdx as u8)) as u8); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 34; continue; } bb = 21; continue; }
            21 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 34; continue; } bb = 22; continue; }
            22 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 23; continue; }
            23 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 29; continue; } bb = 24; continue; }
            24 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rbx as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); bb = 26; continue; }
            26 => { r9 = ((r14 as u32)) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rcx = rsi; rdx = ((r14 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 36; continue; }
            27 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x40u64))) as u32 as u64; rcx = rsi; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 35; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = ((r14.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 33; continue; } bb = 30; continue; }
            30 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 33; continue; } bb = 31; continue; }
            31 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); rcx = rsi; r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r14 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 36; continue; }
            33 => { crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); bb = 26; continue; }
            34 => { rbx = 0x64; bb = 35; continue; }
            35 => { r14 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4du64))) as u32 as u64; bb = 36; continue; }
            36 => { rax = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); crate::simd_ops::wu8(rbp.wrapping_add(0x5fu64), ((rax as u8)) as u8); crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { r8 = rbp.wrapping_add(0x5f); rcx = r15; rdx = rbp.wrapping_sub(0x29); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { rbx = ((rax as u32)) as u32 as u64; bb = 39; continue; }
            39 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 40; continue; }
            40 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 114; continue; } bb = 41; continue; }
            41 => { r9 = rbp.wrapping_sub(0x55); crate::simd_ops::wu32(rbp.wrapping_sub(0x55u64), ((rdi as u32)) as u32); r8 = ((rbx.wrapping_add(0x7u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xffffffffu32) as u32); rdx = rbp.wrapping_sub(0x49); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 114; continue; } bb = 42; continue; }
            42 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x55u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x8; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 113; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x5fu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 46; continue; } bb = 44; continue; }
            44 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x41544144; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 46; continue; } bb = 45; continue; }
            45 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x58444944; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 112; continue; } bb = 46; continue; }
            46 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x53564e45; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 91; continue; } bb = 47; continue; }
            47 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 75; continue; } bb = 48; continue; }
            48 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x41544144; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 55; continue; } bb = 49; continue; }
            49 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x43524948; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 54; continue; } bb = 50; continue; }
            50 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x44495453; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 53; continue; } bb = 51; continue; }
            51 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x474d5453; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 62; continue; } bb = 52; continue; }
            52 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 40; continue; }
            53 => { r8 = (crate::simd_ops::ru64(r13)) as u64; rcx = r15; rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 40; continue; }
            54 => { rdx = (crate::simd_ops::ru64(r13)) as u64; r8 = ((r14 as u32)) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 40; continue; }
            55 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x29u64))) as u32 as u64; fl_r = 0x87; rcx = 0x10; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rcx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x1du64))) as u32) as u32 as u64; bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru64(r13)) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x6cu64), ((rcx as u32)) as u32); rcx = ((r12 as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 64; continue; } bb = 58; continue; }
            58 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 61; continue; } bb = 59; continue; }
            59 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 62; continue; } bb = 60; continue; }
            60 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 61; continue; }
            61 => { r9 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x7fu64))) as u32) as u32 as u64; rcx = r15; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(r13)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 40; continue; }
            62 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; r8 = rbp.wrapping_sub(0x55); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x55u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 63; continue; }
            63 => { rbx = 0x7; bb = 114; continue; }
            64 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x51u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 67; continue; } bb = 65; continue; }
            65 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x35u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 67; continue; } bb = 66; continue; }
            66 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; rdi = rax; rdx = (crate::simd_ops::ru64(r13)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), ((rcx as u32)) as u32); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r13)) as u64; rdx = rdi; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            67 => { rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; rbx = 0x1; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 73; continue; } bb = 68; continue; }
            68 => { rbx = (crate::simd_ops::ru64(r13)) as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x68u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x6cu64))) as u32 as u64; rcx = ((((rcx as u32)) & (0x8u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(27u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x20000003u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 70; continue; } bb = 69; continue; }
            69 => { rbx = ((rax.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 39; continue; }
            70 => { r9 = rbp.wrapping_sub(0x39); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rdi as u32)) as u32); r8 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), (0x0u32) as u32); rdx = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xffffffffu32) as u32); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 73; continue; } bb = 71; continue; }
            71 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x39u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 73; continue; } bb = 72; continue; }
            72 => { rbx = ((rax.wrapping_add(0x6u64)) as u32) as u32 as u64; bb = 39; continue; }
            73 => { rax = (crate::simd_ops::ru64(r13)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 74; continue; }
            74 => { r8 = rax; rdx = rdi; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            75 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 90; continue; } bb = 76; continue; }
            76 => { r12 = ((rdi as u32)) as u32 as u64; r13 = rdi; bb = 77; continue; }
            77 => { r15 = ((rdi as u32)) as u32 as u64; r14 = rdi; bb = 78; continue; }
            78 => { rdi = 0x1; rdx = rbp.wrapping_sub(0x50); r8 = ((rdi as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 87; continue; } bb = 79; continue; }
            79 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_sub(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rcx = ((r14.wrapping_add(r13)) as u64) as u64; rax = (crate::simd_ops::ru64(P21)) as u64; r8 = ((rdi as u32)) as u32 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(rax), ((rdx as u8)) as u8); rdx = rbp.wrapping_sub(0x4f); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = 0; fl_l = 0; fl_r = 0; rbx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x59u64), ((rdi as u16)) as u16); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 88; continue; } bb = 80; continue; }
            80 => { r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rsi; rdx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 88; continue; } bb = 81; continue; }
            81 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x59u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 89; continue; } bb = 82; continue; }
            82 => { rcx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x59u64))) as u32) as u32 as u64; rdx = rax; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u32) as u32 as u64; rcx = rsi; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 86; continue; } bb = 83; continue; }
            83 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x4fu64))) as u32) as u32 as u64; r8 = ((r15 as u32)) as u32 as u64; r9 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x59u64))) as u32) as u32 as u64; rdx = ((r12 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P21)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(1)) as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x3; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 78; continue; } bb = 84; continue; }
            84 => { r12 = (((r12 as u32)).wrapping_add(1)) as u32 as u64; r13 = ((r13).wrapping_add(0x3u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x2; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 88; continue; } bb = 85; continue; }
            85 => { bb = 77; continue; }
            86 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 87; continue; }
            87 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 88; continue; }
            88 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x57u64))) as u64; r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x67u64))) as u64; r12 = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; r14 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4du64))) as u32 as u64; bb = 40; continue; }
            89 => { rbx = 0x34; bb = 87; continue; }
            90 => { r12 = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; rbx = 0x38; r14 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4du64))) as u32 as u64; bb = 40; continue; }
            91 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x54414c50; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 111; continue; } bb = 92; continue; }
            92 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x54494e49; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 110; continue; } bb = 93; continue; }
            93 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x58444944; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 62; continue; } bb = 94; continue; }
            94 => { rcx = ((r12 as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 98; continue; } bb = 95; continue; }
            95 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 98; continue; } bb = 96; continue; }
            96 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 62; continue; } bb = 97; continue; }
            97 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 98; continue; }
            98 => { rdi = (crate::simd_ops::ru64(r13)) as u64; rbx = 0x1; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 100; continue; } bb = 99; continue; }
            99 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; r8 = rbp.wrapping_sub(0x31); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4du64))) as u32 as u64; bb = 39; continue; }
            100 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; rax = (0xaaaaaaaaaaaaaaabu64) as u64; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r15 = rdx; r15 = ((r15).wrapping_shr(3u32)) as u64; r14 = ((r15.wrapping_add(r15.wrapping_mul(2))) as u32) as u32 as u64; r14 = (((r14 as u32)).wrapping_shl(2u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x35u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 104; continue; } bb = 101; continue; }
            101 => { rdx = ((r14 as u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 103; continue; } bb = 102; continue; }
            102 => { crate::simd_ops::lk_or_u8(rdi.wrapping_add(0x68u64), (0x2u8) as u8); r8 = ((r14 as u32)) as u32 as u64; rdx = rax; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 107; continue; }
            103 => { rbx = 0x34; bb = 107; continue; }
            104 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 106; continue; } bb = 105; continue; }
            105 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); bb = 106; continue; }
            106 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 107; continue; }
            107 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 109; continue; } bb = 108; continue; }
            108 => { crate::simd_ops::wu32(rdi.wrapping_add(0x38u64), ((r15 as u32)) as u32); bb = 109; continue; }
            109 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x57u64))) as u64; r14 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x4du64))) as u32 as u64; bb = 39; continue; }
            110 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 40; continue; }
            111 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x45u64))) as u32 as u64; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P25, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 40; continue; }
            112 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x29u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P26, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = 0x40; bb = 114; continue; }
            113 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = 0x7; rbx = (if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; bb = 114; continue; }
            114 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P27, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; bb = 115; continue; }
            115 => { rsp = ((rsp).wrapping_add(0x98u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b7ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426890; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1415bb990; const P3: u64 = 0x141524330; const P4: u64 = 0x1415b74b0; const P5: u64 = 0x1426452a8; const P6: u64 = 0x1415bccb0; const P7: u64 = 0x1415bc6e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; fl_l = (crate::simd_ops::ru32(r8.wrapping_add(0x34u64))) as u32 as u64; fl_r = 0x0; r14 = r8; rdi = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r12); r12 = 0x34; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((r12 as u32)) as u32); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r13); r13 = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rbp); crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), r15); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rsi); bb = 5; continue; }
            5 => { rcx = ((r13.wrapping_mul(2)) as u64) as u64; rcx = ((rcx).wrapping_add(r13)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_r = 0x0; r15 = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r15); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 6; continue; }
            6 => { rbx = rdi.wrapping_add(0x68); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru32(r15)) as u32 as u64; rsi = rdi.wrapping_add(0x90); rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 10; continue; }
            10 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; r12 = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { rdx = 0x28; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            15 => { rdi = rcx.wrapping_add(0x10); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 16; continue; }
            16 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdi = rdx.wrapping_add(0x10); rcx = (crate::simd_ops::ru64(rax.wrapping_add(r12.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx, rcx); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rbp as u32)) as u32); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), (0xffffffffu32) as u32); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r12.wrapping_mul(8)), rdx); fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(r15)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rax as u32)) as u32); rcx = rdi; r12 = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r15 = (crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32 as u64; r15 = ((r15).wrapping_add(crate::simd_ops::ru64(rsp.wrapping_add(0x88u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rax, r15); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r12 as u32)) as u32); r12 = 0x1; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(0xcu64), (0x2u8) as u8); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x34u64), (1) as u32); r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 26; continue; }
            26 => { bb = 30; continue; }
            27 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rcx = rsi; r12 = 0x34; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            28 => { r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; bb = 29; continue; }
            29 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { r12 = 0x1; rax = ((r12 as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            33 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 35; continue; }
            35 => { rdx = r14; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { rax = ((r12 as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            37 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b8180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415bbb10; const P2: u64 = 0x141524320; const P3: u64 = 0x1426452a8; const P4: u64 = 0x1415b1290; const P5: u64 = 0x1415b12e0; const P6: u64 = 0x1415b0d60; const P7: u64 = 0x1415b0d10; const P8: u64 = 0x1415b5f80; const P9: u64 = 0x1415242e0; const P10: u64 = 0x14183acbc; const P11: u64 = 0x1415bb3a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; r13 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r14 = rdx; r12 = rcx; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7cu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); rbx = rcx.wrapping_add(0x68); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(r12.wrapping_add(0x98u64))) as u32 as u64; rdi = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r12.wrapping_add(0x90u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x10u64), (1) as u32); rax = rdi; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rdi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { bb = 20; continue; }
            15 => { rsi = rdx.wrapping_add(0x10); bb = 9; continue; }
            16 => { rdi = 0x1; bb = 39; continue; }
            17 => { rdx = ((rdi as u32)) as u32 as u64; rcx = r12.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdi = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 39; continue; }
            19 => { crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rdi as u32)) as u32); rdi = (crate::simd_ops::ru64(rax)) as u64; bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru16(r14.wrapping_add(0x1au64))) as u16 as u64; fl_r = (0x3u16) as u16 as u64; rax = 0x60000003; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r15 = 0x20000003; r15 = (if (((fl_l as u16)) == ((fl_r as u16))) { (rax as u32) } else { (r15 as u32) }) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 22; continue; }
            22 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 23; continue; }
            23 => { rdx = r13; r8 = 0x10; rcx = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 24; continue; }
            24 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); r12 = ((r12).wrapping_add(0x8u64)) as u64; rcx = r12; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = ((crate::simd_ops::ru16(r14.wrapping_add(0x1au64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0xcu64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); rax = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1cu16) as u16 as u64; if (((fl_l as u16)) <= ((fl_r as u16))) { bb = 27; continue; } bb = 25; continue; }
            25 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rcx as u8)) as u8); rcx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { r8 = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r9 = rsp.wrapping_add(0x78); rdx = rbp; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x0u32) as u32); rcx = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xffffffffu32) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = ((rax as u32)) as u32 as u64; rdi = 0x2; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) != ((fl_r as u32))) { (rdi as u32) } else { (rdx as u32) }) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; bb = 30; continue; }
            30 => { rcx = r12; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { r9 = ((r15 as u32)) as u32 as u64; r8 = ((r13 as u32)) as u32 as u64; rdx = rbp; rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 38; continue; } bb = 32; continue; }
            32 => { rdx = rbp; rcx = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 37; continue; }
            33 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 34; continue; }
            34 => { r13 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = 0x10; rdx = ((r13 as u32)) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((r13 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            36 => { rdi = 0x34; bb = 37; continue; }
            37 => { rdx = rsi; rcx = r12; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            38 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 39; continue; }
            39 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            40 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b8430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rax = 0; fl_l = 0; fl_r = 0; r10 = rcx; r11 = rdx; xmm0 = 0u128; crate::simd_ops::wu128(r8, (xmm0) as u128); rbx = 0x2; crate::simd_ops::wu128(r8.wrapping_add(0x10u64), (xmm0) as u128); crate::simd_ops::wu64(r8.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(r8.wrapping_add(0x4u64), ((r9 as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rcx, rax); rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(r10, rax); crate::simd_ops::wu32(r8.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(0xcu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(r10)) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(r10, rax); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(r10)) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; rdx = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(r10, rax); rcx = ((((rcx as u32)) & (0x8u32))) as u32 as u64; rax = 0; fl_l = 0; fl_r = 0; rcx = (((rcx as u32)).wrapping_shl(7u32)) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { (rbx as u32) } else { (rax as u32) }) as u32 as u64; rdx = ((((rdx as u32)) & (0x1u32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x14u64))) as u32 as u64; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; rax = ((((rax as u32)) & (0xfffffbfcu32))) as u32 as u64; rdx = ((crate::simd_ops::ru8(r8.wrapping_add(0x4u64))) as u32) as u32 as u64; rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0xfu8))) as u8 as u64)); crate::simd_ops::wu32(r8.wrapping_add(0x14u64), ((rcx as u32)) as u32); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) & (0xffffff8fu32))) as u32 as u64; rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((((rcx as u32)) | (0xcu32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x14u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0x38; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { rcx = ((((rcx as u32)) & (0xffffff87u32))) as u32 as u64; rax = 0x1; fl_l = ((((rcx as u32)) | (0x4u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x14u64), ((rcx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(r10)) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(r10, rax); crate::simd_ops::wu32(r8.wrapping_add(0x20u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(r10)) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x18u64), rax); crate::simd_ops::lk_add_u64(r10, (rcx) as u64); rax = (crate::simd_ops::ru32(r8.wrapping_add(0x20u64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(r11)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(r11, ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rax = 0x1; rcx = 0x38; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b8560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d1e0; const P1: u64 = 0x1439318d0; const P2: u64 = 0x141834a78; const P3: u64 = 0x14268a230; const P4: u64 = 0x14268a240; const P5: u64 = 0x1439318c8; const P6: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = ((rcx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(P0)) as u32 as u64; rcx = 0x74; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x91; rdx = P3; rax = P4; rax = (if (((fl_l as u32)) < ((fl_r as u32))) { rdx } else { rax }) as u64; rcx = P1; crate::simd_ops::wu64(P5, rax); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b85e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x141613870;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (((crate::simd_ops::ru32(rdx)) as i32 as i64) as u64) as u64; r10 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r11 = ((r8 as u32)) as u32 as u64; r8 = ((r10 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xbu32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rbx) as u64); rbx = ((P0) as u64) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15b8664u64))) as u32 as u64; fl_l = ((r9).wrapping_add(rbx)) as u64 as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; j_tgt = (r9) as u64; bb = 0xffff_fffe; continue; }
            2 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; bb = 6; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x40u64))) as u32 as u64; bb = 6; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { r10 = (crate::simd_ops::ru64(rdx.wrapping_add(0x38u64))) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; r9 = (r10) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0xecu64), ((r11 as u32)) as u32); rcx = ((rcx).wrapping_add(0xf0u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1415b85e0u64) { 0x15 => { bb = 1; continue; } 0x34 => { bb = 2; continue; } 0x3a => { bb = 3; continue; } 0x40 => { bb = 4; continue; } 0x46 => { bb = 5; continue; } 0x4a => { bb = 6; continue; } 0x50 => { bb = 7; continue; } 0x54 => { bb = 8; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b86a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d98; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1416138e0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x1415b77a0; const P5: u64 = 0x141613c30; const P6: u64 = 0x1416139f0; const P7: u64 = 0x1415bb3f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rsi = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r15 = rcx; rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rcx = P0; r12 = (((r9 as u8)) as u32) as u32 as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); rcx = r15.wrapping_add(0xa8); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rbp as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::lk_add_u32(rax.wrapping_add(0x64u64), (1) as u32); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x70u64), (1) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rbx = ((r13.wrapping_add(0x3u64)) as u32) as u32 as u64; bb = 4; continue; }
            4 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xe0u64))) as u32) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x10u64))) as u128; r8 = rsp.wrapping_add(0xc0); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x40); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x20u64))) as u128; rcx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r13); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r12 as u8)) as u8); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rdx = rsp.wrapping_add(0x30); rcx = r15.wrapping_add(0xa8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            6 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x1; bb = 11; continue; }
            7 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x45; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); rcx = r15.wrapping_add(0xa8); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rax = ((rdi as u32)) as u32 as u64; bb = 11; continue; }
            11 => { r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b8830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415b89c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r15 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rbp = ((r8 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; r12 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); r14 = 0x1; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r8 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rdi.wrapping_add(0x110); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x150u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x148u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            11 => { rsi = rdi.wrapping_add(0x110); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rdi.wrapping_add(0x110); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 26; continue; }
            26 => { r8 = rbx; rdx = r15; rcx = r12; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x28u64), (1) as u32)) as u32 as u64; bb = 29; continue; }
            29 => { rcx = 0xffffffff; let xa_src: u32 = (rcx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { rax = ((r14 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0xf; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b89c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x143412830; const P2: u64 = 0x141524330; const P3: u64 = 0x14183acbc; const P4: u64 = 0x1415b86a0; const P5: u64 = 0x1415a0b00; const P6: u64 = 0x1416138e0; const P7: u64 = 0x1415bb3f0; const P8: u64 = 0x143930f88; const P9: u64 = 0x1426452b0; const P10: u64 = 0x1426452a8; const P11: u64 = 0x142645560; const P12: u64 = 0x1415b89c0; const P13: u64 = 0x1415a36b0; const P14: u64 = 0x1415bcaa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; rdi = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; r12 = 0x1; crate::simd_ops::wu32(rsp.wrapping_add(0xf8u64), ((r12 as u32)) as u32); r13 = rdx; r15 = rcx; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 104; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), rbp); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), rsi); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r14); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x34u64))) as u32) as u32 as u64; rcx = 0x403; rdx = 0x2103; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 36; continue; } bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x8u64), ((r8 as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x4u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(r15.wrapping_add(0xc8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r15.wrapping_add(0xc0u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(r13.wrapping_add(0x10u64), (0x0u64) as u64); rax = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu8(r13.wrapping_add(0x18u64), (0x0u8) as u8); bb = 12; continue; }
            12 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = (0x4u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 19; continue; } bb = 14; continue; }
            14 => { rcx = rax.wrapping_sub(0x4); rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(rbx)) as u64; r8 = P1; fl_l = r8; fl_r = rcx; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 15; continue; }
            15 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(rcx))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 16; continue; }
            16 => { rcx = ((rcx).wrapping_add(1)) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(r13.wrapping_add(0x10u64), rbx); crate::simd_ops::wu8(r13.wrapping_add(0x18u64), (0x0u8) as u8); bb = 32; continue; }
            18 => { fl_l = ((rdx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 8; continue; }
            19 => { crate::simd_ops::wu64(r13.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu8(r13.wrapping_add(0x18u64), (0x0u8) as u8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 20; continue; }
            20 => { rdx = (0xffffffffffffffffu64) as u64; bb = 21; continue; }
            21 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 22; continue; }
            22 => { r15 = ((rdx as u32)) as u32 as u64; rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = r15; fl_r = (0x4u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 27; continue; } bb = 23; continue; }
            23 => { rcx = r15.wrapping_sub(0x4); r8 = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(rbx)) as u64; r9 = P1; fl_l = r9; fl_r = rcx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 24; continue; }
            24 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rcx))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rcx = ((rcx).wrapping_add(1)) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 24; continue; } bb = 26; continue; }
            26 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rcx = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = (((rbp as u8)) as u32) as u32 as u64; r14 = ((rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { r8 = r15; rdx = rbx; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu32(r15.wrapping_add(rsi), (0x6b6e622eu32) as u32); bb = 30; continue; }
            30 => { crate::simd_ops::wu8(rsi.wrapping_add(r14).wrapping_sub(0x1u64), (0x0u8) as u8); crate::simd_ops::wu64(r13.wrapping_add(0x10u64), rsi); crate::simd_ops::wu8(r13.wrapping_add(0x18u64), (0x1u8) as u8); bb = 31; continue; }
            31 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 32; continue; }
            32 => { xmm0 = (crate::simd_ops::ru128(r13)) as u128; r9 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x50); xmm1 = (crate::simd_ops::ru128(r13.wrapping_add(0x10u64))) as u128; rcx = r15; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0x20u64))) as u128; r8 = ((r9.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(r13.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(r13.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0xf8u64), ((rax as u32)) as u32); r12 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0xf8u64), ((rax as u32)) as u32); r12 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 65; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rcx = r15.wrapping_add(0xa8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 35; continue; }
            35 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 69; continue; }
            36 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 64; continue; } bb = 37; continue; }
            37 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(P8)) as u64; bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 49; continue; } bb = 39; continue; }
            39 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 40; continue; }
            40 => { rbp = rbx.wrapping_add(0x110); bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 42; continue; }
            42 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x148u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 43; continue; }
            43 => { bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 45; continue; }
            45 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 44; continue; } bb = 46; continue; }
            46 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 47; continue; }
            47 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 52; continue; }
            49 => { rbp = rbx.wrapping_add(0x110); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 41; continue; } bb = 50; continue; }
            50 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            51 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 59; continue; } bb = 53; continue; }
            53 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 56; continue; } bb = 54; continue; }
            54 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 53; continue; } bb = 55; continue; }
            55 => { bb = 60; continue; }
            56 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 61; continue; } bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 61; continue; } bb = 58; continue; }
            58 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 61; continue; }
            59 => { rbp = rbx.wrapping_add(0x110); bb = 60; continue; }
            60 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 61; continue; }
            61 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 62; continue; }
            62 => { r8 = rsi; rdx = r13; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xf8u64), ((rax as u32)) as u32); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 64; continue; }
            64 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 65; continue; }
            65 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 66; continue; }
            66 => { rax = ((r12 as u32)) as u32 as u64; bb = 67; continue; }
            67 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            68 => { r12 = 0x2; crate::simd_ops::wu32(rsp.wrapping_add(0xf8u64), ((r12 as u32)) as u32); bb = 69; continue; }
            69 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 66; continue; } bb = 70; continue; }
            70 => { r13 = 0x403; r12 = 0x2103; bb = 71; continue; }
            71 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 74; continue; } bb = 72; continue; }
            72 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rcx = r15.wrapping_add(0xa8); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 102; continue; } bb = 73; continue; }
            73 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 102; continue; }
            74 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 102; continue; } bb = 75; continue; }
            75 => { r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(P8)) as u64; bb = 76; continue; }
            76 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 87; continue; } bb = 77; continue; }
            77 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rsi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 76; continue; } bb = 78; continue; }
            78 => { rbp = rsi.wrapping_add(0x110); bb = 79; continue; }
            79 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x150u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 89; continue; } bb = 80; continue; }
            80 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x148u64))) as u64; r14 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 84; continue; } bb = 81; continue; }
            81 => { bb = 82; continue; }
            82 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 85; continue; } bb = 83; continue; }
            83 => { r14 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 82; continue; } bb = 84; continue; }
            84 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 85; continue; }
            85 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 90; continue; } bb = 86; continue; }
            86 => { fl_l = (crate::simd_ops::lk_add_u32(r14.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 90; continue; }
            87 => { rbp = rsi.wrapping_add(0x110); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 79; continue; } bb = 88; continue; }
            88 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 76; continue; }
            89 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 90; continue; }
            90 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 97; continue; } bb = 91; continue; }
            91 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rsi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 94; continue; } bb = 92; continue; }
            92 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 91; continue; } bb = 93; continue; }
            93 => { bb = 98; continue; }
            94 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 99; continue; } bb = 95; continue; }
            95 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 99; continue; } bb = 96; continue; }
            96 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 99; continue; }
            97 => { rbp = rsi.wrapping_add(0x110); bb = 98; continue; }
            98 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 99; continue; }
            99 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 102; continue; } bb = 100; continue; }
            100 => { rdx = r14; rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(r14.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 102; continue; } bb = 101; continue; }
            101 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 102; continue; }
            102 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 71; continue; } bb = 103; continue; }
            103 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xf8u64))) as u32 as u64; bb = 67; continue; }
            104 => { rax = ((r12 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b8fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f60; const P1: u64 = 0x1415afb90; const P2: u64 = 0x1415b9670; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1415b85e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x40u64))) as u32) as u32 as u64; rsi = rdx; r13 = rcx; rdi = 0x1; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(0xe8u64), ((rdi as u8)) as u8); bb = 2; continue; }
            2 => { r15 = (crate::simd_ops::ru32(rdx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru32(rdx.wrapping_add(0x48u64))) as u32 as u64; r9 = ((rbp as u32)) as u32 as u64; r14 = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; r8 = ((r14 as u32)) as u32 as u64; r15 = (crate::simd_ops::ru32(rdx.wrapping_add(0x38u64))) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rbx as u8)) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 5; continue; }
            5 => { rcx = r13; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r9 = ((rbp as u32)) as u32 as u64; r8 = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rdx = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            7 => { rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x40u64))) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru32(rdx.wrapping_add(0x3cu64))) as u32 as u64; bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; r8 = ((r12 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rbp = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x40u64))) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 11; continue; }
            11 => { bb = 15; continue; }
            12 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru32(r14)) as u32 as u64; r8 = ((r12 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = r14.wrapping_add(0x4); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 21; continue; } bb = 16; continue; }
            16 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 17; continue; }
            17 => { rcx = r13; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32 as u64; rbp = r15; r12 = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; bb = 20; continue; }
            20 => { r9 = (crate::simd_ops::ru32(rbx)) as u32 as u64; r8 = ((r14 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rbx.wrapping_add(0x4); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu8(r13.wrapping_add(0xe8u64), (0x0u8) as u8); crate::simd_ops::wu32(r13.wrapping_add(0xe0u64), (0x0u32) as u32); bb = 23; continue; }
            23 => { r8 = ((rdi as u32)) as u32 as u64; rdx = rsi; rcx = r13; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b9180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1426452a8; const P2: u64 = 0x1415bbb10; const P3: u64 = 0x141524320; const P4: u64 = 0x1415b1500; const P5: u64 = 0x14183acbc; const P6: u64 = 0x1415b0d60; const P7: u64 = 0x1415bce80; const P8: u64 = 0x1415b74b0; const P9: u64 = 0x1415242e0; const P10: u64 = 0x1415b70e0; const P11: u64 = 0x1415bccb0; const P12: u64 = 0x1415bb640;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu8(r11.wrapping_add(0x20u64), ((r9 as u8)) as u8); crate::simd_ops::wu32(r11.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = ((r8 as u32)) as u32 as u64; r13 = rdx; r15 = rcx; rdi = 0x1; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 69; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x34u64))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r12); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rbx as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r13.wrapping_add(0x38u64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 44; continue; } bb = 2; continue; }
            2 => { rsi = r13.wrapping_add(0x20); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x4u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rdi = (crate::simd_ops::ru32(r8)) as u32 as u64; rax = (((rax as u32)).wrapping_sub((r9 as u32))) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rax as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); r9 = ((rcx as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdi = 0x1; crate::simd_ops::wu32(r13.wrapping_add(0x34u64), ((rax as u32)) as u32); bb = 41; continue; }
            5 => { r12 = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; rbx = r15.wrapping_add(0x68); rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r8 = r15.wrapping_add(0x90); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r8); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r8)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::lk_add_u32(r14.wrapping_add(0x10u64), (1) as u32); rax = rdi; rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rdi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { bb = 19; continue; }
            15 => { r14 = rax.wrapping_add(0x10); bb = 10; continue; }
            16 => { crate::simd_ops::lk_add_u32(r13.wrapping_add(0x34u64), (1) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0x1; bb = 40; continue; }
            17 => { rdx = ((rdi as u32)) as u32 as u64; rcx = r8; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rdi as u32)) as u32); rdi = (crate::simd_ops::ru64(rax)) as u64; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; r15 = ((crate::simd_ops::ru8(r13.wrapping_add(0x68u64))) as u32) as u32 as u64; r15 = ((((r15 as u32)) & (0x8u32))) as u32 as u64; r15 = (((r15 as u32)).wrapping_shl(27u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r15 = ((((r15 as u32)) | (0x20000003u32))) as u32 as u64; rax = ((rdi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 22; continue; }
            22 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 23; continue; }
            23 => { rdx = r12; r8 = 0x10; rcx = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 24; continue; }
            24 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; r8 = rsp.wrapping_add(0x3c); rbp = ((rbp).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x0u32) as u32); rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 27; continue; }
            27 => { bb = 32; continue; }
            28 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 29; continue; }
            29 => { r12 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; r8 = 0x10; rdx = ((r12 as u32)) as u32 as u64; rcx = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((r12 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            31 => { fl_l = ((rbp).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbp = ((fl_l as u64)) as u64; bb = 32; continue; }
            32 => { r9 = rsp.wrapping_add(0x3c); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x0u32) as u32); r8 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xffffffffu32) as u32); rdx = rsi; rcx = rbp; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x3cu64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::lk_add_u32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xc8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { r8 = rsp.wrapping_add(0x40); rcx = ((r15 as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 36; continue; }
            36 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 50; continue; } bb = 37; continue; }
            37 => { rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; bb = 38; continue; }
            38 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu64(rax, rsi); rcx = rbx; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r12 as u32)) as u32); rdi = 0x1; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(0xcu64), (0x1u8) as u8); crate::simd_ops::lk_add_u32(r13.wrapping_add(0x34u64), (1) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; rsi = r13.wrapping_add(0x20); bb = 40; continue; }
            40 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::lk_add_u32(rsp.wrapping_add(0x34u64), (1) as u32); rcx = ((rcx).wrapping_add(0xcu64)) as u64; rdx = (crate::simd_ops::ru32(r13.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r13.wrapping_add(0x38u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 42; continue; }
            42 => { rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32 as u64; bb = 44; continue; }
            44 => { rax = (((rax as u32)).wrapping_sub((r9 as u32))) as u32 as u64; fl_l = (((rbp as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rbp = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) == 0) { bb = 67; continue; } bb = 45; continue; }
            45 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r8 = rsp.wrapping_add(0xc0); rdx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), (0x0u32) as u32); rcx = r15.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 67; continue; } bb = 46; continue; }
            46 => { rdi = 0x38; fl_l = ((r13).wrapping_add(0x20u64)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; bb = 61; continue; }
            47 => { rdi = 0x34; bb = 53; continue; }
            48 => { rcx = rbx; rdi = 0x38; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); bb = 50; continue; }
            49 => { rdi = 0x34; bb = 50; continue; }
            50 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { rdx = rsi; rcx = ((r15 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 52; continue; }
            52 => { rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rcx = r14; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            55 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 57; continue; }
            57 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 58; continue; }
            58 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x38u64))) as u32 as u64; bb = 43; continue; }
            59 => { rcx = rbx; rdi = 0x34; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; r13 = rsi; bb = 61; continue; }
            60 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 61; continue; }
            61 => { rbx = ((((rbx as u32)) as i32 as i64) as u64) as u64; fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 68; continue; } bb = 62; continue; }
            62 => { rsi = ((rbx.wrapping_add(rbx.wrapping_mul(2))) as u64) as u64; fl_l = ((rsi).wrapping_shl(2u32)) as u64; rsi = ((fl_l as u64)) as u64; bb = 63; continue; }
            63 => { rax = (crate::simd_ops::ru64(r13)) as u64; rsi = rsi.wrapping_sub(0xc); rbx = ((rbx).wrapping_sub(1)) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(rax))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 65; continue; } bb = 64; continue; }
            64 => { rcx = r15; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 65; continue; }
            65 => { fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 63; continue; } bb = 66; continue; }
            66 => { bb = 68; continue; }
            67 => { fl_l = (crate::simd_ops::lk_or_u8(r13.wrapping_add(0x68u64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 68; continue; }
            68 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; bb = 69; continue; }
            69 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x90u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b9600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0x1415b5e50; const P2: u64 = 0x10; const P3: u64 = 0x1415b8180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rdx;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rcx = ((rcx).wrapping_add(0xd8u64)) as u64; r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0x34; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; } xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rax, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(P2))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P2), (xmm1) as u128); rax = 0x1; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b9670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0xe0; const P2: u64 = 0x1415b8180; const P3: u64 = 0x1415bb640;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); bb = 2; continue; }
            2 => { rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x20u64)) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P0)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x20u64)) as u64; fl_l = rdi; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rax = ((rbp as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b9730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b0ff0; const P1: u64 = 0x1439318b8; const P2: u64 = 0x1439318bc; const P3: u64 = 0x1439318c0; const P4: u64 = 0x1439318c4; const P5: u64 = 0x1439318c6; const P6: u64 = 0x1415b1500; const P7: u64 = 0x1415b8560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx.wrapping_add(0x8); crate::simd_ops::wu8(r8, (0x0u8) as u8); r14 = r8; rbx = rdx; rcx = rsi; rdx = rsp.wrapping_add(0x50); r8 = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_r = 0x44484b42; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 2; continue; }
            2 => { rbp = 0x14; rdx = rbx; r8 = ((rbp as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; crate::simd_ops::lk_xor_u32(rbx, ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::lk_xor_u32(rbx.wrapping_add(0x4u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::lk_xor_u32(rbx.wrapping_add(0x8u64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; crate::simd_ops::lk_xor_u16(rbx.wrapping_add(0xcu64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; crate::simd_ops::lk_xor_u16(rbx.wrapping_add(0xeu64), ((rax as u16)) as u16); fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = 0x8f; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 4; continue; }
            4 => { rbp = ((rdx.wrapping_add(0x27u64)) as u32) as u32 as u64; rcx = rsi; r8 = ((rdx.wrapping_add(0x3u64)) as u32) as u32 as u64; rdx = rbx.wrapping_add(0x14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 7; continue; }
            7 => { rdx = rbx.wrapping_add(0x18); rcx = rsi; r8 = ((rbp.wrapping_sub(0x18u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rbp as u32))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 10; continue; }
            10 => { r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); rdx = ((rdi as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rdx = ((rax.wrapping_add(0x37u64)) as u32) as u32 as u64; bb = 21; continue; }
            13 => { rdx = 0x64; bb = 21; continue; }
            14 => { rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x54u64))) as u32 as u64; xmm0 = 0u128; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), (0x0u32) as u32); fl_l = (((rdi as u32)).wrapping_add(0xffffffecu32)) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu128(rbx.wrapping_add(0x18u64), (xmm0) as u128); bb = 9; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x91; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x76; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu8(r14, (0x1u8) as u8); bb = 21; continue; }
            18 => { if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x40; bb = 21; continue; }
            20 => { rdx = 0x7; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rdx as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b98a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415b1130; const P2: u64 = 0x1; const P3: u64 = 0x142646960; const P4: u64 = 0x141588b00; const P5: u64 = 0x1415b1260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rbx); rbx = 0x1; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rdi); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rsi = ((rcx.wrapping_add(P0)) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; rdx = rax; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = ((rax.wrapping_add(P2)) as u64) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = ((r8.wrapping_add(P2)) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_add(0xfu64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (0xffffffffffffff0u64) as u64; bb = 7; continue; }
            7 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = rsp.wrapping_add(0x20); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rdi; crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rsp = rbp; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b9950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x1415b0ff0; const P2: u64 = 0x141588dd0; const P3: u64 = 0x14158a2d0; const P4: u64 = 0x141588ff0; const P5: u64 = 0x141588fa0; const P6: u64 = 0x143930f60; const P7: u64 = 0x1415af4c0; const P8: u64 = 0x1415b0720; const P9: u64 = 0x1415af690; const P10: u64 = 0x143930fa0; const P11: u64 = 0x1415c3a80; const P12: u64 = 0x1415c4e80; const P13: u64 = 0x1415c4f10; const P14: u64 = 0x1415c3c70; const P15: u64 = 0x1415fc9c0; const P16: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rbp = rsp.wrapping_add(0x30); rdi = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 64; continue; } bb = 1; continue; }
            1 => { rcx = rbp.wrapping_add(0xc0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rdi).wrapping_add(0x8u64)) as u64; rdx = rbp.wrapping_add(0xb8); rcx = rdi; r8 = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(rbp.wrapping_add(0xb8u64))) as u32) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r8 = ((rbx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0xb8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0xb8u64))) as u32) as u128; rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 4; continue; }
            4 => { r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0xb8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru16(rbp.wrapping_add(0xb8u64))) as u32) as u32 as u64; rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0xb8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((crate::simd_ops::ru16(rbp.wrapping_add(0xb8u64))) as u32) as u32 as u64; rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), ((r15 as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 9; continue; }
            9 => { r8 = ((r15.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x1c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 10; continue; }
            10 => { r14 = ((r15 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x1cu64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 24; continue; } bb = 11; continue; }
            11 => { r8 = 0x4; crate::simd_ops::wu32(rbp.wrapping_add(0xb8u64), ((r15 as u32)) as u32); rdx = rbp.wrapping_add(0xb8); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((r15 as u32)) as u32); rcx = rdi; crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 12; continue; }
            12 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0xb8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0xb8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 15; continue; }
            15 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 16; continue; }
            16 => { rsi = ((r15 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 23; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { r8 = 0x4; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((r15 as u32)) as u32); rdx = rbp.wrapping_add(0x4); crate::simd_ops::wu32(rbp, ((r15 as u32)) as u32); rcx = rdi; crate::simd_ops::wu32(rbp.wrapping_add(0xcu64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 19; continue; }
            19 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0xc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32 as u64; r9 = (crate::simd_ops::ru32(rbp)) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0xb8u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r15 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 23; continue; }
            23 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x1cu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 11; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), ((r15 as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 25; continue; }
            25 => { r8 = ((rbx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 26; continue; }
            26 => { rsi = ((r15 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 38; continue; } bb = 27; continue; }
            27 => { r12 = (0xffffffffffffff0u64) as u64; bb = 28; continue; }
            28 => { r8 = 0x4; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((r15 as u32)) as u32); rdx = rbp.wrapping_add(0x4); crate::simd_ops::wu32(rbp, ((r15 as u32)) as u32); rcx = rdi; crate::simd_ops::wu8(rbp.wrapping_add(0xb8u64), ((r15 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 29; continue; }
            29 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 30; continue; }
            30 => { r8 = ((rax as u32)) as u32 as u64; rdx = rbp.wrapping_add(0xb8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 31; continue; }
            31 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 33; continue; }
            33 => { r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rcx = r8.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = r12; bb = 35; continue; }
            35 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; rcx = rdi; r14 = rsp.wrapping_add(0x30); rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; r9 = r14; rcx = (crate::simd_ops::ru64(P10)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0xb8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xcu64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x28u64), (0x2u8) as u8); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x24u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 28; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((r15 as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 39; continue; }
            39 => { r8 = ((rbx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 51; continue; } bb = 41; continue; }
            41 => { bb = 42; continue; }
            42 => { r8 = 0x4; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((r15 as u32)) as u32); rdx = rbp.wrapping_add(0xc8); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), ((r15 as u32)) as u32); rcx = rdi; crate::simd_ops::wu32(rbp, ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xcu64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 43; continue; }
            43 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32) as u128; rcx = (crate::simd_ops::ru64(P10)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x28u64), ((r15 as u8)) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x24u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rbx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 45; continue; }
            45 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0xc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 46; continue; }
            46 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; r8 = rbp.wrapping_add(0x40); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32) as u128; rcx = (crate::simd_ops::ru64(P10)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x30u64), ((r15 as u8)) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x2cu64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rbx as u32)) as u32 as u64; rdx = rbp.wrapping_add(0xb8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 48; continue; }
            48 => { rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0xb8u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P10)) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x34u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x38u64), ((r15 as u8)) as u8); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x34u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 50; continue; }
            50 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x14u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 42; continue; } bb = 51; continue; }
            51 => { crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), ((r15 as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 52; continue; }
            52 => { r8 = ((rbx.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 63; continue; } bb = 54; continue; }
            54 => { r8 = 0x4; rdx = rbp.wrapping_add(0xb8); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 55; continue; }
            55 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0xc8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 56; continue; }
            56 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x4); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 57; continue; }
            57 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 58; continue; }
            58 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0xc); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 59; continue; }
            59 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 60; continue; }
            60 => { r8 = ((rax.wrapping_add(0x3u64)) as u32) as u32 as u64; rcx = rdi; rdx = rbp.wrapping_add(0x24); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 61; continue; }
            61 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0xc8u64))) as u32) as u128; rdx = rbp.wrapping_add(0x50); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32) as u128; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x54u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0xcu64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x5cu64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x24u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x64u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), ((rcx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 63; continue; } bb = 62; continue; }
            62 => { fl_l = (crate::simd_ops::lk_add_u32(rbp.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 54; continue; } bb = 63; continue; }
            63 => { rcx = rbp.wrapping_add(0xc0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbx as u32)) as u32 as u64; rsp = rbp.wrapping_add(0x70); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            64 => { rax = 0x1; rsp = rbp.wrapping_add(0x70); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ba030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b0ff0; const P1: u64 = 0x14159a6d0; const P2: u64 = 0x140000000; const P3: u64 = 0x1415ba8e0; const P4: u64 = 0x143930f88; const P5: u64 = 0x1415b40b0; const P6: u64 = 0x1415b2be0; const P7: u64 = 0x1415b4ee0; const P8: u64 = 0x1415b5490; const P9: u64 = 0x1415b4c00; const P10: u64 = 0x1415b2fa0; const P11: u64 = 0x1415b51c0; const P12: u64 = 0x1415bac20; const P13: u64 = 0x1415b3820; const P14: u64 = 0x1415b3280; const P15: u64 = 0x1415b4920; const P16: u64 = 0x1415b3dd0; const P17: u64 = 0x1415b5770; const P18: u64 = 0x1415b3b00; const P19: u64 = 0x1415b4650; const P20: u64 = 0x1415b3550; const P21: u64 = 0x1415b4380; const P22: u64 = 0x143930f28; const P23: u64 = 0x1415b1500; const P24: u64 = 0x143930f08; const P25: u64 = 0x143931114; const P26: u64 = 0x1415a5a70;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r12 = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), (0x0u32) as u32); rdi = (rdx) as u64; r14 = (rcx) as u64; r8 = (0x4u32) as u32 as u64; rdx = ((rbp.wrapping_add(0x58u64)) as u64) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(0x50u64)) as u64) as u64; rdx = ((rax as u32)) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rbx = (0x34u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; rcx = (0x1u32) as u32 as u64; rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rbx as u32) }) as u32 as u64; bb = 3; continue; }
            3 => { rsi = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_add(0x40u64), (0x0u8) as u8); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 33; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (r13) as u64); r13 = ((P2) as u64) as u64; bb = 5; continue; }
            5 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 6; continue; }
            6 => { r8 = ((rbx.wrapping_add(0x4u64)) as u32) as u32 as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_sub(0xcu64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x15u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 27; continue; } bb = 8; continue; }
            8 => { rax = (rax as u32 as i32 as i64) as u64; rcx = (crate::simd_ops::ru32(r13.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15ba3b4u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(r13)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            9 => { r8 = (rdi) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            10 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x108u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            11 => { r9 = ((r12 as u32)) as u32 as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            12 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            13 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            14 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            15 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            16 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0xb0u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            17 => { r9 = ((rbp.wrapping_add(0x40u64)) as u64) as u64; r8 = (rdi) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            18 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x58u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            19 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x210u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            20 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x268u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            21 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x268u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            22 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x268u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            23 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x318u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            24 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x3c8u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            25 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x420u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            26 => { r9 = (crate::simd_ops::ru64(P4)) as u64; rdx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; r9 = ((r9).wrapping_add(0x370u64)) as u64; r8 = (rdi) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; bb = 30; continue; }
            27 => { rax = (crate::simd_ops::ru64(P22)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { r8 = ((r12 as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0xcu64)) as u64) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((rax) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0xbu64))) as u32 as u64; r8 = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0x0u32) as u32); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0xbu64))) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_sub(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (0x7u32) as u32 as u64; rbx = (if (((fl_l as u32)) != ((fl_r as u32))) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; bb = 30; continue; }
            30 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(P24)) as u64; rdx = ((P25) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P26) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 33; continue; }
            33 => { rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1415ba030u64) { 0x3f => { bb = 1; continue; } 0x43 => { bb = 2; continue; } 0x60 => { bb = 3; continue; } 0x6e => { bb = 4; continue; } 0x80 => { bb = 5; continue; } 0x89 => { bb = 6; continue; } 0xa5 => { bb = 7; continue; } 0xb4 => { bb = 8; continue; } 0xc3 => { bb = 9; continue; } 0xd9 => { bb = 10; continue; } 0xfd => { bb = 11; continue; } 0x116 => { bb = 12; continue; } 0x133 => { bb = 13; continue; } 0x150 => { bb = 14; continue; } 0x16d => { bb = 15; continue; } 0x18a => { bb = 16; continue; } 0x1ae => { bb = 17; continue; } 0x1c8 => { bb = 18; continue; } 0x1e9 => { bb = 19; continue; } 0x20d => { bb = 20; continue; } 0x231 => { bb = 21; continue; } 0x255 => { bb = 22; continue; } 0x279 => { bb = 23; continue; } 0x29d => { bb = 24; continue; } 0x2c1 => { bb = 25; continue; } 0x2e2 => { bb = 26; continue; } 0x303 => { bb = 27; continue; } 0x30f => { bb = 28; continue; } 0x322 => { bb = 29; continue; } 0x347 => { bb = 30; continue; } 0x352 => { bb = 31; continue; } 0x35d => { bb = 32; continue; } 0x370 => { bb = 33; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8); } } }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ba410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b1130; const P1: u64 = 0x143931558; const P2: u64 = 0x143931560; const P3: u64 = 0x143931568; const P4: u64 = 0x143931570; const P5: u64 = 0x142645710; const P6: u64 = 0x14158e6b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rbx); rcx = ((rcx).wrapping_add(0x8u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), r12); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rbx.wrapping_add(0x38u64)) as u32) as u32 as u64; bb = 24; continue; }
            2 => { r14 = (crate::simd_ops::ru32(rax)) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 3; continue; }
            3 => { r15 = 0; fl_l = 0; fl_r = 0; r12 = (0xffffffffffffff0u64) as u64; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru32(rbx)) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rdi = rbx; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rbx)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(P1)) as u64; rbx = ((rbx).wrapping_add(1)) as u64; rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rdx = ((rcx.wrapping_add(rax)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = rdx; rcx = rax.wrapping_add(0x8); rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(P3)) as u64; rcx = (crate::simd_ops::ru32(P4)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r9 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 14; continue; }
            14 => { fl_l = rax; fl_r = r9; rcx = rax.wrapping_add(0x8); rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { rax = (0xffffffffffffffffu64) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rax).wrapping_add(0x1u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; rax = rax.wrapping_add(0x1); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { rax = ((rax.wrapping_mul(2).wrapping_add(0x2u64)) as u64) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = r12; bb = 19; continue; }
            19 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; rax = (0xffffffffffffffffu64) as u64; rsi = rsp.wrapping_add(0x30); bb = 20; continue; }
            20 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rax))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 21; continue; }
            21 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; r9 = 0xffffffff; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r8 = rdi; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rcx = 0xfde9; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { fl_l = (((r14 as u32)).wrapping_add(0xffffffffu32)) as u32 as u64; fl_r = fl_l; r14 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 4; continue; } bb = 23; continue; }
            23 => { rax = 0x1; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rsp = rbp; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ba5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426890; const P1: u64 = 0x1415b0ff0; const P2: u64 = 0x1415b1500; const P3: u64 = 0x141524330; const P4: u64 = 0x1415bb990; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = ((rdx as u32)) as u32 as u64; rbx = rcx; r14 = 0x1; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; bb = 2; continue; }
            2 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 3; continue; }
            3 => { r12 = rbx.wrapping_add(0x8); rcx = r12; r8 = ((r14.wrapping_add(0x7u64)) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 4; continue; }
            4 => { rbp = (((rbp as u32)).wrapping_add(0xfffffff8u32)) as u32 as u64; r13 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 38; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { r8 = 0x4; rdx = rsp.wrapping_add(0x98); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 7; continue; }
            7 => { r8 = ((rax as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x88); rcx = r12; rbp = (((rbp as u32)).wrapping_add(0xfffffffcu32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 8; continue; }
            8 => { rdi = rbx.wrapping_add(0xc0); rbp = (((rbp as u32)).wrapping_sub(1)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; r8 = rsp.wrapping_add(0x20); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rbp = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 33; continue; }
            16 => { fl_l = ((rdx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 14; continue; }
            17 => { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = (((rdx as u32)).wrapping_add(0x5u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 18; continue; }
            18 => { rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rdx = rax; crate::simd_ops::wu8(rcx.wrapping_add(rax), (0x2eu8) as u8); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x1u64), (0x62u8) as u8); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x2u64), (0x6eu8) as u8); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x3u64), (0x6bu8) as u8); rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(0x4u64), (0x0u8) as u8); rcx = r12; r8 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 19; continue; }
            19 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbp = (((rbp as u32)).wrapping_sub((rax as u32))) as u32 as u64; rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 35; continue; } bb = 23; continue; }
            23 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; r15 = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 26; continue; }
            26 => { bb = 28; continue; }
            27 => { fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 32; continue; } bb = 28; continue; }
            28 => { rdx = 0x18; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            30 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(r15.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx, rcx); rcx = rdx.wrapping_add(0x10); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r15.wrapping_mul(8)), rdx); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 31; continue; }
            31 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rcx, rsi); bb = 33; continue; }
            33 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 34; continue; }
            34 => { bb = 38; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r14 = 0x34; bb = 36; continue; }
            36 => { rdx = rsi; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            37 => { r14 = 0x34; bb = 38; continue; }
            38 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 39; continue; }
            39 => { rax = ((r14 as u32)) as u32 as u64; bb = 40; continue; }
            40 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            41 => { rax = ((r14 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ba880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x141613850; const P3: u64 = 0x1415b6870; const P4: u64 = 0x18; const P5: u64 = 0x10; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; rbx = rdx; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rcx = ((rcx).wrapping_add(0xf0u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = 0x1; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ba8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b1130; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415b16f0; const P6: u64 = 0x1415bcd90; const P7: u64 = 0x14160f560; const P8: u64 = 0x141534ef0; const P9: u64 = 0x1415b1830; const P10: u64 = 0x14160ff40; const P11: u64 = 0x1415b1260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = rcx.wrapping_add(0x8); r13 = rdx; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; rcx = rax; r14 = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r14.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rdi = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); fl_l = ((rdi).wrapping_add(0x160u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; r12 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((crate::simd_ops::ru16(rax.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbp = rdi.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rbp = rdi.wrapping_add(0x8); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rbp; bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rcx = rdi.wrapping_add(0x8); bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rsi = 0xffffffff; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 53; continue; } bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0xa8u64), ((r15 as u8)) as u8); r13 = (r13 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { r8 = rsp.wrapping_add(0xa8); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r12 as u32)) as u32); rdx = rax; rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { r13 = (((r14 as u8)) as u32) as u32 as u64; bb = 42; continue; }
            30 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x0u32) as u32); rax = ((rsi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { r15 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xa8u64))) as u32) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 38; continue; }
            38 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 39; continue; }
            39 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 42; continue; }
            41 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x0u32) as u32); bb = 44; continue; }
            44 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rcx = rsp.wrapping_add(0x90); r14 = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 64; continue; }
            46 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 51; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdx = r15; rcx = rbx; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x1u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 48; continue; }
            48 => { let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 50; continue; }
            50 => { rcx = rsp.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 64; continue; }
            51 => { rcx = rsp.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 60; continue; } bb = 52; continue; }
            52 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x403; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 54; continue; }
            54 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x36u64))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 60; continue; } bb = 55; continue; }
            55 => { rcx = rsp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(r13.wrapping_add(0x1u64))) as u32 as u64; rdx = r15; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 59; continue; } bb = 56; continue; }
            56 => { let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 58; continue; }
            58 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 59; continue; }
            59 => { rcx = rsp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 60; continue; }
            60 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 64; continue; } bb = 61; continue; }
            61 => { r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r8.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 64; continue; } bb = 62; continue; }
            62 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r8.wrapping_add(0x50u64))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 63; continue; }
            63 => { crate::simd_ops::wu64(r8, rbx); bb = 64; continue; }
            64 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((r14 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bac20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931910; const P1: u64 = 0x143931908; const P2: u64 = 0x143930f88; const P3: u64 = 0x1415b1130; const P4: u64 = 0x1426452b0; const P5: u64 = 0x1426452a8; const P6: u64 = 0x142645560; const P7: u64 = 0x1415b16f0; const P8: u64 = 0x1415bcd90; const P9: u64 = 0x1415c86a0; const P10: u64 = 0x141534ef0; const P11: u64 = 0x1415b1830; const P12: u64 = 0x143931914; const P13: u64 = 0x14153e940; const P14: u64 = 0x1415e97d0; const P15: u64 = 0x143931114; const P16: u64 = 0x1415b72f0; const P17: u64 = 0x1415cb610; const P18: u64 = 0x1415b1260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r15 = r8; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(P1)) as u64; rbx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; rax = rcx.wrapping_add(0x8); rdi = (crate::simd_ops::ru64(P2)) as u64; rcx = rax; rbp = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rbp.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); r12 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 8; continue; }
            8 => { r14 = rdi.wrapping_add(0x60); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 10; continue; }
            10 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { rsi = rbx; bb = 14; continue; }
            14 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 19; continue; }
            16 => { r14 = rdi.wrapping_add(0x60); rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 9; continue; } bb = 17; continue; }
            17 => { rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            18 => { rsi = rbx; bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 27; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 21; continue; } bb = 23; continue; }
            23 => { rcx = r14; bb = 28; continue; }
            24 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 29; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            27 => { rcx = rdi.wrapping_add(0x60); bb = 28; continue; }
            28 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { fl_l = rsi; fl_r = rsi; rbx = rsi.wrapping_sub(0x10); rax = 0x0; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 56; continue; } bb = 30; continue; }
            30 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x90u64), ((r15 as u8)) as u8); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rsi = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { r8 = rsp.wrapping_add(0x90); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r12 as u32)) as u32); rdx = rax.wrapping_add(0x10); rcx = rdi.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 45; continue; } bb = 32; continue; }
            32 => { r8 = 0; fl_l = 0; fl_r = 0; rax = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r8 as u32)) as u32); let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbx.wrapping_add(0x10); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; bb = 34; continue; }
            34 => { r15 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as u32) as u32 as u64; rbx = r8; bb = 36; continue; }
            35 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 36; continue; }
            36 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 45; continue; } bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 50; continue; } bb = 38; continue; }
            38 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 43; continue; } bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 40; continue; } bb = 42; continue; }
            42 => { rbx = r8; bb = 43; continue; }
            43 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32); fl_l = ((rbx).wrapping_add(0xfffffffffffffff0u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 45; continue; }
            45 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x58u64), (1) as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { crate::simd_ops::wu32(rdi.wrapping_add(0x5cu64), ((r12 as u32)) as u32); bb = 48; continue; }
            48 => { rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 51; continue; } bb = 49; continue; }
            49 => { rcx = rsp.wrapping_add(0x20); rbp = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 71; continue; }
            50 => { r12 = 0; fl_l = 0; fl_r = 0; rbx = ((r12 as u32)) as u32 as u64; bb = 46; continue; }
            51 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x1u64))) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x260u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 55; continue; } bb = 52; continue; }
            52 => { rcx = rbx.wrapping_add(0x10); let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 54; continue; }
            54 => { rbx = r12; bb = 55; continue; }
            55 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 71; continue; }
            56 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x80u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 73; continue; } bb = 57; continue; }
            57 => { rcx = rsp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(P1)) as u64; rdx = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = r8; rdi = ((rdx as u32)) as u32 as u64; rcx = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r9 = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 60; continue; } bb = 58; continue; }
            58 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 60; continue; } bb = 59; continue; }
            59 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 58; continue; } bb = 60; continue; }
            60 => { r12 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = r9; rcx = ((r12 as u32)) as u32 as u64; rcx = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 70; continue; } bb = 61; continue; }
            61 => { rcx = (crate::simd_ops::ru32(P12)) as u32 as u64; fl_l = rdi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 64; continue; } bb = 62; continue; }
            62 => { rcx = P1; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 67; continue; } bb = 63; continue; }
            63 => { rcx = (crate::simd_ops::ru32(P12)) as u32 as u64; rdx = (crate::simd_ops::ru32(P0)) as u32 as u64; r8 = (crate::simd_ops::ru64(P1)) as u64; bb = 64; continue; }
            64 => { rax = ((rcx as u32)) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 67; continue; } bb = 65; continue; }
            65 => { rax = ((rdx as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(P0, ((rdx as u32)) as u32); rcx = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 67; continue; } bb = 66; continue; }
            66 => { crate::simd_ops::wu64(rcx, rbx); bb = 70; continue; }
            67 => { rbp = 0x34; rcx = rbx.wrapping_add(0x10); rsi = 0xffffffff; let xa_src: u32 = (rsi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rsi = (xa_old) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 68; continue; }
            68 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 69; continue; }
            69 => { rbx = r12; bb = 70; continue; }
            70 => { rcx = rsp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 71; continue; }
            71 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 87; continue; } bb = 72; continue; }
            72 => { bb = 74; continue; }
            73 => { r12 = 0; fl_l = 0; fl_r = 0; bb = 74; continue; }
            74 => { rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 78; continue; } bb = 75; continue; }
            75 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 84; continue; } bb = 76; continue; }
            76 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x64u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 84; continue; } bb = 77; continue; }
            77 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 84; continue; } bb = 78; continue; }
            78 => { rdx = (crate::simd_ops::ru32(P15)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 82; continue; } bb = 79; continue; }
            79 => { rcx = rbx; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 81; continue; } bb = 80; continue; }
            80 => { rcx = rbx; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P15, ((rax as u32)) as u32); bb = 83; continue; }
            81 => { rdx = (crate::simd_ops::ru32(P15)) as u32 as u64; bb = 82; continue; }
            82 => { rcx = rbx; rax = crate::simd_ops::call_stk(P17, rax, rcx, rdx, r8, r9, rsp); bb = 83; continue; }
            83 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu8(rax, (0x1u8) as u8); bb = 84; continue; }
            84 => { rdx = (crate::simd_ops::ru32(r15.wrapping_add(0x58u64))) as u32 as u64; r8 = rbx.wrapping_add(0x10); fl_l = rbx; fl_r = rbx; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { r8 }) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 87; continue; } bb = 85; continue; }
            85 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r15.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x50u64))) as u64; r9 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 87; continue; } bb = 86; continue; }
            86 => { crate::simd_ops::wu64(r9, r8); bb = 87; continue; }
            87 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rbp as u32)) as u32 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x68u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bb080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d98; const P1: u64 = 0x1426452b0; const P2: u64 = 0x60; const P3: u64 = 0x18; const P4: u64 = 0x68; const P5: u64 = 0x1415242e0; const P6: u64 = 0x143930f00; const P7: u64 = 0x1415bc6e0; const P8: u64 = 0x64; const P9: u64 = 0x1426452a8; const P10: u64 = 0x1415bc680; const P11: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = (((rdx as u8)) as u32) as u32 as u64; rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0x8u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(27u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x20000003u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rcx = P0; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x68u64), (0xfdu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 8; continue; }
            8 => { rdx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (0x0u64) as u64); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rcx = rbx; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            12 => { rcx = P0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bb180(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415242e0; const P2: u64 = 0x14183acc8; const P3: u64 = 0x1415b70e0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x1415bb080;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rbp = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = rcx.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); r12 = rcx.wrapping_add(0x90); rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 34; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rsi = (crate::simd_ops::ru64(r12)) as u64; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); rsi = (crate::simd_ops::ru64(rsi.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r14 = rsi; rsi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r13 = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; r8 = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdx); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0xeu64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { bb = 19; continue; }
            12 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; rax = (rax & !0xffffu64) | ((((fl_l as u16).wrapping_sub(fl_r as u16)) as u16 as u64)); crate::simd_ops::wu16(rbx.wrapping_add(0xeu64), ((rax as u16)) as u16); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xdu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = ((rdx).wrapping_sub(rbx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = rbx; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rbx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 22; continue; }
            22 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = ((r10 as u32)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { r8 = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; bb = 24; continue; }
            24 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(r13)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 26; continue; }
            26 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(r14, r9); bb = 29; continue; }
            28 => { rax = (crate::simd_ops::ru64(r12)) as u64; rcx = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), r9); bb = 29; continue; }
            29 => { rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rbx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rbx); crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((rbx as u32)) as u32); bb = 31; continue; }
            31 => { rdx = rsi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(r12.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 32; continue; }
            32 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 33; continue; }
            33 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 34; continue; }
            34 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x58u64)) as u64; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            36 => { rsp = ((rsp).wrapping_add(0x58u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bb3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x1415b70e0; const P2: u64 = 0x14; const P3: u64 = 0x90; const P4: u64 = 0x1415bccb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32);
    rbx = rdx;
    rsi = rcx;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = rdx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdi = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; rcx = ((rsi.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rdi as u32)) as u32 as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bb3f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d98; const P1: u64 = 0x1426452b0; const P2: u64 = 0x143930f00; const P3: u64 = 0x1415b70e0; const P4: u64 = 0x1415242e0; const P5: u64 = 0x1426452a8; const P6: u64 = 0x1416139f0; const P7: u64 = 0x1415bb740;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rsi = rcx; rbx = (((rdx as u8)) as u32) as u32 as u64; rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rsi.wrapping_add(0x64u64), (0x0u32) as u32); bb = 3; continue; }
            2 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x64u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 31; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r13); r13 = (crate::simd_ops::ru64(P2)) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 5; continue; }
            5 => { rcx = r13.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 24; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rcx = ((rbp.wrapping_mul(2)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x90u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r12 = ((rdx.wrapping_mul(8)) as u64) as u64; rdi = (crate::simd_ops::ru64(r12.wrapping_add(rdi))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { r15 = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { bb = 22; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(r15, rcx); bb = 19; continue; }
            18 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(r12.wrapping_add(rax), rcx); bb = 19; continue; }
            19 => { rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), (0x0u32) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), (0x0u32) as u32); bb = 21; continue; }
            21 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(r13.wrapping_add(0xa0u64), (1) as u32)) as u32 as u64; bb = 22; continue; }
            22 => { rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 23; continue; }
            23 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 24; continue; }
            24 => { rcx = r13.wrapping_add(0x68); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_and_u8(rsi.wrapping_add(0x68u64), (0xfbu8) as u8)) as u8 as u64; fl_r = fl_l; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x60u64))) as u32 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 31; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0xa8u64)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = P0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x68u64))) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0x8u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(27u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x20000003u32))) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), (0x0u64) as u64); bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_and_u8(rsi.wrapping_add(0x68u64), (0xfdu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 30; continue; }
            30 => { rdx = rsi; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), (0x0u64) as u64); rcx = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x48u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
            31 => { rcx = P0; rsp = ((rsp).wrapping_add(0x48u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bb640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415b70e0; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rcx; rsi = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0x68u64)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x98u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; rax = ((rsi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = ((rdx.wrapping_mul(8)) as u64) as u64; rdi = (crate::simd_ops::ru64(r15.wrapping_add(rdi))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { r14 = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 14; continue; }
            5 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r14, rcx); bb = 11; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(r15.wrapping_add(rax), rcx); bb = 11; continue; }
            11 => { rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), (0x0u32) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), (0x0u32) as u32); bb = 13; continue; }
            13 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0xa0u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 15; continue; }
            15 => { rcx = rbp.wrapping_add(0x68); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bb740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x58; const P2: u64 = 0x1415b16f0; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1415b1830; const P6: u64 = 0x1415242e0; const P7: u64 = 0x5c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = 0x1; bb = 2; continue; }
            2 => { rcx = rsp.wrapping_add(0x30); rdi = (((rbp as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P3), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdi = (rdi & !0xffffu64) | (((((rdi as u16)).wrapping_add(1)) as u16 as u64)); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 9; continue; }
            9 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = rdx; bb = 10; continue; }
            10 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P1), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P0), rdi); crate::simd_ops::wu32(rsi.wrapping_add(P7), ((rdi as u32)) as u32); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bb810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415bc250; const P2: u64 = 0x1415b70e0; const P3: u64 = 0x1415bccb0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x1415b16f0; const P6: u64 = 0x1415975f0; const P7: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = ((r8 as u32)) as u32 as u64; r13 = rdx; rsi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(0x1fu64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = rcx.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); rbp = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r12 = rdi; r15 = rsi.wrapping_add(0x90); rsi = r13; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(r15.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r15)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); r14 = (r14 & !0xffu64) | ((((((r14 as u8)) | ((rax as u8)))) as u8 as u64)); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = r15; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rsi = ((rsi).wrapping_add(0x18u64)) as u64; fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 18; continue; }
            18 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 19; continue; }
            19 => { rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = r13.wrapping_add(0x8); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = rbx.wrapping_add(0x18); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rdi = rax.wrapping_add(0x10); bb = 12; continue; }
            23 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = 0x63; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bbb10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426890; const P1: u64 = 0x1415bb990; const P2: u64 = 0x141524330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rbx = rcx; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x10u64))) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rdx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = ((rdi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { bb = 10; continue; }
            9 => { rax = rcx.wrapping_add(0x10); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = 0x28; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rsi.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx, rcx); rcx = rdx.wrapping_add(0x10); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rdi as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), (0xffffffffu32) as u32); bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rsi.wrapping_mul(8)), rdx); rax = rcx; crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x10u64), (1) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bbc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = r8; rdi = rdx; rsi = rcx; rbx = 0x1; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { r11 = 0; fl_l = 0; fl_r = 0; r8 = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rsi, r11); rcx = r8; crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), ((r11 as u8)) as u8); bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(rcx))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = r8; bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rax))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_sub(rcx)) as u64; r9 = ((r11 as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rdi)) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r10 = r14; fl_l = r10; fl_r = rdx; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(r10.wrapping_add(rdx))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 12; continue; } bb = 10; continue; }
            10 => { r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = ((r9 as u32)) as u32 as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsi, rdi); rax = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), ((r11 as u8)) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { crate::simd_ops::wu64(rsi, r11); crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), ((r11 as u8)) as u8); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rdx = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 14; continue; }
            14 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(rdx))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 15; continue; }
            15 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { r8 = ((r8).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(r8))) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 17; continue; }
            17 => { bb = 19; continue; }
            18 => { r8 = ((r11 as u32)) as u32 as u64; bb = 19; continue; }
            19 => { rbp = ((r8 as u32)) as u32 as u64; r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); r13 = ((rdx as u32)) as u32 as u64; fl_l = r13; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 26; continue; } bb = 20; continue; }
            20 => { rcx = ((r13 as u32)) as u32 as u64; r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_sub(rbp)) as u64; r8 = ((r11 as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(rdi)) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 21; continue; }
            21 => { r9 = r14; fl_l = r9; fl_r = rcx; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 22; continue; }
            22 => { rax = ((crate::simd_ops::ru8(r9.wrapping_add(rcx))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 23; continue; }
            23 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((r8 as u32)) as u32 as u64; fl_l = rax; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 24; continue; }
            24 => { bb = 27; continue; }
            25 => { r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 26; continue; }
            26 => { r11 = rbp; bb = 27; continue; }
            27 => { r12 = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r12 = ((r12).wrapping_add(r11)) as u64; rdx = r12; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { r8 = r13; rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rcx = ((rbx.wrapping_add(r13)) as u64) as u64; r8 = rbp; rdx = r14; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { crate::simd_ops::wu8(rbx.wrapping_add(r12).wrapping_sub(0x1u64), (0x0u8) as u8); crate::simd_ops::wu64(rsi, rbx); rbx = 0x1; crate::simd_ops::wu8(rsi.wrapping_add(0x8u64), (0x1u8) as u8); bb = 32; continue; }
            31 => { rbx = 0x34; bb = 32; continue; }
            32 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 33; continue; }
            33 => { rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bbdc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f60; const P2: u64 = 0x1415afed0; const P3: u64 = 0x1415b1830;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64));
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bbdf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rdx = (crate::simd_ops::ru64(rcx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, (0x0u64) as u64); crate::simd_ops::wu8(rbx.wrapping_add(P0), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rcx, (0x0u64) as u64);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bbe30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bc4c0; const P1: u64 = 0x141537600; const P2: u64 = 0x1415b70e0; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1415b1690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = rdi.wrapping_add(0xa8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r12 = 0; fl_l = 0; fl_r = 0; r14 = ((r12 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 2; continue; }
            2 => { rax = ((r14 as u32)) as u32 as u64; r15 = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; rsi = (crate::simd_ops::ru64(r15.wrapping_add(rax))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rbp = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r12); crate::simd_ops::wu32(rsi.wrapping_add(0x1cu64), ((r12 as u32)) as u32); bb = 6; continue; }
            6 => { rdx = rsi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = rbp; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r15.wrapping_add(rax), r12); fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0xa0u64), ((r12 as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x90u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x98u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), r12); crate::simd_ops::wu32(rdi.wrapping_add(0x9cu64), ((r12 as u32)) as u32); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xc8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; rbx = ((r12 as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 13; continue; }
            13 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { bb = 18; continue; }
            16 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xc8u64))) as u32 as u64; bb = 20; continue; }
            20 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 22; continue; }
            22 => { bb = 18; continue; }
            23 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 24; continue; }
            24 => { rsi = ((r12 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0xc8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 30; continue; } bb = 25; continue; }
            25 => { bb = 26; continue; }
            26 => { rax = ((rsi as u32)) as u32 as u64; r14 = ((rax.wrapping_mul(8)) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(rax))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(rax), r12); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xc8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 26; continue; } bb = 30; continue; }
            30 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xd0u64), ((r12 as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0xc8u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0xc0u64), r12); crate::simd_ops::wu32(rdi.wrapping_add(0xccu64), ((r12 as u32)) as u32); bb = 32; continue; }
            32 => { rcx = rdi.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0xe0u64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), r12); crate::simd_ops::wu32(rdi.wrapping_add(0xe4u64), ((r12 as u32)) as u32); bb = 34; continue; }
            34 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bc070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415bc250; const P2: u64 = 0x1415b70e0; const P3: u64 = 0x1415bccb0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x1403cd310; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415aa7b0; const P8: u64 = 0x1415aa630;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = ((r8 as u32)) as u32 as u64; r15 = r9; r14 = rdx; rsi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; rdi = r9; rax = 0x1f; while rcx != 0 { crate::simd_ops::wu32(rdi, rax as u32); rdi = rdi.wrapping_add(4); rcx = rcx.wrapping_sub(1); } bb = 4; continue; }
            4 => { rax = 0x1f; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rdi = rcx.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); r12 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbp = ((r12.wrapping_add(0x63u64)) as u32) as u32 as u64; r13 = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 8; continue; }
            8 => { rbp = rsi.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r14)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 10; continue; }
            10 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbp)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 15; continue; }
            15 => { rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (r12 as u32) } else { (rcx as u32) }) as u32 as u64; r12 = ((rcx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            20 => { rbx = rax.wrapping_add(0x10); bb = 14; continue; }
            21 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = ((rsi.wrapping_add(0x19u64)) as u32) as u32 as u64; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x4u64), rcx); rcx = rdi; rax = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rax = 0x63; rax = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r13 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(r15, ((rax as u32)) as u32); bb = 25; continue; }
            25 => { r14 = ((r14).wrapping_add(0x18u64)) as u64; r15 = ((r15).wrapping_add(0x4u64)) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_r = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 26; continue; }
            26 => { rbp = 0x63; bb = 27; continue; }
            27 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(P6)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (r13 as u32) } else { (rbp as u32) }) as u32 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bc250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xe; const P2: u64 = 0xd; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x18; const P5: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rcx = r9; r10 = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; r8 = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x1u8) as u8); rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = ((r10.wrapping_add(P3)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = ((rcx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P0), (1) as u32)) as u32 as u64; bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bc300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931d98; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x1416138e0; const P4: u64 = 0x1415bb3f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rbx); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rsi); r14 = ((r15 as u32)) as u32 as u64; rsi = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rdi); rcx = P0; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm6) as u128); rdi = ((r15 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rsi).wrapping_add(0xa8u64)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (0xffffffffffffff0u64) as u64; bb = 3; continue; }
            3 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rsp = ((rsp).wrapping_sub(rcx)) as u64; r14 = rsp.wrapping_add(0x20); fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { bb = 10; continue; }
            9 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 10; continue; }
            10 => { r9 = r14; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x70u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu128(r9, (xmm0) as u128); fl_l = ((r9).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; bb = 15; continue; }
            15 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 17; continue; }
            17 => { bb = 11; continue; }
            18 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 19; continue; }
            19 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((((rdi as u32)) as i32 as i64) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 29; continue; } bb = 20; continue; }
            20 => { rdi = rbx; rdi = ((rdi).wrapping_shl(4u32)) as u64; fl_l = ((rdi).wrapping_add(r14)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 21; continue; }
            21 => { xmm6 = (crate::simd_ops::ru128(rdi.wrapping_sub(0x10u64))) as u128; rdi = rdi.wrapping_sub(0x10); rbx = ((rbx).wrapping_sub(1)) as u64; rcx = P0; crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm6) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x20); crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm6) as u128); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 23; continue; }
            23 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rdx = rbp; crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); rcx = rsi; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x70u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x70u64), ((rcx as u32)) as u32); bb = 26; continue; }
            26 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            27 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) > 0) { bb = 21; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x30u64))) as u128; rsp = rbp.wrapping_add(0x40); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bc4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b6290; const P1: u64 = 0x1415bc300; const P2: u64 = 0x1415bb740; const P3: u64 = 0x143931d98; const P4: u64 = 0x1426452b0; const P5: u64 = 0x1415242e0; const P6: u64 = 0x143930f00; const P7: u64 = 0x1415bc6e0; const P8: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rbx.wrapping_add(0xa8); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; r9 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; r8 = ((rbp as u32)) as u32 as u64; r10 = ((rbp as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rdx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r10); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { r11 = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; bb = 9; continue; }
            9 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(r9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(r10.wrapping_add(0x10u64), rax); bb = 15; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)), rax); bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x10u64), (1) as u32); rcx = rbx; xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P3; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x60u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 22; continue; } bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x68u64))) as u32) as u32 as u64; rcx = ((((rcx as u32)) & (0x8u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(27u32)) as u32 as u64; rcx = ((((rcx as u32)) | (0x20000003u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x64u64))) as u32 as u64; rcx = P3; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 23; continue; } bb = 19; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_and_u8(rbx.wrapping_add(0x68u64), (0xfdu8) as u8)) as u8 as u64; fl_r = fl_l; bb = 21; continue; }
            21 => { rdx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rbp); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            22 => { rcx = P3; bb = 23; continue; }
            23 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 24; continue; }
            24 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 7; continue; }
            26 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 27; continue; }
            27 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bc680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x48; const P2: u64 = 0x8; const P3: u64 = 0x143930f00; const P4: u64 = 0xec; const P5: u64 = 0x141613870;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru32(rcx)) as u32 as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P4), (0x1u32) as u32); rcx = ((rcx).wrapping_add(0xf0u64)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bc6e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x14183acc8; const P2: u64 = 0x1415b70e0; const P3: u64 = 0x1415242e0; const P4: u64 = 0x1426452a8;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; r13 = rdx; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); rbx = rcx.wrapping_add(0x68); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(r13.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 35; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rbx = (0x2aaaaaaaaaaaaaabu64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x34u64), ((rax as u32)) as u32); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x20u64))) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 33; continue; } bb = 4; continue; }
            4 => { rbp = rdi.wrapping_add(0x90); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 33; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rbp)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } r12 = ((rdx as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rdi.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rsi = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; r8 = rdi.wrapping_add(0x18); r10 = (crate::simd_ops::ru32(r8)) as u32 as u64; rcx = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r8); r14 = rbp; rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; rdx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xcu64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rcx = ((rcx).wrapping_add(0x18u64)) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { bb = 19; continue; }
            16 => { rax = ((r10.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = ((rdx).wrapping_sub(rcx)) as u64; rax = rbx; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = rcx.wrapping_add(0x18); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_sub_u32(r8, (1) as u32)) as u32 as u64; bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 22; continue; }
            22 => { r9 = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((r12 as u32)) as u32 as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; bb = 24; continue; }
            24 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(r14)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 26; continue; }
            26 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rsi, r9); bb = 29; continue; }
            28 => { rax = (crate::simd_ops::ru64(rbp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r12.wrapping_mul(8)), r9); bb = 29; continue; }
            29 => { rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x18u64), (0x0u32) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), (0x0u32) as u32); bb = 31; continue; }
            31 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 32; continue; }
            32 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 35; continue; }
            35 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 36; continue; }
            36 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bc910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415bcaa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(P0)) as u64; r14 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { r8 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r8 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rdi.wrapping_add(0x110); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x150u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x148u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            11 => { rsi = rdi.wrapping_add(0x110); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rdi.wrapping_add(0x110); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; rdi = 0xffffffff; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 26; continue; }
            26 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rax as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { rdx = rbx; rcx = r14; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0xf; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bcaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415a36b0; const P1: u64 = 0x1416138e0; const P2: u64 = 0x1415bb3f0; const P3: u64 = 0x143930f88; const P4: u64 = 0x1426452b0; const P5: u64 = 0x1426452a8; const P6: u64 = 0x142645560; const P7: u64 = 0x1415bcaa0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; r15 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); rcx = 0x2103; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); r13 = 0x403; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x34u64))) as u32) as u32 as u64; rdi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rcx = r15.wrapping_add(0xa8); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 4; continue; }
            4 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            5 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 34; continue; } bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(P3)) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 8; continue; }
            8 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 9; continue; }
            9 => { rbp = rdi.wrapping_add(0x110); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x150u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x148u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { rsi = r12; bb = 16; continue; }
            16 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            18 => { rbp = rdi.wrapping_add(0x110); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 19; continue; }
            19 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            20 => { rsi = r12; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 28; continue; } bb = 22; continue; }
            22 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 22; continue; } bb = 24; continue; }
            24 => { bb = 29; continue; }
            25 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            28 => { rbp = rdi.wrapping_add(0x110); bb = 29; continue; }
            29 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 31; continue; }
            31 => { rdx = rsi; rcx = r15; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rcx = 0x2103; bb = 34; continue; }
            34 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 35; continue; }
            35 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 36; continue; }
            36 => { rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bccb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1415b70e0; const P3: u64 = 0x18; const P4: u64 = 0x1415242e0; const P5: u64 = 0x1c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; r9 = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = ((r9 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rax, rcx); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rdx, rcx); bb = 8; continue; }
            8 => { rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x0u32) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P5), (0x0u32) as u32); bb = 10; continue; }
            10 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(P1), (1) as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bcd70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xec;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; return rax; }
    rax = ((r8 as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bcd90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1434268f0; const P2: u64 = 0x1426456f8; const P3: u64 = 0x1439318d4; const P4: u64 = 0x142645618; const P5: u64 = 0x142645748; const P6: u64 = 0x142645790;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rcx = ((rcx).wrapping_add(0x8u64)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = ((crate::simd_ops::ru64(P1)) as u64) as u128; rdi = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm0 = crate::simd_ops::vcvtps2pd(xmm0); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(xmm6 as u64)).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; crate::simd_ops::lk_add_u64(rsp.wrapping_add(0x40u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_r = rax; if ((((fl_l as u64)) as i64) > (((fl_r as u64)) as i64)) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x30u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rdx = 0xffffffff; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x30u64), (1) as u32)) as u32 as u64; bb = 1; continue; }
            11 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bce80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415cf650; const P1: u64 = 0x141524320; const P2: u64 = 0x14158e930; const P3: u64 = 0x14183acbc; const P4: u64 = 0x14183acc2; const P5: u64 = 0x1415b16f0; const P6: u64 = 0x14158eaa0; const P7: u64 = 0x1415b1830; const P8: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x1u8) as u8); rax = r11.wrapping_sub(0x60); r15 = r8; r14 = rdx; rdi = 0; fl_l = 0; fl_r = 0; r8 = r11.wrapping_sub(0x38); crate::simd_ops::wu64(r11.wrapping_sub(0x80u64), rdi); r12 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); r9 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = r11.wrapping_sub(0x5c); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = r11.wrapping_sub(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = r11.wrapping_sub(0x3c); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x6cu64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rdx as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 25; continue; } bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rbp); rbp = 0x3040; r8 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xcfbe; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 5; continue; }
            5 => { rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rsi); fl_l = ((((rax as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (((rdx as u32)).wrapping_sub((rax as u32))) as u32 as u64; fl_l = (((rdx as u32)).wrapping_add(0xcu32)) as u32 as u64; fl_r = fl_l; rdx = ((fl_l as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rax = 0xffff; fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = (r8 & !0xffffu64) | (((((r8 as u16)).wrapping_sub((rbp as u16))) as u16 as u64)); fl_l = ((r8 as u16)) as u16 as u64; fl_r = (0x1u16) as u16 as u64; if (((fl_l as u16)) > ((fl_r as u16))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(r9.wrapping_add(0x2u64))) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(r9.wrapping_add(0x18u64))) as i32) as u32) as u32 as u64; rdi = ((rdx.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; bb = 10; continue; }
            10 => { rdx = ((rdi as u32)) as u32 as u64; r8 = 0x4; rcx = ((r12 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rbx); rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r13); r13 = ((rcx as u32)) as u32 as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x2u64))) as u32) as u32 as u64; r13 = (((r13 as u32)).wrapping_sub(crate::simd_ops::ru32(r14))) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x64u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rcx = 0xffff; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rbx as u32)) as u32); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rbp = 0x40001; bb = 16; continue; }
            13 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rbp = 0x130001; bb = 16; continue; }
            15 => { rcx = 0x3041; rbp = 0x1; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; rdx = 0x140001; rbp = (if (((fl_l as u16)) == ((fl_r as u16))) { (rdx as u32) } else { (rbp as u32) }) as u32 as u64; bb = 16; continue; }
            16 => { rcx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = 0xfffe; rdx = ((rbx.wrapping_add(rsi)) as u64) as u64; crate::simd_ops::wu16(r13.wrapping_add(rsi), ((rax as u16)) as u16); rcx = ((rbx as u32)) as u32 as u64; rax = 0x10; crate::simd_ops::wu16(r13.wrapping_add(rsi).wrapping_add(0xeu64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x64u64))) as u32) as u32 as u64; rax = (rax & !0xffffu64) | (((((rax as u16)).wrapping_add((rax as u16))) as u16 as u64)); crate::simd_ops::wu16(r13.wrapping_add(rsi).wrapping_add(0xcu64), ((rax as u16)) as u16); r13 = rdx.wrapping_sub(0x8); fl_l = ((((rcx as u32)) & (0x3u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(r13)) as u64; r8 = 0x4; r8 = (((r8 as u32)).wrapping_sub((rcx as u32))) as u32 as u64; crate::simd_ops::wu32(r13, (0x4b4e554au32) as u32); rcx = rdx.wrapping_add(0x8); crate::simd_ops::wu32(r13.wrapping_add(0x4u64), ((r8 as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x8u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(r13.wrapping_add(0x4u64)))) as u32 as u64; r13 = ((rcx as u32)) as u32 as u64; r13 = ((r13).wrapping_add(0xfffffffffffffff8u64)) as u64; fl_l = ((r13).wrapping_add(rsi)) as u64; fl_r = fl_l; r13 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r13, rbx); rbx = ((rcx as u32)) as u32 as u64; bb = 19; continue; }
            19 => { rcx = rsp.wrapping_add(0xe8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = r13.wrapping_add(0x4); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = (((r8 as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rbx as u32)) as u32 as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = ((rbp as u32)) as u32 as u64; rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0xe8); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = ((r12 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rsi); crate::simd_ops::wu32(r15, ((rdi as u32)) as u32); bb = 22; continue; }
            21 => { rdx = rsi; rcx = ((r12 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 23; continue; }
            23 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 24; continue; }
            24 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 25; continue; }
            25 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bd140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14268a678; const P1: u64 = 0x10; const P2: u64 = 0x14;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x1u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bd1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rbx = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x2; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); }
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bd1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1415bcd90; const P2: u64 = 0x1426452a8; const P3: u64 = 0x1415b16f0; const P4: u64 = 0x1415b1830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; rdi = rcx; rax = ((rdx as u32)) as u32 as u64; rsi = (((rax) as i64).wrapping_mul((0x58u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rsi)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); bb = 3; continue; }
            3 => { rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { r9 = (crate::simd_ops::ru64(P0)) as u64; r9 = ((r9).wrapping_add(rsi)) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r9.wrapping_add(0x40u64))) as u64; r10 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r10)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { bb = 15; continue; }
            11 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); bb = 14; continue; }
            13 => { crate::simd_ops::wu64(r10, rax); bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_sub_u32(r9.wrapping_add(0x50u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), (0x0u32) as u32); bb = 17; continue; }
            17 => { rcx = rbx.wrapping_add(0x8); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bd2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14268a698; const P1: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bd5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14268a138; const P2: u64 = 0x143930f60; const P3: u64 = 0x1415af630; const P4: u64 = 0x1415af3d0; const P5: u64 = 0x1415242e0; const P6: u64 = 0x1415af8b0; const P7: u64 = 0x1415bda50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rsi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rdx = 0x48; rcx = ((rdx.wrapping_sub(0x46u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rax = P1; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rbx, rax); r8 = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rdi as u32)) as u32); rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), (0x2ca33bdbu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), r14); crate::simd_ops::wu8(rbx.wrapping_add(0x40u64), ((r14 as u8)) as u8); rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = ((r14.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r14); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rcx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), ((rax as u32)) as u32); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rax = rbx; bb = 8; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bd700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415af950;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = r8; rdi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdi = ((crate::simd_ops::ru8(rax.wrapping_add(0x40u64))) as u32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rdi as u32)) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu32(rdx, ((rdi as u32)) as u32); bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rsi.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 15; continue; }
            15 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 16; continue; }
            16 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rdx, ((rdi as u32)) as u32); bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bd7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x9; const P2: u64 = 0x2; const P3: u64 = 0x143930f80; const P4: u64 = 0x1415adef0; const P5: u64 = 0x18; const P6: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; rbx = ((rax.wrapping_mul(2).wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rbp = ((rbp).wrapping_add(1)) as u64; rdi = ((rdx.wrapping_add(P2)) as u64) as u64; rax = 0xfffffff8; rbx = (((rbx) & (rax))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rbp = ((rdx.wrapping_add(rbp.wrapping_mul(2))) as u64) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, r14); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdi = ((rdi).wrapping_add(0x2u64)) as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 7; continue; }
            7 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bd8a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x2; const P4: u64 = 0x9; const P5: u64 = 0x143930f80; const P6: u64 = 0x1415adef0; const P7: u64 = 0x18; const P8: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = 0xfffffff8; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rsi = ((rcx.wrapping_add(P3)) as u64) as u64; rbp = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rbp = ((rbp).wrapping_add(1)) as u64; rdi = ((rax.wrapping_mul(2).wrapping_add(P4)) as u64) as u64; rdi = (((rdi) & (r15))) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rbp = ((rcx.wrapping_add(rbp.wrapping_mul(2))) as u64) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdi).wrapping_add(rcx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P5)) as u64; r8 = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, r14); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rsi = ((rsi).wrapping_add(0x2u64)) as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 10; continue; }
            10 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bd9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x8; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = ((rdi as u32)) as u32 as u64;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bda10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(rcx.wrapping_mul(4)))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bda50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f80; const P1: u64 = 0x1415ada30; const P2: u64 = 0x141524330; const P3: u64 = 0x14183acbc; const P4: u64 = 0x1415242e0; const P5: u64 = 0x1415bd7f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rbx); rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); r11 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rdi = 0xfffffff8; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); r8 = 0xfffffffc; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rcx; rdx = rax.wrapping_add(0x8); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r11 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; r14 = rcx.wrapping_add(0x2); r12 = ((rax.wrapping_mul(2).wrapping_add(0x5u64)) as u64) as u64; rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; r12 = (((r12) & (r8))) as u64; r12 = ((r12).wrapping_add(rcx)) as u64; rax = ((rcx.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rax); fl_l = r14; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 33; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; xmm6 = ((crate::simd_ops::ru32(r12)) as u32) as u128; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { r9 = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; rcx = ((r11 as u32)) as u32 as u64; r10 = ((crate::simd_ops::ru16(r14)) as u32) as u32 as u64; bb = 12; continue; }
            12 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rdx.wrapping_mul(2)))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru16(r14)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x98u64), ((rax as u16)) as u16); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 15; continue; }
            15 => { rsi = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; bb = 21; continue; }
            16 => { rax = ((r9.wrapping_mul(2).wrapping_add(0x9u64)) as u32) as u32 as u64; r9 = ((rcx as u32)) as u32 as u64; rax = (((rax) & (rdi))) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = ((r9).wrapping_add(rax)) as u64; fl_r = fl_l; r9 = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 17; continue; }
            17 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = r10; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r11 as u32)) as u32); xmm3 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r11 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            19 => { crate::simd_ops::wu32(r9, ((xmm6 as u32)) as u32); bb = 29; continue; }
            20 => { rsi = ((r11 as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rdx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = (((rdx as u32)).wrapping_shl(4u32)) as u32 as u64; r13 = ((rsi.wrapping_add(rsi)) as u32) as u32 as u64; rbp = ((r13.wrapping_add(0xbu64)) as u32) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; rax = ((((rax as u32)) & ((rdi as u32)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 22; continue; }
            22 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdi = ((rsi as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { r8 = ((rdi.wrapping_add(rdi)) as u64) as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rcx = rax.wrapping_add(0x2); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((r13.wrapping_add(0x9u64)) as u32) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; r13 = 0xfffffff8; r8 = ((rdi as u32)) as u32 as u64; rdx = (((rdx) & (r13))) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x20u64)))) as u64; rcx = (((rcx) & (r13))) as u64; rcx = ((rcx).wrapping_add(r15)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 25; continue; }
            24 => { r13 = 0xfffffff8; bb = 25; continue; }
            25 => { rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x98u64))) as u32) as u32 as u64; rcx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu16(r15.wrapping_add(rcx.wrapping_mul(2)), ((rax as u16)) as u16); rbp = (((rbp) & (r13))) as u64; rsi = (rsi & !0xffffu64) | (((((rsi as u16)).wrapping_add(1)) as u16 as u64)); rdi = ((rdi).wrapping_shl(4u32)) as u64; crate::simd_ops::wu16(r15, ((rsi as u16)) as u16); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r15); rax = ((r15.wrapping_add(rbp)) as u64) as u64; fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { r11 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi, ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), r11); rdi = 0xfffffff8; bb = 29; continue; }
            27 => { rdi = 0xfffffff8; bb = 28; continue; }
            28 => { r11 = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { r14 = ((r14).wrapping_add(0x2u64)) as u64; r12 = ((r12).wrapping_add(0x4u64)) as u64; fl_l = r14; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 30; continue; }
            30 => { bb = 32; continue; }
            31 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 32; continue; }
            32 => { r8 = 0xfffffffc; bb = 33; continue; }
            33 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; r11 = 0x0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 34; continue; }
            34 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; bb = 36; continue; }
            36 => { rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bdd20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x2; const P4: u64 = 0x9; const P5: u64 = 0x143930f80; const P6: u64 = 0x1415add10; const P7: u64 = 0x1415adf30; const P8: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r14 = (((rdx as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = 0xfffffff8; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rsi = ((rcx.wrapping_add(P3)) as u64) as u64; rbp = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rbp = ((rbp).wrapping_add(1)) as u64; rdi = ((rax.wrapping_mul(2).wrapping_add(P4)) as u64) as u64; rdi = (((rdi) & (r15))) as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; rbp = ((rcx.wrapping_add(rbp.wrapping_mul(2))) as u64) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdi).wrapping_add(rcx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P5)) as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            9 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rsi = ((rsi).wrapping_add(0x2u64)) as u64; rdi = ((rdi).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 14; continue; }
            14 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bddf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415be300; const P1: u64 = 0x1415bd330;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = rdx; r12 = rcx; rdx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r8 = rax.wrapping_add(0x1); rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r8); rdi = ((((rdi as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r8); rcx = ((((rcx as u32)) & (0x7fu32))) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; rdi = ((((rdi as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r12)) as u64; rcx = r12; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r12)) as u64; rcx = r12; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdx = ((rdi as u32)) as u32 as u64; rsi = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { r10 = 0; fl_l = 0; fl_r = 0; r11 = ((rdi as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r9 = rax.wrapping_add(0x1); r8 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r9); r8 = ((((r8 as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rdx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r9); rcx = ((((rcx as u32)) & (0x7fu32))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu16(r10.wrapping_add(rax), ((r8 as u16)) as u16); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu8(r10.wrapping_add(rax).wrapping_add(0x2u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(r10).wrapping_add(0x3u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(r10).wrapping_add(0x3u64), ((rcx as u8)) as u8); r10 = ((r10).wrapping_add(0x4u64)) as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; r8 = rax.wrapping_add(0x1); rbp = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r8); rbp = ((((rbp as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rax = ((rbp as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r8); rcx = ((((rcx as u32)) & (0x7fu32))) as u32 as u64; rbp = ((rcx as u32)) as u32 as u64; rbp = ((((rbp as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 13; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(r12)) as u64; rcx = r12; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = 0x34; bb = 30; continue; }
            17 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = r12; rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rax = (crate::simd_ops::ru64(r12)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; crate::simd_ops::wu64(rbx, rcx); crate::simd_ops::wu8(rax.wrapping_add(0x40u64), ((rdx as u8)) as u8); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r9 = rcx.wrapping_add(0x1); r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r9); r8 = ((((r8 as u32)) & (0x7fu32))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) >= 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { rdx = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; r9 = ((r9).wrapping_add(1)) as u64; rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbx, r9); rcx = ((((rcx as u32)) & (0x7fu32))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; r8 = ((((r8 as u32)) | ((rax as u32)))) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 22; continue; } bb = 23; continue; }
            23 => { rsi = (((r8 as u16)) as u32) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rcx = r15; r8 = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = ((rax).wrapping_add(0x4u64)) as u64; crate::simd_ops::wu64(rbx, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 26; continue; }
            26 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 27; continue; }
            27 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(r12)) as u64; rcx = r12; crate::simd_ops::wu8(r12.wrapping_add(0x8u64), (0x1u8) as u8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; bb = 29; continue; }
            29 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 30; continue; }
            30 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            31 => { rax = 0x2; bb = 29; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415be080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f60; const P1: u64 = 0x1415affb0; const P2: u64 = 0x1401e5030; const P3: u64 = 0x141541e90; const P4: u64 = 0x1415af3d0; const P5: u64 = 0x1415242e0; const P6: u64 = 0x1415bda50;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; r15 = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r12 = 0; fl_l = 0; fl_r = 0; rdx = ((r12 as u32)) as u32 as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 21; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rcx); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = rbx; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 19; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rdi = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { fl_l = rax; fl_r = rcx; rsi = rax.wrapping_add(0x8); rsi = (if (((fl_l as u64)) == ((fl_r as u64))) { r12 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rdx = ((rdi as u32)) as u32 as u64; rcx = rbx.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 10; continue; } bb = 18; continue; }
            18 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 19; continue; }
            19 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x2; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = r15; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 22; continue; }
            22 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415be1c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f60; const P1: u64 = 0x1415affb0; const P2: u64 = 0x1401e5030; const P3: u64 = 0x141541e90; const P4: u64 = 0x1415af3d0; const P5: u64 = 0x1415242e0; const P6: u64 = 0x1415bda50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r12 = (((rdx as u8)) as u32) as u32 as u64; r15 = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rbp = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r14 = rax.wrapping_add(0x8); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); r13 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rcx = r14; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = r13; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rcx, rax); r8 = rbp; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 10; continue; }
            10 => { fl_l = rax; fl_r = rcx; rdi = rax.wrapping_add(0x8); rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { r13 } else { rdi }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 15; continue; }
            15 => { rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbp; rcx = 0x2; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(r14)) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 16; continue; }
            16 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = r15; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415be300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdi = rcx; r14 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 20; continue; } bb = 1; continue; }
            1 => { rbx = ((rax as u32)) as u32 as u64; r15 = 0xffff; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 15; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; rax = ((r14.wrapping_mul(4)) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { r8 = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { bb = 21; continue; }
            5 => { rdx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 7; continue; }
            7 => { fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdx = rsi; r8 = rbx; bb = 10; continue; }
            10 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_and_u8(rdx.wrapping_add(0x3u64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu16(rdx, ((r15 as u16)) as u16); crate::simd_ops::wu8(rdx.wrapping_add(0x2u64), (0x2u8) as u8); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdx; rcx = ((rcx).wrapping_sub(rsi)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r14 as u32)) as u32); bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(0x3u64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu16(rcx, ((r15 as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(0x2u64), (0x2u8) as u8); bb = 19; continue; }
            19 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = rbx; fl_r = r14; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((r14 as u32)) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415be410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x141524330; const P2: u64 = 0x1415bda50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((r9 as u8)) as u8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r13 = r8; rbp = ((rdx as u32)) as u32 as u64; r12 = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 33; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r12)) as u64; rcx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r12)) as u64; rcx = r12; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; rbx = rax; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = ((rsi as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru64(rax)) as u64; bb = 4; continue; }
            4 => { rax = ((rcx as u32)) as u32 as u64; rdx = ((r9.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((rax.wrapping_mul(8)) as u64) as u64; rax = ((crate::simd_ops::ru16(r8.wrapping_add(r13))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rdx)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(r8.wrapping_add(r13).wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x2u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 10; continue; }
            10 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 27; continue; } bb = 12; continue; }
            12 => { rdi = ((rax as u32)) as u32 as u64; rdx = 0xffff; fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 23; continue; } bb = 13; continue; }
            13 => { rdx = ((rbp.wrapping_mul(4)) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { fl_l = rax; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 16; continue; }
            16 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { rcx = r15; rdx = ((rdi as u32)) as u32 as u64; r8 = 0xffff; bb = 18; continue; }
            18 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(0x3u64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu16(rcx, ((r8 as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(0x2u64), (0x2u8) as u8); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_sub(r15)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx))) as u32 as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { crate::simd_ops::wu64(rbx, r15); rdx = 0xffff; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 23; continue; }
            23 => { fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 27; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = ((rax.wrapping_add(rdi.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(0x3u64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu16(rcx, ((rdx as u16)) as u16); crate::simd_ops::wu8(rcx.wrapping_add(0x2u64), (0x2u8) as u8); bb = 26; continue; }
            26 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = rbp; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 24; continue; } bb = 27; continue; }
            27 => { r9 = rbp; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); r8 = r13.wrapping_add(0x5); bb = 28; continue; }
            28 => { rax = ((crate::simd_ops::ru16(r8.wrapping_sub(0x5u64))) as u32) as u32 as u64; rsi = rsi.wrapping_add(0x4); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = r8.wrapping_add(0x8); crate::simd_ops::wu16(rsi.wrapping_add(rcx).wrapping_sub(0x4u64), ((rax as u16)) as u16); rax = ((crate::simd_ops::ru8(r8.wrapping_sub(0x9u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu8(rsi.wrapping_add(rcx).wrapping_sub(0x2u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(r8.wrapping_sub(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(rsi).wrapping_sub(0x1u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfeu8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(rsi).wrapping_sub(0x1u64), ((rcx as u8)) as u8); fl_l = r9; fl_r = (0x1u64) as u64; r9 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 29; continue; }
            29 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = r12; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 33; continue; }
            33 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415be610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r11 = ((rdx as u32)) as u32 as u64; rsi = (((r9 as u8)) as u32) as u32 as u64; r11 = (((r11 as u32)).wrapping_shr(16u32)) as u32 as u64; r10 = rdx; rdi = rcx; fl_l = ((r11 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; bb = 3; continue; }
            2 => { xmm0 = 0u128; bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { r9 = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rdx.wrapping_mul(2)))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 13; continue; } bb = 6; continue; }
            6 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (((r11 as u8)) as u32) as u32 as u64; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; rdx = (((r10 as u16)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rbx as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rdx = 0xfffffff8; rax = ((r9.wrapping_mul(2).wrapping_add(0x9u64)) as u32) as u32 as u64; rax = (((rax) & (rdx))) as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; fl_l = ((rdx).wrapping_add(rax)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 7; continue; } bb = 14; continue; }
            14 => { xmm1 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdx, ((xmm2 as u32)) as u32); xmm0 = (xmm1) as u128; fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), (0x0u64) as u64); rbx = (rbx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415be700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bd330; const P1: u64 = 0x1401e5030; const P2: u64 = 0x141541e90; const P3: u64 = 0x1415bda50;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; r13 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rbx); r14 = rcx; rbx = r8; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 66; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r14)) as u64; rdi = 0x1; rcx = r14; crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((rdi as u32)) as u32); r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = 0; fl_l = 0; fl_r = 0; rsi = ((r10 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((r10 as u32)) as u32); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rax = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rsi as u32)) as u32); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r10); r12 = r10; r8 = (0xffffffffffffff0u64) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rdx = ((rsi as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; rax = rdx.wrapping_add(0xf); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = r8; bb = 8; continue; }
            8 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; r12 = rsp.wrapping_add(0x20); fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 9; continue; }
            9 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r12, ((rax as u32)) as u32); r12 = ((r12).wrapping_add(0x4u64)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 12; continue; }
            12 => { r12 = ((r12).wrapping_sub(rdx)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r12); r10 = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 60; continue; } bb = 14; continue; }
            14 => { r13 = rbx.wrapping_add(0x4); bb = 15; continue; }
            15 => { rdx = ((r10 as u32)) as u32 as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(r13.wrapping_sub(0x4u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r12.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; bb = 16; continue; }
            19 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; bb = 20; continue; }
            20 => { if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 21; continue; }
            21 => { rcx = ((rdx as u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(r12.wrapping_add(rsi.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(r12.wrapping_add(rcx.wrapping_mul(4)), ((rax as u32)) as u32); rcx = r14; rax = (crate::simd_ops::ru64(r14)) as u64; rbx = (crate::simd_ops::ru32(r13.wrapping_sub(0x4u64))) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 26; continue; }
            26 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rax = ((crate::simd_ops::ru8(r13.wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x40u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) != ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((((r15 as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; r15 = (r15 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp, ((r15 as u8)) as u8); bb = 29; continue; }
            28 => { rax = (crate::simd_ops::ru64(r14)) as u64; r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (crate::simd_ops::ru32(r13.wrapping_sub(0x4u64))) as u32 as u64; r8 = (((r15 as u8)) as u32) as u32 as u64; rcx = r14; crate::simd_ops::wu8(rbp, ((r15 as u8)) as u8); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 29; continue; }
            29 => { rcx = ((crate::simd_ops::ru8(r13.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x40u64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(2u32)) as u64; rax = rcx.wrapping_add(0xf); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = (0xffffffffffffff0u64) as u64; bb = 31; continue; }
            31 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; rbx = ((r10 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; r14 = rsp.wrapping_add(0x20); r9 = r14; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 32; continue; }
            32 => { bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r9 = r9.wrapping_add(0x4); rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(r9.wrapping_sub(0x4u64), ((rcx as u32)) as u32); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 34; continue; }
            34 => { rdx = rcx; bb = 35; continue; }
            35 => { rsi = ((r10 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r13)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 46; continue; } bb = 36; continue; }
            36 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; r12 = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; bb = 37; continue; }
            37 => { rax = ((r10 as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 41; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; bb = 39; continue; }
            39 => { rcx = ((rax as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 39; continue; } bb = 41; continue; }
            41 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rdx = ((rax as u32)) as u32 as u64; fl_l = (((rbx as u32)).wrapping_sub(1)) as u32 as u64; rbx = ((fl_l as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(r14.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(rdx.wrapping_mul(4)), ((rax as u32)) as u32); bb = 44; continue; }
            43 => { r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x4u64))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(r15)) as u32 as u64; rcx = rdi; crate::simd_ops::wu8(rbp, (0x1u8) as u8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; r12 = (if (((fl_l as u32)) != ((fl_r as u32))) { (rax as u32) } else { (r12 as u32) }) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; bb = 44; continue; }
            44 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r15 = ((r15).wrapping_add(0x8u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r13)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 37; continue; } bb = 45; continue; }
            45 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((r12 as u32)) as u32); r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), r15); bb = 46; continue; }
            46 => { r15 = ((crate::simd_ops::ru8(rbp)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); r15 = (r15 & !0xffu64) | ((((((r15 as u8)) | ((rax as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp, ((r15 as u8)) as u8); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 57; continue; } bb = 47; continue; }
            47 => { r15 = ((rbx as u32)) as u32 as u64; bb = 48; continue; }
            48 => { rsi = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = rdx; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 51; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 51; continue; }
            51 => { fl_l = rax; fl_r = rcx; rbx = rax.wrapping_add(0x8); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { r10 } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 52; continue; }
            52 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 54; continue; }
            54 => { rdx = ((rsi as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x28); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r10 = 0; fl_l = 0; fl_r = 0; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; bb = 55; continue; }
            55 => { r14 = ((r14).wrapping_add(0x4u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 48; continue; } bb = 56; continue; }
            56 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; r15 = ((crate::simd_ops::ru8(rbp)) as u32) as u32 as u64; bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x68u64))) as u32 as u64; r13 = ((r13).wrapping_add(0xcu64)) as u64; rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 58; continue; }
            58 => { rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; bb = 60; continue; }
            59 => { rdi = 0x34; bb = 60; continue; }
            60 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); r15 = (r15 & !0xffu64) | ((((((r15 as u8)) | ((rax as u8)))) as u8 as u64)); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 63; continue; } bb = 61; continue; }
            61 => { rbx = ((rsi as u32)) as u32 as u64; bb = 62; continue; }
            62 => { r9 = (crate::simd_ops::ru64(r14)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru32(r12)) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r12 = r12.wrapping_add(0x4); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 62; continue; } bb = 63; continue; }
            63 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 65; continue; } bb = 64; continue; }
            64 => { rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 65; continue; }
            65 => { rax = ((rdi as u32)) as u32 as u64; bb = 67; continue; }
            66 => { rax = 0x34; bb = 67; continue; }
            67 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rsp = rbp.wrapping_add(0x20); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415beac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x48;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdx as u8)) as u8);
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415beae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141593180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rsi = rdx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0x380;
    rbx = r9;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r9 = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415beb80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a718; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1415bd8a0; const P4: u64 = 0x1415be1c0; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x1415242e0; const P8: u64 = 0x24; const P9: u64 = 0x1415bd2f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(P2), rsi); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P5), rsi); crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rsi as u32)) as u32); bb = 6; continue; }
            6 => { rdx = rdi; rcx = 0x2; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bec10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a718; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1415bd8a0; const P4: u64 = 0x1415be1c0; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x1415242e0; const P8: u64 = 0x24; const P9: u64 = 0x1415bd2f0; const P10: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rcx, rax); rsi = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(P2), rbp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P5), rbp); crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rbp as u32)) as u32); bb = 6; continue; }
            6 => { rdx = rdi; rcx = 0x2; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = 0x18; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415becc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x2; const P3: u64 = 0x141524330; const P4: u64 = 0x8; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0xffffffffffffffd8; const P8: u64 = 0x14159edc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); fl_l = rbx; fl_r = rbx; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rcx }) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); fl_l = rax; fl_r = rax; bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bed30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    rcx = ((rdx as u32)) as u32 as u64;
    fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64;
    if !(((fl_l as u64) == 0)) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bed50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x238;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = rcx;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rcx).wrapping_add(0xffffffffffffffd8u64)) as u64; rdx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rcx }) as u64; fl_l = ((r8).wrapping_add(0x10u64)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bed80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rdi); rax = rcx.wrapping_add(0x30); rsi = rcx; crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), r14); fl_l = rsi; fl_r = rsi; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), r15); rcx = 0x58; r14 = (((rdx as u16)) as u32) as u32 as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64; r15 = ((r9 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = ((rdi).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = (((r14 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); rbp = 0x1; rbp = ((rbp).wrapping_shl(((rcx as u32) & 63))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); rbx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(r11.wrapping_add(0x20u64), r12); r12 = 0xd4; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32) as u128; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = rsi.wrapping_add(0xac); fl_l = rsi; fl_r = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); xmm0 = (xmm6) as u128; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), (0x3u8) as u8); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rax }) as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); r8 = ((r14 as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x30); xmm3 = (xmm6) as u128; rax = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(7u32)) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x35u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0xb0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            13 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415beee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bddf0; const P1: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r10 = ((rax as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = ((r10 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rax.wrapping_add(P1), r9);
    r11 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru32(r11.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64;
    if (((fl_l as u32)) <= ((fl_r as u32))) { rax = ((r10 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = ((r9 as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(1)) as u32 as u64; rdx = ((crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = (((rax) >> (((rdx) as u32 & 63))) & 1) as u64; rax = ((rax) | ((1u64) << (((rdx) as u32 & 63)))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r11.wrapping_add(0x20u64))) as u32 as u64; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    rax = ((r10 as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bef40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415be410; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1415bda50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (((r9 as u8)) as u32) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; rbx = r8; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(P1), (0x0u64) as u64); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { r8 = ((rsi as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rbx = ((rbx.wrapping_add(P2)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = (((rcx) >> (((rax) as u32 & 63))) & 1) as u64; rcx = ((rcx) | ((1u64) << (((rax) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx); fl_l = r8; fl_r = (0x1u64) as u64; r8 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415befc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffd8; const P1: u64 = 0x118; const P2: u64 = 0x14159cab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rsi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rbp = 0;
    fl_l = 0; fl_r = 0;
    rcx = rsi;
    fl_l = rdi; fl_r = rdi;
    rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rcx }) as u64;
    rdx = ((rbp.wrapping_add(0x1u64)) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rdi; fl_r = rdi;
    rbx = (((rax as u8)) as u32) as u32 as u64;
    rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rsi }) as u64;
    rcx = rsi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rax = (((rbx as u8)) as u32) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffd8; const P1: u64 = 0x1; const P2: u64 = 0x120; const P3: u64 = 0x1415a1730;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = ((rcx.wrapping_add(P0)) as u64) as u64;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    rcx = rdi;
    fl_l = rbx; fl_r = rbx;
    rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { rcx }) as u64;
    rdx = ((rsi.wrapping_add(P1)) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rbx; fl_r = rbx;
    rdi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsi } else { rdi }) as u64;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415aef80; const P1: u64 = 0x48;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14159bc00; const P1: u64 = 0xe8; const P2: u64 = 0x14268a798; const P3: u64 = 0x142677588; const P4: u64 = 0x8; const P5: u64 = 0x14268aa00; const P6: u64 = 0x10; const P7: u64 = 0x14268aa28; const P8: u64 = 0x28; const P9: u64 = 0xe9; const P10: u64 = 0xd8; const P11: u64 = 0xe0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u8(rbx.wrapping_add(P1), (0xc7u8) as u8);
    rax = P2;
    crate::simd_ops::wu64(rbx, rax);
    rax = P3;
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    rax = P5;
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    rax = P7;
    crate::simd_ops::wu64(rbx.wrapping_add(P8), rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::lk_and_u8(rbx.wrapping_add(P9), (0x80u8) as u8);
    crate::simd_ops::wu64(rbx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P11), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf0f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a798; const P1: u64 = 0x142677588; const P2: u64 = 0x8; const P3: u64 = 0x14268aa00; const P4: u64 = 0x10; const P5: u64 = 0x14268aa28; const P6: u64 = 0x28; const P7: u64 = 0x14159e510; const P8: u64 = 0xe0; const P9: u64 = 0x1415242e0; const P10: u64 = 0xc; const P11: u64 = 0xd8; const P12: u64 = 0x14159c0b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rax = P3; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rax = P5; crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rsi as u32)) as u32); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0x2; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P8), rsi); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P11))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0x2; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), rsi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x1415242e0; const P4: u64 = 0x24;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rdi = rdx; rbp = 0; fl_l = 0; fl_r = 0; rbx = rcx; crate::simd_ops::wu128(rcx, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(P0), ((xmm1 as u64)) as u64); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rbp as u32)) as u32); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P2), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P4), ((rax as u32)) as u32); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu64(rdi.wrapping_add(P2), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x20; const P2: u64 = 0x14; const P3: u64 = 0x1415242e0; const P4: u64 = 0x24; const P5: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = rcx; r8 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P2), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbx = rdx; bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = rbx; crate::simd_ops::wu32(rdi.wrapping_add(P1), (0x0u32) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu32(rdi.wrapping_add(P4), (0x0u32) as u32); bb = 8; continue; }
            8 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = 0x28; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0xd8; const P2: u64 = 0xa0; const P3: u64 = 0x8; const P4: u64 = 0x80; const P5: u64 = 0xd2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = rcx; r8 = (((rdx as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu16(rcx.wrapping_add(P2), ((rdx as u16)) as u16); bb = 7; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu16(rcx.wrapping_add(P4), ((r8 as u16)) as u16); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; rax = ((rax).wrapping_add(0x10u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { r8 = (r8 & !0xffffu64) | ((((((r8 as u16)) ^ (crate::simd_ops::ru16(r9.wrapping_add(P5))))) as u16 as u64)); rax = 0x3ff; r8 = (r8 & !0xffffu64) | ((((((r8 as u16)) & ((rax as u16)))) as u16 as u64)); fl_l = (crate::simd_ops::lk_xor_u16(r9.wrapping_add(P5), ((r8 as u16)) as u16)) as u16 as u64; fl_r = fl_l; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14153e940; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0xe0u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbp = rdx; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = ((rbx.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax, r14); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r14); bb = 4; continue; }
            3 => { rbx = r14; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(0xe0u64), rbx); bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdx, rbp); r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x180u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; bb = 15; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe0u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = 0x2; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(0xe0u64), r14); bb = 14; continue; }
            14 => { rax = 0x34; bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf4b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x70; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x10; const P4: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rdx; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = rcx; rax = ((r10 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { r9 = rbx; r11 = rbx; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(5u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = (((rcx as u8)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; fl_l = (((rcx) >> (((rdx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = ((r10.wrapping_add(P2)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; r9 = r11; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rdx = ((rbx.wrapping_add(P3)) as u64) as u64; rcx = ((((rcx as u32)) & (0xfffffff0u32))) as u32 as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P1), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; bb = 7; continue; }
            6 => { fl_l = ((rbx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rax = ((r10 as u32)) as u32 as u64; r9 = r11; bb = 7; continue; }
            7 => { r10 = ((rax as u32)) as u32 as u64; r11 = r9; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x1416111e0; const P2: u64 = 0x20; const P3: u64 = 0x38; const P4: u64 = 0xe8; const P5: u64 = 0x1416117b0; const P6: u64 = 0x8; const P7: u64 = 0x141611800; const P8: u64 = 0x2f8; const P9: u64 = 0x141610260; const P10: u64 = 0x90; const P11: u64 = 0x141611a90; const P12: u64 = 0x141611810; const P13: u64 = 0x1416117f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; rdx = ((rcx.wrapping_add(P0)) as u64) as u64; rbp = ((r8 as u32)) as u32 as u64; rcx = 0x503; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; rdx = ((rdi.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rcx as u32)) as u32); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(P8))) as u32) as u32 as u64; rcx = rbx; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(2u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P10))) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2f8u64))) as u32) as u32 as u64; rcx = rbx; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(1u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x80u64))) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0xf8u64), rax); bb = 3; continue; }
            3 => { rax = rbx; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf650(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x1415a3f40; const P2: u64 = 0x141611a20; const P3: u64 = 0x1416119b0; const P4: u64 = 0x141611820; const P5: u64 = 0x1415864b0; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415a4f40; const P8: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; r14 = rcx; rdx = 0x48; rcx = 0; fl_l = 0; fl_r = 0; rsi = r9; r15 = ((r8 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r14)) as u64; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r8 = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x84u64))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rsi)) as u64; r8 = (((r8 as u32)).wrapping_add((r15 as u32))) as u32 as u64; rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = ((r9.wrapping_add(0xfu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rsi); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1cu64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x24u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rax as u32)) as u32); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_add(0x1cu64), rdi); r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(P6)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = 0x34; bb = 11; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = 0x34; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf7c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0x10; const P2: u64 = 0x14159d900; const P3: u64 = 0x70; const P4: u64 = 0xd8; const P5: u64 = 0x14159d300; const P6: u64 = 0x14159d380; const P7: u64 = 0x28; const P8: u64 = 0x80; const P9: u64 = 0x110; const P10: u64 = 0x78;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rdx = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rcx = rbx; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(P0))) as u32) as u32 as u64; rsi = ((crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u32) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(P1), rbx); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rdi.wrapping_add(P7), (0x0u8) as u8); rcx = 0xfffe; rax = (rax & !0xffffu64) | ((((((rax as u16)) & ((rcx as u16)))) as u16 as u64)); rdx = rdi; crate::simd_ops::wu16(rdi.wrapping_add(P0), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rdi.wrapping_add(P7), ((rsi as u8)) as u8); rdx = rdi; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bf880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141620d10; const P1: u64 = 0x141620d50; const P2: u64 = 0x1415bf570; const P3: u64 = 0x1415c0b70; const P4: u64 = 0x1415a0690; const P5: u64 = 0x141610ef0; const P6: u64 = 0x141610fc0; const P7: u64 = 0x1438152ac; const P8: u64 = 0x1434269f8; const P9: u64 = 0x142677278; const P10: u64 = 0x1415bf650; const P11: u64 = 0x1415c2070;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); xmm0 = 0u128; rbp = (crate::simd_ops::ru64(r8.wrapping_add(0x88u64))) as u64; xmm2 = 0u128; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rdi = 0x34; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x10u64), r14); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r15); r14 = r8; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); xmm6 = (xmm1) as u128; crate::simd_ops::wu32(rax.wrapping_sub(0x48u64), ((rsi as u32)) as u32); r15 = rcx; crate::simd_ops::wu64(rax.wrapping_sub(0x40u64), rsi); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), rsi); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm0) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm2) as u128); fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 3; continue; }
            3 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = ((rax.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 18; continue; }
            5 => { rbp = rsp.wrapping_add(0x20); bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(r15.wrapping_add(0x20u64))) as u32 as u64; rdx = r14; rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x2f0u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0xc8u64), rcx); rcx = ((rcx).wrapping_shr(32u32)) as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(0xccu64), ((rcx as u8)) as u8); rcx = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0xd0u64), ((rcx as u32)) as u32); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rsi = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; bb = 10; continue; }
            9 => { rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdi = 0x2; rsi = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rdi as u32) } else { (rsi as u32) }) as u32 as u64; bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0xa8u64))) as u64; r9 = r14.wrapping_add(0x98); rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x88u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = ((rdx).wrapping_add(0x28u64)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(P7)) as u32 as u64; xmm2 = 0u128; xmm0 = 0u128; xmm1 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm6 as u32)) as f64).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm2 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); fl_l = ((xmm2 as u64)) as u64; fl_r = ((xmm1 as u64)) as u64; if !(f64::from_bits(fl_l) > f64::from_bits(fl_r)) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru64(P8)) as u64) as u128; bb = 15; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru64(P9)) as u64) as u128; bb = 15; continue; }
            15 => { xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm2 as u64)).to_bits() as u128); r9 = rbx; rdx = r14; rcx = rbp; r8 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 18; continue; }
            18 => { rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bfa60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe0; const P1: u64 = 0x8; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x14183acc8; const P4: u64 = 0x180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = r8; rax = ((r8.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = ((r9.wrapping_add(P2)) as u32) as u32 as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_sub(rcx)) as u64; rdx = ((rcx.wrapping_add(P1)) as u64) as u64; rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P1), (1) as u32); r8 = 0; fl_l = 0; fl_r = 0; r9 = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bfb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0x30; const P2: u64 = 0x1415a0610; const P3: u64 = 0x90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; rsi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415bfb60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143815044; const P1: u64 = 0x1415bfb60; const P2: u64 = 0x14381502c; const P3: u64 = 0x14268957c; const P4: u64 = 0x14159b8a0; const P5: u64 = 0x143930fa0; const P6: u64 = 0x14153ad70; const P7: u64 = 0x143426780; const P8: u64 = 0x143815034; const P9: u64 = 0x143815038; const P10: u64 = 0x143815030; const P11: u64 = 0x142689580; const P12: u64 = 0x14381503c; const P13: u64 = 0x143815040;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rdi = r9; crate::simd_ops::wu32(rbp.wrapping_add(0x57u64), ((rax as u32)) as u32); r15 = ((r8 as u32)) as u32 as u64; r12 = rdx; r13 = rcx; r11 = 0xfffffffc; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(r10))) as u8 as u64; fl_r = (0x35u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 3; continue; }
            3 => { r8 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = rbx; bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(r12.wrapping_add(0x1u64))) as u32) as u32 as u64; rax = rbp.wrapping_add(0x57); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), rsi); fl_l = rdx; fl_r = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), r14); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm8) as u128); rax = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = ((((rax as u32)) >> ((((rcx as u32)) as u32 & 31))) & 1) as u64; if ((fl_l as u32) == 0) { bb = 79; continue; } bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; fl_l = (crate::simd_ops::ru8(r9)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); r8 = ((r15 as u32)) as u32 as u64; rdx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 80; continue; } bb = 9; continue; }
            9 => { r11 = 0xfffffffc; bb = 10; continue; }
            10 => { crate::simd_ops::wu8(rdi, ((rbx as u8)) as u8); bb = 11; continue; }
            11 => { rcx = ((crate::simd_ops::ru8(r12)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(r12.wrapping_add(0x1u64))) as u32) as u32 as u64; rax = ((rcx.wrapping_add(0x80u64)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xcfu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(r12.wrapping_add(0x4u64))) as u32) as u32 as u64; bb = 16; continue; }
            14 => { rax = ((rdx.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (((rax) & (r11))) as u64; fl_l = ((rax).wrapping_add(r10)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            15 => { rax = (rax & !0xffu64) | (((0xffu8) as u8 as u64)); bb = 16; continue; }
            16 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x19u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(P2)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r14); crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x21u64), ((rdx as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x57u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x2fu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 19; continue; }
            19 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 18; continue; } bb = 20; continue; }
            20 => { rdx = rbx; bb = 21; continue; }
            21 => { rdi = (crate::simd_ops::ru32(P3)) as u32 as u64; rax = rbp.wrapping_add(0x57); fl_l = rdx; fl_r = rdx; rcx = r13.wrapping_add(0x28); xmm8 = 0u128; xmm6 = 0u128; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rdx = ((rdi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rdx = ((rdi as u32)) as u32 as u64; rcx = r13.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (((rax) >> (((rdi) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rdx = r13.wrapping_add(0x48); r9 = rbp.wrapping_sub(0x39); r8 = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 26; continue; }
            26 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; xmm7 = ((crate::simd_ops::ru32(P7)) as u32) as u128; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 29; continue; } bb = 27; continue; }
            27 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); bb = 30; continue; }
            28 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 21; continue; }
            29 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); bb = 30; continue; }
            30 => { r14 = ((crate::simd_ops::ru8(r12.wrapping_add(0x4u64))) as u32) as u32 as u64; rax = 0x7f; rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rsi as u32))) as u32 as u64; fl_l = (((r14 as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_r = fl_l; r14 = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { r14 = ((rbx as u32)) as u32 as u64; bb = 33; continue; }
            32 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r14 = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rax as u32) } else { (r14 as u32) }) as u32 as u64; bb = 33; continue; }
            33 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x90u64))) as u64; r10 = (crate::simd_ops::ru32(P8)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((r10 as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 34; continue; }
            34 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 35; continue; }
            35 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x31u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 57; continue; } bb = 36; continue; }
            36 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 35; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru32(P9)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 38; continue; }
            38 => { r11 = 0xfffffffc; bb = 39; continue; }
            39 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 40; continue; }
            40 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x32u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 58; continue; } bb = 41; continue; }
            41 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 40; continue; } bb = 42; continue; }
            42 => { rdx = rbx; r11 = 0xfffffffc; bb = 43; continue; }
            43 => { fl_l = rdx; fl_r = rdx; rax = rbp.wrapping_sub(0x59); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 79; continue; } bb = 44; continue; }
            44 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 79; continue; } bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(P10)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x57u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 46; continue; }
            46 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 47; continue; }
            47 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x30u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 59; continue; } bb = 48; continue; }
            48 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 47; continue; } bb = 49; continue; }
            49 => { rdx = rbx; bb = 50; continue; }
            50 => { rdi = (crate::simd_ops::ru32(P11)) as u32 as u64; rax = rbp.wrapping_add(0x57); fl_l = rdx; fl_r = rdx; rcx = r13.wrapping_add(0x28); xmm6 = (xmm8) as u128; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rdx = ((rdi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { rdx = ((rdi as u32)) as u32 as u64; rcx = r13.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (((rax) >> (((rdi) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 55; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rdx = r13.wrapping_add(0x48); r9 = rbp.wrapping_sub(0x39); r8 = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 55; continue; }
            55 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 60; continue; } bb = 56; continue; }
            56 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); bb = 61; continue; }
            57 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; r11 = 0xfffffffc; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(P9)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = rdx; rax = rbp.wrapping_sub(0x49); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; r10 = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 39; continue; }
            58 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 43; continue; }
            59 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 50; continue; }
            60 => { xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); bb = 61; continue; }
            61 => { rax = ((crate::simd_ops::ru8(r12.wrapping_add(0x5u64))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; r9 = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm6 as u32)))) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 63; continue; } bb = 62; continue; }
            62 => { r9 = 0x1; bb = 64; continue; }
            63 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x7f; rax = 0x7f; r9 = (if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { (rax as u32) } else { (r9 as u32) }) as u32 as u64; bb = 64; continue; }
            64 => { r8 = (crate::simd_ops::ru64(r13.wrapping_add(0x90u64))) as u64; r11 = (crate::simd_ops::ru32(P12)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x41u64), ((r11 as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 65; continue; }
            65 => { r10 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 66; continue; }
            66 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x33u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 74; continue; } bb = 67; continue; }
            67 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 66; continue; } bb = 68; continue; }
            68 => { rax = (crate::simd_ops::ru32(P13)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 76; continue; } bb = 69; continue; }
            69 => { rdi = 0xfffffffc; bb = 70; continue; }
            70 => { r10 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 71; continue; }
            71 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x34u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 75; continue; } bb = 72; continue; }
            72 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 71; continue; } bb = 73; continue; }
            73 => { bb = 76; continue; }
            74 => { rax = ((r10.wrapping_add(0x4u64)) as u32) as u32 as u64; rdi = 0xfffffffc; rax = (((rax) & (rdi))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(P13)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = rdx; rax = rbp.wrapping_sub(0x41); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; r11 = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 70; continue; }
            75 => { rax = ((r10.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (rdi))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rbx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 76; continue; }
            76 => { fl_l = rbx; fl_r = rbx; rcx = rbp.wrapping_sub(0x51); rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rbx } else { rcx }) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 79; continue; } bb = 77; continue; }
            77 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 79; continue; } bb = 78; continue; }
            78 => { crate::simd_ops::wu8(r12.wrapping_add(0x4u64), ((r14 as u8)) as u8); rax = 0x1; crate::simd_ops::wu8(r12.wrapping_add(0x5u64), ((r9 as u8)) as u8); bb = 80; continue; }
            79 => { rax = 0x52; bb = 80; continue; }
            80 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415c0040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415a3f30; const P1: u64 = 0x143930fa0; const P2: u64 = 0x14153ad70; const P3: u64 = 0x1426891e0; const P4: u64 = 0x143930f44; const P5: u64 = 0x1426891e8; const P6: u64 = 0x1426891e4; const P7: u64 = 0x1415bf4b0; const P8: u64 = 0x14159f700; const P9: u64 = 0x14159cfe0; const P10: u64 = 0x14268a040; const P11: u64 = 0x1439310e8; const P12: u64 = 0x1415c0c30; const P13: u64 = 0x1415d8ec0; const P14: u64 = 0x141622380;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xb7u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = rcx.wrapping_add(0x98); r12 = r9; rdi = r8; rsi = rdx; r14 = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = rax; fl_r = (0x1f000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r9)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu16(rsi.wrapping_add(0xb4u64), ((rax as u16)) as u16); rax = (rax & !0xffu64) | (((0x2u8) as u8 as u64)); bb = 5; continue; }
            5 => { crate::simd_ops::lk_and_u8(rsi.wrapping_add(0xb7u64), (0xfdu8) as u8); fl_l = (crate::simd_ops::lk_or_u8(rsi.wrapping_add(0xb7u64), ((rax as u8)) as u8)) as u8 as u64; fl_r = fl_l; bb = 6; continue; }
            6 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0xb6u64))) as u32) as u32 as u64; rdx = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = ((crate::simd_ops::ru8(r14.wrapping_add(0xe9u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x78u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xdfu8))) as u8 as u64)); rdx = (((rcx as u8)) as u32) as u32 as u64; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x20u8))) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) | ((rax as u8)))) as u8 as u64)); fl_l = ((((rdx as u8)) | (0x40u8))) as u8 as u64; fl_r = fl_l; rdx = (rdx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0xb6u64), ((rdx as u8)) as u8); bb = 10; continue; }
            10 => { rbp = 0; fl_l = 0; fl_r = 0; rcx = (((rdx as u8)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x94u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; r10 = 0xfffffffc; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 11; continue; }
            11 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((((fl_l as u8)) & ((fl_r as u8))) as i8) < 0) { bb = 24; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = (((rax) >> (20u32)) & 1) as u64; if ((fl_l as u64) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x78u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x94u64))) as u32 as u64; fl_l = (((rax) >> (1u32)) & 1) as u64; rax = ((rax) & !((1u64) << (1u32))) as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x94u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = (0x2000000000u64) as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = r14.wrapping_add(0x48); r9 = r12; r8 = 0x25; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            17 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), (0x41400000u32) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; bb = 19; continue; }
            19 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x21u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 30; continue; } bb = 20; continue; }
            20 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 19; continue; } bb = 21; continue; }
            21 => { rdx = rbp; bb = 22; continue; }
            22 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x98); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x98u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32) as u128; bb = 23; continue; }
            23 => { crate::simd_ops::wu32(rsi.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0xe9u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(6u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(rsi.wrapping_add(0xb7u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::lk_xor_u8(rsi.wrapping_add(0xb7u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::lk_or_u8(rsi.wrapping_add(0xb6u64), (0x80u8) as u8)) as u8 as u64; fl_r = fl_l; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0xb6u64))) as u32) as u32 as u64; bb = 24; continue; }
            24 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 34; continue; } bb = 25; continue; }
            25 => { rax = (0x7f83c00000u64) as u64; fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x78u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x3cu64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x4cu64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x44u64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x54u64), rbp); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x88u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x90u64), (0x780000u32) as u32); crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x94u64), (0xf0000000u32) as u32); fl_l = (crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x98u64), (0xfu32) as u32)) as u32 as u64; fl_r = fl_l; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x7cu64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x80u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x84u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x88u64), ((rcx as u32)) as u32); bb = 33; continue; }
            30 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r10))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 22; continue; }
            31 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(rsi.wrapping_add(0x7cu64), rbp); crate::simd_ops::wu64(rsi.wrapping_add(0x84u64), rbp); bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::lk_or_u8(rsi.wrapping_add(0xb6u64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0xb6u64))) as u32) as u32 as u64; bb = 34; continue; }
            34 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 38; continue; } bb = 35; continue; }
            35 => { rax = (0x18000200000u64) as u64; fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x78u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | (0x2u8))) as u8 as u64)); crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0xb6u64), ((rcx as u8)) as u8); rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ (crate::simd_ops::ru8(r14.wrapping_add(0xd5u64))))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0xb6u64), ((rax as u8)) as u8); crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x90u64), (0x800000u32) as u32); fl_l = (crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x98u64), (0x30u32) as u32)) as u32 as u64; fl_r = fl_l; bb = 38; continue; }
            38 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0xb6u64))) as u32) as u32 as u64; r13 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xc0u64))) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 54; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x80u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 54; continue; } bb = 40; continue; }
            40 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 53; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(P3)) as u32 as u64; rdx = rsi.wrapping_add(0x9c); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(4)), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 44; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x1000000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { rcx = rbp; fl_l = (((rcx) >> (24u32)) & 1) as u64; rcx = ((rcx) | ((1u64) << (24u32))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rcx as u32)) as u32); bb = 45; continue; }
            44 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(P5)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(4)), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 48; continue; } bb = 46; continue; }
            46 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x4000000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rax = ((rcx as u32)) as u32 as u64; fl_l = (((rax) >> (26u32)) & 1) as u64; rax = ((rax) | ((1u64) << (26u32))) as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(P6)) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(rax.wrapping_mul(4)), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru32(P4)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x2000000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = ((rcx as u32)) as u32 as u64; fl_l = (((rax) >> (25u32)) & 1) as u64; rax = ((rax) | ((1u64) << (25u32))) as u64; rcx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; rax = ((((rax as u32)) | ((r8 as u32)))) as u32 as u64; fl_l = ((((rax as u32)) | ((rcx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 53; continue; } bb = 52; continue; }
            52 => { rdx = rsp.wrapping_add(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            53 => { crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x90u64), (0x7000000u32) as u32); fl_l = (crate::simd_ops::lk_or_u8(rsi.wrapping_add(0xb6u64), (0x8u8) as u8)) as u8 as u64; fl_r = fl_l; rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0xb6u64))) as u32) as u32 as u64; bb = 54; continue; }
            54 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 58; continue; } bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = (((rax) >> (30u32)) & 1) as u64; if ((fl_l as u64) != 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x78u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | (0x10u8))) as u8 as u64)); crate::simd_ops::wu32(rsi.wrapping_add(0x64u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rsi.wrapping_add(0xb6u64), ((rcx as u8)) as u8); rax = (crate::simd_ops::ru32(r14.wrapping_add(0xa0u64))) as u32 as u64; fl_l = (crate::simd_ops::lk_or_u32(rsi.wrapping_add(0x98u64), (0x100u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu32(rsi.wrapping_add(0x8cu64), ((rax as u32)) as u32); bb = 58; continue; }
            58 => { r8 = r12; rdx = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = r12; r8 = rdi; rdx = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_and_u32(rsi.wrapping_add(0x90u64), (0xf807ffffu32) as u32); crate::simd_ops::lk_and_u32(rsi.wrapping_add(0x94u64), (0xfffffffu32) as u32); crate::simd_ops::lk_and_u32(rsi.wrapping_add(0x98u64), (0xfffffec0u32) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 104; continue; } bb = 59; continue; }
            59 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0xd8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 104; continue; } bb = 60; continue; }
            60 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; bb = 61; continue; }
            61 => { xmm4 = ((crate::simd_ops::ru64(P10)) as u64) as u128; rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; r10 = (0x5851f42d4c957f2du64) as u64; xmm2 = 0u128; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 69; continue; } bb = 62; continue; }
            62 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 61; continue; } bb = 63; continue; }
            63 => { r11 = 0xfffffffc; bb = 64; continue; }
            64 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0xd8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 77; continue; } bb = 65; continue; }
            65 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; bb = 66; continue; }
            66 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x6u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 73; continue; } bb = 67; continue; }
            67 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 66; continue; } bb = 68; continue; }
            68 => { bb = 77; continue; }
            69 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; r11 = 0xfffffffc; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 70; continue; }
            70 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 72; continue; } bb = 71; continue; }
            71 => { rax = (crate::simd_ops::ru64(P11)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm3); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P11, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 72; continue; }
            72 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi))).to_bits() as u128); crate::simd_ops::wu32(rdi, ((xmm0 as u32)) as u32); bb = 64; continue; }
            73 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 77; continue; } bb = 74; continue; }
            74 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 76; continue; } bb = 75; continue; }
            75 => { rax = (crate::simd_ops::ru64(P11)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm3); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P11, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 76; continue; }
            76 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((xmm0 as u32)) as u32); bb = 77; continue; }
            77 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0xd8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 86; continue; } bb = 78; continue; }
            78 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; bb = 79; continue; }
            79 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 82; continue; } bb = 80; continue; }
            80 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 79; continue; } bb = 81; continue; }
            81 => { bb = 86; continue; }
            82 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 86; continue; } bb = 83; continue; }
            83 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 85; continue; } bb = 84; continue; }
            84 => { rax = (crate::simd_ops::ru64(P11)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm3); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P11, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 85; continue; }
            85 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); bb = 86; continue; }
            86 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0xd8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 95; continue; } bb = 87; continue; }
            87 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; bb = 88; continue; }
            88 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 91; continue; } bb = 89; continue; }
            89 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 88; continue; } bb = 90; continue; }
            90 => { bb = 95; continue; }
            91 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 95; continue; } bb = 92; continue; }
            92 => { xmm3 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 94; continue; } bb = 93; continue; }
            93 => { rax = (crate::simd_ops::ru64(P11)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm3); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P11, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 94; continue; }
            94 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); bb = 95; continue; }
            95 => { r8 = (crate::simd_ops::ru64(r14.wrapping_add(0xd8u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 104; continue; } bb = 96; continue; }
            96 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbp as u32)) as u32 as u64; bb = 97; continue; }
            97 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 100; continue; } bb = 98; continue; }
            98 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 97; continue; } bb = 99; continue; }
            99 => { bb = 104; continue; }
            100 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r11))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 104; continue; } bb = 101; continue; }
            101 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm2 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 103; continue; } bb = 102; continue; }
            102 => { rax = (crate::simd_ops::ru64(P11)) as u64; rax = (((rax) as i64).wrapping_mul((r10) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm0); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P11, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm4 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm2 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 103; continue; }
            103 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x10u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((xmm2 as u32)) as u32); bb = 104; continue; }
            104 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64; rcx = 0x1; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 112; continue; } bb = 105; continue; }
            105 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 112; continue; } bb = 106; continue; }
            106 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x20u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 112; continue; } bb = 107; continue; }
            107 => { rbx = (crate::simd_ops::ru32(rbp.wrapping_add(0x148u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x14cu64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 109; continue; } bb = 108; continue; }
            108 => { rdx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 125; continue; } bb = 109; continue; }
            109 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x14cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 125; continue; } bb = 110; continue; }
            110 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x148u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x140u64))) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x148u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 125; continue; } bb = 111; continue; }
            111 => { crate::simd_ops::wu64(r8, rdx); rax = r14.wrapping_add(0x8); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdx); rcx = r14.wrapping_add(0x48); crate::simd_ops::wu32(r8.wrapping_add(0x10u64), ((rdx as u32)) as u32); fl_l = r14; fl_r = r14; crate::simd_ops::wu64(r8.wrapping_add(0x18u64), rdx); crate::simd_ops::wu64(r8.wrapping_add(0x20u64), rdx); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; rdx = r8; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(r8, r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 112; continue; }
            112 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 126; continue; } bb = 113; continue; }
            113 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x80u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 126; continue; } bb = 114; continue; }
            114 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x78u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 117; continue; } bb = 115; continue; }
            115 => { fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 117; continue; } bb = 116; continue; }
            116 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); r8 = r15; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x140u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 117; continue; }
            117 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); bb = 118; continue; }
            118 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r8 = r15; rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x140u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 119; continue; }
            119 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0xe0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 123; continue; } bb = 120; continue; }
            120 => { rbx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 123; continue; } bb = 121; continue; }
            121 => { bb = 122; continue; }
            122 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbp); r8 = rsi; rdx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 122; continue; } bb = 123; continue; }
            123 => { rax = 0x1; bb = 124; continue; }
            124 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            125 => { rax = 0x34; bb = 124; continue; }
            126 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x78u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 119; continue; } bb = 127; continue; }
            127 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 119; continue; } bb = 128; continue; }
            128 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r13 as u8)) as u8); bb = 118; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415c0930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160f380; const P1: u64 = 0x14160e370; const P2: u64 = 0x14159ed60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x98u64))) as u32 as u64; r14 = r9; rsi = r8; r15 = ((rdx as u32)) as u32 as u64; rbp = rcx; fl_l = rax; fl_r = (0x1f000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x1b0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); rcx = rbp.wrapping_add(0xb8); r8 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x40); rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsi, rcx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { r8 = r14; rdx = ((r15 as u32)) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = (((rax as u16)) as u32) as u32 as u64; bb = 13; continue; }
            9 => { rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rbx = 0xffffffff; crate::simd_ops::wu64(rsi, rdi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 14; continue; }
            14 => { crate::simd_ops::wu16(rsi.wrapping_add(0x8u64), ((rdi as u16)) as u16); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { let xa_src: u32 = (rbx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415c0a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0x78; const P2: u64 = 0x1b8; const P3: u64 = 0xb8; const P4: u64 = 0x14160f380; const P5: u64 = 0x14160e370; const P6: u64 = 0x48; const P7: u64 = 0x40; const P8: u64 = 0x14; const P9: u64 = 0x18;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = r9; rbx = ((r8 as u32)) as u32 as u64; r8 = rcx; fl_l = rax; fl_r = (0x1f000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = ((rbx as u32)) as u32 as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            3 => { rcx = ((r8.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu32(r9, (0xffffffffu32) as u32); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdx = rsp.wrapping_add(0x40); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P6))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(rbx.wrapping_mul(4)))) as u32 as u64; bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rdi, ((r8 as u32)) as u32); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(P8), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415c0b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0b70; const P1: u64 = 0x1; const P2: u64 = 0x1415a0690;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = ((rbx.wrapping_add(P1)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    rcx = 0x2;
    rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rbx as u32) }) as u32 as u64;
    rax = ((rbx as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415c0b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c0b70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    rax = ((rcx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415c0b70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x78; const P1: u64 = 0xd5; const P2: u64 = 0x90; const P3: u64 = 0x143815028; const P4: u64 = 0x1; const P5: u64 = 0x4;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r10 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(P3)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rdx = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x2eu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { bb = 6; continue; }
            9 => { rax = ((r9.wrapping_add(P5)) as u32) as u32 as u64; rdx = 0xfffffffc; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r10 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 10; continue; }
            10 => { fl_l = r10; fl_r = r10; rax = rsp.wrapping_add(0x8); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r10 } else { rax }) as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415c0bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xd4; const P1: u64 = 0x78; const P2: u64 = 0xe8; const P3: u64 = 0xe9;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P0))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rcx = rax; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415c0c30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524320; const P1: u64 = 0x1415bf1b0; const P2: u64 = 0x1415bf250; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r12 = (crate::simd_ops::ru32(rcx.wrapping_add(0x14cu64))) as u32 as u64; rsi = rcx; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x148u64))) as u32 as u64; r12 = (((r12 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r12.wrapping_add(r12.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rdx; fl_r = (0x140u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbp = rcx; bb = 3; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; r8 = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = rax; bb = 3; continue; }
            3 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = rbp.wrapping_add(0x24); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = (0xffffffffffffffdcu64) as u64; r15 = ((r15).wrapping_sub(rbp)) as u64; r13 = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rcx = rbx.wrapping_sub(0x24); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rcx, r13); crate::simd_ops::wu64(rbx.wrapping_sub(0x1cu64), r13); crate::simd_ops::wu32(rbx.wrapping_sub(0x14u64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_sub(0xcu64), r13); crate::simd_ops::wu64(rbx.wrapping_sub(0x4u64), r13); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; rdi = ((r15.wrapping_add(rbx)) as u64) as u64; rdx = ((rdx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = ((rcx).wrapping_add(rdi)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x140u64))) as u64; fl_l = rsi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsi.wrapping_add(0x140u64), rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rsi.wrapping_add(0x14cu64), ((r12 as u32)) as u32); rsp = ((rsp).wrapping_add(0x28u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415c0d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14381505c; const P1: u64 = 0x1426895ac; const P2: u64 = 0x14159b8a0; const P3: u64 = 0x143930fa0; const P4: u64 = 0x14153ad70; const P5: u64 = 0x1439310e8; const P6: u64 = 0x14268a040; const P7: u64 = 0x1415c1600; const P8: u64 = 0x1415bf880;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x2f8u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rdi = rdx; rbp = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x2f4u64))) as u32) as u32 as u64; rsi = 0x1; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((((rax as u8)) & (0xfeu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdx.wrapping_add(0x2f4u64), ((rax as u8)) as u8); bb = 28; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), r12); r12 = 0xfffffffc; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rax as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm8) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = (0x3bu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 7; continue; }
            7 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdx = rbx; bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = rsp.wrapping_add(0xb8); fl_l = rdx; fl_r = rdx; rcx = rbp.wrapping_add(0x28); xmm8 = 0u128; xmm7 = 0u128; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rdx = ((r14 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xb8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (xmm0) as u128; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = (((rax) >> (((r14) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rbp.wrapping_add(0x48); r9 = rsp.wrapping_add(0x20); r8 = ((r14 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 15; continue; }
            15 => { r8 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 16; continue; }
            16 => { rcx = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x3bu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rbx = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 24; continue; }
            19 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = (((rax) & (r12))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 9; continue; }
            20 => { rax = ((r8.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rax = (((rax) & (r12))) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm8) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(P5)) as u64; rcx = (0x5851f42d4c957f2du64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm2); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P5, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P6))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm1); bb = 23; continue; }
            23 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdx))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 24; continue; }
            24 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 28; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x90u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm6 as u32)) as f64).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x84u64), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 28; continue; }
            27 => { r8 = rdi; xmm1 = (xmm6) as u128; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = ((rsi as u32)) as u32 as u64; bb = 29; continue; }
            29 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; rsp = ((rsp).wrapping_add(0x90u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
