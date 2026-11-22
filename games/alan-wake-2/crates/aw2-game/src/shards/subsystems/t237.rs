#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141644a90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415d78a0; const P1: u64 = 0x141593150; const P2: u64 = 0x1415d55a0; const P3: u64 = 0x1415d7950; const P4: u64 = 0x141641ee0; const P5: u64 = 0x141651fc0; const P6: u64 = 0x141651f00; const P7: u64 = 0x14183acc2; const P8: u64 = 0x141591fa0; const P9: u64 = 0x1415d7b20; const P10: u64 = 0x1415d44d0; const P11: u64 = 0x14183acbc; const P12: u64 = 0x143427a60; const P13: u64 = 0x1415d4800; const P14: u64 = 0x1415d63c0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xd8u64)) as u64; rbp = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru32(r8.wrapping_add(0x1a0u64))) as u32 as u64; r14 = r8; crate::simd_ops::wu128(rbp.wrapping_add(0x70u64), (xmm6) as u128); rdi = rcx; r12 = (0xffffffffffffff0u64) as u64; rax = ((rbx.wrapping_mul(8)) as u64) as u64; r10 = rax.wrapping_add(0xf); fl_l = r10; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = r12; bb = 2; continue; }
            2 => { r10 = (((r10) & (0xfffffffffffffff0u64))) as u64; rax = r10; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = ((rbx.wrapping_mul(4)) as u64) as u64; rcx = rax.wrapping_add(0xf); r13 = rsp.wrapping_add(0x50); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = r12; bb = 4; continue; }
            4 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; rdx = r8.wrapping_add(0x198); r8 = r13; rcx = rdi; rbx = rsp.wrapping_add(0x50); r9 = rbx; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x184u64))) as u32) as u128; r9 = r13; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x180u64))) as u32) as u128; rcx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x170u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x148u64))) as u32 as u64; r9 = rax; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; rsi = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rdx); rax = rcx.wrapping_add(0xf); crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rcx); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = r12; bb = 9; continue; }
            9 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; r12 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r12); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 10; continue; }
            10 => { rdi = rdx.wrapping_add(0x60); crate::simd_ops::wu64(rbp, rcx); bb = 11; continue; }
            11 => { crate::simd_ops::wu8(r12, (0x0u8) as u8); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xacu64))) as u32 as u64; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x60u64))) as u32) as u32 as u64; r14 = (((rdx as u8)) as u32) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 12; continue; }
            12 => { rcx = rdi.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; bb = 15; continue; }
            15 => { rbx = (((rcx as u8)) as u32) as u32 as u64; rsi = rdi.wrapping_add(0x88); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rbx as u32)) as u32 as u64; rdx = ((r14 as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 16; continue; }
            16 => { rax = ((rbx.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shl(2u32)) as u64; r8 = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; r8 = rbx; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbp)) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu8(r12, (0x1u8) as u8); bb = 20; continue; }
            19 => { rsi = rdi.wrapping_add(0x88); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rcx); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (0x0u64) as u64); fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_sub(0x50u64))) as u32 as u64; bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(rax.wrapping_mul(8)))) as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x68u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 24; continue; }
            24 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 23; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; bb = 27; continue; }
            27 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x60u64))) as u32) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x7u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { r8 = ((rcx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { rcx = (crate::simd_ops::ru64(rbp)) as u64; bb = 30; continue; }
            30 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rdi = ((rdi).wrapping_add(0x118u64)) as u64; r12 = ((r12).wrapping_add(1)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(rbp, rcx); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 31; continue; }
            31 => { r12 = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0xe0u64))) as u64; bb = 32; continue; }
            32 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 35; continue; } bb = 34; continue; }
            34 => { r9 = rdi.wrapping_add(0x28); r8 = rbx; rdx = r13; rcx = ((r15 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xe8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 36; continue; }
            36 => { rdx = rdi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 46; continue; } bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; fl_l = ((rbx).wrapping_add(0xc0u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru8(r12)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 45; continue; } bb = 40; continue; }
            40 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x48u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 45; continue; } bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_sub(0x8u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 43; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 43; continue; }
            43 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x7u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 45; continue; }
            45 => { r12 = ((r12).wrapping_add(1)) as u64; rbx = ((rbx).wrapping_add(0x118u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 46; continue; }
            46 => { xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x70u64))) as u128; rsp = rbp.wrapping_add(0x88); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            47 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rcx); xmm6 = ((crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32) as u128; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 48; continue; }
            48 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 49; continue; }
            49 => { rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x4u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_sub(0x8u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_sub(0x4u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi))).to_bits() as u128); crate::simd_ops::wu32(rdi, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x19u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xdu64))) as u32) as u32 as u64; bb = 51; continue; }
            51 => { xmm0 = (crate::simd_ops::ru128(P12)) as u128; rcx = rdi.wrapping_sub(0x38); crate::simd_ops::wu8(rdi.wrapping_add(0xdu64), ((rax as u8)) as u8); r8 = rbp.wrapping_add(0x8); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x1au64))) as u32) as u32 as u64; rdx = 0x1; crate::simd_ops::lk_and_u8(rdi.wrapping_add(0xeu64), ((rax as u8)) as u8); rax = rbp.wrapping_add(0x40); rbx = ((crate::simd_ops::ru8(rdi.wrapping_add(0xcu64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); r9 = (((rbx as u8)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x64u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x6cu64), (0xffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), (0x0u32) as u32); crate::simd_ops::wu128(rbp.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (0x3f800000u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 53; continue; } bb = 52; continue; }
            52 => { crate::simd_ops::wu8(rdi.wrapping_add(0xcu64), ((rax as u8)) as u8); bb = 53; continue; }
            53 => { r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; r9 = rdi.wrapping_sub(0x60); rdx = rbp.wrapping_add(0x40); xmm0 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141644f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415d78a0; const P1: u64 = 0x141593150; const P2: u64 = 0x1415d55a0; const P3: u64 = 0x1415d7950; const P4: u64 = 0x141641ee0; const P5: u64 = 0x141651fc0; const P6: u64 = 0x141651f00; const P7: u64 = 0x14183acc2; const P8: u64 = 0x1415d4800; const P9: u64 = 0x1415d6570; const P10: u64 = 0x1415d44d0; const P11: u64 = 0x14183acbc;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rbp = rsp.wrapping_add(0x50); r13 = rcx; rbx = r8; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x1a0u64))) as u32 as u64; r12 = (0xffffffffffffff0u64) as u64; rax = ((rcx.wrapping_mul(8)) as u64) as u64; r10 = rax.wrapping_add(0xf); fl_l = r10; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = r12; bb = 2; continue; }
            2 => { r10 = (((r10) & (0xfffffffffffffff0u64))) as u64; rax = r10; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = ((rcx.wrapping_mul(4)) as u64) as u64; rcx = rax.wrapping_add(0xf); rsi = rsp.wrapping_add(0x50); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = r12; bb = 4; continue; }
            4 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; rdx = r8.wrapping_add(0x198); r8 = rsi; rcx = r13; r14 = rsp.wrapping_add(0x50); r9 = r14; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x60u64), ((rdi as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm2 = ((crate::simd_ops::ru32(r13.wrapping_add(0x184u64))) as u32) as u128; r9 = rsi; xmm1 = ((crate::simd_ops::ru32(r13.wrapping_add(0x180u64))) as u32) as u128; rcx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x170u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; bb = 8; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x170u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 40; continue; } bb = 8; continue; }
            8 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 9; continue; }
            9 => { rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = rcx.wrapping_add(0xf); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = r12; bb = 11; continue; }
            11 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x148u64))) as u32 as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0x140u64))) as u64; rdx = (((rcx) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; r8 = rsp.wrapping_add(0x50); rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), r8); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 12; continue; }
            12 => { rdi = rax.wrapping_add(0x60); bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xacu64))) as u32 as u64; r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x60u64))) as u32) as u32 as u64; rbx = (((rdx as u8)) as u32) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 14; continue; }
            14 => { rcx = rdi.wrapping_add(0x60); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; bb = 17; continue; }
            17 => { r12 = (((rcx as u8)) as u32) as u32 as u64; r14 = rdi.wrapping_add(0x88); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((r12 as u32)) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 18; continue; }
            18 => { rax = ((r12.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shl(2u32)) as u64; r8 = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; r8 = rbx; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; r12 = (r12 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 22; continue; }
            21 => { r14 = rdi.wrapping_add(0x88); bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru64(r14)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rcx); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(r14.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_sub(0x8u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_sub(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_sub(0x4u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi))).to_bits() as u128); crate::simd_ops::wu32(rdi, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0x14u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x19u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xdu64))) as u32) as u32 as u64; bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; rcx = rdi.wrapping_sub(0x38); crate::simd_ops::wu8(rdi.wrapping_add(0xdu64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x1au64))) as u32) as u32 as u64; crate::simd_ops::lk_and_u8(rdi.wrapping_add(0xeu64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rbx = ((crate::simd_ops::ru8(rdi.wrapping_add(0xcu64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); r9 = (((rbx as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); r8 = rsi; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(r14.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu8(rdi.wrapping_add(0xcu64), ((rax as u8)) as u8); bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 31; continue; }
            30 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; rcx = rdi.wrapping_sub(0x60); r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rcx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rdx as u8)) as u8); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rdx = r13; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 39; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xa8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 39; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x58u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x60u64))) as u32 as u64; bb = 37; continue; }
            37 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x60u64))) as u32) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x7u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64))) as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(r13.wrapping_add(0x148u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x118u64)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rcx = (((rax) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rax = rdi.wrapping_sub(0x60); rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x140u64)))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 40; continue; }
            40 => { rsp = rbp.wrapping_add(0x18); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141645320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162ee70; const P1: u64 = 0x143931be9; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rdi = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r15 = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x150u64))) as u64; rbp = r14; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 4; continue; }
            4 => { rbp = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 25; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), r14); bb = 26; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x150u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rax.wrapping_add(0x164u64), (0x42ca0000u32) as u32); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rcx = rbx; rax = ((crate::simd_ops::ru8(P1)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r15 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu8(P1, ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = r14; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = r10; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 14; continue; }
            14 => { bb = 20; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r8); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), r8); bb = 18; continue; }
            18 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdx); bb = 20; continue; }
            20 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rax.wrapping_add(0x28u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rax); bb = 23; continue; }
            22 => { crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); bb = 23; continue; }
            23 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x5; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            25 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rax); bb = 26; continue; }
            26 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 27; continue; }
            27 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((r14 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            28 => { crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((r14 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416454a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x194; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416454d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x198; const P2: u64 = 0x158; const P3: u64 = 0x1c0; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = 0; fl_l = 0; fl_r = 0; r9 = ((rdx as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(2u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (((r8 as u32)).wrapping_add(1)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = ((r8 as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141645510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434264e4; const P1: u64 = 0x141591390; const P2: u64 = 0x14162ee90; const P3: u64 = 0x141644f50; const P4: u64 = 0x141644a90; const P5: u64 = 0x141644820; const P6: u64 = 0x141644400; const P7: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x10u64)))).to_bits() as u128); xmm0 = 0u128; r12 = (((r9 as u8)) as u32) as u32 as u64; rsi = rdx; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x150u64))) as u32) as u128; r15 = rcx; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x164u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x40u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x154u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x168u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x158u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x16cu64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x15cu64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0x170u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x160u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru8(rdx.wrapping_add(0x174u64))) as u32) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((rdx as u8)))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rdx as u8)))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_sub(0x24u64), (0x0u8) as u8); fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbp.wrapping_sub(0x24u64), (0x1u8) as u8); bb = 7; continue; }
            7 => { xmm2 = ((crate::simd_ops::ru32(P0)) as u32) as u128; rcx = rsi.wrapping_add(0x178); xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x15cu64))) as u32) as u128; rdx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x160u64))) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), (0x0u8) as u8); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x27u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x3cu64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x154u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0x26u64), (0x1u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), ((xmm1 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x158u64))) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x34u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x2cu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((xmm1 as u32)) as u32); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rdx = ((crate::simd_ops::ru8(rsi.wrapping_add(0x18cu64))) as u32) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(3u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), ((rax as u8)) as u8); fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x20bu64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rax.wrapping_add(0x1fdu64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(1u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_sub(0x26u64), ((rcx as u8)) as u8); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rax.wrapping_add(0x2); rax = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = (0x0u64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; rax = (if (((fl_l as u64)) != ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x1feu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(0x2u64)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_r = (0x0u64) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rax = (if (((fl_l as u64)) != ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); r13 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x198u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x198u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r13 as u32)) as u32); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 41; continue; } bb = 18; continue; }
            18 => { bb = 20; continue; }
            19 => { xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_sub(0x40u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x188u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x160u64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x194u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 22; continue; }
            22 => { rdx = rbp.wrapping_add(0x40); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = r14; rdx = rbp.wrapping_sub(0x40); r8 = rsi; rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rcx = ((((rcx as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x300; rcx = rbx; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            24 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            25 => { rdx = rbp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0xf00; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { r8 = r14; rdx = rbp.wrapping_sub(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            27 => { rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x198u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r13 as u32)) as u32); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x148u64))) as u32 as u64; rdx = (((rcx) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 41; continue; } bb = 30; continue; }
            30 => { rdi = rax.wrapping_add(0xc0); bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 36; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_sub(0x8u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; bb = 34; continue; }
            34 => { rax = ((crate::simd_ops::ru8(rdi)) as u32) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x7u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = ((((rcx as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; r8 = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x148u64))) as u32 as u64; rdi = ((rdi).wrapping_add(0x118u64)) as u64; rcx = (((rax) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; rax = rdi.wrapping_sub(0xc0); rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x140u64)))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 37; continue; }
            37 => { bb = 41; continue; }
            38 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 41; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r13 as u32)) as u32); bb = 41; continue; }
            41 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 42; continue; }
            42 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 43; continue; }
            43 => { xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x150u64))) as u128; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x160u64))) as u32 as u64; crate::simd_ops::wu128(rsi.wrapping_add(0x164u64), (xmm0) as u128); crate::simd_ops::wu32(rsi.wrapping_add(0x174u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141645880(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x10; const P5: u64 = 0xfffffffffffffff0; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0x5; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax.wrapping_add(P4)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rcx).wrapping_add(P5))) as u128; crate::simd_ops::wu128(rax.wrapping_add(rbx).wrapping_add(P5), (xmm0) as u128); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141645b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x140; const P2: u64 = 0x148; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r9 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; r8 = (((rdx) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rax.wrapping_add(P3), r9); rax = ((rax).wrapping_add(0x118u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; r8 = (((rdx) as i64).wrapping_mul((0x118u64) as i64) as u64) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141645bf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931be9; const P1: u64 = 0x14162ee70; const P2: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdi = r8; rbp = rdx; rsi = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rax.wrapping_add(0x164u64), (0x42ca0000u32) as u32); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rcx = 0x1; rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu8(P0, ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(r8)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0; fl_l = 0; fl_r = 0; r10 = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = r10; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 11; continue; }
            6 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r9); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), r9); bb = 9; continue; }
            9 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdx); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u32(rax.wrapping_add(0x28u64), (1) as u32); rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rax); bb = 14; continue; }
            13 => { crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rbp; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu128(rbp, (xmm0) as u128); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141645cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141645880; const P1: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; rsi = rdx; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(r11))) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = rdx; fl_r = rsi; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(r8, (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = ((((r10 as u32)) as i32 as i64) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; fl_l = ((rcx).wrapping_add(rdi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = ((rdi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r14)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax).wrapping_add(r14)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 16; continue; }
            11 => { crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rax, rsi); bb = 19; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141645e30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416229d0; const P1: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; rsi = rdx; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(r11))) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = rdx; fl_r = rsi; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(r8, (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = ((((r10 as u32)) as i32 as i64) as u64) as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; fl_l = ((rcx).wrapping_add(rdi)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = ((rdi as u32)) as u32 as u64; r14 = ((r14).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r14)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = ((rax).wrapping_add(r14)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 16; continue; }
            11 => { crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = ((rax as u32)) as u32 as u64; rcx = 0x1; rdx = (((rdx as u32)).wrapping_shr(1u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 16; continue; }
            16 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(rax, rsi); bb = 19; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141645f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x198; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rcx = (((rdx as u8)) as u32) as u32 as u64;
    rdx = ((rcx.wrapping_add(rcx)) as u32) as u32 as u64;
    loop { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rcx = ((((rcx as u32)) & (0xfffffffdu32))) as u32 as u64; rcx = ((((rcx as u32)) | ((rdx as u32)))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P1), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426564; const P1: u64 = 0x142673708; const P2: u64 = 0x1426736fc; const P3: u64 = 0x142673700; const P4: u64 = 0x1426736ec; const P5: u64 = 0x1426736e8; const P6: u64 = 0x1426736f0; const P7: u64 = 0x143815b3c;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; r10 = rcx; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { xmm4 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P4)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu128(rsp, (xmm10) as u128); xmm10 = ((crate::simd_ops::ru32(P6)) as u32) as u128; bb = 2; continue; }
            2 => { r9 = (crate::simd_ops::ru64(rdx)) as u64; xmm3 = ((crate::simd_ops::ru32(r9.wrapping_add(0x14u64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(r9.wrapping_add(0x1d0u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) }).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm4 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm2 = 0u128; bb = 5; continue; }
            4 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm2 = (xmm0) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x68u64)))).to_bits() as u128); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x160u64))) as u64; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x198u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; xmm0 = (xmm2) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x188u64)))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(0x18cu64)))).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0x188u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P7)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x18cu64), ((xmm1 as u32)) as u32); r8 = (r8 & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rcx = ((((rcx as u32)) & (0xfffffffeu32))) as u32 as u64; fl_l = ((((rcx as u32)) | ((r8 as u32)))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x198u64), ((rcx as u32)) as u32); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(r10.wrapping_add(0x58u64))) as u32 as u64; rdx = ((rdx).wrapping_add(0x10u64)) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(0x50u64)))) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 10; continue; }
            10 => { xmm10 = (crate::simd_ops::ru128(rsp)) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 11; continue; }
            11 => { rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x142691598; const P4: u64 = 0x20; const P5: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r9);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1426914f0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rdx as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x143930f00; const P3: u64 = 0x1415bb180;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { r8 = rcx; rcx = (crate::simd_ops::ru64(P2)) as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xfffffffffffffff0; const P3: u64 = 0x143930f00; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0x1415bb180; const P6: u64 = 0x1415242e0; const P7: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r14 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = ((rbx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rsi = ((rbx.wrapping_add(P1)) as u64) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P3)) as u64; r8 = ((rsi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x28u64)) as u64; rsi = ((rsi).wrapping_add(0x28u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rbx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r14.wrapping_add(P0), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(r14, rbx); crate::simd_ops::wu32(r14.wrapping_add(P7), ((rbx as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691448; const P1: u64 = 0x141646970; const P2: u64 = 0x14268dac0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = P2;
    crate::simd_ops::wu64(rbx, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691448; const P1: u64 = 0x141646970; const P2: u64 = 0x14268dac0; const P3: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = P2;
    crate::simd_ops::wu64(rdi, rax);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x30; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691448; const P1: u64 = 0x141646970; const P2: u64 = 0x14268dac0; const P3: u64 = 0x141834d30;
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
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = P2;
    crate::simd_ops::wu64(rdi, rax);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x28; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f00; const P1: u64 = 0x1415b7320; const P2: u64 = 0x141647100; const P3: u64 = 0x1415bb180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r15 = ((r8 as u32)) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rsi = rcx; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; xmm0 = 0u128; rdi = 0; fl_l = 0; fl_r = 0; rdx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r14); r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((r14.wrapping_add(r14.wrapping_mul(4))) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rcx; rbx = rax.wrapping_add(0x8); rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rdi } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdi = rbx; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = rsp.wrapping_add(0x20); rdx = ((r15 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 11; continue; }
            11 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rdi); crate::simd_ops::wu64(r8.wrapping_add(0x10u64), rdi); crate::simd_ops::wu32(r8.wrapping_add(0x18u64), ((rdi as u32)) as u32); bb = 15; continue; }
            15 => { rbx = rdi; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(r8, ((rbp as u32)) as u32); rbx = r8.wrapping_add(0x8); bb = 17; continue; }
            17 => { rdi = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rcx); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rcx); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((r15 as u32)) as u32); bb = 10; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646c50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    r9 = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x1416380f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = r8;
    r8 = ((rdx as u32)) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x143930f00; const P3: u64 = 0x1415b7320; const P4: u64 = 0x8; const P5: u64 = 0x28; const P6: u64 = 0x1415bb180;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rbx = rcx; rcx = rdx; rdi = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; r8 = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P2)) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { r8 = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbx, rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { r8 = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646d50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x14; const P2: u64 = 0x143426564; const P3: u64 = 0x142673708; const P4: u64 = 0x1426736fc; const P5: u64 = 0x142673700; const P6: u64 = 0x1426736ec; const P7: u64 = 0x1426736e8; const P8: u64 = 0x1426736f0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32) as u128;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64;
    if !((f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { xmm0 = 0u128; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128);
    rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64;
    fl_l = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32);
    xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x8u64))) as u32) as u128;
    xmm0 = (xmm1) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rax as u32)) as u32);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x10u64)))).to_bits() as u128);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x141638b30;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = rdx;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rdx, (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1416454a0; const P2: u64 = 0x14162ee90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    rbx = rdx;
    crate::simd_ops::lk_and_u32(rdx, (0xffu32) as u32);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = rsp.wrapping_add(0x30); rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rax)) as u32 as u64; rax = 0x1; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0x2;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646ea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0x141638cd0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = r8;
    r8 = rdx;
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x28; const P4: u64 = 0x1416340d0; const P5: u64 = 0x141646b20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r14 = ((rcx.wrapping_add(P1)) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rdi = r9; rsi = r8; rbx = ((rdx as u32)) as u32 as u64; r11 = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; r10 = ((rax.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rbp = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = r10; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; r9 = rsp.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0xffffffffu32) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rbx as u32)) as u32 as u64; rcx = r14; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsi, rax); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rsi, rbp); crate::simd_ops::wu32(rdi, ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141646f70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x20; const P4: u64 = 0xffffffffffffffe8; const P5: u64 = 0x1415945a0; const P6: u64 = 0x141646b20;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = r9; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = r8; rbx = ((rdx as u32)) as u32 as u64; rbp = rcx; r11 = ((r10.wrapping_add(r10.wrapping_mul(4))) as u64) as u64; r10 = ((rax.wrapping_add(r11.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r15 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = r10; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; r9 = rsp.wrapping_add(0x50); rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rax; fl_r = rax; rcx = ((rax.wrapping_add(P4)) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rcx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((rbx as u32)) as u32 as u64; rcx = rbp.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rdi, ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(rsi, rax); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rsi, r15); crate::simd_ops::wu32(rdi, ((r15 as u32)) as u32); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x1416454a0; const P2: u64 = 0x158; const P3: u64 = 0x1415e9250; const P4: u64 = 0x60; const P5: u64 = 0x90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu8(rdx, (0x0u8) as u8);
    rdi = r8;
    crate::simd_ops::lk_and_u32(rdx, (0xffu32) as u32);
    rbx = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = ((rcx).wrapping_add(0x70u64)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; rax = 0x1; crate::simd_ops::wu128(rdi, (xmm0) as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x2;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x143930f00; const P3: u64 = 0x1415bb180; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; r15 = ((rdx as u32)) as u32 as u64; rcx = 0x1; r15 = (((r15 as u32)).wrapping_shr(1u32)) as u32 as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; r15 = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (r15 as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; r15 = (((r15 as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((r15.wrapping_add(r15.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = rax.wrapping_add(0x18); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r14 = (0xffffffffffffffe8u64) as u64; r14 = ((r14).wrapping_sub(rax)) as u64; r12 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rdx = rbx.wrapping_sub(0x18); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rbx.wrapping_sub(0x10u64), r12); crate::simd_ops::wu64(rbx.wrapping_sub(0x8u64), r12); crate::simd_ops::wu32(rbx, ((r12 as u32)) as u32); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((r14.wrapping_add(rbx)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax))) as u128; crate::simd_ops::wu128(rdx, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rcx.wrapping_add(rax).wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), ((xmm0 as u64)) as u64); rdx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rdx.wrapping_add(0x8); r8 = ((r8).wrapping_add(rcx)) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(r8.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi)) as u64; bb = 13; continue; }
            13 => { rbx = ((rbx).wrapping_add(0x28u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 15; continue; }
            15 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi, rbp); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((r15 as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x141646c90;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = (((r9 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r14 = r8; rbp = rdx; rdi = rcx; r10 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rbx.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rdx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x28u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x141646c90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbp = r8; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = rdx; rdi = rcx; r9 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rbx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = rsi; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x28u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 5; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14067b660;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8);
    rcx = rdx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rdx = ((rax as u32)) as u32 as u64;
    r9 = (0xffffffffffffffffu64) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416473d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu64(rbx, rax); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x4;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r10 = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; r10 = ((((r10 as u32)) as i32).wrapping_mul(((rbx as u32)) as i32) as u32) as u32 as u64; r10 = (((r10 as u32)).wrapping_add(0x1fu32)) as u32 as u64; r10 = (((r10 as u32)).wrapping_shr(5u32)) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P3), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P4))) as u64; rdx = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rdx as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); bb = 2; continue; }
            5 => { crate::simd_ops::wu32(r8, ((rdi as u32)) as u32); rax = r8; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rbx as u32)) as u32); crate::simd_ops::wu32(r8.wrapping_add(P0), (0x0u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = 0x1; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; let bit_in: u32 = (rdx as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), (0x0u32) as u32); rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rax = (!((rax as u32))) as u32 as u64; rax = ((((rax as u32)) & ((rdx as u32)))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; let d = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } crate::simd_ops::wu32(r8, ((rax as u32)) as u32); rax = r8; crate::simd_ops::wu32(r8.wrapping_add(P5), ((rdx as u32)) as u32); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416474c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4; const P2: u64 = 0x1; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x14;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; r11 = (crate::simd_ops::ru32(rdx)) as u32 as u64; r8 = rcx; rdi = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu64(rcx, rsi); rdx = (crate::simd_ops::ru32(r10)) as u32 as u64; rbx = ((rsi.wrapping_add(P2)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r11 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rdi as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rsi as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P5), r10); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r9 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rbx as u32)) as u32); r9 = ((((r9 as u32)) as i32).wrapping_mul(((r11 as u32)) as i32) as u32) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; r9 = (((r9 as u32)).wrapping_add(0x1fu32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(5u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(r10.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rdx as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32); bb = 2; continue; }
            5 => { crate::simd_ops::wu32(r8.wrapping_add(P6), ((rsi as u32)) as u32); bb = 8; continue; }
            6 => { rax = ((rsi as u32)) as u32 as u64; bb = 7; continue; }
            7 => { let bit_in: u32 = (rdx as u32); if bit_in != 0 { rcx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; rbx = (((rbx as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rax = (((rax as u32)).wrapping_add((rcx as u32))) as u32 as u64; rbx = (!((rbx as u32))) as u32 as u64; rbx = ((((rbx as u32)) & ((rdx as u32)))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = ((rdi as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } crate::simd_ops::wu32(r8.wrapping_add(P6), ((rbx as u32)) as u32); rdi = ((rdx as u32)) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = r8; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu32(r8.wrapping_add(P1), ((rdi as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu32(r8, ((r11 as u32)) as u32); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0xc; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x14; const P5: u64 = 0x10;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    r8 = ((((r8 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = rcx;
    r8 = (((r8 as u32)).wrapping_add(0x1fu32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), (0x0u32) as u32);
    fl_l = (((r8 as u32)).wrapping_shr(5u32)) as u32 as u64;
    r8 = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P5), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416475b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x141524330; const P2: u64 = 0x8; const P3: u64 = 0x14183acc2;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32); rdx = ((((rdx as u32)) as i32).wrapping_mul(((r8 as u32)) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((r8 as u32)) as u32); rcx = 0x5; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdx = (((rdx as u32)).wrapping_add(0x1fu32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(5u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(2u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P2), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x34; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { r8 = rdi; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = 0x1f; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141524340; const P2: u64 = 0x5; const P3: u64 = 0x1415242e0; const P4: u64 = 0x14183acc2; const P5: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((rdx as u32)) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r8 as u32)) as i32) as u32) as u32 as u64; rbx = rcx; rcx = 0x5; rax = (((rax as u32)).wrapping_add(0x1fu32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(5u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(2u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx, rdi); rax = 0x34; bb = 5; continue; }
            4 => { r8 = r14; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; crate::simd_ops::wu32(rbx, ((rbp as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416476e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((((rdx as u32)) as i32).wrapping_mul((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rdx = (((rdx as u32)).wrapping_add((r8 as u32))) as u32 as u64;
    r8 = ((rdx as u32)) as u32 as u64;
    rdx = ((((rdx as u32)) & (0x1fu32))) as u32 as u64;
    r8 = ((r8).wrapping_shr(5u32)) as u64;
    rcx = (((rdx as u8)) as u32) as u32 as u64;
    r9 = ((rax.wrapping_add(r8.wrapping_mul(4))) as u64) as u64;
    rax = 0x1;
    rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64;
    fl_l = (crate::simd_ops::lk_or_u32(r9, ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647710(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158f6c0; const P1: u64 = 0x142691658; const P2: u64 = 0x1426916e8; const P3: u64 = 0x68; const P4: u64 = 0x208; const P5: u64 = 0x210; const P6: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rbx, rax);
    rax = P2;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691658; const P1: u64 = 0x1426916e8; const P2: u64 = 0x68; const P3: u64 = 0x14158f890; const P4: u64 = 0x141834d30;
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
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x218; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416477b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (0x7ffffcffffc4020u64) as u64;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416477d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c0; const P1: u64 = 0x118; const P2: u64 = 0x38; const P3: u64 = 0x30; const P4: u64 = 0x1415ae930; const P5: u64 = 0x141591df0; const P6: u64 = 0x141591ae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdx = 0x1; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rsi; rbx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (((rbx as u8)) as u32) as u32 as u64; rdx = rdi; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143932028; const P1: u64 = 0x208; const P2: u64 = 0x70; const P3: u64 = 0x141645950; const P4: u64 = 0x210;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r14 = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rbp = rax; rsi = r14; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rbx = rax; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdi = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P4), rax); bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P0)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(P0, rax); rbx = rax; crate::simd_ops::wu64(rcx, r14); crate::simd_ops::wu64(rdi, r14); bb = 10; continue; }
            8 => { crate::simd_ops::wu64(rsi.wrapping_add(P2), rax); rbx = rax; crate::simd_ops::wu64(rcx, r14); crate::simd_ops::wu64(rdi, r14); bb = 10; continue; }
            9 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rdi)) as u64; bb = 10; continue; }
            10 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(P0)) as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1c0; const P1: u64 = 0x1f8; const P2: u64 = 0x19c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::lk_and_u8(rcx.wrapping_add(P1), (0xfeu8) as u8);
    fl_l = (crate::simd_ops::lk_or_u8(rax.wrapping_add(P2), (0x30u8) as u8)) as u8 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x13f; const P1: u64 = 0x143930f44; const P2: u64 = 0x90; const P3: u64 = 0xb8; const P4: u64 = 0xc4; const P5: u64 = 0xcc; const P6: u64 = 0xd4; const P7: u64 = 0xdc; const P8: u64 = 0xe4; const P9: u64 = 0xf0; const P10: u64 = 0xf8; const P11: u64 = 0x142689180;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; r8 = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); crate::simd_ops::wu64(rcx.wrapping_add(P7), rax); crate::simd_ops::wu64(rcx.wrapping_add(P8), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P9))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P10))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r9 = P11; bb = 5; continue; }
            5 => { rcx = ((crate::simd_ops::ru8(rax)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; rax = ((rax).wrapping_add(0x10u64)) as u64; rdx = (crate::simd_ops::ru32(r9.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x88u64))) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(r8.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x88u64))) }).to_bits() as u128); crate::simd_ops::wu32(r8.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm0 as u32)) as u32); rcx = (crate::simd_ops::ru32(r8.wrapping_add(0xf8u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0xf0u64)))) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_and_u8(r8.wrapping_add(0x13fu64), (0xfbu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(0x13fu64), (0xfbu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415a3d00; const P1: u64 = 0x1415ca1a0; const P2: u64 = 0x14159ed60; const P3: u64 = 0x1415a3f30; const P4: u64 = 0x14159f2d0; const P5: u64 = 0x141591e10; const P6: u64 = 0x143932028; const P7: u64 = 0x141592a80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x78u64)) as u64; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rbx = rcx.wrapping_add(0x88); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); xmm0 = 0u128; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r14); crate::simd_ops::wu128(rbx, (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rbx.wrapping_add(0x10u64), (xmm0) as u128); rdi = rcx; r8 = rcx.wrapping_add(0x150); crate::simd_ops::wu128(rbx.wrapping_add(0x20u64), (xmm0) as u128); rdx = rcx.wrapping_add(0x8); crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rbx.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rbx.wrapping_add(0x70u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), r12); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x7cu64), r12); crate::simd_ops::wu64(rbx.wrapping_add(0x84u64), r12); crate::simd_ops::lk_and_u8(rbx.wrapping_add(0xb7u64), (0xf8u8) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x8cu64), ((r12 as u32)) as u32); crate::simd_ops::wu16(rbx.wrapping_add(0xb4u64), ((r12 as u16)) as u16); crate::simd_ops::wu32(rbx.wrapping_add(0x78u64), (0x3f800000u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xb6u64), ((r12 as u8)) as u8); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x1b8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x1c0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1c0u64))) as u64; r8 = rdi.wrapping_add(0x8); rdx = rbx; crate::simd_ops::wu32(rdi.wrapping_add(0x164u64), (0x42ca0000u32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x1c0u64))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x208u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 1; continue; }
            1 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rbx = ((r12 as u32)) as u32 as u64; rsi = ((r12 as u32)) as u32 as u64; bb = 3; continue; }
            3 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = ((rbx as u32)) as u32 as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x208u64))) as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; rdx = (rdx & !0xffu64) | (((if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (crate::simd_ops::ru16(rsi.wrapping_add(r9).wrapping_add(0xd4u64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u16)) != ((fl_r as u16))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::lk_or_u8(r9.wrapping_add(0x1b4u64), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 5; continue; }
            5 => { crate::simd_ops::wu16(rsi.wrapping_add(r9).wrapping_add(0xd4u64), ((rax as u16)) as u16); fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x1b4u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rax.wrapping_add(0x164u64), (0x42ca0000u32) as u32); bb = 8; continue; }
            8 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rsi).wrapping_add(0x30u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x208u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (crate::simd_ops::ru16(r9.wrapping_add(0x1b0u64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u16)) != ((fl_r as u16))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_or_u8(r9.wrapping_add(0x1b4u64), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(0x1b4u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; crate::simd_ops::wu16(r9.wrapping_add(0x1b0u64), ((rax as u16)) as u16); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rax.wrapping_add(0x164u64), (0x42ca0000u32) as u32); bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1f8u64))) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsp.wrapping_add(0x20); r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rcx = rdi; crate::simd_ops::wu16(rsp.wrapping_add(0x28u64), (0xff00u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1c0u64))) as u64; r8 = rdi.wrapping_add(0x78); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x138u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1c0u64))) as u64; r9 = rdi.wrapping_add(0x178); r8 = rdi.wrapping_add(0x150); rdx = rdi.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1f8u64))) as u32) as u32 as u64; fl_l = ((((rax as u8)) & (0xfbu8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 16; continue; }
            16 => { rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x3u8))) as u8 as u64)); rcx = rdi; crate::simd_ops::wu8(rdi.wrapping_add(0x1f8u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x210u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x70u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 20; continue; }
            20 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rax.wrapping_add(0x210u64), rdi); rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(P6, rdi); rcx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), r12); rsp = ((rsp).wrapping_add(0x78u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            23 => { crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(P6, rdi); bb = 24; continue; }
            24 => { rcx = rdi; rsp = ((rsp).wrapping_add(0x78u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1b0; const P1: u64 = 0x1b4; const P2: u64 = 0x8; const P3: u64 = 0x164;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64;
    r8 = (r8 & !0xffu64) | (((if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P0))) as u16 as u64; fl_r = (0x0u16) as u16 as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u16)) != ((fl_r as u16))) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { fl_l = (crate::simd_ops::lk_or_u8(rcx.wrapping_add(P1), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rdx as u16)) as u16);
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu32(rax.wrapping_add(P3), (0x42ca0000u32) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647d40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x30; const P2: u64 = 0x1415aef20; const P3: u64 = 0x1c0; const P4: u64 = 0x120; const P5: u64 = 0x210; const P6: u64 = 0x143932028; const P7: u64 = 0x70; const P8: u64 = 0x141592ae0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsi = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = rsp.wrapping_add(0x30); rcx = rdi; rbx = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (((rbx as u8)) as u32) as u32 as u64; rcx = rdi; rdx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rdx = 0x1; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rcx.wrapping_add(P5), rdx); rax = (crate::simd_ops::ru64(P6)) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(P6, rcx); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx); bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(P5), rax); crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); bb = 9; continue; }
            9 => { rdx = (((rsi as u8)) as u32) as u32 as u64; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141647e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x142689180; const P2: u64 = 0x143930f44; const P3: u64 = 0x1415317a0; const P4: u64 = 0x141647cf0; const P5: u64 = 0x141592e90;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut j_tgt, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = ((r8.wrapping_sub(0x5u64)) as u32) as u32 as u64; r11 = ((r8 as u32)) as u32 as u64; rbx = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1f; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 32; continue; } bb = 1; continue; }
            1 => { rcx = P0; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(rax).wrapping_add(0x1648068u64))) as u32) as u32 as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x1648050u64))) as u32 as u64; fl_l = ((r10).wrapping_add(rcx)) as u64; fl_r = fl_l; r10 = ((fl_l as u64)) as u64; j_tgt = r10; bb = 0xffff_fffe; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); rdi = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1a; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x124u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 21; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm7) as u128); xmm7 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32) as u128; fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm7 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 15; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rsi); rcx = rbx.wrapping_add(0xf0); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm6) as u128); r8 = rsp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rdx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rsi as u32)) as u32); xmm6 = 0u128; crate::simd_ops::wu8(rsp.wrapping_add(0x90u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rsi as u32)) as u32); bb = 9; continue; }
            9 => { xmm7 = ((crate::simd_ops::ru32(rax.wrapping_add(0xcu64))) as u32) as u128; xmm6 = (xmm7) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm8 as u32)).to_bits() as u128); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x130u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64))) as u32) as u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 18; continue; } bb = 12; continue; }
            12 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x13fu64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 14; continue; }
            14 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; bb = 15; continue; }
            15 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 17; continue; }
            17 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            18 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 19; continue; }
            19 => { if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm6 as u32)) as u32); bb = 14; continue; }
            21 => { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x88u64), ((xmm0 as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            22 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::lk_or_u8(rax.wrapping_add(0x19cu64), (0x10u8) as u8); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            24 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x1c0u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::lk_or_u8(rax.wrapping_add(0x19cu64), (0x20u8) as u8); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x208u64))) as u64; rax = ((r11.wrapping_sub(0x1du64)) as u32) as u32 as u64; r10 = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64)))))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_add(r9)) as u64; rax = ((crate::simd_ops::ru16(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0xd4u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = (((r10 as u16)).wrapping_add((rax as u16))) as u16 as u64; fl_r = fl_l; r10 = (r10 & !0xffffu64) | ((((fl_l as u16)) as u16 as u64)); rax = (rax & !0xffu64) | (((if (((fl_l as u16)) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::lk_or_u8(r8.wrapping_add(0x1b4u64), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 28; continue; }
            28 => { crate::simd_ops::wu16(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0xd4u64), ((r10 as u16)) as u16); fl_l = (crate::simd_ops::ru8(r8.wrapping_add(0x1b4u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu32(rax.wrapping_add(0x164u64), (0x42ca0000u32) as u32); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            31 => { rdx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0xa8u64)))))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x208u64))) as u64; rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_add(crate::simd_ops::ru16(rcx.wrapping_add(0x1b0u64)))) as u16 as u64)); rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            32 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x141647e10u64) { 0x19 => { bb = 1; continue; } 0x36 => { bb = 2; continue; } 0x4d => { bb = 3; continue; } 0x5a => { bb = 4; continue; } 0x67 => { bb = 5; continue; } 0x89 => { bb = 6; continue; } 0xca => { bb = 7; continue; } 0xd4 => { bb = 8; continue; } 0xd7 => { bb = 9; continue; } 0xe9 => { bb = 10; continue; } 0xfa => { bb = 11; continue; } 0x108 => { bb = 12; continue; } 0x10d => { bb = 13; continue; } 0x114 => { bb = 14; continue; } 0x119 => { bb = 15; continue; } 0x124 => { bb = 16; continue; } 0x12c => { bb = 17; continue; } 0x132 => { bb = 18; continue; } 0x135 => { bb = 19; continue; } 0x137 => { bb = 20; continue; } 0x142 => { bb = 21; continue; } 0x16b => { bb = 22; continue; } 0x177 => { bb = 23; continue; } 0x184 => { bb = 24; continue; } 0x190 => { bb = 25; continue; } 0x19d => { bb = 26; continue; } 0x1d3 => { bb = 27; continue; } 0x1db => { bb = 28; continue; } 0x1f2 => { bb = 29; continue; } 0x1ff => { bb = 30; continue; } 0x20f => { bb = 31; continue; } 0x230 => { bb = 32; continue; } _ => { return crate::simd_ops::tail_stk_xmm(j_tgt, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); } } }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x141524320; const P2: u64 = 0x14183acbc; const P3: u64 = 0x14163ccf0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbp = rsp.wrapping_add(0x40); rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r11 = ((rdx as u32)) as u32 as u64; r10 = ((crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u32) as u32 as u64; rdx = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm6) as u128); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r10 = (if (((fl_l as u32)) > ((fl_r as u32))) { (rax as u32) } else { (r10 as u32) }) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r10.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r11 as u32)) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (0xffffffffffffff0u64) as u64; bb = 4; continue; }
            4 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rax = ((crate::simd_ops::ru16(r8.wrapping_add(0x12u64))) as u32) as u32 as u64; rsp = ((rsp).wrapping_sub(rcx)) as u64; xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; r14 = rsp.wrapping_add(0x40); xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; r15 = rdi; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(rdi).wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(rdi).wrapping_add(0x98u64))) as u128; crate::simd_ops::wu128(rbp, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_add(0xa8u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(rdi).wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x2; rdx = rdi.wrapping_add(0x8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; r13 = r8; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r13 = (if (((fl_l as u32)) == ((fl_r as u32))) { r9 } else { r13 }) as u64; r12 = (((r8 as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; rsi = (((r9 as u8)) as u32) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(0x7u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_shr(3u32)) as u32 as u64; rsi = ((((rsi as u32)) as i32).wrapping_mul(((r12 as u32)) as i32) as u32) as u32 as u64; rsi = (((rsi as u32)).wrapping_shl(5u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xc0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_add(0xb8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((crate::simd_ops::ru8(r13.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = (((rdx as u32)).wrapping_add(0x7u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((r12 as u32)) as i32) as u32) as u32 as u64; r8 = ((rcx.wrapping_add(0x10u64)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(5u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(rdi).wrapping_add(0xb8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = ((rsi as u32)) as u32 as u64; rdx = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_add(0xb8u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x12u64))) as u32) as u32 as u64; r8 = rdi.wrapping_add(0x50); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = rbp; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm6 as u32)) as u32); r8 = ((r8).wrapping_add(rbx)) as u64; rdx = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_add(0xb8u64))) as u64; rdx = r14; r8 = ((rsi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_add(0xc0u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x20u64))) as u128; rsp = rbp.wrapping_add(0x30); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x141524320; const P2: u64 = 0x14183acbc; const P3: u64 = 0x14163ccf0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbp = rsp.wrapping_add(0x40); rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r11 = ((rdx as u32)) as u32 as u64; r10 = ((crate::simd_ops::ru8(r8.wrapping_add(0x8u64))) as u32) as u32 as u64; rdx = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), rbx); crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm6) as u128); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; r10 = (if (((fl_l as u32)) > ((fl_r as u32))) { (rax as u32) } else { (r10 as u32) }) as u32 as u64; bb = 2; continue; }
            2 => { rax = ((r10.wrapping_add(0x7u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r11 as u32)) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (0xffffffffffffff0u64) as u64; bb = 4; continue; }
            4 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rax = ((crate::simd_ops::ru16(r8.wrapping_add(0x12u64))) as u32) as u32 as u64; rsp = ((rsp).wrapping_sub(rcx)) as u64; xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm0 = (((rax as u32)) as u32) as u128; r14 = rsp.wrapping_add(0x40); xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; r15 = rdi; bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(rdi).wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 7; continue; }
            7 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(rdi).wrapping_add(0x98u64))) as u128; crate::simd_ops::wu128(rbp, (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_add(0xa8u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), ((xmm1 as u64)) as u64); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(rdi).wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x2; rdx = rdi.wrapping_add(0x8); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; r13 = r8; r8 = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; r13 = (if (((fl_l as u32)) == ((fl_r as u32))) { r9 } else { r13 }) as u64; r12 = (((r8 as u8)) as u32) as u32 as u64; rdx = ((rdx).wrapping_add(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; rsi = (((r9 as u8)) as u32) as u32 as u64; rsi = (((rsi as u32)).wrapping_add(0x7u32)) as u32 as u64; rsi = (((rsi as u32)).wrapping_shr(3u32)) as u32 as u64; rsi = ((((rsi as u32)) as i32).wrapping_mul(((r12 as u32)) as i32) as u32) as u32 as u64; rsi = (((rsi as u32)).wrapping_shl(5u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x88u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_add(0xb8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = ((crate::simd_ops::ru8(r13.wrapping_add(0x8u64))) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = (((rdx as u32)).wrapping_add(0x7u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((r12 as u32)) as i32) as u32) as u32 as u64; r8 = ((rcx.wrapping_add(0x10u64)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(5u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(rdi).wrapping_add(0xb8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = ((rsi as u32)) as u32 as u64; rdx = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_add(0xb8u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(rbp.wrapping_add(0x12u64))) as u32) as u32 as u64; r8 = rdi.wrapping_add(0x50); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = rbp; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm6 as u32)) as u32); r8 = ((r8).wrapping_add(rbx)) as u64; rdx = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(rdi).wrapping_add(0xb8u64))) as u64; rdx = r14; r8 = ((rsi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_add(0xc0u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x20u64))) as u128; rsp = rbp.wrapping_add(0x30); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141635060; const P1: u64 = 0x1416345c0; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1416352a0; const P4: u64 = 0x141648090;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rbp); rbx = r8; crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), r13); rbp = rdx; r13 = 0; fl_l = 0; fl_r = 0; r14 = rcx; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), r12); r12 = rcx.wrapping_add(0x20); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r15); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r15); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rbx = (crate::simd_ops::ru64(r15.wrapping_add(rbx))) as u64; rdi = (crate::simd_ops::ru64(r15.wrapping_add(rax))) as u64; r8 = rbx; r9 = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x30); r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); r8 = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x3; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x12u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; rax = (rax & !0xffffu64) | (((if (((fl_l as u16)) >= ((fl_r as u16))) { (rdx as u16) } else { (rax as u16) }) as u16 as u64)); r8 = (((rax as u16)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r15 = ((rdx.wrapping_mul(4)) as u32) as u32 as u64; r12 = ((r8 as u32)) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; rbp = ((rcx as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = r15; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r12)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; r12 = r14.wrapping_add(0x20); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x12u64), ((rax as u16)) as u16); bb = 11; continue; }
            10 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) <= ((fl_r as u32))) { (rcx as u32) } else { (rsi as u32) }) as u32 as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbp)) as u64; r15 = ((r15).wrapping_add(0x8u64)) as u64; r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r15); fl_l = ((r13 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x12u64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x12u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((((r8 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x2cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x12u64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x12u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((((r8 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = ((rsi as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rsi as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416486c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141635060; const P1: u64 = 0x1416345c0; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1416352a0; const P4: u64 = 0x141648290;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r9 = (crate::simd_ops::ru64(rdx)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); rbx = r8; crate::simd_ops::wu64(rax.wrapping_sub(0x30u64), r13); rsi = rdx; r13 = 0; fl_l = 0; fl_r = 0; r14 = rcx; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r12); crate::simd_ops::wu64(rax.wrapping_sub(0x38u64), r15); r15 = rcx.wrapping_add(0x20); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rbx = ((r13.wrapping_mul(2)) as u64) as u64; rbx = ((rbx).wrapping_add(r13)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rbx = ((rbx).wrapping_shl(4u32)) as u64; rdx = rsp.wrapping_add(0x30); rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r9))) as u64; r8 = rbx; rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; r9 = rdi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x30); r9 = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r8 = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0xb0u64))) as u32 as u64; fl_r = 0x3; rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x12u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; rax = (rax & !0xffffu64) | (((if (((fl_l as u16)) >= ((fl_r as u16))) { (rdx as u16) } else { (rax as u16) }) as u16 as u64)); r8 = (((rax as u16)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r15 = ((rdx.wrapping_mul(4)) as u32) as u32 as u64; r12 = ((r8 as u32)) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = ((rcx as u32)) as u32 as u64; bb = 7; continue; }
            7 => { rcx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = r15; rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(r12)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r15 = r14.wrapping_add(0x20); bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(0x12u64), ((rax as u16)) as u16); bb = 11; continue; }
            10 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rbp = (if (((fl_l as u32)) <= ((fl_r as u32))) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru64(rsi)) as u64; r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r9.wrapping_add(0x8u64))) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rdx, ((rcx as u32)) as u32); rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(r14)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x12u64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x12u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((((r8 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x2cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x10u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rcx.wrapping_add(0x12u64), ((rax as u16)) as u16); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; r8 = ((crate::simd_ops::ru8(rcx.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x12u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((((r8 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(2u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdx = ((rbp as u32)) as u32 as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rbp as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; r8 = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416488e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x20; const P6: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rcx, rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r9);
    xmm0 = (crate::simd_ops::ru128(rax)) as u128;
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), (0x0u32) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x38; const P2: u64 = 0x40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141649730; const P1: u64 = 0x288; const P2: u64 = 0x142691718; const P3: u64 = 0x142677048; const P4: u64 = 0x10; const P5: u64 = 0x142691790; const P6: u64 = 0x18; const P7: u64 = 0x142691820; const P8: u64 = 0x80; const P9: u64 = 0x3e4; const P10: u64 = 0x3e0; const P11: u64 = 0x98;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    rax = P2;
    crate::simd_ops::wu64(rbx, rax);
    rax = P3;
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    rax = P5;
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    rax = P7;
    crate::simd_ops::wu64(rbx.wrapping_add(P8), rax);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(P9), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(P10), (0x1u8) as u8);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P11))) as u32 as u64;
    crate::simd_ops::wu32(rbx.wrapping_add(0x3e8u64), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xa0u64))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(0x3f0u64), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691718; const P1: u64 = 0x142677048; const P2: u64 = 0x10; const P3: u64 = 0x142691790; const P4: u64 = 0x18; const P5: u64 = 0x142691820; const P6: u64 = 0x80; const P7: u64 = 0x1416498d0; const P8: u64 = 0x141834d30;
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
    rax = P5;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x3f8; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x141620d10; const P5: u64 = 0x141620d50; const P6: u64 = 0x14152ec40; const P7: u64 = 0x1415864b0; const P8: u64 = 0x1438152ac; const P9: u64 = 0x1415945c0; const P10: u64 = 0x1415bfb60; const P11: u64 = 0x1415c0d60; const P12: u64 = 0x143931d40; const P13: u64 = 0x14152ede0; const P14: u64 = 0x1415c2070;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 115>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x270); rsp = ((rsp).wrapping_sub(0x370u64)) as u64; rbx = (crate::simd_ops::ru64(P0)) as u64; r12 = ((rdx as u32)) as u32 as u64; r13 = ((((r8 as u32)) as i32 as i64) as u64) as u64; r15 = rcx; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { r9 = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r9 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = r14; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 13; continue; }
            13 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            14 => { rdi = r14; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x8); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { fl_l = rdi; fl_r = rdi; rsi = rdi.wrapping_sub(0x10); rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rax = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 49; continue; }
            26 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x278u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x280u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = r15.wrapping_add(0x158); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x324u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x0u64) as u64); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rcx = rbp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x9u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (0x0u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), r14); crate::simd_ops::wu32(rbp.wrapping_sub(0x14u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbp, (0x0u64) as u64); crate::simd_ops::wu16(rbp.wrapping_add(0x8u64), (0xff00u16) as u16); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r14); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; crate::simd_ops::lk_and_u8(rbp.wrapping_add(0x264u64), (0xfeu8) as u8); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf3u8))) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) | (0x3u8))) as u8 as u64)); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x25cu64))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x288u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x270u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x5cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x264u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x26cu64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x260u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x2u32) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru32(P8)) as u32 as u64; rax = (0x20c49ba5e353f7cfu64) as u64; rcx = (((rcx) as i64).wrapping_mul((r13) as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rbx); let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rdx = ((((rdx) as i64).wrapping_shr(7u32) as u64)) as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0xfdu8))) as u8 as u64)); rax = rdx; crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((r14 as u32)) as u32); rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rcx as u8)) as u8); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = rsp.wrapping_add(0x38); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x358u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rax); rax = (crate::simd_ops::ru32(r15.wrapping_add(0x318u64))) as u32 as u64; crate::simd_ops::wu32(rbp, ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((r12 as u32)) as u32); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xdu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 36; continue; } bb = 34; continue; }
            34 => { rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x2b8u64), ((rcx as u8)) as u8); rcx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r8 = ((rax as u32)) as u32 as u64; r9 = rbp.wrapping_add(0x2b8); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x5cu64))) as u32 as u64; rdx = rbp.wrapping_add(0x8); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rcx = rsi; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 35; continue; }
            35 => { rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x268u64))) as u32) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x2b8u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | ((((((rcx as u8)) ^ ((rax as u8)))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & ((rbx as u8)))) as u8 as u64)); fl_l = ((((rax as u8)) ^ ((rcx as u8)))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_add(0x268u64), ((rax as u8)) as u8); bb = 36; continue; }
            36 => { rdx = rsp.wrapping_add(0x70); rcx = rsi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rbx = ((rax.wrapping_sub(0x2u64)) as u32) as u32 as u64; bb = 40; continue; }
            38 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = rsp.wrapping_add(0x70); rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x250u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; bb = 40; continue; }
            40 => { rax = ((crate::simd_ops::ru8(r15.wrapping_add(0x304u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { fl_l = ((((rax as u8)) | (0x2u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(r15.wrapping_add(0x304u64), ((rax as u8)) as u8); bb = 42; continue; }
            42 => { rcx = rsi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            44 => { crate::simd_ops::lk_add_u32(P12, (1) as u32); rcx = rbp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            46 => { rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 48; continue; }
            48 => { rax = ((rbx as u32)) as u32 as u64; bb = 49; continue; }
            49 => { r11 = rsp.wrapping_add(0x370); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141649ee0; const P1: u64 = 0x1415f5900; const P2: u64 = 0x141648a80; const P3: u64 = 0x143930f90; const P4: u64 = 0x14159aa90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdi = (((rdx as u8)) as u32) as u32 as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3dcu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x304u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x3d4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rbx.wrapping_add(0x3e0u64), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x3e0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x3e4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); r9 = rbx.wrapping_add(0x25c); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); r8 = rsp.wrapping_add(0x50); rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x288u64))) as u64; rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rcx = rsi; r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbx.wrapping_add(0x3e0u64), (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r8 = ((rdi as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x270u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = rbx.wrapping_add(0x25c); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); r8 = rsp.wrapping_add(0x50); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r14 as u32)) as u32); rdx = rsp.wrapping_add(0x40); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3d8; const P1: u64 = 0x3d0; const P2: u64 = 0x258; const P3: u64 = 0x143930f90; const P4: u64 = 0x14159aa90; const P5: u64 = 0x14164a700;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (((rdx as u8)) as u32) as u32 as u64;
    rbx = rcx;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rcx = (crate::simd_ops::ru64(P3)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (((rdi as u8)) as u32) as u32 as u64;
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x142691858; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x10;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P1; crate::simd_ops::wu64(rax.wrapping_add(P2), (0x0u64) as u64); crate::simd_ops::wu64(rax, rcx); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141648fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691858; const P1: u64 = 0x8; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); }
    rdx = rbx;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649020(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x38;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r9 = rsp.wrapping_add(0x50);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r9);
    xmm0 = 0u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32);
    r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64));
    crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x143932030; const P2: u64 = 0x20; const P3: u64 = 0x38;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rdi = ((r9 as u32)) as u32 as u64; rsi = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x1; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), (0x1u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rbp as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rbp as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); r9 = ((rbp.wrapping_add(0x2u64)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x64u8) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rbp); bb = 5; continue; }
            5 => { rax = ((rdi as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x2; const P2: u64 = 0x48; const P3: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = rcx; rsi = ((r8 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdx); xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); r9 = (((rdi as u8)) as u32) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416491f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bf090; const P1: u64 = 0xf8; const P2: u64 = 0x100; const P3: u64 = 0x142691880; const P4: u64 = 0x142677588; const P5: u64 = 0x8; const P6: u64 = 0x142677ec8; const P7: u64 = 0x10; const P8: u64 = 0x142677ef0; const P9: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rbx, rax);
    rax = P4;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    rax = P6;
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rax);
    rax = P8;
    crate::simd_ops::wu64(rbx.wrapping_add(P9), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x268; const P2: u64 = 0x8; const P3: u64 = 0x78; const P4: u64 = 0x20; const P5: u64 = 0xf0; const P6: u64 = 0x1415355f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; rsi = rcx; rcx = rdx; rdi = rdx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rbp = ((rax as u32)) as u32 as u64; rbx = 0x1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = 0xffff; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) < ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rbx.wrapping_add(P2)) as u32) as u32 as u64; bb = 9; continue; }
            2 => { rax = ((rbp.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x4; bb = 9; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0x15; bb = 9; continue; }
            6 => { rdi = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; rcx = ((rsi.wrapping_add(P5)) as u64) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x5; bb = 9; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(P4))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rax = 0x17; rbx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rax as u32) } else { (rbx as u32) }) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416493a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x14153e940; const P4: u64 = 0x1; const P5: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = ((rcx.wrapping_add(P1)) as u64) as u64; rsi = ((r8 as u32)) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rsi as u32)) as u32); rax = 0x1; crate::simd_ops::wu32(rdx, ((rbp as u32)) as u32); bb = 6; continue; }
            5 => { rax = 0x2; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x1415af950; const P4: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rbx = ((rcx.wrapping_add(P1)) as u64) as u64; rsi = ((rdx as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rax, ((rsi as u32)) as u32); bb = 6; continue; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    r8 = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rdx; fl_r = r8;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    loop { rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rdx.wrapping_add(P2)))) as u32 as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; fl_l = rdx; fl_r = r8; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416494b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1415242e0; const P3: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x0u32) as u32); }
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0x0u8) as u8 as u64)); return rax; }
            4 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; r10 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { r9 = (r9 & !0xffffu64) | (((((r9 as u16)).wrapping_add(1)) as u16 as u64)); rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            4 => { crate::simd_ops::wu16(r8, ((r9 as u16)) as u16); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416495a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = r9; rcx = ((r9.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = r9; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            4 => { rax = ((rax).wrapping_sub(r9)) as u64; fl_l = ((((rax) as i64).wrapping_shr(2u32) as u64)) as u64; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu16(r8, ((rax as u16)) as u16); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416495e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (((rdx as u16)) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416495f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = 0xc350;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (((rdx as u16)) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(8)))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (((rdx as u16)) as u32) as u32 as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((rdx as u32)) as u32 as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdi = rcx; rax = ((rbx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = rbx; r8 = rbx; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = ((r9.wrapping_add(P2)) as u32) as u32 as u64; rax = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = r8; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_sub(rbx)) as u64; rdx = ((rbx.wrapping_add(P0)) as u64) as u64; rax = ((((rax as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = rbx; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(P1), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; bb = 7; continue; }
            6 => { fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rax = ((r9 as u32)) as u32 as u64; rdx = r8; bb = 7; continue; }
            7 => { rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r9 = ((rax as u32)) as u32 as u64; r8 = rdx; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416496b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xffffffffffffffff; const P3: u64 = 0x4; const P4: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; rcx = r8; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((r8.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = ((r9.wrapping_add(P2)) as u32) as u32 as u64; rax = ((r8.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_sub(rcx)) as u64; rdx = ((rcx.wrapping_add(P3)) as u64) as u64; rax = ((((rax as u32)) & (0xfffffffcu32))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P1), (1) as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdx = (((rdx as u16)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rax.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P1), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141593180; const P1: u64 = 0x142691bc0; const P2: u64 = 0x142677048; const P3: u64 = 0x142691c38; const P4: u64 = 0x142691cc8; const P5: u64 = 0x141620d00;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); rdi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rbx = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rbx, rax); rax = P2; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); rax = P3; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rax); rax = P4; crate::simd_ops::wu64(rbx.wrapping_add(0x80u64), rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x380u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x3dcu64))) as u32) as u32 as u64; crate::simd_ops::lk_and_u32(rbx.wrapping_add(0x3d8u64), (0xfffffff0u32) as u32); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf8u8))) as u8 as u64)); crate::simd_ops::wu64(rbx.wrapping_add(0x3ccu64), rsi); crate::simd_ops::wu32(rbx.wrapping_add(0x3d4u64), ((rsi as u32)) as u32); rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2f8u64))) as u32) as u32 as u64; rcx = (rcx & !0xffu64) | (((((rcx as u8)).wrapping_shr(1u32)) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) & (0x1u8))) as u8 as u64)); rcx = (rcx & !0xffu64) | ((((((rcx as u8)) | ((rax as u8)))) as u8 as u64)); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x288u64))) as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x3dcu64), ((rcx as u8)) as u8); crate::simd_ops::lk_add_u32(rax.wrapping_add(0x14u64), (1) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x324u64), ((rcx as u32)) as u32); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x278u64))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x278u64), rcx); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rax, rsi); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x280u64))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x280u64), rcx); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rsi); bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rbx.wrapping_add(0x388u64), ((rsi as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x38u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x388u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x398u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x58u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x3a8u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x68u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x3b8u64), (xmm1) as u128); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x3c8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x7cu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x3ccu64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x388u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3ccu64))) as u32 as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx.wrapping_add(0x388u64), ((rdx as u32)) as u32); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 8; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { crate::simd_ops::wu32(rbx.wrapping_add(0x388u64), ((rdx as u32)) as u32); bb = 8; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1416498d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691bc0; const P1: u64 = 0x142677048; const P2: u64 = 0x10; const P3: u64 = 0x142691c38; const P4: u64 = 0x18; const P5: u64 = 0x142691cc8; const P6: u64 = 0x80; const P7: u64 = 0x288; const P8: u64 = 0x14; const P9: u64 = 0x380; const P10: u64 = 0x141620d50; const P11: u64 = 0x1415935e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = P5;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    rax = 0xffffffff;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P8), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P9), (0x0u64) as u64); }
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691bc0; const P1: u64 = 0x142677048; const P2: u64 = 0x10; const P3: u64 = 0x142691c38; const P4: u64 = 0x18; const P5: u64 = 0x142691cc8; const P6: u64 = 0x80; const P7: u64 = 0x288; const P8: u64 = 0x14; const P9: u64 = 0x380; const P10: u64 = 0x141620d50; const P11: u64 = 0x1415935e0; const P12: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rdi = ((rdx as u32)) as u32 as u64; rax = P1; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); rax = P3; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rax = P5; crate::simd_ops::wu64(rcx.wrapping_add(P6), rax); rax = 0xffffffff; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P7))) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P8), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P9))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P9), (0x0u64) as u64); bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = 0x3e0; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649a10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1d8; const P1: u64 = 0x78; const P2: u64 = 0x70; const P3: u64 = 0x1415fbfb0; const P4: u64 = 0x3dc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = rcx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rbp = 0x1; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 4; continue; }
            4 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; rdi = (((rdi as u8)) as u32) as u32 as u64; rdi = (if (((fl_l as u32)) == ((fl_r as u32))) { (rbp as u32) } else { (rdi as u32) }) as u32 as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(P4))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415947d0; const P1: u64 = 0x143930f80; const P2: u64 = 0x1415ad990;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x260u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rsi = rbx.wrapping_sub(0x18); fl_l = rsi; fl_r = rsi; r8 = rbx.wrapping_sub(0x8); r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdi.wrapping_add(0xdu64)) as u32) as u32 as u64; r8 = ((rdi.wrapping_sub(0x2u64)) as u32) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2eau64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfau8))) as u8 as u64)); crate::simd_ops::wu64(rbx.wrapping_add(0x260u64), rbp); fl_l = ((((rax as u8)) | (0x2u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x2eau64), ((rax as u8)) as u8); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x268u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rsi = rbx.wrapping_sub(0x18); fl_l = rsi; fl_r = rsi; r8 = rbx.wrapping_sub(0x8); r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0xe; rcx = rsi; r8 = ((rdx.wrapping_sub(0xfu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(0x268u64), rbp); bb = 9; continue; }
            9 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x2eau64))) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x7u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30cu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x268u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xf9u8))) as u8 as u64)); fl_l = ((((rax as u8)) | (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x2eau64), ((rax as u8)) as u8); bb = 13; continue; }
            13 => { rax = ((rdi as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1438152ac; const P1: u64 = 0x143426858; const P2: u64 = 0x1434269f8; const P3: u64 = 0x142677278;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    xmm1 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((((rax) as i64) as f64).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128);
    xmm0 = 0u128;
    xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P1))).to_bits() as u128);
    fl_l = ((xmm1 as u64)) as u64; fl_r = ((xmm0 as u64)) as u64;
    if !(!(f64::from_bits(fl_l) > f64::from_bits(fl_r))) { xmm0 = ((crate::simd_ops::ru64(P2)) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128); rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = ((crate::simd_ops::ru64(P3)) as u64) as u128;
    xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) + f64::from_bits(xmm1 as u64)).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm0 as u64)))) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415c1150; const P5: u64 = 0x141620bb0; const P6: u64 = 0x141621270; const P7: u64 = 0x141620d00; const P8: u64 = 0x141620d50; const P9: u64 = 0x141594d90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); r15 = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rdi = rcx; r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x3d4u64))) as u32 as u64; rsi = ((r12 as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 17; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbp = rbx.wrapping_add(0x8); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rsi = r12; bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 14; continue; }
            14 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { bb = 23; continue; }
            17 => { rbp = rbx.wrapping_add(0x8); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 18; continue; }
            18 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rbp = rbx.wrapping_add(0x8); bb = 23; continue; }
            23 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = ((rsi).wrapping_add(0xfffffffffffffff0u64)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 27; continue; }
            26 => { rsi = r12; bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x3dcu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x3d4u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = r15; rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::lk_and_u8(rdi.wrapping_add(0x3dcu64), (0xfbu8) as u8); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x3d4u64), ((r12 as u32)) as u32); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 47; continue; } bb = 32; continue; }
            32 => { bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; r9 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(r9.wrapping_sub(0x28u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = r15; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 47; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x388u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x388u64), ((rax as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 36; continue; }
            36 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 40; continue; } bb = 37; continue; }
            37 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x3ccu64))) as u32 as u64; fl_l = (((rdx) >> (((rcx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) != 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu32(rdi.wrapping_add(0x388u64), ((rcx as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 36; continue; } bb = 39; continue; }
            39 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rcx.wrapping_sub(0x5); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 46; continue; }
            40 => { crate::simd_ops::wu32(rdi.wrapping_add(0x388u64), ((rcx as u32)) as u32); bb = 36; continue; }
            41 => { rdx = (crate::simd_ops::ru64(r9.wrapping_sub(0x8u64))) as u64; r8 = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rcx.wrapping_sub(0x5); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 42; continue; }
            42 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x380u64), rbx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 43; continue; }
            43 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            46 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 50; continue; } bb = 48; continue; }
            48 => { crate::simd_ops::wu64(rdi.wrapping_add(0x380u64), r12); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 50; continue; }
            50 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 51; continue; }
            51 => { rcx = rsi.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 53; continue; }
            53 => { rdx = r15; rcx = rdi; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141649ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415fa780; const P1: u64 = 0x141620bb0; const P2: u64 = 0x141621270; const P3: u64 = 0x141620d00; const P4: u64 = 0x141620d50; const P5: u64 = 0x1415fbfb0; const P6: u64 = 0x1415fbfc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x3dcu64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; r14 = (((rdx as u8)) as u32) as u32 as u64; rdi = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x380u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { r15 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x10u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rax = rbx.wrapping_sub(0x3); rax = ((rsi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = rbx.wrapping_sub(0x4); rcx = ((rsi.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); r9 = rsp.wrapping_add(0xa0); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x28u64))) as u64; r8 = rsp.wrapping_add(0x90); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x388u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x388u64), ((rdx as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rax = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x3ccu64))) as u32 as u64; fl_l = (((rcx) >> (((rax) as u32 & 63))) & 1) as u64; if ((fl_l as u64) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rdi.wrapping_add(0x388u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rcx.wrapping_sub(0x5); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            11 => { crate::simd_ops::wu32(rdi.wrapping_add(0x388u64), ((rax as u32)) as u32); bb = 7; continue; }
            12 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 22; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x8u64))) as u64; r8 = rsp.wrapping_add(0xa8); crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rcx.wrapping_sub(0x5); rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 14; continue; }
            14 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x380u64), rbx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0x380u64), r15); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_or_u8(rdi.wrapping_add(0x3dcu64), (0x4u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x388u64))) as u32 as u64; rax = ((crate::simd_ops::ru16(rsp.wrapping_add(0x90u64))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub(1)) as u32 as u64; crate::simd_ops::wu16(rdi.wrapping_add(rcx.wrapping_mul(2)).wrapping_add(0x38cu64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x3d4u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x28u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(rdi.wrapping_add(0x3d8u64))))) as u32 as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; crate::simd_ops::lk_xor_u32(rdi.wrapping_add(0x3d8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3d8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(28u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(28u32) as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { xmm0 = 0u128; bb = 26; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; rdx = rsp.wrapping_add(0x30); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_sub(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x270u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rdi.wrapping_add(0x3d0u64), ((xmm0 as u32)) as u32); rcx = rbp.wrapping_add(0x10); crate::simd_ops::lk_or_u8(rdi.wrapping_add(0x3dcu64), (0x4u8) as u8); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x380u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x380u64), r15); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 29; continue; }
            29 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 30; continue; }
            30 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164a1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416111e0; const P1: u64 = 0x141524330; const P2: u64 = 0x1415a3f40; const P3: u64 = 0x1416117b0; const P4: u64 = 0x141611800; const P5: u64 = 0x141610260; const P6: u64 = 0x1415c0b00; const P7: u64 = 0x141610ef0; const P8: u64 = 0x141610fd0; const P9: u64 = 0x141611a20; const P10: u64 = 0x1416119b0; const P11: u64 = 0x141611820; const P12: u64 = 0x141611aa0; const P13: u64 = 0x141611a90; const P14: u64 = 0x14161b1c0; const P15: u64 = 0x143930f08; const P16: u64 = 0x1415a4f40; const P17: u64 = 0x1415242e0; const P18: u64 = 0x141620d50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x304u64))) as u8 as u64; fl_r = (0x10u8) as u8 as u64; rbx = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3d8u64))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 19; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x3d4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); rdx = rbx.wrapping_add(0x380); rcx = 0x503; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { rdx = 0x48; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x324u64))) as u32 as u64; rdx = rbx.wrapping_add(0x388); crate::simd_ops::wu32(rdi.wrapping_add(0xe8u64), ((rcx as u32)) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x3d4u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); rcx = rdi; crate::simd_ops::wu8(rsp.wrapping_add(0x34u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x288u64))) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x210u64))) as u32) as u32 as u64; rcx = rdi; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(6u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1d8u64))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x358u64))) as u64; r9 = rbx.wrapping_add(0x348); rdx = ((rax as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x90); rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x278u64))) as u64; r8 = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x280u64))) as u64; r8 = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = rbx.wrapping_add(0x158); rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x304u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rdx = rbx.wrapping_add(0x25c); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rdi); rcx = rsi.wrapping_add(0x1c); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P15)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P17, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 13; continue; }
            13 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x304u64), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x380u64))) as u64; rdi = 0; fl_l = 0; fl_r = 0; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x380u64), rdi); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rbx.wrapping_add(0x3d4u64), ((rdi as u32)) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 19; continue; }
            19 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164a3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141649a10; const P1: u64 = 0x141595b60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    rdi = (((r8 as u8)) as u32) as u32 as u64;
    xmm6 = (xmm1) as u128;
    rbx = rcx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { r8 = (((rdi as u8)) as u32) as u32 as u64; xmm1 = (xmm6) as u128; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164a410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141649a10; const P1: u64 = 0x141595bc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = (((r8 as u8)) as u32) as u32 as u64;
    rsi = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164a450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426e64; const P1: u64 = 0x1416111e0; const P2: u64 = 0x141524330; const P3: u64 = 0x1415a3f40; const P4: u64 = 0x1416117b0; const P5: u64 = 0x141611800; const P6: u64 = 0x141610260; const P7: u64 = 0x1415c0b00; const P8: u64 = 0x141610ef0; const P9: u64 = 0x141610fd0; const P10: u64 = 0x1416119b0; const P11: u64 = 0x141611820; const P12: u64 = 0x143426780; const P13: u64 = 0x141649bd0; const P14: u64 = 0x141611790; const P15: u64 = 0x14161b1c0; const P16: u64 = 0x143930f08; const P17: u64 = 0x1415a4f40; const P18: u64 = 0x1438152b4; const P19: u64 = 0x1415a6020; const P20: u64 = 0x1415242e0; const P21: u64 = 0x141620d50;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x304u64))) as u32) as u32 as u64; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); xmm6 = (xmm1) as u128; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x3d8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(28u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(28u32) as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 21; continue; } bb = 3; continue; }
            3 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 21; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x3d4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rdx = rbx.wrapping_add(0x380); rcx = 0x503; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 6; continue; }
            6 => { rdx = 0x48; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x324u64))) as u32 as u64; rdx = rbx.wrapping_add(0x388); crate::simd_ops::wu32(rsi.wrapping_add(0xe8u64), ((rcx as u32)) as u32); rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x3d4u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); rcx = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x288u64))) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x210u64))) as u32) as u32 as u64; rcx = rsi; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(6u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x1d8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x358u64))) as u64; r9 = rbx.wrapping_add(0x348); rdx = ((rax as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x90); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x280u64))) as u64; r8 = rdi; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0x158); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm2 = (xmm6) as u128; xmm0 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P12))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) < f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x3d0u64))) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x3d0u64))) }).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | (((f32::from_bits(xmm6 as u32)) as f64).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm2 as u32)))) as u32 as u64; rdx = rbx; rcx = rsi; rbp = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsi)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rcx = rsi; rdx = ((r9.wrapping_add(0xfu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rdx = rbx.wrapping_add(0x25c); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rsi); rcx = rdi.wrapping_add(0x1c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P16)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru16(P18)) as u32) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x324u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P16)) as u64; rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            12 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 15; continue; }
            15 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_or_u8(rbx.wrapping_add(0x304u64), (0x2u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x380u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x380u64), (0x0u64) as u64); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rbx.wrapping_add(0x3d4u64), (0x0u32) as u32); bb = 21; continue; }
            21 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164a6e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3dc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add((rdx as u8))) as u8 as u64));
    rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfdu8))) as u8 as u64));
    fl_l = ((((rax as u8)) | ((rdx as u8)))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rax as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164a700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141593f10; const P1: u64 = 0x143930f08; const P2: u64 = 0x1415a49b0; const P3: u64 = 0x143930f88; const P4: u64 = 0x1426452b0; const P5: u64 = 0x1426452a8; const P6: u64 = 0x142645560; const P7: u64 = 0x1416111e0; const P8: u64 = 0x1416117b0; const P9: u64 = 0x141611800; const P10: u64 = 0x141610260; const P11: u64 = 0x1415c0b00; const P12: u64 = 0x141610ef0; const P13: u64 = 0x141610fd0; const P14: u64 = 0x141611a90; const P15: u64 = 0x141524330; const P16: u64 = 0x1415a3f40; const P17: u64 = 0x141611a20; const P18: u64 = 0x1416119b0; const P19: u64 = 0x141611820; const P20: u64 = 0x141649bd0; const P21: u64 = 0x141611810; const P22: u64 = 0x1438152b4; const P23: u64 = 0x143931104; const P24: u64 = 0x14161b1c0; const P25: u64 = 0x1415a4f40; const P26: u64 = 0x1415a6020; const P27: u64 = 0x1415242e0; const P28: u64 = 0x141620d50; const P29: u64 = 0x141595ec0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); rcx = ((rcx).wrapping_add(0xffffffffffffffe8u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r15 = (((rdx as u8)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_sub(0x18u64))) as u64; rcx = rdi.wrapping_sub(0x18); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x3bcu64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rdi.wrapping_sub(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x3bcu64))) as u32 as u64; r12 = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 60; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2ecu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 60; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x20u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3c0u64))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(28u32)) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(28u32) as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 60; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 19; continue; } bb = 9; continue; }
            9 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 10; continue; }
            10 => { rbp = rbx.wrapping_add(0x8); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } bb = 12; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { rsi = r12; bb = 17; continue; }
            17 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 22; continue; }
            19 => { rbp = rbx.wrapping_add(0x8); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 20; continue; }
            20 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            21 => { rsi = r12; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 29; continue; } bb = 23; continue; }
            23 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 23; continue; } bb = 25; continue; }
            25 => { bb = 30; continue; }
            26 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 31; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            29 => { rbp = rbx.wrapping_add(0x8); bb = 30; continue; }
            30 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { fl_l = rsi; fl_r = rsi; r14 = rsi.wrapping_sub(0x10); r14 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { r14 }) as u64; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 32; continue; }
            32 => { rdx = rdi.wrapping_add(0x368); rcx = 0x503; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; rbp = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30cu64))) as u32 as u64; rdx = rdi.wrapping_add(0x370); crate::simd_ops::wu32(rax.wrapping_add(0xe8u64), ((rcx as u32)) as u32); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); rcx = rbx; crate::simd_ops::wu8(rsp.wrapping_add(0x64u64), ((r12 as u8)) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x270u64))) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x1f8u64))) as u32) as u32 as u64; rcx = rbx; rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_shr(6u32)) as u8 as u64)); rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x1c0u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x340u64))) as u64; r9 = rdi.wrapping_add(0x330); rdx = ((rax as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi.wrapping_add(0x78); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3c0u64))) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0xfu8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x300u64))) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { rdx = 0x48; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 37; continue; }
            37 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x260u64))) as u64; r8 = rax; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x268u64))) as u64; r8 = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 39; continue; }
            39 => { rdx = rdi.wrapping_add(0x140); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x2ecu64))) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 43; continue; } bb = 40; continue; }
            40 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x3c0u64))) as u32 as u64; rax = ((((rax as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 48; continue; } bb = 42; continue; }
            42 => { xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x3b8u64))) as u32) as u128; r10 = (crate::simd_ops::ru64(rbx)) as u64; xmm0 = crate::simd_ops::vcvtps2pd(xmm0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); r8 = ((rax as u32)) as u32 as u64; rcx = rbx; rdx = ((r9.wrapping_add(0xfu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            43 => { rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x3c4u64))) as u32) as u32 as u64; rcx = rbx; rdx = (rdx & !0xffu64) | ((((((rdx as u8)) & (0x1u8))) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x3b8u64))) as u32) as u128; xmm0 = crate::simd_ops::vcvtps2pd(xmm0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((crate::simd_ops::ru16(P22)) as u32) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rdx = (crate::simd_ops::ru32(P23)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xau32)) as u32 as u64; rdx = ((((rdx as u32)) as i32).wrapping_mul(((rcx as u32)) as i32) as u32) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x3c0u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xfu32))) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 45; continue; } bb = 44; continue; }
            44 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 46; continue; } bb = 45; continue; }
            45 => { r8 = ((rdx as u32)) as u32 as u64; bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rdx = ((r9.wrapping_add(0xfu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 47; continue; }
            47 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 48; continue; }
            48 => { rdx = rdi.wrapping_add(0x244); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rbx); rcx = rsi.wrapping_add(0x1c); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P1)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P25, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x3b8u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 53; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x30cu64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 53; continue; } bb = 50; continue; }
            50 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x268u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 53; continue; } bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P26, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            52 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P27, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 53; continue; }
            53 => { rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 55; continue; }
            55 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2ecu64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 57; continue; } bb = 56; continue; }
            56 => { fl_l = ((((rax as u8)) | (0x2u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x2ecu64), ((rax as u8)) as u8); bb = 57; continue; }
            57 => { rcx = r14.wrapping_add(0x10); let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 59; continue; } bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 59; continue; }
            59 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 60; continue; }
            60 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x368u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x368u64), r12); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 61; continue; }
            61 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P28, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 62; continue; }
            62 => { rcx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x2ecu64))) as u32) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x3u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 64; continue; } bb = 63; continue; }
            63 => { fl_l = ((((rcx as u8)) & (0xfeu8))) as u8 as u64; fl_r = fl_l; rcx = (rcx & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x2ecu64), ((rcx as u8)) as u8); bb = 64; continue; }
            64 => { rdx = (((r15 as u8)) as u32) as u32 as u64; rcx = rdi; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P29, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164ab50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3dc; const P1: u64 = 0x3d4; const P2: u64 = 0x380; const P3: u64 = 0x141620d50; const P4: u64 = 0x1415971d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::lk_or_u8(rcx.wrapping_add(P0), (0x4u8) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    rdi = (((r8 as u8)) as u32) as u32 as u64;
    rsi = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rax);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    r8 = (((rdi as u8)) as u32) as u32 as u64;
    rdx = ((rsi as u32)) as u32 as u64;
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164abb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141611fb0; const P1: u64 = 0x70; const P2: u64 = 0x142691cf8; const P3: u64 = 0x58; const P4: u64 = 0x60; const P5: u64 = 0x68;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64;
    r10 = r9;
    r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    r11 = r8;
    rbx = rdx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    r8 = r10;
    rdx = r11;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64;
    rax = P2;
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((((rcx as u32)) & (0xfffffffeu32))) as u32 as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rax);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64;
    rcx = ((((rcx as u32)) | ((rax as u32)))) as u32 as u64;
    rax = rdi;
    rcx = ((((rcx as u32)) & (0xfffffff9u32))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((rcx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164ac20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691cf8; const P1: u64 = 0x68; const P2: u64 = 0x10; const P3: u64 = 0x60; const P4: u64 = 0x1415242e0; const P5: u64 = 0x58; const P6: u64 = 0x1416129e0; const P7: u64 = 0x141612060; const P8: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rcx, rax); rsi = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rdx = ((rbx.wrapping_add(P1)) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), rbp); rcx = rbp; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdx, rcx); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rax; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = 0x78; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164acd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x141524330; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0xc; const P5: u64 = 0x68; const P6: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; rbx = ((r8 as u32)) as u32 as u64; rsi = rdx; rdi = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdx = 0x18; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru64(rsi)) as u64) as u128; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P2), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(P3), rcx); crate::simd_ops::wu32(rax.wrapping_add(P4), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P4))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(r9.wrapping_add(P3), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi.wrapping_add(P5), r9); bb = 8; continue; }
            7 => { crate::simd_ops::wu64(rcx.wrapping_add(P3), r9); bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(P6), r9); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164ad80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141612350; const P1: u64 = 0x2c; const P2: u64 = 0x143930f90; const P3: u64 = 0x141599ce0; const P4: u64 = 0x58; const P5: u64 = 0x141612550;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = ((rax as u32)) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164add0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x4c; const P2: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164adf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141612300; const P1: u64 = 0x1416121e0; const P2: u64 = 0x1415242e0; const P3: u64 = 0x141612370;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rbp = ((r8 as u32)) as u32 as u64; r15 = rdx; rdi = rcx; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x70u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r12); r12 = 0; fl_l = 0; fl_r = 0; rsi = ((r12 as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r14); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; rcx = rbx.wrapping_add(0xc); r14 = rbx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rax); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rsi); bb = 11; continue; }
            11 => { r9 = (crate::simd_ops::ru32(rcx)) as u32 as u64; r8 = r14; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((r12 as u8)) as u8); rcx = rdi; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r12 as u8)) as u8); rdx = r15; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rbx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = r14; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rsi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 16; continue; }
            16 => { rcx = rdi; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            17 => { rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164aef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x70; const P2: u64 = 0x141612c90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::lk_or_u32(rbx.wrapping_add(P1), (0x4u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rdx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::lk_or_u32(rbx.wrapping_add(P1), (0x4u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164af20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); r15 = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x48u64), (0x0u8) as u8); rbx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rdx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdi = r14; bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            11 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { rdi = r14; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x8); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rdi.wrapping_sub(0x10); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rax }) as u64; fl_l = rax; fl_r = rax; crate::simd_ops::wu64(r15.wrapping_add(0x40u64), rax); rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x70; const P2: u64 = 0x141612d50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::lk_and_u32(rbx.wrapping_add(P1), (0xfffffffbu32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::lk_and_u32(rbx.wrapping_add(P1), (0xfffffffbu32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x2c; const P2: u64 = 0x143930f90; const P3: u64 = 0x14159b3d0; const P4: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rbx = rcx;
    fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rax = ((((rax as u32)) | (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffde; const P1: u64 = 0x141524330; const P2: u64 = 0x141634230; const P3: u64 = 0x141634d40; const P4: u64 = 0xf; const P5: u64 = 0x5; const P6: u64 = 0x142691d38; const P7: u64 = 0x8; const P8: u64 = 0x141634f20; const P9: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rdx as u32)) as u32 as u64; rdx = 0x28; rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = ((rdi as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rdx = ((rax.wrapping_add(P4)) as u32) as u32 as u64; rcx = ((rax.wrapping_add(P5)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = P6; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rax.wrapping_add(P7), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x6; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x141634f20; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rbx = rdx;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x6; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64;
    fl_l = rdi; fl_r = rdi;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x6; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = rbx;
    rcx = 0x6;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b1e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1416488e0; const P2: u64 = 0x141648930;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    r9 = r8;
    r8 = rdx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64;
    rcx = rsp.wrapping_add(0x30);
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0xb8u64))) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141642f70; const P1: u64 = 0x8; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = ((rdx as u32)) as u32 as u64;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14164b7b0; const P1: u64 = 0x1438152b4; const P2: u64 = 0x1415cf3c0; const P3: u64 = 0x143426780; const P4: u64 = 0x143427068; const P5: u64 = 0x14183acc2; const P6: u64 = 0x141643130; const P7: u64 = 0x14162c820; const P8: u64 = 0x1416431e0; const P9: u64 = 0x141602a80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x11; rdi = rdx; rbx = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(0xf0u64), (0x1u8) as u8); bb = 2; continue; }
            2 => { rbp = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x92u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0x12u64))) as u16 as u64; fl_r = ((rbp as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x2b; crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { xmm0 = (crate::simd_ops::ru128(rdi)) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x80u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdi.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x90u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); rsi = rbx.wrapping_add(0xb0); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; rcx = rsi; crate::simd_ops::wu32(rbx.wrapping_add(0xb8u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; crate::simd_ops::wu16(rbx.wrapping_add(0xc0u64), ((rax as u16)) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xf2u64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x2acu64))) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x328u64))) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm2 = (((rax as u32)) as u32) as u128; xmm2 = crate::simd_ops::vcvtdq2ps(xmm2); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; bb = 15; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128; bb = 15; continue; }
            15 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); r12 = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x8u64))) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { r15 = ((r12 as u32)) as u32 as u64; r14 = ((rax as u32)) as u32 as u64; fl_l = ((r15).wrapping_shl(2u32)) as u64; r15 = ((fl_l as u64)) as u64; bb = 18; continue; }
            18 => { rcx = ((crate::simd_ops::ru16(rsi.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = r15; rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) as i64).wrapping_mul((rbp) as i64) as u64) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rbp).wrapping_add(1)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), ((r12 as u32)) as u32); bb = 21; continue; }
            21 => { crate::simd_ops::wu8(rbx.wrapping_add(0xf2u64), (0x0u8) as u8); bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; r8 = ((crate::simd_ops::ru8(rax.wrapping_add(0x305u64))) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x2fcu64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x92u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { bb = 29; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rcx as u32)) as u32); rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_sub((rcx as u16))) as u16 as u64)); crate::simd_ops::wu16(rbx.wrapping_add(0x92u64), ((rdx as u16)) as u16); crate::simd_ops::lk_and_u8(rax.wrapping_add(0x305u64), (0x3fu8) as u8); fl_l = (crate::simd_ops::lk_and_u8(rax.wrapping_add(0x306u64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu32(rax.wrapping_add(0x2fcu64), ((rbp as u32)) as u32); bb = 26; continue; }
            26 => { r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rdx = rbx.wrapping_add(0x80); r14 = ((crate::simd_ops::ru16(rbx.wrapping_add(0x92u64))) as u32) as u32 as u64; rcx = rbx.wrapping_add(0x10); r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r13); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (r14 & !0xffffu64) | (((((r14 as u16)).wrapping_sub(crate::simd_ops::ru16(rbx.wrapping_add(0x92u64)))) as u16 as u64)); rbp = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xe0u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x110u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64); bb = 31; continue; }
            28 => { crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), (0x2u32) as u32); bb = 42; continue; }
            29 => { crate::simd_ops::lk_and_u8(rax.wrapping_add(0x306u64), (0xfeu8) as u8); rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x2fcu64), ((rcx as u32)) as u32); r8 = (r8 & !0xffu64) | ((((((r8 as u8)) & (0x3fu8))) as u8 as u64)); crate::simd_ops::wu8(rax.wrapping_add(0x305u64), ((r8 as u8)) as u8); rcx = rbx; crate::simd_ops::wu16(rbx.wrapping_add(0x92u64), ((rbp as u16)) as u16); rdx = rdi; crate::simd_ops::wu16(rdi.wrapping_add(0x12u64), ((rbp as u16)) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xf0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = 0x2b; rbp = 0x11; rax = (if (((fl_l as u8)) != ((fl_r as u8))) { (rbp as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rax as u32)) as u32); bb = 42; continue; }
            30 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x108u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); bb = 31; continue; }
            31 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); rdx = rbx.wrapping_add(0x80); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rcx = rsp.wrapping_add(0x30); rax = (((r14 as u16)) as u32) as u32 as u64; r9 = ((r12 as u32)) as u32 as u64; r8 = rsi; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 34; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xf1u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 34; continue; } bb = 33; continue; }
            33 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0xa0u64))) as u128; fl_l = (((rax as u32)).wrapping_add((r12 as u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0xf1u64), (0x1u8) as u8); crate::simd_ops::wu128(rbx.wrapping_add(0xd0u64), (xmm0) as u128); crate::simd_ops::wu32(rbx.wrapping_add(0xd0u64), ((rax as u32)) as u32); bb = 34; continue; }
            34 => { rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0x92u64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xd4u64), ((xmm0 as u32)) as u32); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 38; continue; } bb = 35; continue; }
            35 => { rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xf0u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 38; continue; } bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe0u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 37; continue; }
            37 => { rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rax as u32)) as u32 as u64; bb = 38; continue; }
            38 => { rax = ((rbp as u32)) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rax); bb = 41; continue; }
            41 => { crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rbp as u32)) as u32); bb = 42; continue; }
            42 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            43 => { rbp = 0x11; bb = 40; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x54; const P2: u64 = 0xe8; const P3: u64 = 0xf1; const P4: u64 = 0x8; const P5: u64 = 0x308; const P6: u64 = 0x38; const P7: u64 = 0x141643700; const P8: u64 = 0x92; const P9: u64 = 0x14164b270; const P10: u64 = 0xf0; const P11: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P0))) as u32) as u32 as u64; rdi = rcx; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(29u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = (((((r8 as u32)) as i32).wrapping_shr(29u32) as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; rbx = (rbx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rdi.wrapping_add(P0)) as u64) as u64; r8 = (((rbx as u8)) as u32) as u32 as u64; xmm1 = (xmm0) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(P8))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rsi; rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            2 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P10))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsi.wrapping_add(P11), (0x11u32) as u32); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b6a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe8; const P1: u64 = 0xf0; const P2: u64 = 0xf2; const P3: u64 = 0x141643200;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), r8);
    r8 = ((r9 as u32)) as u32 as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x1u8) as u8);
    fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b6d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x48; const P2: u64 = 0x4; const P3: u64 = 0x10; const P4: u64 = 0x1416436a0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((rdi.wrapping_add(P2)) as u64) as u64; r8 = rax; rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0xc8; const P2: u64 = 0x1415cf540; const P3: u64 = 0xc; const P4: u64 = 0x18; const P5: u64 = 0x20; const P6: u64 = 0x24; const P7: u64 = 0x28; const P8: u64 = 0x2c; const P9: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = 0;
    fl_l = 0; fl_r = 0;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdi = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rsi);
    fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rsi;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rsi); crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x2bu64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P4), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P5), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P7), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x1u32) as u32); crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rsi as u32)) as u32); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b7b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x8c; const P3: u64 = 0x80; const P4: u64 = 0x98; const P5: u64 = 0xa0; const P6: u64 = 0xa4; const P7: u64 = 0xa8; const P8: u64 = 0xac; const P9: u64 = 0x12; const P10: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = rdx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x2bu64) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P7), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x1u32) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rdi, rax);
    crate::simd_ops::wu16(rdi.wrapping_add(P9), ((rax as u16)) as u16);
    crate::simd_ops::wu64(rdi.wrapping_add(P10), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((rax).wrapping_sub(r8)) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x8c; const P3: u64 = 0x80; const P4: u64 = 0x98; const P5: u64 = 0xa0; const P6: u64 = 0xa4; const P7: u64 = 0xa8; const P8: u64 = 0xac; const P9: u64 = 0x141643690; const P10: u64 = 0xf0; const P11: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x2bu64) as u64);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P5), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P7), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x1u32) as u32);
    rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P10))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b8e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0xe8; const P3: u64 = 0x38; const P4: u64 = 0x1438152ac; const P5: u64 = 0x2a0; const P6: u64 = 0x10; const P7: u64 = 0xb0; const P8: u64 = 0x141643930; const P9: u64 = 0xf0;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    rsi = rcx;
    rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rbx);
    rcx = rdx;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru32(P4)) as u32 as u64;
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    xmm2 = (xmm0) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rcx = ((rsi.wrapping_add(P6)) as u64) as u64;
    r9 = ((rsi.wrapping_add(P7)) as u64) as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu8(rsi.wrapping_add(P9), (0x0u8) as u8);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x1415cf540; const P2: u64 = 0x10; const P3: u64 = 0x92; const P4: u64 = 0x98; const P5: u64 = 0xe8; const P6: u64 = 0x141643b10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    rcx = ((rcx).wrapping_add(0xb0u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rdi;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu16(rbx.wrapping_add(P3), ((rdi as u16)) as u16);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164b9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1416438a0; const P1: u64 = 0x14269143c; const P2: u64 = 0x143426780; const P3: u64 = 0x1438152b4; const P4: u64 = 0x142691440;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; r14 = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (xmm0) as u128; rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x44u64))) as u32 as u64; xmm1 = 0u128; rsi = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rbp = ((rsi.wrapping_add(0x2du64)) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); rdi = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdx = ((crate::simd_ops::ru16(rbx.wrapping_add(0x92u64))) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xf0u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x40); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(0xf0u64), (0x1u8) as u8); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x305u64))) as u8 as u64; fl_r = (0x40u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x2fcu64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { r8 = ((rdx as u32)) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rcx = (((rcx as u32)).wrapping_sub((rdx as u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rcx as u32)) as u32); crate::simd_ops::lk_and_u8(rax.wrapping_add(0x305u64), (0x3fu8) as u8); fl_l = (crate::simd_ops::lk_and_u8(rax.wrapping_add(0x306u64), (0xfeu8) as u8)) as u8 as u64; fl_r = fl_l; crate::simd_ops::wu32(rax.wrapping_add(0x2fcu64), ((r8 as u32)) as u32); bb = 12; continue; }
            12 => { rdx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x40u64))) as u32) as u32 as u64; bb = 13; continue; }
            13 => { rax = (((rdx as u16)) as u32) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rcx as u32))) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rcx as u32))) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; rdx = (rdx & !0xffffu64) | ((((fl_l as u16).wrapping_sub(fl_r as u16)) as u16 as u64)); crate::simd_ops::wu16(rbx.wrapping_add(0x92u64), ((rdx as u16)) as u16); if (((fl_l as u16)) != ((fl_r as u16))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xf0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 16; continue; }
            16 => { bb = 18; continue; }
            17 => { rbp = 0x11; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x44u64))) as u32 as u64; xmm0 = 0u128; xmm2 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm1 = 0u128; rax = ((rbp as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); rcx = ((rsi as u32)) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; crate::simd_ops::wu32(r14, ((rcx as u32)) as u32); bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bb30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x92; const P1: u64 = 0xf0; const P2: u64 = 0x8; const P3: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16); }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bb60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x80; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x8c; const P4: u64 = 0x98; const P5: u64 = 0xa0; const P6: u64 = 0xa4; const P7: u64 = 0xa8; const P8: u64 = 0xac; const P9: u64 = 0x141643690; const P10: u64 = 0xf0; const P11: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x2bu64) as u64); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); crate::simd_ops::wu32(rbx.wrapping_add(P5), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P7), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x1u32) as u32); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rbx.wrapping_add(P10), ((rsi as u8)) as u8); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P10))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141647250; const P1: u64 = 0xc8; const P2: u64 = 0xca; const P3: u64 = 0xcc; const P4: u64 = 0x8; const P5: u64 = 0xffffffffffffffe8; const P6: u64 = 0x18; const P7: u64 = 0x38; const P8: u64 = 0x1416087b0; const P9: u64 = 0x30; const P10: u64 = 0xc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = r8; rdi = rcx; rdx = rbx; rcx = ((rcx).wrapping_add(0x10u64)) as u64; r8 = ((r9 as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rdi.wrapping_add(P1), (0x0u8) as u8); crate::simd_ops::wu16(rdi.wrapping_add(P2), ((r8 as u16)) as u16); crate::simd_ops::wu8(rdi.wrapping_add(P3), ((r8 as u8)) as u8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = rax; rcx = ((rax.wrapping_add(P5)) as u64) as u64; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r8 = ((rcx.wrapping_add(P6)) as u64) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rdi.wrapping_add(P7)) as u64) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P9))) as u32 as u64; rax = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(P10), ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bcc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bcd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x8; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bcf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x8; const P2: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdx as u8)) as u8); }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bd10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0xca; const P2: u64 = 0x8; const P3: u64 = 0x13c; const P4: u64 = 0xcc; const P5: u64 = 0xc8; const P6: u64 = 0x10; const P7: u64 = 0xb; const P8: u64 = 0x141524320; const P9: u64 = 0xd8; const P10: u64 = 0x12; const P11: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rcx; rbx = rdx; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = rcx; rax = ((rcx.wrapping_add(P3)) as u64) as u64; rdx = 0x154; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rdi.wrapping_add(P4), (0x0u8) as u8); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rdi.wrapping_add(P5), (0x1u8) as u8); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u32) as u32 as u64; rcx = 0x5; rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P6))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; r8 = ((rcx.wrapping_add(P7)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P9), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rbx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu16(rbx.wrapping_add(P10), ((rax as u16)) as u16); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; rdx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), (0x2u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xccu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xccu64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164be10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x30; const P2: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x11u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164be40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0xe0; const P2: u64 = 0xd0; const P3: u64 = 0xd8; const P4: u64 = 0x14164bc10; const P5: u64 = 0x10; const P6: u64 = 0x88; const P7: u64 = 0x143815b50; const P8: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64;
    rbx = rcx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi);
    rax = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    r8 = ((rbx.wrapping_add(P5)) as u64) as u64;
    r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64;
    rdx = P7;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd8; const P1: u64 = 0x1415242e0; const P2: u64 = 0x8; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x5; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bf20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x143815b50; const P2: u64 = 0x8; const P3: u64 = 0xd8; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x5; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bf80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xc8; const P4: u64 = 0x30;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu8(rbx.wrapping_add(P3), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x14164bcc0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164bfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0xd0; const P2: u64 = 0xca; const P3: u64 = 0x30; const P4: u64 = 0x8; const P5: u64 = 0x13c; const P6: u64 = 0x40; const P7: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = rdx; rbx = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P2))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rdx; fl_r = rdx; rax = ((rdx.wrapping_add(P5)) as u64) as u64; r8 = 0x154; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rax }) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164c060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x10; const P2: u64 = 0x14164bcf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rbx as u32)) as u32 as u64;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164c0a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524320; const P1: u64 = 0x143427a70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rbx = rcx; rdi = rdx; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0xcau64))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 14; continue; } bb = 1; continue; }
            1 => { fl_l = rcx; fl_r = rcx; rax = rcx.wrapping_add(0x13c); rdx = 0x154; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 14; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rbx.wrapping_add(0xccu64), (0x0u8) as u8); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x11; crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rsi); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu8(rbx.wrapping_add(0xc8u64), (0x1u8) as u8); bb = 4; continue; }
            4 => { rsi = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u32) as u32 as u64; rax = rdi.wrapping_add(0x8); rax = rdi.wrapping_add(0x10); rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = ((rsi.wrapping_add(0x5u64)) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; r8 = ((rsi.wrapping_add(0x10u64)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu16(rdi.wrapping_add(0x12u64), ((rsi as u16)) as u16); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rdx = rbp.wrapping_add(0x27); xmm0 = (crate::simd_ops::ru128(P1)) as u128; crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = rbp.wrapping_sub(0x79); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), rax); rax = rbp.wrapping_add(0x67); crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rax); rax = rbp.wrapping_add(0x77); crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x4du64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), (0xffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rsi); crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rsi); crate::simd_ops::wu128(rbp.wrapping_sub(0x69u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rsi); crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), (0x3f800000u32) as u32); crate::simd_ops::wu16(rbp.wrapping_sub(0x2du64), ((rsi as u16)) as u16); crate::simd_ops::wu8(rbp.wrapping_sub(0x2bu64), (0x1u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x25u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rsi); crate::simd_ops::wu32(rbp.wrapping_sub(0x9u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rsi); crate::simd_ops::wu8(rbp.wrapping_add(0xfu64), ((rsi as u8)) as u8); crate::simd_ops::wu8(rbp.wrapping_add(0x17u64), (0x32u8) as u8); crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), rdi); crate::simd_ops::wu32(rbp.wrapping_add(0x27u64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rsi); crate::simd_ops::wu32(rbp.wrapping_sub(0xdu64), ((rsi as u32)) as u32); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xfu64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rsp = ((rsp).wrapping_add(0xe0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), (0x2u32) as u32); bb = 12; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xccu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rbx.wrapping_add(0xccu64), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0xe0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164c280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x8; const P2: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = 0x11; return rax; }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164c2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524320; const P1: u64 = 0x14183acc2; const P2: u64 = 0x1434268d0; const P3: u64 = 0x1415e1400; const P4: u64 = 0x14163ccf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = rsp.wrapping_add(0x50); fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x11; r14 = rdx; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rbx); rdi = rcx; crate::simd_ops::wu128(rbp, (xmm6) as u128); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(0xc8u64), (0x1u8) as u8); bb = 2; continue; }
            2 => { rbx = 0; fl_l = 0; fl_r = 0; r15 = rcx.wrapping_add(0xe0); fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0xf2u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rcx.wrapping_add(0x150u64), ((rbx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(r15, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r15.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r15.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64) as u128; crate::simd_ops::wu64(r15.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rbx); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; rsi = rcx.wrapping_add(0x118); crate::simd_ops::wu32(rcx.wrapping_add(0xecu64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x120u64))) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(0xd8u64))) as u32) as u32 as u64; rcx = 0x5; rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64; r8 = ((rcx.wrapping_add(0xbu64)) as u32) as u32 as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x120u64))) as u32 as u64; crate::simd_ops::wu64(rsi, rax); rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0xd8u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rsi.wrapping_add(0x10u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rsi.wrapping_add(0x12u64), ((rbx as u16)) as u16); crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rcx as u32)) as u32); bb = 7; continue; }
            7 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0xf2u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x154u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0xcau64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = rcx; rax = rcx.wrapping_add(0x13c); rdx = 0x154; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; r9 = rsi; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x150u64))) as u32 as u64; rdx = r15; crate::simd_ops::wu8(rdi.wrapping_add(0xccu64), ((rbx as u8)) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            10 => { crate::simd_ops::wu32(r14.wrapping_add(0x30u64), (0x2u32) as u32); bb = 30; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xccu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { rcx = ((crate::simd_ops::ru16(rdi.wrapping_add(0xf2u64))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x128u64))) as u32) as u32 as u64; fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0xccu64), (0x1u8) as u8); rcx = (rcx & !0xffffu64) | (((if (((fl_l as u16)) >= ((fl_r as u16))) { (rax as u16) } else { (rcx as u16) }) as u16 as u64)); r13 = (((rcx as u16)) as u32) as u32 as u64; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x120u64))) as u32) as u32 as u64; r12 = ((r13.wrapping_add(0xfu64)) as u32) as u32 as u64; r12 = ((((r12 as u32)) & (0xfffffff0u32))) as u32 as u64; rcx = ((r12.wrapping_mul(4)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { r14 = ((rcx as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru16(rsi.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = r14; rcx = ((rbx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x120u64))) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; bb = 18; continue; }
            18 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 19; continue; }
            19 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xe8u64))) as u32) as u32 as u64; rcx = ((r12 as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r13 as u32))) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx as u32)).wrapping_shl(2u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { r14 = ((rcx as u32)) as u32 as u64; bb = 21; continue; }
            21 => { rax = ((crate::simd_ops::ru16(r15.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = r14; rcx = ((rbx as u32)) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (((rcx) as i64).wrapping_mul((rax) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(r15)) as u64; rcx = ((rcx).wrapping_add(r13)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0xe8u64))) as u32) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 21; continue; } bb = 22; continue; }
            22 => { r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsi = rdi.wrapping_add(0x118); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x120u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xe8u64))) as u32 as u64; r8 = (((rcx as u8)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0x7u32)) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(3u32)) as u32 as u64; r8 = ((((r8 as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(5u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; r8 = ((r8).wrapping_add(0xfu64)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 25; continue; } bb = 24; continue; }
            24 => { r8 = (0xffffffffffffff0u64) as u64; bb = 25; continue; }
            25 => { r8 = (((r8) & (0xfffffffffffffff0u64))) as u64; rax = r8; xmm6 = ((crate::simd_ops::ru32(P2)) as u32) as u128; rsp = ((rsp).wrapping_sub(r8)) as u64; xmm3 = (xmm6) as u128; xmm2 = 0u128; xmm1 = 0u128; xmm0 = 0u128; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); rbx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; rcx = (((r12 as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); r8 = rbp.wrapping_add(0x50); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r13) as i64) as f32).to_bits() as u128); r9 = rbx; crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), (0x3f800000u32) as u32); rdx = rsi; crate::simd_ops::wu32(rbp.wrapping_add(0x54u64), (0x3f800000u32) as u32); rcx = r15; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = 0; fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xc8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; crate::simd_ops::wu16(rdi.wrapping_add(0xf2u64), ((rbx as u16)) as u16); if (((fl_l as u8)) == ((fl_r as u8))) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu32(rdi.wrapping_add(0x124u64), (0x11u32) as u32); bb = 29; continue; }
            28 => { rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0x128u64))) as u32) as u32 as u64; rcx = 0x2b; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(0x12au64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; rdx = 0x2d; rcx = (if (((fl_l as u16)) == ((fl_r as u16))) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x124u64), ((rcx as u32)) as u32); bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = r14; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rbp)) as u128; rsp = rbp.wrapping_add(0x10); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164c5e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x10; const P2: u64 = 0xd8; const P3: u64 = 0x30; const P4: u64 = 0x80; const P5: u64 = 0xf2;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = ((crate::simd_ops::ru16(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu16(rcx.wrapping_add(P2), ((rax as u16)) as u16);
    if !((((fl_l as u8)) == ((fl_r as u8)))) { crate::simd_ops::wu32(rdx.wrapping_add(P3), (0x11u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(P5))) as u16 as u64; fl_r = (0x0u16) as u16 as u64;
    if !((((fl_l as u16)) == ((fl_r as u16)))) { rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164c630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0xe0; const P2: u64 = 0xf8; const P3: u64 = 0x118; const P4: u64 = 0x130; const P5: u64 = 0xec; const P6: u64 = 0x100; const P7: u64 = 0x104; const P8: u64 = 0x108; const P9: u64 = 0x10c; const P10: u64 = 0x124; const P11: u64 = 0x138; const P12: u64 = 0x14164bc10; const P13: u64 = 0x143815b50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); rbx = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), (0x2bu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P6), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P7), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P8), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P9), (0x1u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(P10), (0x2bu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P11), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x13cu64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x140u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x144u64), (0x1u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x158u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x15cu64), (0x3f800000u64) as u64); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; r8 = rbx.wrapping_add(0x10); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x88u64))) as u64; rdx = P13; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x120u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164c780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14162c820; const P1: u64 = 0x142691d50; const P2: u64 = 0x1434265f8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0xf2u64))) as u32) as u32 as u64; rbx = rcx; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x154u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; rbp = (((rbp as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x110u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x108u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); bb = 3; continue; }
            3 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; rsi = rbx.wrapping_add(0xe0); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); r8 = rbx.wrapping_add(0x118); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rcx = rsp.wrapping_add(0x30); rdx = rsi; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; rdx = 0x8; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; rax = rcx.wrapping_sub(0x10); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64; fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x100000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x160u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x124u64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rbp as u32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x160u64), ((rcx as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x100u64))) as u32 as u64; fl_r = 0xffffffff; xmm1 = 0u128; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x158u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x160u64), ((r15 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = ((crate::simd_ops::ru16(rbx.wrapping_add(0x12au64))) as u32) as u32 as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x15cu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x158u64), ((xmm1 as u32)) as u32); xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2ps(xmm1); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x15cu64), ((xmm1 as u32)) as u32); if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10cu64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x2cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rbx.wrapping_add(0x100u64)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x158u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x15cu64)))).to_bits() as u128); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((xmm0 as u32)) as u32); bb = 8; continue; }
            8 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x150u64), ((rbp as u32)) as u32); fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0xf2u64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi, r15); crate::simd_ops::wu64(rsi.wrapping_add(0xcu64), (0x2bu64) as u64); crate::simd_ops::wu64(rsi.wrapping_add(0x18u64), r15); crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x24u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x28u64), (0xffffffffu32) as u32); crate::simd_ops::wu32(rsi.wrapping_add(0x2cu64), (0x1u32) as u32); crate::simd_ops::wu64(rdi, r15); crate::simd_ops::wu16(rdi.wrapping_add(0x12u64), ((r15 as u16)) as u16); bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x124u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(0x138u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x118u64))) as u128; crate::simd_ops::wu128(rdi, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x128u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x138u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x130u64), r15); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x124u64))) as u32 as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x30u64), ((rax as u32)) as u32); rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164c9b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x130; const P1: u64 = 0x118; const P2: u64 = 0x5; const P3: u64 = 0x1415242e0; const P4: u64 = 0x124; const P5: u64 = 0x138; const P6: u64 = 0x13c; const P7: u64 = 0x140; const P8: u64 = 0x144;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P2)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x2bu64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P5), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x3f800000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P7), (0xffffffffu32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x1u32) as u32); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164ca30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0xec; const P3: u64 = 0xe0; const P4: u64 = 0xf8; const P5: u64 = 0x100; const P6: u64 = 0x104; const P7: u64 = 0x108; const P8: u64 = 0x10c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x2bu64) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rax);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    crate::simd_ops::wu32(rbx.wrapping_add(P5), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P7), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P8), (0x1u32) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164caa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x143815b50; const P2: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P1; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164cae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xe0; const P4: u64 = 0xec; const P5: u64 = 0xf8; const P6: u64 = 0x100; const P7: u64 = 0x104; const P8: u64 = 0x108; const P9: u64 = 0x10c; const P10: u64 = 0xc8; const P11: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x2bu64) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P6), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P7), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P8), (0xffffffffu32) as u32);
    crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x1u32) as u32);
    crate::simd_ops::wu8(rbx.wrapping_add(P10), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P11))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164cb60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x14164bcc0; const P2: u64 = 0x118; const P3: u64 = 0x5; const P4: u64 = 0x130; const P5: u64 = 0x1415242e0; const P6: u64 = 0x128; const P7: u64 = 0xf8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = ((rdi.wrapping_add(P3)) as u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rdi as u32)) as u32); }
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rdi);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164cbc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0xca; const P2: u64 = 0x30; const P3: u64 = 0x8; const P4: u64 = 0x13c; const P5: u64 = 0xd0; const P6: u64 = 0x40; const P7: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x11; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P1))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; rcx = 0x2d; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdx as u32)) as u32); r8 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = r8; fl_r = r8; rax = ((r8.wrapping_add(P4)) as u64) as u64; r9 = 0x154; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r9 } else { rax }) as u64; fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2d; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x11; rcx = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P0), ((rax as u8)) as u8); bb = 8; continue; }
            8 => { rax = ((rcx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164cc60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x10; const P2: u64 = 0x70; const P3: u64 = 0x14164bcf0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rdi as u32)) as u32 as u64;
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164ccd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691d58; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1415bd8a0; const P4: u64 = 0x1415be1c0; const P5: u64 = 0x18; const P6: u64 = 0x1415242e0; const P7: u64 = 0x1c; const P8: u64 = 0x1415bd2f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(P2), rsi); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), rsi); crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rsi as u32)) as u32); bb = 6; continue; }
            6 => { rdx = rdi; rcx = 0x2; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164cd60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691d58; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0x1415bd8a0; const P4: u64 = 0x1415be1c0; const P5: u64 = 0x18; const P6: u64 = 0x1415242e0; const P7: u64 = 0x1c; const P8: u64 = 0x1415bd2f0; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rcx, rax); rsi = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rdi.wrapping_add(P2), rbp); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P1), rbp); crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rbp as u32)) as u32); bb = 6; continue; }
            6 => { rdx = rdi; rcx = 0x2; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = 0x18; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164ce10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x2; const P3: u64 = 0x141524330; const P4: u64 = 0x8; const P5: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64;
    rcx = ((rax.wrapping_add(P2)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rax.wrapping_add(P4), rcx); fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); crate::simd_ops::wu64(rax.wrapping_add(P5), rcx); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = rcx;
    crate::simd_ops::wu64(rbx.wrapping_add(P0), rcx);
    fl_l = rcx; fl_r = rcx;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164ce70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141609670;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = rcx;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = ((rcx).wrapping_add(0xffffffffffffffe8u64)) as u64;
    fl_l = rax; fl_r = rax;
    rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rcx }) as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164ce90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x14160a240; const P2: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = rcx; fl_r = rcx; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rax = 0xa0; rsi = ((r9 as u32)) as u32 as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rbx }) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32) as u128; rdi = (((rdx as u16)) as u32) as u32 as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm6 as u32)) as u32); r9 = ((rdi as u32)) as u32 as u64; r8 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rbp as u8)) as u8); rdx = 0x2; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164cf20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415be410; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x48; const P4: u64 = 0x8; const P5: u64 = 0x2; const P6: u64 = 0x1415bda50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (((r9 as u8)) as u32) as u32 as u64; rbx = rcx; r9 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; r9 = ((rbx.wrapping_add(P3)) as u64) as u64; rax = 0x60; r9 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r9 }) as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(P4))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 4; continue; }
            4 => { r8 = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(rax)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 9; continue; }
            8 => { rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(rax.wrapping_add(P5), ((rcx as u8)) as u8); bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_add(0x4u64)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164cfd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x142691dd8; const P1: u64 = 0x8; const P2: u64 = 0x143931580; const P3: u64 = 0x18; const P4: u64 = 0x10; const P5: u64 = 0x143931588; const P6: u64 = 0x143931578; const P7: u64 = 0x142691de8; const P8: u64 = 0x20; const P9: u64 = 0x28; const P10: u64 = 0x30; const P11: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = 0; fl_l = 0; fl_r = 0; rax = P0; crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx); crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P3), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rdx); fl_l = (crate::simd_ops::ru64(P5)) as u64; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(P5, rcx); bb = 3; continue; }
            2 => { rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P4), rcx); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::lk_add_u32(P6, (1) as u32)) as u32 as u64; rax = P7; crate::simd_ops::wu64(P2, rcx); crate::simd_ops::wu64(rcx, rax); rax = rcx; crate::simd_ops::wu64(rcx.wrapping_add(P8), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P9), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P10), rdx); crate::simd_ops::wu64(rcx.wrapping_add(P11), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x40u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x48u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x58u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x68u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x70u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x78u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x88u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0x90u64), (0x3f800000u64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0xa8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0xd8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0xe0u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x64u64), (0x3f800000u32) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x98u64), ((rdx as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0xb0u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0xb8u64), rdx); crate::simd_ops::wu64(rcx.wrapping_add(0xc0u64), rdx); crate::simd_ops::wu32(rcx.wrapping_add(0xc8u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xccu64), (0x101u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xd0u64), ((rdx as u8)) as u8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164d160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142691de8; const P1: u64 = 0x14164d350; const P2: u64 = 0x14164da20; const P3: u64 = 0x1415242e0; const P4: u64 = 0x142691dd8; const P5: u64 = 0x143931588; const P6: u64 = 0x143931580; const P7: u64 = 0x143931578; const P8: u64 = 0x142688f88; const P9: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = ((rdx as u32)) as u32 as u64; rax = P0; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx, rax); rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), ((rbp as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xccu64), (0x1u8) as u8); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rbp as u32)) as u32); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x30u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), ((rbp as u32)) as u32); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0x9; crate::simd_ops::wu32(rbx.wrapping_add(0xe0u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0xe4u64), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = P4; crate::simd_ops::wu64(rbx, rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); bb = 15; continue; }
            15 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P5)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(P5, rax); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rax); bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rax }) as u64; crate::simd_ops::lk_sub_u32(P7, (1) as u32); crate::simd_ops::wu64(P6, rax); rax = P8; crate::simd_ops::wu64(rbx, rax); fl_l = ((rsi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = 0xe8; rcx = rbx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164d2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r9 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; r11 = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r10 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rdx = ((rdx.wrapping_mul(4)) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rdi))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; fl_l = (((r9 as u32)).wrapping_shr(1u32)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(r8, (0x0u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(r8, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; fl_l = ((rax).wrapping_add(rdx)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164d350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xffffffffffffffff;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdi); r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; r10 = ((rdx as u32)) as u32 as u64; rdi = rcx; r9 = ((r11 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { rax = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add((r9 as u32))) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rbx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = (crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r9 as u32) } else { (rax as u32) }) as u32 as u64; r9 = ((rax as u32)) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (r8 as u32) } else { (rax as u32) }) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; fl_l = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; r8 = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = ((((r9 as u32)) as i32 as i64) as u64) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rcx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; r11 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rcx } else { r11 }) as u64; rax = r11; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164d400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158ec20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rbp = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm3) as u128; rdi = rdx; rsi = rcx; r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru32(rcx.wrapping_add(0x60u64))) as u32 as u64; bb = 2; continue; }
            2 => { r12 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xceu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = (xmm6) as u128; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbp as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(0x64u64)))).to_bits() as u128); crate::simd_ops::wu64(rax, r15); crate::simd_ops::wu64(rax.wrapping_add(0x14u64), r12); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x3u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0xceu64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { crate::simd_ops::wu64(rdx, r15); xmm0 = (xmm6) as u128; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rsi.wrapping_add(0x64u64)))).to_bits() as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x14u64), r12); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x3u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x4u64)) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 20; continue; }
            20 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164d550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158ec20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x68u64))) as u64; fl_r = (0xffffffffffffffffu64) as u64; rbp = (0xfffffffffffffffcu64) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rbx = rdx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm6 = (xmm2) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r14); rdi = rcx; r14 = rbp; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); r14 = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rcx.wrapping_add(0x68u64)) } else { r14 }) as u64; xmm8 = (xmm3) as u128; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x70u64))) as u32 as u64; fl_r = 0x0; xmm7 = 0u128; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x74u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x70u64))) as u32 as u64; xmm0 = (xmm6) as u128; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x74u64)))).to_bits() as u128); crate::simd_ops::wu64(rdx, r14); crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((xmm8 as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32) as u128; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((xmm0 as u32)) as u32); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rbp }) as u64; fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x8cu64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(5u32)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = 0; fl_l = 0; fl_r = 0; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x8cu64)))).to_bits() as u128); crate::simd_ops::wu64(rdx, rbp); crate::simd_ops::wu64(rdx.wrapping_add(0x14u64), rax); crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), (0x1u32) as u32); crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164d6b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = rbx; r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax, r8); crate::simd_ops::wu8(rax.wrapping_add(0x8u64), ((r8 as u8)) as u8); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rax; rdx = ((rdx).wrapping_sub(rbx)) as u64; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(rcx))) as u64) as u128; crate::simd_ops::wu64(rax, ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rcx).wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rcx as u32)) as u32); rax = ((rax).wrapping_add(0xcu64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14164d790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x38; const P2: u64 = 0x20; const P3: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((rdx as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = ((rax.wrapping_add(r8.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x4u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(0xcu64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            7 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}
