#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ab3c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141611730; const P1: u64 = 0x143930f90; const P2: u64 = 0x14159b3d0; const P3: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; rdi = rdx; rsi = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r12); r12 = 0x503; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r13); r13 = 0x403; crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r14); r14 = 0; fl_l = 0; fl_r = 0; rbp = ((r14.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r14 = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 18; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), rax); bb = 13; continue; }
            12 => { crate::simd_ops::wu64(r14, rax); bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), r14); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rbx, rax); rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x84u64), (1) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), rbx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { r14 = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 22; continue; }
            22 => { bb = 35; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x0u32) as u32); rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 26; continue; } bb = 24; continue; }
            24 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x70); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rax); bb = 30; continue; }
            29 => { crate::simd_ops::wu64(r14, rax); bb = 30; continue; }
            30 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), r14); bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx, rax); crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x5cu64), (1) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rbx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; bb = 36; continue; }
            36 => { rax = 0x1; rsp = ((rsp).wrapping_add(0xc8u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ab5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14160f710; const P1: u64 = 0x141611730; const P2: u64 = 0x143930f90; const P3: u64 = 0x14159b3d0; const P4: u64 = 0x1415242e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; rax = ((r9 as u32)) as u32 as u64; r14 = rdx; r12 = r8; r13 = rcx; rdx = 0x1820; r9 = 0x503; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 41; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = r12; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 4; continue; }
            4 => { rbp = rbx.wrapping_add(0x30); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 39; continue; } bb = 7; continue; }
            7 => { rax = rbp; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 8; continue; }
            8 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 9; continue; }
            9 => { fl_l = r14; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xd4u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 17; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 14; continue; }
            14 => { fl_l = r14; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { bb = 23; continue; }
            17 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { fl_l = r14; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 21; continue; }
            21 => { bb = 23; continue; }
            22 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 23; continue; }
            23 => { rcx = rdx.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 39; continue; } bb = 26; continue; }
            26 => { rax = rbx.wrapping_add(0x30); r9 = 0x503; bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = 0x403; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbp; bb = 31; continue; }
            31 => { rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 32; continue; }
            32 => { crate::simd_ops::wu64(r13.wrapping_add(0x38u64), rdi); bb = 34; continue; }
            33 => { crate::simd_ops::wu64(r15, rdi); bb = 34; continue; }
            34 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu64(r13.wrapping_add(0x40u64), r15); bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = 0xffffffff; crate::simd_ops::lk_sub_u32(r13.wrapping_add(0x5cu64), (1) as u32); crate::simd_ops::wu64(r13.wrapping_add(0x48u64), rcx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 40; continue; }
            39 => { r15 = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; rdx = 0x1820; r9 = 0x503; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 41; continue; }
            41 => { rdi = (crate::simd_ops::ru64(r13.wrapping_add(0x60u64))) as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 82; continue; } bb = 42; continue; }
            42 => { bb = 43; continue; }
            43 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { fl_l = r12; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 80; continue; } bb = 45; continue; }
            45 => { rbp = rbx.wrapping_add(0x30); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 80; continue; } bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 80; continue; } bb = 48; continue; }
            48 => { rax = rbp; fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 49; continue; }
            49 => { rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 80; continue; } bb = 50; continue; }
            50 => { fl_l = r14; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 63; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0xd4u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) as i8) >= (((fl_r as u8)) as i8)) { bb = 58; continue; } bb = 52; continue; }
            52 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 55; continue; } bb = 53; continue; }
            53 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 55; continue; }
            55 => { fl_l = r14; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 63; continue; } bb = 56; continue; }
            56 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x80u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 55; continue; } bb = 57; continue; }
            57 => { bb = 64; continue; }
            58 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 59; continue; }
            59 => { bb = 60; continue; }
            60 => { fl_l = r14; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 63; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 60; continue; } bb = 62; continue; }
            62 => { bb = 64; continue; }
            63 => { rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 64; continue; }
            64 => { rcx = rdx.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 66; continue; } bb = 65; continue; }
            65 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 66; continue; }
            66 => { fl_l = ((rsi as u8)) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 80; continue; } bb = 67; continue; }
            67 => { rax = rbx.wrapping_add(0x30); bb = 68; continue; }
            68 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = 0x403; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x0u32) as u32); rdx = ((crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 71; continue; } bb = 69; continue; }
            69 => { r8 = 0x503; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 72; continue; } bb = 70; continue; }
            70 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 71; continue; }
            71 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x70); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbp; bb = 72; continue; }
            72 => { rdx = (crate::simd_ops::ru32(rax)) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x60u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 74; continue; } bb = 73; continue; }
            73 => { crate::simd_ops::wu64(r13.wrapping_add(0x60u64), rdi); bb = 75; continue; }
            74 => { crate::simd_ops::wu64(r15, rdi); bb = 75; continue; }
            75 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(r13.wrapping_add(0x68u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 77; continue; } bb = 76; continue; }
            76 => { crate::simd_ops::wu64(r13.wrapping_add(0x68u64), r15); bb = 77; continue; }
            77 => { rax = (crate::simd_ops::ru64(r13.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = 0xffffffff; crate::simd_ops::lk_sub_u32(r13.wrapping_add(0x84u64), (1) as u32); crate::simd_ops::wu64(r13.wrapping_add(0x70u64), rcx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 79; continue; } bb = 78; continue; }
            78 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 79; continue; }
            79 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 81; continue; }
            80 => { r15 = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 81; continue; }
            81 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x108u64))) as u32 as u64; rdx = 0x1820; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 43; continue; } bb = 82; continue; }
            82 => { r11 = rsp.wrapping_add(0xc0); rax = 0x1; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ab990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415a5c60; const P1: u64 = 0x141611730; const P2: u64 = 0x143930f90; const P3: u64 = 0x14159b3d0; const P4: u64 = 0x1415242e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; r15 = ((r9 as u32)) as u32 as u64; r12 = r8; rbp = rdx; rsi = rcx; r14 = ((r13 as u32)) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = rbp; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 4; continue; }
            4 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r8 = r12; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 20; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = 0x1820; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 20; continue; } bb = 8; continue; }
            8 => { rdx = 0x403; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r13 as u32)) as u32); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = 0x503; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), rdi); bb = 15; continue; }
            14 => { crate::simd_ops::wu64(r14, rdi); bb = 15; continue; }
            15 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsi.wrapping_add(0x40u64), r14); bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = 0xffffffff; crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x5cu64), (1) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rcx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            20 => { r14 = rdi; rdi = (crate::simd_ops::ru64(rdi)) as u64; bb = 21; continue; }
            21 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 22; continue; }
            22 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; r14 = r13; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = rbp; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 41; continue; } bb = 25; continue; }
            25 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = r12; rcx = rsi; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 41; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = 0x1820; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 41; continue; } bb = 29; continue; }
            29 => { rdx = 0x403; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((r13 as u32)) as u32); fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rdx = 0x503; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 33; continue; } bb = 31; continue; }
            31 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x70); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x60u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), rbx); bb = 36; continue; }
            35 => { crate::simd_ops::wu64(r14, rbx); bb = 36; continue; }
            36 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x68u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 37; continue; }
            37 => { crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), r14); bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = 0xffffffff; crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x84u64), (1) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x70u64), rcx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 42; continue; }
            41 => { r14 = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 42; continue; }
            42 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 43; continue; }
            43 => { r11 = rsp.wrapping_add(0xc0); rax = 0x1; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415abc00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f08; const P1: u64 = 0xffffffffffffffd5; const P2: u64 = 0x1415aa7b0; const P3: u64 = 0x8; const P4: u64 = 0xa; const P5: u64 = 0x4; const P6: u64 = 0x1415aa630;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (((rdx as u8)) as u32) as u32 as u64;
    rbp = rcx;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rdx = 0x37;
    rdi = (((r8 as u8)) as u32) as u32 as u64;
    rsi = ((r9 as u32)) as u32 as u64;
    r8 = ((rdx.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = rbp;
    crate::simd_ops::wu16(rax.wrapping_add(P3), ((rbx as u16)) as u16);
    crate::simd_ops::wu16(rax.wrapping_add(P4), ((rdi as u16)) as u16);
    crate::simd_ops::wu32(rax.wrapping_add(P5), ((rsi as u32)) as u32);
    crate::simd_ops::lk_sub_u32(rbp.wrapping_add(0xb4u64), (1) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415abc70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x24; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rbp = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = rcx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P2), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(P1), rcx); bb = 1; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdi = ((rbx.wrapping_add(P4)) as u64) as u64; rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 13; continue; }
            12 => { rdi = ((rbx.wrapping_add(P4)) as u64) as u64; bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rbp); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx, rbp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rbp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rbp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rdi, ((rbp as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415abd30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415aa010; const P1: u64 = 0x141611730; const P2: u64 = 0x143930f90; const P3: u64 = 0x14159b3d0; const P4: u64 = 0x1415242e0; const P5: u64 = 0x1415abc70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); r15 = rcx; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r13); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r15.wrapping_add(0x60u64))) as u64; r14 = r15.wrapping_add(0x60); r12 = 0; fl_l = 0; fl_r = 0; r13 = 0x503; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { r15 = 0x403; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(r14)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r14, rsi); bb = 9; continue; }
            8 => { crate::simd_ops::wu64(r12, rsi); bb = 9; continue; }
            9 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r12); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rbx, rax); rax = 0xffffffff; crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x24u64), (1) as u32); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rbx); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 14; continue; }
            14 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbp); r14 = 0x403; bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((r12 as u32)) as u32); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x34u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x70); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(r15.wrapping_add(0x38u64), rbp); bb = 24; continue; }
            23 => { crate::simd_ops::wu64(r12, rbp); bb = 24; continue; }
            24 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu64(r15.wrapping_add(0x40u64), r12); bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rdi, rax); rax = 0xffffffff; crate::simd_ops::lk_sub_u32(r15.wrapping_add(0x5cu64), (1) as u32); crate::simd_ops::wu64(r15.wrapping_add(0x48u64), rdi); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rsi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = rbp; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 29; continue; }
            29 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; r14 = r15.wrapping_add(0x60); bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x20u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = 0x10; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r15.wrapping_add(0x20u64), r12); bb = 32; continue; }
            32 => { rcx = r15.wrapping_add(0x38); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = r14; rsp = ((rsp).wrapping_add(0xc8u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415abf50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f08; const P1: u64 = 0x1415a5ba0; const P2: u64 = 0x1415ab3c0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r10 = 0x503; r8 = rdx; rdx = rcx; fl_l = (crate::simd_ops::ru16(rbp.wrapping_add(0x34u64))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 29; continue; } bb = 1; continue; }
            1 => { rax = ((r8.wrapping_sub(0x1000000u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfeffffff; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 26; continue; } bb = 2; continue; }
            2 => { rax = ((r8.wrapping_sub(0x4000000u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfbffffff; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 29; continue; } bb = 3; continue; }
            3 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (0x0u64) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x4000000; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 8; continue; }
            8 => { r14 = rbx; rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { bb = 24; continue; }
            13 => { r9 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x34u64))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 15; continue; } bb = 14; continue; }
            14 => { r9 = (crate::simd_ops::ru32(rax.wrapping_add(0xe8u64))) as u32 as u64; fl_l = (((r9 as u32)).wrapping_sub(1)) as u32 as u64; r9 = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0xe8u64), (0x0u32) as u32); bb = 15; continue; }
            15 => { rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rax); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(r14, rax); bb = 18; continue; }
            18 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), r14); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x5cu64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x48u64), rbx); bb = 24; continue; }
            21 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x34u64))) as u16 as u64; fl_r = ((r10 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rax.wrapping_add(0xe8u64))) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x34u64), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 24; continue; }
            23 => { fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x34u64), (1) as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rbp.wrapping_add(0xe8u64), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            26 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (0x0u64) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x2000000; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            29 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x88; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x1415a5ab0; const P4: u64 = 0x8; const P5: u64 = 0x34; const P6: u64 = 0x141611730; const P7: u64 = 0x50; const P8: u64 = 0x30; const P9: u64 = 0x143930f90; const P10: u64 = 0x14159b3d0; const P11: u64 = 0x14; const P12: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = rsp.wrapping_add(0x88); rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rdx.wrapping_add(P1)))) as u32 as u64; rbx = rdx; crate::simd_ops::lk_add_u32(rdx.wrapping_add(P2), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x38u64)) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = 0x403; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rax = ((crate::simd_ops::ru16(rcx.wrapping_add(P5))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = 0x503; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; rcx = (crate::simd_ops::ru64(P9)) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P11), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            8 => { rsp = ((rsp).wrapping_add(0x70u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x1439310e8; const P2: u64 = 0x14268a040; const P3: u64 = 0x1434269f8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (crate::simd_ops::ru64(P1)) as u64; r8 = (0x5851f42d4c957f2du64) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; xmm0 = (((rdx as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2pd(xmm0); rax = ((rax).wrapping_add(1)) as u64; crate::simd_ops::wu64(P1, rax); fl_l = ((rax).wrapping_shr(33u32)) as u64; rax = ((fl_l as u64)) as u64; xmm1 = (((rax as u32)) as u32) as u128; xmm1 = crate::simd_ops::vcvtdq2pd(xmm1); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(crate::simd_ops::ru64(P2))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) * f64::from_bits(xmm0 as u64)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm1 as u64) + f64::from_bits(crate::simd_ops::ru64(P3))).to_bits() as u128); r8 = (crate::simd_ops::cvt_f64_i32((f64::from_bits(xmm1 as u64)))) as u32 as u64; }
    rax = (crate::simd_ops::ru32(rcx)) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415bf090; const P1: u64 = 0x142689d30; const P2: u64 = 0x142677588; const P3: u64 = 0x8; const P4: u64 = 0x142689f98; const P5: u64 = 0x10; const P6: u64 = 0x142689fc0; const P7: u64 = 0x28;
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
    rax = P4;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rax);
    rax = P6;
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x141593c10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac340(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f90; const P1: u64 = 0x14159a550;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x70u64))) as u64; r14 = r9; rdi = r8; r15 = rdx; rbp = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { r8 = r9; rdx = rdi; rax = crate::simd_ops::call_stk(r15, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            3 => { rdx = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rax)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; rdx = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 38; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x1d8u64))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 21; continue; } bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x38u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 18; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x40u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 21; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { r8 = r14; rdx = rdi; rax = crate::simd_ops::call_stk(r15, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 22; continue; }
            22 => { bb = 38; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 30; continue; }
            30 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x18u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 32; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x20u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 32; continue; }
            32 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x20u64))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0xffu8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 34; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x28u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 37; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = rbx.wrapping_sub(0x18); r8 = r14; rdx = rdi; rax = crate::simd_ops::call_stk(r15, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ac1a0; const P1: u64 = 0x1415a0660;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(0x90u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rsi = rdx; rdi = rcx; rbp = 0xfffffffc; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r9 = ((r8 as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rdx = ((r9.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r10))) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r9 = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = r8; bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; rax = rsp.wrapping_add(0x30); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { rdx } else { rax }) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 6; continue; }
            6 => { r9 = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; bb = 7; continue; }
            7 => { rcx = ((r8.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { r8 = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 9; continue; }
            9 => { bb = 13; continue; }
            10 => { rax = ((r11.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; rax = (((rax) & (rbp))) as u64; fl_l = ((rax).wrapping_add(r10)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            11 => { rax = ((r9.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (((rax) & (rbp))) as u64; rax = ((rax).wrapping_add(rdx)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (((rbx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; bb = 13; continue; }
            13 => { fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 16; continue; }
            15 => { rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x90; const P1: u64 = 0x1; const P2: u64 = 0xd8; const P3: u64 = 0x4; const P4: u64 = 0x1415ac1a0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rdi = rcx; rsi = 0xfffffffc; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r11 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r8 = ((rdx as u32)) as u32 as u64; bb = 2; continue; }
            2 => { r9 = ((r8.wrapping_add(P1)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r9.wrapping_add(r10))) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 3; continue; }
            3 => { r8 = ((r9 as u32)) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { r8 = rdx; bb = 5; continue; }
            5 => { fl_l = r8; fl_r = r8; rax = rsp.wrapping_add(0x30); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { r8 } else { rax }) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rbx = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r9 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; bb = 7; continue; }
            7 => { rax = ((rdx.wrapping_add(P1)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r8))) as u8 as u64; fl_r = (0x3au8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 9; continue; }
            9 => { rax = (((rbx as u16)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = ((r11.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rax = (((rax) & (rsi))) as u64; fl_l = ((rax).wrapping_add(r10)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; bb = 5; continue; }
            11 => { rax = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; rax = (((rax) & (rsi))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (((rax as u32)).wrapping_add((rbx as u32))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x141594b70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = r8;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    xmm6 = (xmm1) as u128;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rdi; xmm1 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac6f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x20; const P4: u64 = 0x141594b70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = r9; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rdi = r8; xmm6 = (xmm1) as u128; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rsi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r9 = (((rsi as u8)) as u32) as u32 as u64; r8 = rbp; xmm1 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x8; const P4: u64 = 0x141594bd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = r9; rsi = ((r8 as u32)) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rsi as u32)) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x8; const P4: u64 = 0x141596380;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = r8; rsi = ((rdx as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0x141592030;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rdx;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x8; const P4: u64 = 0x258; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = ((r9 as u32)) as u32 as u64; rbp = r8; rsi = rdx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x141594f30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rdx;
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x1415920b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = r9; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rsi = ((r8 as u32)) as u32 as u64; xmm6 = (xmm1) as u128; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { r8 = ((rsi as u32)) as u32 as u64; xmm1 = (xmm6) as u128; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ac9c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0x1415924d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = (((r8 as u8)) as u32) as u32 as u64;
    rsi = (((rdx as u8)) as u32) as u32 as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { r8 = (((rdi as u8)) as u32) as u32 as u64; rdx = (((rsi as u8)) as u32) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415aca20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x70; const P1: u64 = 0x18; const P2: u64 = 0xffffffffffffffe8; const P3: u64 = 0x141596380;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rdx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415acaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1f; const P1: u64 = 0x141524330; const P2: u64 = 0x1415c64e0; const P3: u64 = 0x8; const P4: u64 = 0x141612370;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; rsi = r9; rdi = r8; rbp = rdx; r14 = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r8.wrapping_add(P0)) as u32) as u32 as u64; bb = 9; continue; }
            2 => { rdx = 0x1a8; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r10 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xc0u64))) as u32 as u64; r9 = rdi; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = r14; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rcx as u32)) as u32); rdx = rbp; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = r10; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rax)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { bb = 9; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx, (0x0u64) as u64); bb = 8; continue; }
            8 => { rax = 0x2; bb = 9; continue; }
            9 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415acbb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1415c6dd0; const P3: u64 = 0x1434268d0; const P4: u64 = 0x143426780; const P5: u64 = 0x143427068; const P6: u64 = 0x14; const P7: u64 = 0x20; const P8: u64 = 0x141613100;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); xmm6 = (xmm2) as u128; rdi = ((rdx as u32)) as u32 as u64; rsi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = rbx; rdx = rsp.wrapping_add(0x20); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r8 = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { xmm2 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm5 = 0u128; xmm3 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rax.wrapping_add(P6))) as u32 as u64; xmm1 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm3) as u128; bb = 8; continue; }
            7 => { xmm0 = (xmm4) as u128; bb = 8; continue; }
            8 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i32((f32::from_bits(xmm0 as u32)))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P6), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 9; continue; }
            9 => { r8 = ((rdi as u32)) as u32 as u64; rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415acc90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x1415c6dc0; const P3: u64 = 0x1415c7130;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r14 = (((r8 as u8)) as u32) as u32 as u64; rsi = rdx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { rbp = rbx; rdi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rcx = rbp; rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rsi; rcx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rcx = rbp.wrapping_add(0x58); rdx = 0xffffffff; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415acd20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141612300; const P1: u64 = 0x141612370; const P2: u64 = 0x1415242e0; const P3: u64 = 0x141612e10; const P4: u64 = 0x1415c6850;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; r12 = r8; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r14); r13 = rcx; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r15); r15 = rdx; rax = (crate::simd_ops::ru64(r8)) as u64; r9 = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; r11 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(0x2cu64))) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; r9 = ((r9).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = r11; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rbp = 0; fl_l = 0; fl_r = 0; r14 = rax.wrapping_add(0x18); fl_l = rax; fl_r = r9; r14 = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { r14 }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r14); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rsi); bb = 8; continue; }
            8 => { rsi = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x19cu64))) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(r12)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x40u64))) as u64; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x2cu64))) as u32 as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x2cu64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 13; continue; }
            13 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 16; continue; }
            16 => { r14 = rdi; fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rax); bb = 19; continue; }
            18 => { crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); bb = 19; continue; }
            19 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(r15)) as u64; rdi = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(r15, rbp); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(r14)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r14; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 23; continue; }
            22 => { rbp = rdi; rdi = rax; bb = 23; continue; }
            23 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 25; continue; }
            25 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = ((rbp as u32)) as u32 as u64; bb = 26; continue; }
            26 => { r9 = (crate::simd_ops::ru32(r12.wrapping_add(0x14u64))) as u32 as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = rsi; rcx = r13; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rbp as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0xb0u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x31u64), ((rdi as u8)) as u8); crate::simd_ops::wu16(rsp.wrapping_add(0x34u64), (0x40u16) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x10; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 26; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu16(r14.wrapping_add(0x10u64), ((rbp as u16)) as u16); rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 28; continue; }
            28 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 33; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x28u64))) as u8 as u64; fl_r = (0x90u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 33; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x2du64))) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if (((fl_l as u8)) <= ((fl_r as u8))) { bb = 33; continue; } bb = 31; continue; }
            31 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64) as u128; r8 = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rdx = rsi; r9 = (crate::simd_ops::ru32(r12.wrapping_add(0x14u64))) as u32 as u64; rcx = r13; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x80u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 34; continue; }
            34 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 35; continue; }
            35 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 36; continue; }
            36 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 37; continue; }
            37 => { rdi = (crate::simd_ops::ru64(r12)) as u64; rdx = rsp.wrapping_add(0x50); r9 = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; xmm0 = 0u128; r8 = (crate::simd_ops::ru32(r12.wrapping_add(0x14u64))) as u32 as u64; r9 = ((((r9 as u32)) & (0x3fffffffu32))) as u32 as u64; rcx = rdi; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); bb = 41; continue; }
            40 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 45; continue; } bb = 42; continue; }
            42 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x9u64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0x9u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 45; continue; } bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0xb0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0xcu64))) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(r12.wrapping_add(0xcu64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 46; continue; } bb = 45; continue; }
            45 => { r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32 as u64; r8 = rbx.wrapping_add(0x8); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = rdi; rcx = r13; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 47; continue; }
            47 => { rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415acff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1438152ac; const P1: u64 = 0x1438152b4; const P2: u64 = 0x143426f34; const P3: u64 = 0x14342668c; const P4: u64 = 0x1438152a8; const P5: u64 = 0x1438152b0; const P6: u64 = 0xffffffffffffffff; const P7: u64 = 0x1438152bc; const P8: u64 = 0x1438152b8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((rcx as u32)) as u32 as u64;
    xmm0 = 0u128;
    r8 = (((rdx as u16)) as u32) as u32 as u64;
    crate::simd_ops::wu32(P0, ((rcx as u32)) as u32);
    rcx = (((rcx as u32)).wrapping_shl(7u32)) as u32 as u64;
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128);
    crate::simd_ops::wu32(P1, ((rdx as u32)) as u32);
    xmm1 = (((r8 as u32)) as u32) as u128;
    xmm1 = crate::simd_ops::vcvtdq2ps(xmm1);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128);
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128);
    rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64;
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128);
    crate::simd_ops::wu32(P4, ((rax as u32)) as u32);
    rax = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64;
    crate::simd_ops::wu32(P5, ((rax as u32)) as u32);
    rax = 0x57619f1;
    let prod = (rax as u32 as u64).wrapping_mul(((rcx as u32)) as u32 as u64);
    rax = prod as u32 as u64; rdx = (prod >> 32) as u32 as u64; fl_l = (rdx != 0) as u64;
    rax = ((r8.wrapping_add(P6)) as u32) as u32 as u64;
    rcx = ((rdx as u32)) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = (((rcx as u32)).wrapping_shr(10u32)) as u32 as u64;
    rcx = (((rcx as u32)).wrapping_add(0x7u32)) as u32 as u64;
    rcx = ((((rcx as u32)) & (0xfffffff8u32))) as u32 as u64;
    let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64);
    if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; }
    crate::simd_ops::wu32(P7, ((rcx as u32)) as u32);
    fl_l = (((rax as u16)).wrapping_add(1)) as u16 as u64;
    rax = (rax & !0xffffu64) | ((((fl_l as u16)) as u16 as u64));
    crate::simd_ops::wu16(P8, ((rax as u16)) as u16);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ad070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x28; const P2: u64 = 0x30; const P3: u64 = 0x3c; const P4: u64 = 0x20; const P5: u64 = 0x44;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::lk_and_u8(rcx.wrapping_add(P0), (0xfcu8) as u8);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx, (0x20000000u64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ad0e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ad4d0; const P1: u64 = 0x1434268d0; const P2: u64 = 0x14267648c; const P3: u64 = 0x143426e1c; const P4: u64 = 0x143426eb8; const P5: u64 = 0x14267664c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x1cu64))) as u32 as u64; rdi = rcx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rsi = (rsi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rax = ((rcx.wrapping_add(r8)) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm6 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32) as u128; rdx = rdi.wrapping_add(0x10); rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 10; continue; }
            2 => { rbx = ((crate::simd_ops::ru8(rdi.wrapping_add(0x40u64))) as u32) as u32 as u64; xmm3 = 0u128; fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x44u64))) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm6 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32) as u128; rdx = rdi.wrapping_add(0x10); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x14u64)))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x14u64)))).to_bits() as u128); bb = 8; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm3 = 0u128; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r8) as i64) as f32).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((xmm3 as u32)) as u32); xmm0 = (xmm3) as u128; r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0x38u64))) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (xmm0) as u128; rdx = rdi.wrapping_add(0x10); bb = 8; continue; }
            8 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm6 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(23u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; xmm0 = (((rcx as u32)) as u32) as u128; xmm6 = (xmm0) as u128; rcx = (((rax as u8)) as u32) as u32 as u64; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm0) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) - f32::from_bits(crate::simd_ops::ru32(P1))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu32(rdx, ((xmm6 as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = (((rsi as u8)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; xmm2 = (xmm6) as u128; r8 = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (((rsi as u8)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ad260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426564; const P1: u64 = 0x142673708; const P2: u64 = 0x1426736fc; const P3: u64 = 0x142673700; const P4: u64 = 0x1426736ec; const P5: u64 = 0x1426736e8; const P6: u64 = 0x1426736f0; const P7: u64 = 0x1438152a8; const P8: u64 = 0x1438152b4; const P9: u64 = 0x1438152ac; const P10: u64 = 0x14268a058; const P11: u64 = 0x14183ad58; const P12: u64 = 0x14153e940;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rdi = rdx; r9 = ((crate::simd_ops::ru8(rcx.wrapping_add(0x40u64))) as u32) as u32 as u64; rbx = rcx; crate::simd_ops::wu64(rcx, rax); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) ^ (crate::simd_ops::ru8(rdx.wrapping_add(0x24u64))))) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) & (0x1u8))) as u8 as u64)); r9 = (r9 & !0xffu64) | ((((((r9 as u8)) ^ (crate::simd_ops::ru8(rcx.wrapping_add(0x40u64))))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), ((r9 as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x26u64))) as u32) as u32 as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_add((rax as u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((r9 as u8)))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x2u8))) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) ^ ((r9 as u8)))) as u8 as u64)); crate::simd_ops::wu8(rcx.wrapping_add(0x40u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x10u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x24u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm2 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32) as u128; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 1; continue; }
            1 => { xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm5 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm2 = 0u128; bb = 4; continue; }
            3 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm2 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; xmm2 = (xmm0) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)))).to_bits() as u128); bb = 4; continue; }
            4 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm1 = 0u128; bb = 7; continue; }
            6 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm1 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32) as u128; xmm1 = (xmm0) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)))).to_bits() as u128); bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm1 as u32)) as u32); fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x25u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x3u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0x8; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 12; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rax as u32)) as u32); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((r8 as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(P7))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = (crate::simd_ops::ru32(P7)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x2u8) as u8 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((rax as u32)) as u32); if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((crate::simd_ops::ru16(P8)) as u32) as u32 as u64; xmm1 = 0u128; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; xmm2 = (((rcx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vcvtdq2ps(xmm2); xmm0 = (((rax as u32)) as u32) as u128; rax = (crate::simd_ops::ru32(P9)) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rdi = (crate::simd_ops::ru64(rdi)) as u64; rcx = ((rax.wrapping_add(rbp.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 18; continue; }
            18 => { fl_l = rax; fl_r = rcx; bb = 19; continue; }
            19 => { rsi = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 25; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = rbx.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x34u64))) as u32 as u64; fl_l = rbp; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rdx, rdi); bb = 25; continue; }
            25 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0x2; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ad4d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140000000; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143426780; const P3: u64 = 0x142676620; const P4: u64 = 0x14268a064; const P5: u64 = 0x142676610; const P6: u64 = 0x14267663c; const P7: u64 = 0x142676648; const P8: u64 = 0x143426bf8; const P9: u64 = 0x14268a060; const P10: u64 = 0x14342698c; const P11: u64 = 0x14268a05c; const P12: u64 = 0x143426c2c; const P13: u64 = 0x142676644; const P14: u64 = 0x142676634; const P15: u64 = 0x14268a068; const P16: u64 = 0x142676608; const P17: u64 = 0x142676618; const P18: u64 = 0x142676630; const P19: u64 = 0x14267660c; const P20: u64 = 0x142676614; const P21: u64 = 0x142676604; const P22: u64 = 0x142676624; const P23: u64 = 0x142676640;
    let (mut rax, mut fl_l, mut fl_r, mut j_tgt) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); xmm5 = (xmm2) as u128; crate::simd_ops::wu128(rsp, (xmm7) as u128); xmm7 = (xmm1) as u128; xmm6 = (xmm0) as u128; fl_l = ((r9 as u32)) as u32 as u64; fl_r = (0x8u32) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 15; continue; } bb = 1; continue; }
            1 => { rdx = ((P0) as u64) as u64; rax = ((((r9 as u32)) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x15ad800u64))) as u32 as u64; fl_l = ((rcx).wrapping_add(rdx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; j_tgt = (rcx) as u64; bb = 0xffff_fffe; continue; }
            2 => { xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm5) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((P1) as *const u32))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(core::ptr::read_unaligned((P2) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(core::ptr::read_unaligned((P4) as *const u32))).to_bits() as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P5) as *const u32))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(core::ptr::read_unaligned((P6) as *const u32))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(core::ptr::read_unaligned((P7) as *const u32))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm5) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(core::ptr::read_unaligned((P8) as *const u32))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(core::ptr::read_unaligned((P2) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { xmm2 = ((crate::simd_ops::ru32(P9)) as u32) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm3 = (xmm6) as u128; xmm4 = (xmm6) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(core::ptr::read_unaligned((P10) as *const u32))).to_bits() as u128); xmm0 = (xmm3) as u128; xmm1 = (xmm3) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P11) as *const u32))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P12) as *const u32))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P13)) as u32) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(core::ptr::read_unaligned((P14) as *const u32))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm4) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            8 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm7) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(core::ptr::read_unaligned((P15) as *const u32))).to_bits() as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P16) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(core::ptr::read_unaligned((P17) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((P18) as *const u32))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((P19) as *const u32))).to_bits() as u128); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm0 = (xmm1) as u128; bb = 11; continue; }
            11 => { xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm5) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P2)) as u32 as u64; xmm3 = ((crate::simd_ops::ru32(P12)) as u32) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm2 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { xmm1 = ((crate::simd_ops::ru32(P20)) as u32) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(core::ptr::read_unaligned((P15) as *const u32))).to_bits() as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P21) as *const u32))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(core::ptr::read_unaligned((P22) as *const u32))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(core::ptr::read_unaligned((P23) as *const u32))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = ((crate::simd_ops::ru32(P20)) as u32) as u128; xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm1) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(core::ptr::read_unaligned((P21) as *const u32))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(core::ptr::read_unaligned((P22) as *const u32))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(core::ptr::read_unaligned((P23) as *const u32))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm0 = 0u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; rsp = ((rsp).wrapping_add(0x28u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            0xffff_fffe => { match j_tgt.wrapping_sub(0x1415ad4d0u64) { 0x20 => { bb = 1; continue; } 0x36 => { bb = 2; continue; } 0x53 => { bb = 3; continue; } 0x81 => { bb = 4; continue; } 0xd9 => { bb = 5; continue; } 0xfe => { bb = 6; continue; } 0x12c => { bb = 7; continue; } 0x1b3 => { bb = 8; continue; } 0x1e4 => { bb = 9; continue; } 0x228 => { bb = 10; continue; } 0x22b => { bb = 11; continue; } 0x248 => { bb = 12; continue; } 0x264 => { bb = 13; continue; } 0x2bc => { bb = 14; continue; } 0x31c => { bb = 15; continue; } _ => { let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(j_tgt, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; } } }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ad830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = ((rax).wrapping_sub(0x2000000u64)) as u64;
    fl_l = rax; fl_r = (0xfffffffffdffffffu64) as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ad850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x1438152b4; const P2: u64 = 0x1438152ac; const P3: u64 = 0x14268a058; const P4: u64 = 0x14183ad58; const P5: u64 = 0x44;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x2u8) as u8 as u64;
    rbx = rcx;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; xmm1 = 0u128; rax = (0u32.wrapping_sub((rax as u32))) as u32 as u64; xmm2 = (((rdx as u32)) as u32) as u128; xmm2 = crate::simd_ops::vcvtdq2ps(xmm2); xmm0 = (((rax as u32)) as u32) as u128; rax = (crate::simd_ops::ru32(P2)) as u32 as u64; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm2 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((xmm0 as u32)) as u32); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ad8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x141524330; const P2: u64 = 0x1415ad070; const P3: u64 = 0x143930f08; const P4: u64 = 0x88; const P5: u64 = 0x1415ad260; const P6: u64 = 0x8; const P7: u64 = 0xc; const P8: u64 = 0x14153e940; const P9: u64 = 0x1; const P10: u64 = 0x1415ad0a0; const P11: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = ((rcx.wrapping_add(P0)) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x1; rsi = rdx; rdx = 0x50; rdi = (if (((fl_l as u32)) != ((fl_r as u32))) { rcx } else { rdi }) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rsi; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P7))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P7))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; rax = ((rcx.wrapping_add(P9)) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rdx, rbx); rax = rbx; bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ad990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x14153e940; const P4: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rbx = ((rdx.wrapping_add(P0)) as u64) as u64; rsi = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rdi = r8; rcx = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { rax } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rax = ((rcx.wrapping_add(P4)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P1), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rdx, rdi); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x2; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ada30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ad850; const P1: u64 = 0x143426564; const P2: u64 = 0x142673708; const P3: u64 = 0x1426736fc; const P4: u64 = 0x142673700; const P5: u64 = 0x1426736ec; const P6: u64 = 0x1426736e8; const P7: u64 = 0x1426736f0; const P8: u64 = 0x1434268d0; const P9: u64 = 0x14267648c; const P10: u64 = 0x143426e1c; const P11: u64 = 0x143426eb8; const P12: u64 = 0x14267664c; const P13: u64 = 0x143930f08; const P14: u64 = 0x1438152a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (crate::simd_ops::ru64(rdx)) as u64; rbx = rdx; rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = rbx; crate::simd_ops::wu64(rdx, r8); rdi = r8; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rdx = ((rsi as u32)) as u32 as u64; xmm6 = (xmm3) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; xmm5 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm4 = 0u128; xmm1 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm3 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm5 = 0u128; bb = 4; continue; }
            3 => { xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm5 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm5 = (xmm0) as u128; xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) + f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x48u64)))).to_bits() as u128); bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; fl_r = 0x2; xmm0 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((xmm5 as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm4 = (xmm0) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x48u64)))).to_bits() as u128); bb = 7; continue; }
            7 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((xmm4 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm4 as u32)) as u32); rcx = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_shr(23u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; xmm0 = (((rcx as u32)) as u32) as u128; xmm3 = (xmm0) as u128; rcx = (((rax as u8)) as u32) as u32 as u64; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm0) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P9))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rcx) as i64) as f32).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(crate::simd_ops::ru32(P11))).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(crate::simd_ops::ru32(P12))).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((xmm3 as u32)) as u32); bb = 14; continue; }
            8 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((xmm6 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(P4))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm0 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm4 = (xmm0) as u128; xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x48u64)))).to_bits() as u128); bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm4 as u32)) as u32); bb = 14; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; fl_r = 0x2; xmm4 = (xmm6) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((xmm5 as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm4 = (xmm4 & !0xffff_ffffu128) | ((f32::from_bits(xmm4 as u32) + f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((xmm4 as u32)) as u32); bb = 14; continue; }
            13 => { crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((xmm6 as u32)) as u32); bb = 14; continue; }
            14 => { fl_l = ((xmm5 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = ((rcx.wrapping_sub(0x3u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xfffffffd; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = 0x8; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 18; continue; }
            17 => { rax = ((rcx as u32)) as u32 as u64; bb = 18; continue; }
            18 => { crate::simd_ops::wu32(rbx.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(P13)) as u64; rcx = (crate::simd_ops::ru32(P14)) as u32 as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x88u64))) as u32 as u64; rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add((rsi as u32))) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; let d = ((rcx as u32)) as i32 as i64; let n = (((rdx as u32 as u64) << 32) | (rax as u32 as u64)) as i64; if d != 0 { rax = n.wrapping_div(d) as i32 as u32 as u64; rdx = n.wrapping_rem(d) as i32 as u32 as u64; } fl_l = rbp; fl_r = (crate::simd_ops::ru64(rbx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = rdi; fl_r = (0xf000000u64) as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((r8 as u32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(crate::simd_ops::ru32(rbx.wrapping_add(0x18u64)))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) < ((fl_r as u32))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; bb = 22; continue; }
            21 => { rcx = ((rax as u32)) as u32 as u64; bb = 22; continue; }
            22 => { crate::simd_ops::wu32(rbx.wrapping_add(0x1cu64), ((rcx as u32)) as u32); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((r8 as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415add10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x1u32) as u32); return rax; }
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x2u32) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415add30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = rcx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x1u32) as u32); bb = 7; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x2u32) as u32); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415add90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc8; const P1: u64 = 0x1415ad0a0; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1415ad0e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r12 = 0; fl_l = 0; fl_r = 0; rsi = r8; r8 = (crate::simd_ops::ru32(r8.wrapping_add(0x8u64))) as u32 as u64; r15 = ((rdx as u32)) as u32 as u64; rdi = ((r12 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rbp = ((r12.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rdi as u32)) as u32 as u64; r14 = ((rax.wrapping_mul(8)) as u64) as u64; rcx = ((rcx).wrapping_add(r14)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = (((r8 as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rdx = rcx.wrapping_add(0x8); r8 = ((r8.wrapping_mul(8).wrapping_sub(0x8u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            6 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { bb = 11; continue; }
            10 => { rax = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), ((r12 as u32)) as u32); rax = (((rax as u32)).wrapping_sub(crate::simd_ops::ru32(rbx.wrapping_add(0x24u64)))) as u32 as u64; fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32)) as u32 as u64; fl_r = fl_l; bb = 11; continue; }
            11 => { rdx = ((r15 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rcx).wrapping_add(r14)) as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rdx = rcx.wrapping_add(0x8); r8 = ((rax.wrapping_mul(8).wrapping_sub(0x8u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x8u64), (1) as u32); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            15 => { crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), (0x2u32) as u32); bb = 16; continue; }
            16 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbp = ((fl_l as u32)) as u32 as u64; bb = 17; continue; }
            17 => { r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 18; continue; }
            18 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415adec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415add90; const P1: u64 = 0x10;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    r8 = rcx;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = ((rdi.wrapping_add(P1)) as u64) as u64;
    rdx = ((rbx as u32)) as u32 as u64;
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415adef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x28; const P1: u64 = 0x30; const P2: u64 = 0xfffffffffffffff8;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r10 = rdx; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; r9 = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = 0x2; return rax; }
            4 => { rdx = ((r9.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rcx, rdx); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::lk_sub_u32(r10.wrapping_add(P1), (1) as u32)) as u32 as u64; rax = 0x1; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415adf30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x3u32) as u32); return rax; }
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { crate::simd_ops::wu32(rdx.wrapping_add(P0), (0x0u32) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415adf50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x10; const P2: u64 = 0x3c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = rcx; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 2; continue; }
            2 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x3u32) as u32); bb = 7; continue; }
            5 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r8 as u32)) as u32); bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(r9.wrapping_add(P0))) as u32 as u64; rax = ((rax).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 8; continue; }
            8 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415adfb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415adfe0; const P1: u64 = 0x10;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = rcx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415adfe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1c; const P2: u64 = 0x18; const P3: u64 = 0x1415ad0e0; const P4: u64 = 0x1415ad0a0; const P5: u64 = 0x1415242e0; const P6: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rsi = rdx; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(crate::simd_ops::ru32(rdi.wrapping_add(P2)))) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rbx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P0), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi, rbx); crate::simd_ops::wu32(rsi.wrapping_add(P6), ((rbx as u32)) as u32); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426fd0; const P1: u64 = 0x141529ed0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r12 = r8; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r9 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r10 = ((rbx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0xccu64))) as u32) as u32 as u64; r8 = ((r9 as u32)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(32u32)) as u64; fl_l = (((r8) | (rax))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rax))) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r15); r15 = r9; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm8) as u128); xmm8 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), r14); r15 = ((r15).wrapping_shl(32u32)) as u64; rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); fl_l = (((r15) | (rax))) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); bb = 6; continue; }
            6 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4u64))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rcx = (((rcx) | (rax))) as u64; fl_l = rcx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; xmm6 = ((crate::simd_ops::ru32(r12.wrapping_add(0xcu64))) as u32) as u128; crate::simd_ops::wu16(rsp.wrapping_add(0x3cu64), (0x2u16) as u16); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x0u32) as u32); fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x4u64))) as u32 as u64; rsi = rdi.wrapping_add(0x38); r13 = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm7 = ((crate::simd_ops::ru32(rsi)) as u32) as u128; rax = rdi.wrapping_add(0x3c); bb = 11; continue; }
            9 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 13; continue; } bb = 10; continue; }
            10 => { xmm7 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x3cu64))) as u32) as u128; rax = rdi.wrapping_add(0x3c); bb = 11; continue; }
            11 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 13; continue; } bb = 12; continue; }
            12 => { r14 = rdi.wrapping_add(0x18); bb = 17; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; r14 = rdi.wrapping_add(0x18); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; r8 = rsp.wrapping_add(0xd8); xmm1 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0xd8u64), ((rax as u32)) as u32); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32 as u64; xmm7 = (xmm0) as u128; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 16; continue; }
            15 => { xmm7 = 0u128; bb = 16; continue; }
            16 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x30u64), ((xmm6 as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x38u64), ((xmm7 as u32)) as u32); rax = rdi.wrapping_add(0x3c); fl_l = (crate::simd_ops::lk_xor_u8(rdi.wrapping_add(0x14u64), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 17; continue; }
            17 => { xmm6 = ((crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32) as u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 20; continue; } bb = 18; continue; }
            18 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x34u64))) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 21; continue; } bb = 19; continue; }
            19 => { rsi = rax; bb = 20; continue; }
            20 => { xmm1 = ((crate::simd_ops::ru32(rsi)) as u32) as u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 25; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(r14)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0x10u64))) as u32 as u64; r8 = rsp.wrapping_add(0x30); xmm1 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; xmm1 = (xmm0) as u128; crate::simd_ops::wu32(r14.wrapping_add(0x10u64), ((rax as u32)) as u32); bb = 24; continue; }
            23 => { xmm1 = 0u128; bb = 24; continue; }
            24 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x30u64), ((xmm6 as u32)) as u32); rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x14u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x38u64), ((xmm1 as u32)) as u32); fl_l = (crate::simd_ops::lk_xor_u8(rdi.wrapping_add(0x14u64), (0x1u8) as u8)) as u8 as u64; fl_r = fl_l; bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(r12)) as u64; rdx = rsp.wrapping_add(0x38); xmm0 = (xmm1) as u128; r9 = ((r13 as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm7 as u32)).to_bits() as u128); r8 = ((r13 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rbx = ((rbx).wrapping_add(0x18u64)) as u64; rax = (crate::simd_ops::ru32(r11.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(r11)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 26; continue; }
            26 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; bb = 27; continue; }
            27 => { rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae300(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14268a0e0; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x8; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), r8);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((r8 as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0xffu8) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), (0xffu8) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P6), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), r8);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P9), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), r8);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), r8);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x8u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x18u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rcx.wrapping_add(0x28u64), (xmm0) as u128); }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a0e0; const P1: u64 = 0x50; const P2: u64 = 0x1439316b0; const P3: u64 = 0x1415ae980; const P4: u64 = 0x1415a3ab0; const P5: u64 = 0x58; const P6: u64 = 0x60; const P7: u64 = 0x1415242e0; const P8: u64 = 0x64; const P9: u64 = 0x1415cc8f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0xffffffffffffffffu64) as u64);
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rdi;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = rcx; rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r8 = rsp.wrapping_add(0x30); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rdi as u32)) as u32); }
    rcx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426890; const P1: u64 = 0x1415aebd0; const P2: u64 = 0x141524330;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = rdx; rbx = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; rdi = rcx; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 22; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; xmm1 = 0u128; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((r9) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((r9 as u32)) as u32 as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 10; continue; }
            10 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbx as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi)) as u64; rbp = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 13; continue; }
            13 => { bb = 16; continue; }
            14 => { fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 6; continue; }
            15 => { fl_l = ((rcx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) != 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { rdx = 0x20; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 21; continue; }
            18 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rbp.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rdx, rcx); rcx = rdx.wrapping_add(0x10); crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rbx as u32)) as u32); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), (0x0u64) as u64); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rbp.wrapping_mul(8)), rdx); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            21 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 26; continue; }
            24 => { crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), rax); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rsi); bb = 26; continue; }
            25 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd; const P1: u64 = 0x8; const P2: u64 = 0x1415d9480; const P3: u64 = 0x1415cca50; const P4: u64 = 0x58; const P5: u64 = 0x1415c2db0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rbx = rdx; rdi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdx)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P0))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x3cu64), ((rax as u8)) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ae810; const P1: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r13 = 0; fl_l = 0; fl_r = 0; r9 = (crate::simd_ops::ru64(rcx)) as u64; r12 = rdx; rsi = rcx; r10 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r11 = ((r13 as u32)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = ((r8.wrapping_add(r11)) as u32) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru64(r9.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = r10; r8 = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_r = r10; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = ((((r8 as u32)) as i32 as i64) as u64) as u64; rbx = ((rbx).wrapping_shl(4u32)) as u64; fl_l = ((rbx).wrapping_add(r9)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 6; continue; }
            5 => { rbx = r9; bb = 6; continue; }
            6 => { r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = rbx; rcx = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r15); rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rax = rsi.wrapping_add(0xc); rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r15 = rax; bb = 14; continue; }
            12 => { rcx = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rax, rcx); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 32; continue; }
            13 => { rbp = (rbp & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = rsi.wrapping_add(0xc); r15 = rax; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 14; continue; }
            14 => { rbx = ((rbx).wrapping_sub(r9)) as u64; rbx = ((((rbx) as i64).wrapping_shr(4u32) as u64)) as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 16; continue; }
            16 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 31; continue; } bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r15 = ((rbx as u32)) as u32 as u64; r15 = ((r15).wrapping_shl(4u32)) as u64; r14 = ((r14).wrapping_add(r15)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (((rax as u32)).wrapping_sub((rbx as u32))) as u32 as u64; rcx = r14.wrapping_add(0x10); rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; rdx = r14; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { crate::simd_ops::wu64(r14, r13); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), r13); bb = 21; continue; }
            21 => { crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x8u64), (1) as u32); rcx = (crate::simd_ops::ru64(rsi)) as u64; fl_l = ((rcx).wrapping_add(r15)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 31; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(r12.wrapping_add(0x8u64))) as u64; bb = 30; continue; }
            23 => { xmm0 = (crate::simd_ops::ru128(r12)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 25; continue; }
            25 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r15)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 31; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { crate::simd_ops::wu64(rcx, r13); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r13); bb = 28; continue; }
            28 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 30; continue; }
            30 => { rbp = (rbp & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); bb = 31; continue; }
            31 => { rax = (((rbp as u8)) as u32) as u32 as u64; bb = 32; continue; }
            32 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); r9 = (crate::simd_ops::ru64(rcx)) as u64; rbx = r8; r10 = rdx; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = ((rcx.wrapping_add(r11)) as u32) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rdx)) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rbx; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rbx; rax = (if (((fl_l as u64)) >= ((fl_r as u64))) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; r11 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(r9)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r10, rax); rax = r10; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; return rax; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; rax = r10; crate::simd_ops::wu64(rdx, r9); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524340; const P1: u64 = 0x141524330; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 13; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = rbx; r9 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax, r9); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r9); bb = 10; continue; }
            10 => { rdx = rax; rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rax, rcx); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415ae070;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32) as u128;
    r8 = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx);
    xmm1 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32);
    rcx = ((rcx).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), ((xmm3 as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415a13a0; const P1: u64 = 0x50; const P2: u64 = 0x1439316b0; const P3: u64 = 0x1415ae430;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = rcx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi); rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415ae980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x1415aed70;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; r8 = rdx; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x50u64))) as u32 as u64; rsi = 0x2; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 28; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); rax = ((rcx as u32)) as u32 as u64; r10 = 0; fl_l = 0; fl_r = 0; let d = ((r9 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((r10 as u32)) as u32 as u64; r14 = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = rdx; rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = rbx; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r10 = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { bb = 16; continue; }
            11 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rcx); bb = 15; continue; }
            14 => { crate::simd_ops::wu64(r10.wrapping_add(0x48u64), rcx); bb = 15; continue; }
            15 => { rsi = 0x1; bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 17; continue; }
            17 => { r11 = (crate::simd_ops::ru64(rdx)) as u64; r8 = ((r14 as u32)) as u32 as u64; fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { r10 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; bb = 19; continue; }
            19 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 21; continue; }
            21 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { crate::simd_ops::wu64(r9, r11); bb = 24; continue; }
            23 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r14.wrapping_mul(8)), r11); bb = 24; continue; }
            24 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x10u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 27; continue; }
            27 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415aeaa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415c3470; const P1: u64 = 0xd; const P2: u64 = 0x1415ccc10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x1u8) as u8);
    rcx = ((rcx).wrapping_add(0x58u64)) as u64;
    r8 = rsp.wrapping_add(0x30);
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P1))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x44u64), ((rax as u8)) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415aeb00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = (crate::simd_ops::ru64(rcx)) as u64; r11 = rdx; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = rcx; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = ((rdi.wrapping_add(r8)) as u32) as u32 as u64; rdx = (if (rax as u32 as i32) < 0 { 0xffffffffu32 } else { 0u32 }) as u64; rax = (((rax as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rax = (((((rax as u32)) as i32).wrapping_shr(1u32) as u32)) as u32 as u64; rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = rdx; fl_r = r9; rcx = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rdi as u32) } else { (rcx as u32) }) as u32 as u64; r8 = (if (((fl_l as u64)) >= ((fl_r as u64))) { (rax as u32) } else { (r8 as u32) }) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((((r8 as u32)) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = ((rcx).wrapping_add(r10)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 6; continue; }
            5 => { rcx = r10; bb = 6; continue; }
            6 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r10)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = rdx; fl_r = r11; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rax = ((r8.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r10)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = ((rax).wrapping_sub(rcx)) as u64; rdx = rcx.wrapping_add(0x10); rax = ((((rax as u32)) & (0xfffffff0u32))) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415aed50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x1439316b0; const P2: u64 = 0x1415ae430;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx); rdx = rcx; rcx = P1; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415aed70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415242e0; const P2: u64 = 0x10; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); rdi = rcx; rsi = ((rbp as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rsi as u32)) as u32 as u64; r14 = ((rax.wrapping_mul(8)) as u64) as u64; rdx = (crate::simd_ops::ru64(r14.wrapping_add(rdx))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(rax), rbp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 5; continue; }
            5 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((rbp as u32)) as u32); rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rbp); crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rbp as u32)) as u32); bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415aee10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x14159dc40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((r8 as u32)) as u32 as u64; fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; if ((fl_l as u64) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (((rax) >> (((rcx) as u32 & 63))) & 1) as u64; rax = ((rax) & !((1u64) << (((rcx) as u32 & 63)))) as u64; crate::simd_ops::wu64(rdx, rax); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r9.wrapping_add(P0))) as u64; r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rcx = r9; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415aee60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439316b8; const P1: u64 = 0x1439316b0; const P2: u64 = 0x8; const P3: u64 = 0x50; const P4: u64 = 0x1415ae980; const P5: u64 = 0x1415a3ab0; const P6: u64 = 0x48; const P7: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r8 = (crate::simd_ops::ru64(P0)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); let d = ((r8 as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(P1)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = rdi; bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0xffffffffffffffffu64) as u64); fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = rbx; rcx = P1; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; r8 = rsp.wrapping_add(0x38); r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 12; continue; }
            12 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            13 => { rbx = ((rax.wrapping_add(P7)) as u64) as u64; bb = 5; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415aef20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x1439316b0; const P2: u64 = 0x1415ae980; const P3: u64 = 0x1415a3ab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    rdi = (((r8 as u8)) as u32) as u32 as u64;
    rsi = rdx;
    rbx = rcx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = rcx; rcx = P1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r9 = (((rdi as u8)) as u32) as u32 as u64; r8 = rsi; rdx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415aef80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1439316b8; const P1: u64 = 0x1439316b0; const P2: u64 = 0x18; const P3: u64 = 0x40; const P4: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdi = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(P0)) as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rdi.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 16; continue; }
            16 => { bb = 8; continue; }
            17 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 19; continue; }
            19 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 20; continue; }
            20 => { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x60; const P1: u64 = 0x58; const P2: u64 = 0x10; const P3: u64 = 0x1415c33f0; const P4: u64 = 0x1415ccc10; const P5: u64 = 0x38; const P6: u64 = 0x30; const P7: u64 = 0x50; const P8: u64 = 0x1415a13a0; const P9: u64 = 0x1439316b0; const P10: u64 = 0x1415ae430;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r14 = rcx; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rax = ((((rdi as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rsi).wrapping_shl(3u32)) as u64; rsi = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(rsi).wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rsi))) as u64; r8 = rsp.wrapping_add(0x40); rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x1u8) as u8); rcx = ((r14.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = rbx; rcx = r14; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsi = ((rsi).wrapping_sub(0x18u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru64(r14)) as u64; rdx = rsp.wrapping_add(0x40); rcx = r14; rdi = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P6))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(r14.wrapping_add(P7))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r9 = (((rdi as u8)) as u32) as u32 as u64; r8 = rax; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(r14.wrapping_add(P7))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = r14; crate::simd_ops::wu64(r14.wrapping_add(P7), rbx); rcx = P9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x8; const P2: u64 = 0x20; const P3: u64 = 0x48;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    r10 = ((r8 as u32)) as u32 as u64;
    fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x5;
    if (((fl_l as u32)) != ((fl_r as u32))) { xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; r9 = ((r10 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm3 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = r9; fl_r = r9;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; r11 = 0x2b; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(P2))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; r8 = (if (((fl_l as u32)) != ((fl_r as u32))) { (r11 as u32) } else { (r8 as u32) }) as u32 as u64; }
    xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128;
    r9 = ((r10 as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm3 as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x60; const P2: u64 = 0x14268a130; const P3: u64 = 0x64; const P4: u64 = 0x8; const P5: u64 = 0x18; const P6: u64 = 0x28; const P7: u64 = 0x38; const P8: u64 = 0x48;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x1u32) as u32);
    rax = P2;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x1u8) as u8);
    crate::simd_ops::wu128(rcx.wrapping_add(P4), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P6), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P7), (xmm0) as u128);
    crate::simd_ops::wu128(rcx.wrapping_add(P8), (xmm0) as u128);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a130; const P1: u64 = 0x8; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rsi = 0xa; crate::simd_ops::wu64(rcx, rax); rbx = ((rcx.wrapping_add(P1)) as u64) as u64; rbp = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x4; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, rbp); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a130; const P1: u64 = 0x1415242e0; const P2: u64 = 0x141834ee0; const P3: u64 = 0x1415af1b0; const P4: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((rdx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r12 = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); rbp = (crate::simd_ops::ru64(rcx.wrapping_sub(0x8u64))) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), r14); r14 = (((rbp) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), r15); r15 = rcx.wrapping_sub(0x8); r14 = ((r14).wrapping_add(rcx)) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); r12 = P0; crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); crate::simd_ops::wu64(rax.wrapping_sub(0x18u64), r13); r13 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { r14 = r14.wrapping_sub(0x68); fl_l = ((rbp).wrapping_sub(1)) as u64; rbp = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r14, r12); rbx = r14.wrapping_add(0x8); rsi = 0xa; bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; rcx = 0x4; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, r13); bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 8; continue; }
            8 => { r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 9; continue; }
            9 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = (((crate::simd_ops::ru64(r15)) as i64).wrapping_mul((0x68u64) as i64) as u64) as u64; rcx = r15; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rax = r15; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r12 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = 0x68; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = rbx; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x60; const P12: u64 = 0x1426452b8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rdi);
    crate::simd_ops::wu16(rcx.wrapping_add(P6), ((rdi as u16)) as u16);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P10), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P11), rdi);
    rcx = ((rcx).wrapping_add(0x68u64)) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(0x90u64), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(0x98u64), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(0xa0u64), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(0xa8u64), rdi);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a138; const P1: u64 = 0x20; const P2: u64 = 0x9; const P3: u64 = 0x2; const P4: u64 = 0x143930f80; const P5: u64 = 0x1415adef0; const P6: u64 = 0x18; const P7: u64 = 0x10; const P8: u64 = 0x28; const P9: u64 = 0x30; const P10: u64 = 0x1415242e0; const P11: u64 = 0x34;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdi = rcx; crate::simd_ops::wu64(rcx, rax); r14 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbp); rbp = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; rbx = ((rax.wrapping_mul(2).wrapping_add(P2)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rbp = ((rbp).wrapping_add(1)) as u64; rsi = ((rcx.wrapping_add(P3)) as u64) as u64; rax = 0xfffffff8; rbx = (((rbx) & (rax))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rbp = ((rcx.wrapping_add(rbp.wrapping_mul(2))) as u64) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx).wrapping_add(rcx)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P4)) as u64; r8 = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx, r14); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsi = ((rsi).wrapping_add(0x2u64)) as u64; rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P8))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P8), r14); crate::simd_ops::wu32(rdi.wrapping_add(P11), ((r14 as u32)) as u32); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), r14); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af4b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x68u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af4c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x141540670; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((rdx as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = rcx; r8 = ((r8).wrapping_shl(4u32)) as u64; rbp = 0x1; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { r15 = 0; fl_l = 0; fl_r = 0; rcx = rax.wrapping_add(0x8); fl_l = rax; fl_r = r8; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rbp as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdx = 0x28; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rcx = ((rdx.wrapping_sub(0x26u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu32(rax, ((r15 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), (0x2ca33bdbu32) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), r15); rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 11; continue; }
            11 => { fl_l = rax; fl_r = rcx; rdx = rax.wrapping_add(0x8); rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r15 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rdx, rbx); bb = 23; continue; }
            13 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rsi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu32(rax, ((rdi as u32)) as u32); fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax, rbx); if (((fl_l as u64)) != 0) { bb = 21; continue; } bb = 18; continue; }
            18 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rbp = 0x2; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((r15 as u32)) as u32); bb = 20; continue; }
            20 => { rdx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r15); rcx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 21; continue; }
            21 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rbp as u32)) as u32 as u64; bb = 23; continue; }
            22 => { rax = 0x2; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r9 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rax.wrapping_add(P1), r8); rax = 0x1; crate::simd_ops::wu64(r8.wrapping_add(P0), r9); return rax; }
            6 => { crate::simd_ops::wu64(r8.wrapping_add(P0), rcx); crate::simd_ops::wu64(rax.wrapping_add(P1), r8); rax = 0x1; return rax; }
            7 => { rax = 0x14; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b0030;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = ((r9 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rsi = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; rbx = rax.wrapping_add(0x8); fl_l = rax; fl_r = rcx; rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x50); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdi as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x58u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x50); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); rcx = ((rcx).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rsi as u32)) as u32); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; rcx = 0x1; rdx = 0x34; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { (rdx as u32) } else { (rcx as u32) }) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = 0x34; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x14; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((r8.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x14u64))) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r8 = ((r8).wrapping_add(0xcu64)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = r8; fl_r = rcx; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rax }) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x68; const P1: u64 = 0x1426452b0; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0xffffffffffffffd2; const P5: u64 = 0x141524330; const P6: u64 = 0x10; const P7: u64 = 0x20; const P8: u64 = 0x28; const P9: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rsi = rcx; rcx = rbx; rdi = ((r8 as u32)) as u32 as u64; rbp = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rax = 0x90; rcx = 0xa0; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rsi = ((rsi).wrapping_add(rax)) as u64; rdi = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = 0x30; rcx = ((rdx.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((rbp as u32)) as u32); crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); crate::simd_ops::wu64(rdi.wrapping_add(P7), rax); crate::simd_ops::wu64(rdi.wrapping_add(P8), rax); rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsi.wrapping_add(P2), rdi); crate::simd_ops::wu64(rsi, rdi); crate::simd_ops::wu64(rdi, rax); bb = 8; continue; }
            6 => { crate::simd_ops::wu64(rdi, rcx); crate::simd_ops::wu64(rsi.wrapping_add(P2), rdi); bb = 8; continue; }
            7 => { rdi = rax; bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rax.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; r8 = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { r8 }) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r8)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; return rax; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = 0x2ca33bdb; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rdx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rdx as u32) }) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af900(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r9 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(r8, ((rcx as u32)) as u32); return rax; }
            5 => { crate::simd_ops::wu32(r8, (0x2ca33bdbu32) as u32); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415af950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rax.wrapping_mul(4)), ((rdx as u32)) as u32); rax = ((rax).wrapping_add(1)) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415afa10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc; const P1: u64 = 0x8; const P2: u64 = 0x141524340; const P3: u64 = 0x141524330; const P4: u64 = 0xfffffffffffffff4; const P5: u64 = 0xfffffffffffffffc; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rdi = rcx; rbp = ((rdx as u32)) as u32 as u64; rcx = 0x1; rbp = (((rbp as u32)).wrapping_shr(1u32)) as u32 as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rsi = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rbp = (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { (rcx as u32) } else { (rbp as u32) }) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; rbp = (((rbp as u32)).wrapping_add((rdx as u32))) as u32 as u64; rdx = ((rbp.wrapping_mul(2)) as u64) as u64; rdx = ((rdx).wrapping_add(rbp)) as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = rdx; rdx = rcx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = ((rax.wrapping_add(P0)) as u64) as u64; xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(rcx).wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(rbx).wrapping_add(P4), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rax.wrapping_add(rcx).wrapping_add(P5))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(rbx).wrapping_add(P5), ((rcx as u32)) as u32); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rdi, rbx); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(P0), ((rbp as u32)) as u32); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415afae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x28; const P2: u64 = 0x30; const P3: u64 = 0x20; const P4: u64 = 0x18; const P5: u64 = 0x58; const P6: u64 = 0x60; const P7: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdx = 0xc0; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rax); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx.wrapping_add(P2), (0x8u32) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); bb = 2; continue; }
            2 => { rax = ((rdx.wrapping_add(P4)) as u64) as u64; rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rdx, rax); r8 = rdx; rdx = rax; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = 0x100; crate::simd_ops::wu64(r8, (0x0u64) as u64); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x34; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { crate::simd_ops::wu32(rbx.wrapping_add(P6), (0x8u32) as u32); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); bb = 6; continue; }
            6 => { rax = ((rcx.wrapping_add(P3)) as u64) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); r8 = rcx; rcx = rax; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r8, (0x0u64) as u64); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415afb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x141524330; const P2: u64 = 0x1426452a8; const P3: u64 = 0x1415af950;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx.wrapping_add(0x68); rsi = rcx; rcx = rbx; r15 = ((r9 as u32)) as u32 as u64; rbp = ((r8 as u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; rax = 0x90; rcx = 0xa0; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rdi = ((rax.wrapping_add(rsi)) as u64) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rsi).wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 2; continue; }
            2 => { rsi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = 0x30; rcx = ((rdx.wrapping_sub(0x2eu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(0x18u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsi.wrapping_add(0x28u64), rax); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rsi); crate::simd_ops::wu64(rdi, rsi); crate::simd_ops::wu64(rsi, rax); bb = 8; continue; }
            6 => { crate::simd_ops::wu64(rsi, rcx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rsi); bb = 8; continue; }
            7 => { rsi = rax; bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 16; continue; } bb = 10; continue; }
            10 => { rax = ((rsi.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 39; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; r14 = rsi.wrapping_add(0x8); rbx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rdi.wrapping_add(rbx.wrapping_mul(4))) as u64) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 17; continue; } bb = 14; continue; }
            14 => { rdi = ((rdi).wrapping_add(0x4u64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { r12 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { rax = 0x1; bb = 39; continue; }
            17 => { r12 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64; fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { rax = ((rax).wrapping_add(0xfffffffffffffffcu64)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 26; continue; }
            21 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(0xcu64))) as u32 as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rdx, ((r15 as u32)) as u32); bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rbp = 0x1; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 27; continue; }
            27 => { bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = (((r12 as u8)) as u32) as u32 as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 31; continue; } bb = 29; continue; }
            29 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 30; continue; }
            30 => { bb = 38; continue; }
            31 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 33; continue; }
            33 => { bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rsi)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r15 as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 34; continue; } bb = 35; continue; }
            35 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rdx = ((rdx).wrapping_sub(1)) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::lk_sub_u32(r14.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 38; continue; }
            38 => { rax = ((rbp as u32)) as u32 as u64; bb = 39; continue; }
            39 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415afd90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x18; const P2: u64 = 0x141524330; const P3: u64 = 0x10; const P4: u64 = 0x34;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), rax); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rcx, rax); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = 0; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P1), rcx); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx, rdx); crate::simd_ops::lk_add_u32(rbx.wrapping_add(P4), (1) as u32); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = ((rdx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu128(rdx, (xmm0) as u128); bb = 9; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { fl_l = rdx; fl_r = rdx; rax = 0x34; rcx = 0x1; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rax = 0x1f; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415afe30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x20; const P2: u64 = 0x141524330; const P3: u64 = 0x48; const P4: u64 = 0x40; const P5: u64 = 0x64; const P6: u64 = 0x8; const P7: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rax, rcx); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rcx, rax); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rbx.wrapping_add(P3), rcx); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu64(rcx, (0x0u64) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::lk_add_u32(rbx.wrapping_add(P5), (1) as u32); crate::simd_ops::wu128(rax.wrapping_add(P6), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(P7), ((xmm1 as u64)) as u64); rax = 0x1; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415afed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x1415242e0; const P5: u64 = 0x24; const P6: u64 = 0xfffffffffffffff0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((rdx as u8)) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rsi = rcx; rax = ((r8 as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); r14 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rax = ((r8 as u32)) as u32 as u64; bb = 10; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(P3), ((r14 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(P2), r14); crate::simd_ops::wu32(rdi.wrapping_add(P5), ((r14 as u32)) as u32); bb = 7; continue; }
            7 => { rdx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(P1), r14); rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((rcx.wrapping_add(P6)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rbx, (xmm0) as u128); rcx = (crate::simd_ops::ru64(rsi)) as u64; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(P0), (1) as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64; bb = 10; continue; }
            10 => { r8 = ((rax as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 11; continue; }
            11 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x1; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415affb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = r8; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = ((rax.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; r8 = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { r8 }) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { r11 = (crate::simd_ops::ru64(r8)) as u64; r10 = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rax = r10; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { rax = 0x1; return rax; }
            8 => { r8 = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r11.wrapping_add(0x10u64), r8); rax = 0x1; return rax; }
            10 => { crate::simd_ops::wu64(rdx.wrapping_add(P0), r8); bb = 11; continue; }
            11 => { rax = 0x1; return rax; }
            12 => { rax = 0x14; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415afa10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rdx; r14 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r15 = r8; rsi = rcx; rdx = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rdx = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x4cu64))) as u32 as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x4u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0xcu64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rbp = 0; fl_l = 0; fl_r = 0; rdi = rax.wrapping_add(0x8); fl_l = rax; fl_r = rdx; rdi = (if (((fl_l as u64)) == ((fl_r as u64))) { rbp } else { rdi }) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); bb = 7; continue; }
            7 => { rax = rdi; bb = 14; continue; }
            8 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xcu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x8u64), ((rax as u32)) as u32); rcx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rax, rbx); fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru32(r15)) as u32 as u64; crate::simd_ops::wu32(rax, ((rcx as u32)) as u32); bb = 14; continue; }
            13 => { rax = rbp; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x143930f20; const P2: u64 = 0x1415af790; const P3: u64 = 0x1434268d0; const P4: u64 = 0x1415bd980; const P5: u64 = 0x143930f80; const P6: u64 = 0x1415ada30; const P7: u64 = 0x141524330; const P8: u64 = 0x14183acbc; const P9: u64 = 0x1415ad8b0; const P10: u64 = 0x1415be610; const P11: u64 = 0x1415adef0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rbp = (((r9 as u8)) as u32) as u32 as u64; r10 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r15 = ((r8 as u32)) as u32 as u64; r10 = ((r10).wrapping_shl(4u32)) as u64; rsi = ((rdx as u32)) as u32 as u64; r10 = ((r10).wrapping_add(rax)) as u64; rdi = rcx; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r14 = 0; fl_l = 0; fl_r = 0; rcx = rax.wrapping_add(0x8); fl_l = rax; fl_r = r10; rcx = (if (((fl_l as u64)) == ((fl_r as u64))) { r14 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 92; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x138u64), rbx); rbx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0xe0u64), (xmm6) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbx); crate::simd_ops::wu128(rsp.wrapping_add(0xd0u64), (xmm7) as u128); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x150u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 31; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x130u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 28; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), (0x1u8) as u8); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { xmm6 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x130u64))) as u32) as u128; xmm7 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32) as u128; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); r8 = rsp.wrapping_add(0x70); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((r14 as u32)) as u32); xmm3 = (xmm7) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((r14 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x90u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x130u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x134u64), (0x3u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm6 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x39u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 13; continue; }
            13 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; rbx = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 15; continue; }
            15 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbx); bb = 18; continue; }
            17 => { crate::simd_ops::wu64(r14, rbx); bb = 18; continue; }
            18 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), r14); bb = 20; continue; }
            20 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x34u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rdx); bb = 25; continue; }
            23 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rdi.wrapping_add(0x34u64), (1) as u32)) as u32 as u64; bb = 25; continue; }
            24 => { r14 = rdx; bb = 25; continue; }
            25 => { rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu8(rdi.wrapping_add(0x39u64), (0x0u8) as u8); r14 = 0; fl_l = 0; fl_r = 0; bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x38u64), (0x0u8) as u8); bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(P1)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 29; continue; }
            29 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rdx = ((r15 as u32)) as u32 as u64; rcx = ((rsi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 91; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32 as u64; rdi = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x130u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x134u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((r14 as u32)) as u32); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 37; continue; } bb = 32; continue; }
            32 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 91; continue; } bb = 33; continue; }
            33 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; rcx = rbx.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdi = (crate::simd_ops::ru32(rax)) as u32 as u64; bb = 36; continue; }
            35 => { rdi = (crate::simd_ops::ru32(rbx)) as u32 as u64; bb = 36; continue; }
            36 => { crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rdi as u32)) as u32); bb = 37; continue; }
            37 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 90; continue; } bb = 38; continue; }
            38 => { crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), r13); crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm8) as u128); xmm8 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm9) as u128); xmm9 = ((crate::simd_ops::ru32(P3)) as u32) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r14); bb = 39; continue; }
            39 => { r14 = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r14); crate::simd_ops::wu32(rbx.wrapping_add(0x3cu64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax.wrapping_add(0x18); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 41; continue; } bb = 40; continue; }
            40 => { rsi = rsp.wrapping_add(0xa0); bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); r13 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 88; continue; } bb = 42; continue; }
            42 => { rdx = 0; fl_l = 0; fl_r = 0; rbp = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdx as u32)) as u32); fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 88; continue; } bb = 43; continue; }
            43 => { bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru64(r13)) as u64; r10 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = ((rbp as u32)) as u32 as u64; r12 = ((crate::simd_ops::ru8(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x3u64))) as u32) as u32 as u64; r9 = ((crate::simd_ops::ru8(rax.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x2u64))) as u32) as u32 as u64; r11 = (((r12 as u8)) as u32) as u32 as u64; r15 = ((crate::simd_ops::ru16(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32) as u32 as u64; r12 = ((((r12 as u32)) & (0x1u32))) as u32 as u64; r12 = ((r12).wrapping_shl(8u32)) as u64; r11 = (r11 & !0xffu64) | ((((((r11 as u8)) & (0x1u8))) as u8 as u64)); r12 = (((r12) | (r9))) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x130u64), ((r11 as u8)) as u8); r12 = ((r12).wrapping_shl(16u32)) as u64; rax = ((rdi as u32)) as u32 as u64; r12 = (((r12) | (r15))) as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; rax = (if (((fl_l as u8)) == ((fl_r as u8))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rax as u32)) as u32); fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 45; continue; }
            45 => { r8 = ((crate::simd_ops::ru16(r10)) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; bb = 46; continue; }
            46 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r10.wrapping_add(rdx.wrapping_mul(2)))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 52; continue; } bb = 47; continue; }
            47 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 46; continue; } bb = 48; continue; }
            48 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 49; continue; }
            49 => { rdi = rdx; bb = 50; continue; }
            50 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 53; continue; } bb = 51; continue; }
            51 => { xmm7 = (xmm9) as u128; bb = 54; continue; }
            52 => { rdx = 0xfffffff8; rdi = ((rcx as u32)) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rax = ((r8.wrapping_mul(2).wrapping_add(0x9u64)) as u32) as u32 as u64; rax = (((rax) & (rdx))) as u64; rax = ((rax).wrapping_add(r10)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rdx = 0; fl_l = 0; fl_r = 0; bb = 50; continue; }
            53 => { xmm7 = (xmm8) as u128; bb = 54; continue; }
            54 => { r8 = (crate::simd_ops::ru64(rsi)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 55; continue; }
            55 => { r9 = ((crate::simd_ops::ru16(r8)) as u32) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; bb = 56; continue; }
            56 => { rdx = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rdx.wrapping_mul(2)))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 62; continue; } bb = 57; continue; }
            57 => { rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 56; continue; } bb = 58; continue; }
            58 => { xmm6 = (xmm7) as u128; bb = 59; continue; }
            59 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 60; continue; }
            60 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 61; continue; }
            61 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; xmm3 = (xmm6) as u128; rcx = (crate::simd_ops::ru64(P5)) as u64; r8 = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x0u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 86; continue; }
            62 => { rdx = 0xfffffffc; rax = ((r9.wrapping_mul(2).wrapping_add(0x5u64)) as u32) as u32 as u64; rax = (((rax) & (rdx))) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 63; continue; }
            63 => { xmm6 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; bb = 59; continue; }
            64 => { xmm7 = ((crate::simd_ops::ru32(rdi)) as u32) as u128; bb = 65; continue; }
            65 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 86; continue; } bb = 66; continue; }
            66 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 77; continue; } bb = 67; continue; }
            67 => { fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 68; continue; }
            68 => { rsi = ((crate::simd_ops::ru16(r10)) as u32) as u32 as u64; bb = 70; continue; }
            69 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 70; continue; }
            70 => { rcx = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = 0xfffffff8; rcx = (((rcx as u32)).wrapping_shl(4u32)) as u32 as u64; r13 = ((rsi.wrapping_add(rsi)) as u32) as u32 as u64; rbp = ((r13.wrapping_add(0xbu64)) as u32) as u32 as u64; rdx = ((((rdx as u32)) & ((rbp as u32)))) as u32 as u64; rdx = (((rdx as u32)).wrapping_add((rcx as u32))) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 72; continue; } bb = 71; continue; }
            71 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 76; continue; }
            72 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rdi = ((rsi as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 74; continue; } bb = 73; continue; }
            73 => { r8 = ((rsi.wrapping_add(rsi)) as u64) as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rcx = rax.wrapping_add(0x2); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0xfffffff8; rdx = ((r13.wrapping_add(0x9u64)) as u32) as u32 as u64; rdx = (((rdx) & (rax))) as u64; rcx = rbp; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x20u64)))) as u64; rcx = (((rcx) & (rax))) as u64; r8 = ((rdi as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(r14)) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 74; continue; }
            74 => { rax = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; crate::simd_ops::wu16(r14.wrapping_add(rax.wrapping_mul(2)), ((r15 as u16)) as u16); rsi = (rsi & !0xffffu64) | (((((rsi as u16)).wrapping_add(1)) as u16 as u64)); rax = 0xfffffff8; crate::simd_ops::wu16(r14, ((rsi as u16)) as u16); rax = (((rax) & (rbp))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), r14); rax = ((rax).wrapping_add(r14)) as u64; fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; if (((fl_l as u64)) == 0) { bb = 76; continue; } bb = 75; continue; }
            75 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); bb = 76; continue; }
            76 => { rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; r11 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x130u64))) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 77; continue; }
            77 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; xmm0 = (((rax as u32)) as u32) as u128; xmm0 = crate::simd_ops::vcvtdq2ps(xmm0); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm8 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 83; continue; } bb = 78; continue; }
            78 => { rcx = (crate::simd_ops::ru64(P5)) as u64; rdx = rsp.wrapping_add(0x70); r8 = 0x1; crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x84u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8cu64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), (0xcu32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x94u64), ((r11 as u8)) as u8); crate::simd_ops::wu16(rsp.wrapping_add(0x95u64), (0x1u16) as u16); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 84; continue; } bb = 79; continue; }
            79 => { crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 82; continue; } bb = 80; continue; }
            80 => { rdx = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 86; continue; } bb = 81; continue; }
            81 => { r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm2 = (xmm6) as u128; rdx = r12; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 85; continue; }
            82 => { crate::simd_ops::wu32(rdi, ((xmm6 as u32)) as u32); bb = 83; continue; }
            83 => { r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); xmm2 = (xmm6) as u128; rdx = r12; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 86; continue; }
            84 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 86; continue; } bb = 85; continue; }
            85 => { rcx = (crate::simd_ops::ru64(P5)) as u64; r8 = rbx; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 86; continue; }
            86 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rdx = 0x0; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbp as u32)) as u32); fl_l = ((rbp as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 44; continue; } bb = 87; continue; }
            87 => { r15 = (crate::simd_ops::ru32(rsp.wrapping_add(0x140u64))) as u32 as u64; bb = 88; continue; }
            88 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x44u64))) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 39; continue; } bb = 89; continue; }
            89 => { xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0xc0u64))) as u128; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; bb = 90; continue; }
            90 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((r15 as u32)) as u32); bb = 91; continue; }
            91 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0xd0u64))) as u128; bb = 92; continue; }
            92 => { rsp = ((rsp).wrapping_add(0x100u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { r9 = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = rcx; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { r9 } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu32(rax, ((r8 as u32)) as u32); rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; return rax; }
            5 => { rax = 0x14; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x1415b09c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx)) as u64; rbp = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = rcx; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rsi)) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rdi.wrapping_add(0x20u64), ((rbp as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rbp); crate::simd_ops::wu32(rdi.wrapping_add(0x24u64), ((rbp as u32)) as u32); bb = 4; continue; }
            4 => { rdx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rbp); rcx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = rdx.wrapping_sub(0x10); rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; crate::simd_ops::wu128(rsi, (xmm0) as u128); rdx = (crate::simd_ops::ru64(rbx)) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 7; continue; }
            7 => { rsi = rdx; bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = rsi; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rbp as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rbp); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rbp as u32)) as u32); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); bb = 13; continue; }
            13 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x34u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rdx); bb = 10; continue; }
            16 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x34u64), (1) as u32)) as u32 as u64; bb = 10; continue; }
            17 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rdi = rbx.wrapping_add(0x30); rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 23; continue; }
            23 => { bb = 25; continue; }
            24 => { rdi = rbx.wrapping_add(0x30); bb = 25; continue; }
            25 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rbp); bb = 27; continue; }
            27 => { crate::simd_ops::wu32(rdi, ((rbp as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), rbp); bb = 28; continue; }
            28 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rax); fl_l = rdx; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 31; continue; } bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rbp); bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 34; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rdx, rax); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x64u64), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rdx); bb = 28; continue; }
            34 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x64u64), (1) as u32)) as u32 as u64; bb = 28; continue; }
            35 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; rdi = rbx.wrapping_add(0x60); rsi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 40; continue; }
            40 => { rdx = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 41; continue; }
            41 => { bb = 43; continue; }
            42 => { rdi = rbx.wrapping_add(0x60); bb = 43; continue; }
            43 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x58u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x58u64), rbp); bb = 45; continue; }
            45 => { rdx = rbx.wrapping_add(0x90); crate::simd_ops::wu32(rdi, ((rbp as u32)) as u32); rcx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(0x40u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx.wrapping_add(0xa0); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b09c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0x10; const P4: u64 = 0x1415242e0; const P5: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rdi = rdx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rcx = rbx; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rsi = rbx; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rdi.wrapping_add(P0), rax); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(r14, rax); bb = 5; continue; }
            5 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi)) as u64; rbx = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rdi, r14); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsi.wrapping_add(P1), r14); crate::simd_ops::wu64(rsi.wrapping_add(P2), r14); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_add(P3), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P0), r14); crate::simd_ops::wu32(rsi.wrapping_add(P5), ((r14 as u32)) as u32); bb = 9; continue; }
            9 => { rdx = rsi; rcx = 0x2; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P0), r14); crate::simd_ops::wu64(rdi, r14); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P0), r14); crate::simd_ops::wu64(rdx, r14); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = r8; rsi = ((rdx as u32)) as u32 as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; rdx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0ae0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0x38; const P3: u64 = 0x18; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x20; const P7: u64 = 0x34; const P8: u64 = 0x1415242e0; const P9: u64 = 0x39;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r14 = 0; fl_l = 0; fl_r = 0; rbp = rdx; rbx = rcx; rsi = ((r14 as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); bb = 7; continue; }
            6 => { crate::simd_ops::wu64(rsi, rdi); bb = 7; continue; }
            7 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(P3), rsi); bb = 9; continue; }
            9 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((r8.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rax, rcx); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P7), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), rax); rax = rdi; bb = 15; continue; }
            12 => { rdx = rax; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P7), (1) as u32)) as u32 as u64; rax = rdi; bb = 15; continue; }
            13 => { crate::simd_ops::wu64(rax.wrapping_add(P1), r14); rsi = rax; rax = rdi; crate::simd_ops::wu8(rbx.wrapping_add(P9), (0x1u8) as u8); bb = 15; continue; }
            14 => { rsi = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 15; continue; }
            15 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x1; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x8; const P2: u64 = 0x48; const P3: u64 = 0x58; const P4: u64 = 0x60; const P5: u64 = 0x50; const P6: u64 = 0x64; const P7: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = 0; fl_l = 0; fl_r = 0; rbp = rdx; rbx = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rsi, rdi); bb = 6; continue; }
            6 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rbx.wrapping_add(P2), rsi); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(rdx)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rax, rcx); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P6), (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(P5), rax); bb = 13; continue; }
            11 => { rdx = rax; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P6), (1) as u32)) as u32 as u64; bb = 13; continue; }
            12 => { rsi = rax; bb = 13; continue; }
            13 => { rax = rdi; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x1; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x28; const P2: u64 = 0x30; const P3: u64 = 0x38; const P4: u64 = 0x40; const P5: u64 = 0x50; const P6: u64 = 0x58; const P7: u64 = 0x18; const P8: u64 = 0x8; const P9: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P9), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x50; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x28; const P5: u64 = 0x10; const P6: u64 = 0x1415242e0; const P7: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); }
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi);
    crate::simd_ops::wu64(rbx, rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); }
    crate::simd_ops::wu8(rbx.wrapping_add(P7), ((rdi as u8)) as u8);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0d10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x50; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); }
    crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi);
    rax = 0x1;
    crate::simd_ops::wu64(rbx, rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi);
    crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b0ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b0d60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    r9 = rsp.wrapping_add(0x58);
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x0u32) as u32);
    rbx = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xffffffffu32) as u32);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rsp.wrapping_add(0x58u64))) as u32 as u64; rcx = 0x38; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415242e0; const P1: u64 = 0x143931178; const P2: u64 = 0x141524320; const P3: u64 = 0x1415b1500;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rsi = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru32(rbx.wrapping_add(0x30u64)) } else { (rcx as u32) }) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rcx as u32)) as u32); rdx = ((rcx.wrapping_add(rcx)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r8 = ((rcx.wrapping_add(0x20u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = 0x34; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rbx, rcx); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); rax = 0x1; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); rdx = rsi; r8 = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rsi = (((rsi as u32)).wrapping_sub(crate::simd_ops::ru32(rsp.wrapping_add(0x40u64)))) as u32 as u64; r8 = rsp.wrapping_add(0x30); rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rcx = 0x2; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1130(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524320; const P1: u64 = 0x1415b0d60; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x20u64))) as u32 as u64; rbx = rcx; rdi = ((rdx as u32)) as u32 as u64; rcx = ((rcx.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = ((rdi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rax); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (rax) as u64); rax = rcx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); rcx = ((rdi as u32)) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1fu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((((rax as u32)) & (0x1fu32))) as u32 as u64; rcx = ((rdi.wrapping_add(0x1fu64)) as u32) as u32 as u64; rsi = 0x20; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rsi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 6; continue; }
            6 => { rdx = ((rcx as u32)) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; r8 = ((rcx.wrapping_add(0x20u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { rsi = ((rsi).wrapping_add(rax)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0xffffffffu32) as u32); rdx = rsi; r9 = rsp.wrapping_add(0x48); r8 = ((rdi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), (0x0u64) as u64); bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x24; const P2: u64 = 0x18; const P3: u64 = 0x8; const P4: u64 = 0x10; const P5: u64 = 0x48; const P6: u64 = 0x4c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = 0x3; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = 0x1;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P5), (0x4483126fu32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), (0x32u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x38; const P1: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b12b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x4c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = 0u128;
    fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64;
    if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { rax = 0x1f; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x64u8) as u8 as u64;
    if !((((fl_l as u8)) > ((fl_r as u8)))) { crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm1 as u32)) as u32); rax = 0x1; crate::simd_ops::wu8(rcx.wrapping_add(P1), ((r8 as u8)) as u8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = 0x1f;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b12e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0xffffffffffffffcc; const P5: u64 = 0xffffffffffffffc8; const P6: u64 = 0xffffffffffffffe4; const P7: u64 = 0xffffffffffffffe0; const P8: u64 = 0x40; const P9: u64 = 0x143932030; const P10: u64 = 0x1415b1030;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x98u64))) as u32) as u32 as u64; rdi = rcx; crate::simd_ops::wu32(rax.wrapping_add(P4), ((r9 as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_add(P5), ((rcx as u32)) as u32); rbp = ((r8 as u32)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P6), (0xffffffffu32) as u32); r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rax.wrapping_add(P7), ((rbx as u8)) as u8); r14 = ((rdi.wrapping_add(P8)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); rsi = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P9)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P9)) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b13b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645710; const P1: u64 = 0x143932030; const P2: u64 = 0x1415b1030;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rsi); rsi = rcx; crate::simd_ops::wu32(rbp.wrapping_add(0x4u64), ((rax as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); rax = (0xffffffffffffffffu64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), r14); r14 = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu32(rbp, ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1cu64), (0xffffffffu32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x18u64), (0x1u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x24u64), ((r8 as u32)) as u32); bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax).wrapping_add(0x1u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; rax = rax.wrapping_add(0x1); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = ((rax.wrapping_mul(2).wrapping_add(0x2u64)) as u64) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (0xffffffffffffff0u64) as u64; bb = 4; continue; }
            4 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; rax = (0xffffffffffffffffu64) as u64; rbx = rsp.wrapping_add(0x30); bb = 5; continue; }
            5 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(rax))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; r8 = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); r9 = 0xffffffff; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rcx = 0xfde9; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P1)) as u64; rdi = rsi.wrapping_add(0x40); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); r8 = rbp; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rdx = rbx; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P1)) as u64; r8 = rbp; crate::simd_ops::wu8(rbp.wrapping_add(0x18u64), (0x0u8) as u8); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8); rdx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = ((r14 as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsp = rbp.wrapping_add(0x30); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b14e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x50; const P1: u64 = 0x18; const P2: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    rax = 0x1;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r13 = 0; fl_l = 0; fl_r = 0; r12 = r8; r15 = ((rdx as u32)) as u32 as u64; rbx = rcx; r14 = 0x1; rsi = ((r13 as u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x18u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rax = (if (((fl_l as u32)) >= ((fl_r as u32))) { (rdi as u32) } else { (rax as u32) }) as u32 as u64; rsi = (((rsi as u32)).wrapping_add((rax as u32))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_sub((rax as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x50u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x50u64), rax); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x18u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 16; continue; }
            5 => { crate::simd_ops::lk_add_u64(rbx.wrapping_add(0x10u64), (rcx) as u64); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x18u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x18u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); bb = 16; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 18; continue; } bb = 8; continue; }
            8 => { rax = ((r13 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x20u64))) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rbp = ((rax as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(rbp.wrapping_mul(4)).wrapping_add(0x18u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx.wrapping_add(rbp.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), rax); bb = 16; continue; }
            10 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; r9 = rsp.wrapping_add(0x90); rdx = ((rdi as u32)) as u32 as u64; r8 = 0x1; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x24u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((r8 as u32)) as u32 as u64; let d = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } xmm0 = 0u128; crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((r13 as u32)) as u32); r10 = (crate::simd_ops::ru64(rcx)) as u64; r8 = (((r8 as u32)).wrapping_sub((rdx as u32))) as u32 as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(rbp.wrapping_mul(8)))) as u64; r9 = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); rax = rsp.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x4cu64))) as u32) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x48u64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rbx.wrapping_add(rbp.wrapping_mul(4)).wrapping_add(0x18u64), ((rax as u32)) as u32); bb = 16; continue; }
            15 => { rsi = (((rsi as u32)).wrapping_add((rcx as u32))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 16; continue; }
            16 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 17; continue; }
            17 => { bb = 19; continue; }
            18 => { r14 = 0x38; bb = 19; continue; }
            19 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 20; continue; }
            20 => { crate::simd_ops::wu32(r12, ((rsi as u32)) as u32); rax = ((r14 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x8; const P2: u64 = 0x28; const P3: u64 = 0x10; const P4: u64 = 0x1415242e0; const P5: u64 = 0x1; const P6: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdi);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdi);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rdi);
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = ((rdi.wrapping_add(P5)) as u32) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi); crate::simd_ops::wu8(rbx.wrapping_add(P6), ((rdi as u8)) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x1;
    crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rdi as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0xffffffffffffffffu64) as u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu32(rdx, (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b17b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, ((rdx as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0xffffffffffffffffu64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b17e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x1415b25c0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64;
    r10 = r9;
    crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64);
    rbx = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    r8 = r10;
    crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8);
    rcx = rsp.wrapping_add(0x40);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x1439317d0; const P3: u64 = 0x1416135c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = P2; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b18a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru32(rdx)) as u32 as u64;
    crate::simd_ops::wu32(rcx, ((rax as u32)) as u32);
    rax = rcx;
    crate::simd_ops::wu64(rdx.wrapping_add(P0), (0xffffffffffffffffu64) as u64);
    crate::simd_ops::wu32(rdx, (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b18d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415b1c80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = r9; fl_r = r9;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; rcx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b18f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1415b1c80;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    fl_l = r9; fl_r = r9;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((r8 as u32)) as u32 as u64; rcx = ((r9.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rdx = (crate::simd_ops::ru64(r9.wrapping_add(rdx.wrapping_mul(8)))) as u64; return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1415b1eb0; const P3: u64 = 0x14;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rbx = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rdx, (0x34u32) as u32); rax = rdx; crate::simd_ops::wu64(rdx.wrapping_add(P0), (0xffffffffffffffffu64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; r8 = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0x1; crate::simd_ops::wu64(rdi, (0xffffffffffffffffu64) as u64); r8 = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(rbx, ((rcx as u32)) as u32); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rbx.wrapping_add(P0), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b1990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931858; const P1: u64 = 0x143931880; const P2: u64 = 0x1426452a8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = P0;
    crate::simd_ops::wu32(P1, (0x0u32) as u32);
    return crate::simd_ops::call_fwd(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b19b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x18; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x1u32) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x2000u32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b19d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439316d0; const P1: u64 = 0x143931718;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = 0x1; return rax; }
    rax = ((rcx as u32)) as u32 as u64;
    rcx = P1;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b19f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142645700; const P1: u64 = 0x1439316d0; const P2: u64 = 0x143426858; const P3: u64 = 0x1439318d4; const P4: u64 = 0x143931718; const P5: u64 = 0x143931708; const P6: u64 = 0x14393171c; const P7: u64 = 0x143931720; const P8: u64 = 0x1439317d0; const P9: u64 = 0x1416135f0; const P10: u64 = 0x143931728; const P11: u64 = 0x142645290; const P12: u64 = 0x143931750; const P13: u64 = 0x1439316e0; const P14: u64 = 0x1439316f0; const P15: u64 = 0x141524330; const P16: u64 = 0x14183acc2; const P17: u64 = 0x1439316f8; const P18: u64 = 0x14268a140; const P19: u64 = 0x143931700; const P20: u64 = 0x143931740; const P21: u64 = 0x14393174c; const P22: u64 = 0x143931748; const P23: u64 = 0x143931730; const P24: u64 = 0x1415b27d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rcx = rsp.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = (0x0u64) as u64; xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((((crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as i64) as f64).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P2))).to_bits() as u128); xmm0 = crate::simd_ops::vcvtpd2ps(xmm0); crate::simd_ops::wu32(P3, ((xmm0 as u32)) as u32); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x3; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = P4; r14 = 0; fl_l = 0; fl_r = 0; rcx = P5; rdx = 0x4; bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rcx, ((r14 as u32)) as u32); rcx = rcx.wrapping_add(0x4); crate::simd_ops::wu32(rax, (0x1u32) as u32); rax = rax.wrapping_add(0x4); fl_l = rdx; fl_r = (0x1u64) as u64; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rax = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu32(P4, ((rax as u32)) as u32); rax = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0xcu64))) as u32 as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu32(P6, ((rax as u32)) as u32); rax = ((r14 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; bb = 10; continue; }
            10 => { crate::simd_ops::wu32(P7, ((rax as u32)) as u32); rcx = P8; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = ((r14 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rdi = P10; bb = 12; continue; }
            12 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; crate::simd_ops::wu64(rdi, rax); fl_l = 0; fl_r = rcx; rcx = (0u64.wrapping_sub(fl_r as u64)) as u64; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 13; continue; }
            13 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 12; continue; } bb = 14; continue; }
            14 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rdi = ((r14 as u32)) as u32 as u64; rbx = P12; crate::simd_ops::wu128(P1, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(P13, (xmm1) as u128); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(P14, ((xmm0 as u64)) as u64); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), (0x1fffu64) as u64); rdx = 0x20000; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), r14); rcx = 0x5; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 16; continue; }
            16 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x20000; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = r14; r9 = r14; rdx = r14; r10 = 0x2000; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdx.wrapping_add(0x10); r9 = r9.wrapping_add(0x10); crate::simd_ops::wu64(r9.wrapping_add(rax).wrapping_sub(0x10u64), (0xffffffffffffffffu64) as u64); rax = r8; rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r8).wrapping_add(1)) as u64; let tmp_xchg: u64 = crate::simd_ops::atomic_xchg_u64(rdx.wrapping_add(rcx).wrapping_sub(0x8u64), rax); rax = (tmp_xchg) as u64; fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 15; continue; } bb = 19; continue; }
            19 => { rdx = 0x38000; rcx = ((r10.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P17, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x38000; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P18; crate::simd_ops::wu64(P19, (0xfffu64) as u64); crate::simd_ops::wu64(P20, rcx); rax = 0x1; crate::simd_ops::wu8(P21, ((r14 as u8)) as u8); crate::simd_ops::wu32(P22, (0x3u32) as u32); crate::simd_ops::wu8(P23, (0x1u8) as u8); bb = 23; continue; }
            21 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x34; bb = 23; continue; }
            22 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((rbp.wrapping_add(0x2u64)) as u32) as u32 as u64; bb = 23; continue; }
            23 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 24; continue; }
            24 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b2550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b1fc0; const P1: u64 = 0x1439317d0; const P2: u64 = 0x141613740;
    let (mut rax, mut fl_l, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm0: u128 = 0; let mut xmm1: u128 = 0; let mut xmm2: u128 = 0; let mut xmm3: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1f80u32) as u32);
    rbx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32);
    rcx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xffffffffffffffffu64) as u64);
    rbx = ((((rbx as u32)) & (0x8000u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1f80u32) as u32);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64;
    fl_l = ((((rax as u32)) >> (15u32)) & 1) as u64;
    rax = (((rax as u32)) | ((1u32) << (15u32))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1f80u32) as u32);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64;
    fl_l = ((((rax as u32)) >> (15u32)) & 1) as u64;
    rax = (((rax as u32)) & !((1u32) << (15u32))) as u32 as u64;
    rax = ((((rax as u32)) | ((rbx as u32)))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b2750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439316d0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rax = 0x3; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; return rax; }
    rax = ((rcx as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b2780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439316d0; const P1: u64 = 0x143931718;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { return rax; }
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x3;
    if !((((fl_l as u32)) >= ((fl_r as u32)))) { rax = ((rcx as u32)) as u32 as u64; rcx = P1; crate::simd_ops::wu32(rcx.wrapping_add(rax.wrapping_mul(4)), ((rdx as u32)) as u32); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b27d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931708; const P1: u64 = 0x142645618; const P2: u64 = 0x143931750; const P3: u64 = 0x1415242e0; const P4: u64 = 0x10; const P5: u64 = 0x18; const P6: u64 = 0x1439316f8; const P7: u64 = 0x143931728; const P8: u64 = 0x1439316d0; const P9: u64 = 0x1439316e0; const P10: u64 = 0x1439316f0; const P11: u64 = 0x1426455b0; const P12: u64 = 0x1439317d0; const P13: u64 = 0x141613770; const P14: u64 = 0x143931730;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = 0x4; rbx = P0; rdi = ((rsi as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = 0x1; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(rsi)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { rbx = P2; bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = 0x5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx, rdi); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbx.wrapping_add(P4), rdi); crate::simd_ops::wu64(rbx.wrapping_add(P5), rdi); rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru64(P6)) as u64; rcx = ((rsi.wrapping_add(P4)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P7)) as u64; xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P6, rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu128(P8, (xmm0) as u128); crate::simd_ops::wu128(P9, (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64) as u128; crate::simd_ops::wu64(P10, ((xmm0 as u64)) as u64); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = P12; crate::simd_ops::wu64(P7, rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu8(P14, ((rdi as u8)) as u8); rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b28c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b25c0; const P1: u64 = 0x1415b1c80; const P2: u64 = 0x10; const P3: u64 = 0x8; const P4: u64 = 0x1415b1eb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rax = r9; rdi = ((r8 as u32)) as u32 as u64; rbx = rdx; rsi = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); r8 = rax; rcx = rsp.wrapping_add(0x50); r9 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsp.wrapping_add(0x50); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx.wrapping_add(P2)) as u64) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rdx = 0x1; crate::simd_ops::wu32(rsi, ((r14 as u32)) as u32); r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsi.wrapping_add(P3), rbp); rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = rsi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b2960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143931888; const P1: u64 = 0x14397d1e0; const P2: u64 = 0x1439318b0; const P3: u64 = 0x1426452b0; const P4: u64 = 0x14268a150; const P5: u64 = 0x1415b25c0; const P6: u64 = 0x1415b1c80; const P7: u64 = 0x1439316f8; const P8: u64 = 0x1415b26c0; const P9: u64 = 0x1415b24b0; const P10: u64 = 0x143931728; const P11: u64 = 0x142645748; const P12: u64 = 0x1415b1fc0; const P13: u64 = 0x1439317d0; const P14: u64 = 0x141613740; const P15: u64 = 0x141524860; const P16: u64 = 0x1426452a8; const P17: u64 = 0x141834a78; const P18: u64 = 0x1426452b8; const P19: u64 = 0x1426442e0; const P20: u64 = 0x141834d18; const P21: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rbx = P0; rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rcx = 0x74; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); r8 = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r15); rax = (crate::simd_ops::ru32(rcx.wrapping_add(r8))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 18; continue; } bb = 1; continue; }
            1 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x3u32) as u32); r8 = P4; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0xa0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rcx = rsp.wrapping_add(0x98); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (r9 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; xmm0 = 0u128; rsi = 0; fl_l = 0; fl_r = 0; r15 = ((rsi as u32)) as u32 as u64; rbp = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(rbp.wrapping_mul(8)).wrapping_add(0x98u64))) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rax = rcx; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rdx = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(P7))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x30u64))) as u32 as u64; bb = 4; continue; }
            4 => { r8 = ((rax as u32)) as u32 as u64; fl_l = ((((r8 as u32)) >> (31u32)) & 1) as u64; r8 = (((r8 as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (r8 as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdx.wrapping_add(0x30u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7fffffff; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdi = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_add_u32(rsp.wrapping_add(rdi.wrapping_mul(4)).wrapping_add(0x30u64), (1) as u32)) as u32 as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            7 => { r15 = (((r15 as u32)).wrapping_add(1)) as u32 as u64; rbp = ((rbp).wrapping_add(1)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 2; continue; } bb = 8; continue; }
            8 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rdi = rsp.wrapping_add(0x30); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = ((rsi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((rdi).wrapping_add(0x4u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(P10)) as u64; rdx = 0xffffffff; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), (0x1f80u32) as u32); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rdi = ((((rdi as u32)) & (0x8000u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), (0x1f80u32) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; fl_l = ((((rax as u32)) >> (15u32)) & 1) as u64; rax = (((rax as u32)) | ((1u32) << (15u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x3u32) as u32); bb = 14; continue; }
            14 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r14; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = r14; rcx = r14; rax = ((((rax as u32)) & (0xfffu32))) as u32 as u64; rcx = (((rcx) & (0xfffffffffffff001u64))) as u64; rax = (((rax) as i64).wrapping_mul((0x38u64) as i64) as u64) as u64; rcx = (((rcx) | (0x1u64))) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(P7))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(P10)) as u64; rdx = 0xffffffff; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            17 => { crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), (0x1f80u32) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; rcx = P13; fl_l = ((((rax as u32)) >> (15u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (15u32))) as u32 as u64; rax = ((((rax as u32)) | ((rdi as u32)))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rbx; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            18 => { rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 19; continue; }
            19 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P18), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P19; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = P2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b2be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b1130; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415b2f20; const P6: u64 = 0x1415b16f0; const P7: u64 = 0x1415bcd90; const P8: u64 = 0x141610300; const P9: u64 = 0x141534ef0; const P10: u64 = 0x1416109c0; const P11: u64 = 0x1415b1830; const P12: u64 = 0x1415b1260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx); r13 = r8; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x38; rsp = ((rsp).wrapping_add(0x80u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r14); r14 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbp = rbx.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdi = r15; bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            13 => { rbp = rbx.wrapping_add(0x8); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            15 => { rdi = r15; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rbp; bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rcx = rbx.wrapping_add(0x8); bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = rdi; fl_r = rdi; rsi = rdi.wrapping_sub(0x10); rdi = 0x1; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rsi }) as u64; r12 = ((rdi.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rdx = rsi; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; bb = 53; continue; }
            28 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r13 = (r13 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x98u64), ((rdi as u8)) as u8); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 29; continue; }
            29 => { r8 = rsp.wrapping_add(0x98); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r14 as u32)) as u32); rdx = rax.wrapping_add(0x10); rcx = rbx.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { r13 = (r13 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 44; continue; }
            31 => { crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((r15 as u32)) as u32); rax = ((r12 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x98u64))) as u32) as u32 as u64; rsi = r15; bb = 34; continue; }
            34 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 44; continue; } bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 43; continue; } bb = 36; continue; }
            36 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 37; continue; }
            37 => { bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 41; continue; } bb = 39; continue; }
            39 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 38; continue; } bb = 40; continue; }
            40 => { rdx = r15; bb = 41; continue; }
            41 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rsi = rdx.wrapping_sub(0x10); bb = 44; continue; }
            43 => { rsi = r15; bb = 44; continue; }
            44 => { rdi = 0x1; fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((r15 as u32)) as u32); bb = 46; continue; }
            46 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbx = ((rsi.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 52; continue; }
            48 => { fl_l = ((r13 as u8)) as u8 as u64; fl_r = ((r13 as u8)) as u8 as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 50; continue; } bb = 49; continue; }
            49 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; r9 = r13; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 51; continue; }
            50 => { r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rdx = rsi; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 51; continue; }
            51 => { rbx = ((rax as u32)) as u32 as u64; bb = 52; continue; }
            52 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 53; continue; }
            53 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 57; continue; } bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x58u64))) as u32 as u64; rdx = rsi.wrapping_add(0x10); fl_l = rsi; fl_r = rsi; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rdx }) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r13.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 60; continue; } bb = 55; continue; }
            55 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(r13.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(r13.wrapping_add(0x50u64))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 56; continue; }
            56 => { crate::simd_ops::wu64(r8, rdx); bb = 60; continue; }
            57 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x5b; rbx = (if (((fl_l as u32)) == ((fl_r as u32))) { (rdi as u32) } else { (rbx as u32) }) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 58; continue; }
            58 => { rcx = rsi.wrapping_add(0x10); let xa_src: u32 = (r12 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r12 = (xa_old) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 59; continue; }
            59 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 60; continue; }
            60 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b2f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xfc; const P1: u64 = 0x1415b16f0; const P2: u64 = 0x1416109c0; const P3: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; rdi = ((r9 as u32)) as u32 as u64; rsi = r8; rbx = rdx; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x7cu8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (((rcx as u32)).wrapping_shr(7u32)) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rcx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r8 = ((rdi as u32)) as u32 as u64; rdx = rsi; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x1u8) as u8); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x48); rbx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b3820(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b1130; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415b16f0; const P5: u64 = 0x1415bcd90; const P6: u64 = 0x14161ad80; const P7: u64 = 0x141534ef0; const P8: u64 = 0x1415b1830; const P9: u64 = 0x1415b1260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; r12 = rcx.wrapping_add(0x8); rcx = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rbx = r9; r13 = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r13.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbp = rbx.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = r15; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rbp = rbx.wrapping_add(0x8); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rdi = r15; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rbp; bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rcx = rbx.wrapping_add(0x8); bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = rdi; fl_r = rdi; rsi = rdi.wrapping_sub(0x10); rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 54; continue; } bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rdi as u8)) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsi = rax; r12 = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r14 as u32)) as u32); rdx = rax.wrapping_add(0x10); rcx = rbx.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdi = (((r13 as u8)) as u32) as u32 as u64; bb = 44; continue; }
            30 => { crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((r15 as u32)) as u32); rax = ((r12 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; rsi = r15; bb = 33; continue; }
            33 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 35; continue; }
            35 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { rdx = r15; bb = 40; continue; }
            40 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rsi = rdx.wrapping_sub(0x10); bb = 43; continue; }
            42 => { rsi = r15; bb = 43; continue; }
            43 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((r15 as u32)) as u32); bb = 46; continue; }
            46 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { r13 = ((rsi.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 52; continue; }
            48 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 52; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x260u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 52; continue; } bb = 50; continue; }
            50 => { rcx = rsi.wrapping_add(0x10); let xa_src: u32 = (r12 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r12 = (xa_old) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 52; continue; }
            52 => { rcx = rsp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 53; continue; }
            53 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = rsi.wrapping_add(0x10); fl_l = rsi; fl_r = rsi; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { r8 }) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 57; continue; } bb = 55; continue; }
            55 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; r9 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 56; continue; }
            56 => { crate::simd_ops::wu64(r9, r8); bb = 57; continue; }
            57 => { rcx = r12; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 58; continue; }
            58 => { rax = ((r13 as u32)) as u32 as u64; bb = 59; continue; }
            59 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            60 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 58; continue; } bb = 61; continue; }
            61 => { rax = ((r13.wrapping_sub(0x5au64)) as u32) as u32 as u64; bb = 59; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b3dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b1130; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415b16f0; const P5: u64 = 0x1415bcd90; const P6: u64 = 0x141616340; const P7: u64 = 0x141534ef0; const P8: u64 = 0x1415b1830; const P9: u64 = 0x1415b1260; const P10: u64 = 0x1416192e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; rax = rcx.wrapping_add(0x8); rcx = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); rdi = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rbp = r8; r13 = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r13.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r15 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rdi.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rdi.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rsi; bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rcx = rdi.wrapping_add(0x8); bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 53; continue; } bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), ((r14 as u8)) as u8); r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rbp = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r15 as u32)) as u32); rdx = rax; rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { r12 = (((r13 as u8)) as u32) as u32 as u64; bb = 43; continue; }
            30 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x0u32) as u32); rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { r14 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 40; continue; }
            40 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 43; continue; }
            42 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if (((fl_l as u32)) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x0u32) as u32); bb = 45; continue; }
            45 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = rsp.wrapping_add(0x80); r13 = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 57; continue; }
            47 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 51; continue; } bb = 48; continue; }
            48 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 49; continue; }
            49 => { let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 56; continue; }
            51 => { rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 56; continue; } bb = 52; continue; }
            52 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 56; continue; } bb = 54; continue; }
            54 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((rax as u32)) as u32); r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { crate::simd_ops::wu64(r8, rbx); bb = 56; continue; }
            56 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = ((r13 as u32)) as u32 as u64; bb = 58; continue; }
            58 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b4920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b1130; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415b16f0; const P5: u64 = 0x1415bcd90; const P6: u64 = 0x141616340; const P7: u64 = 0x141534ef0; const P8: u64 = 0x1415b1830; const P9: u64 = 0x1415b1260; const P10: u64 = 0x1416192e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; rax = rcx.wrapping_add(0x8); rcx = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); rdi = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rbp = r8; r13 = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r13.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r15 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rdi.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rdi.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rsi; bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rcx = rdi.wrapping_add(0x8); bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 53; continue; } bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), ((r14 as u8)) as u8); r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rbp = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r15 as u32)) as u32); rdx = rax; rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { r12 = (((r13 as u8)) as u32) as u32 as u64; bb = 43; continue; }
            30 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x0u32) as u32); rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { r14 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 40; continue; }
            40 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 43; continue; }
            42 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if (((fl_l as u32)) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x0u32) as u32); bb = 45; continue; }
            45 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = rsp.wrapping_add(0x80); r13 = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 57; continue; }
            47 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 51; continue; } bb = 48; continue; }
            48 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 49; continue; }
            49 => { let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 56; continue; }
            51 => { rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 56; continue; } bb = 52; continue; }
            52 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 56; continue; } bb = 54; continue; }
            54 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((rax as u32)) as u32); r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { crate::simd_ops::wu64(r8, rbx); bb = 56; continue; }
            56 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = ((r13 as u32)) as u32 as u64; bb = 58; continue; }
            58 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b4ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b1130; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415b16f0; const P5: u64 = 0x1415bcd90; const P6: u64 = 0x1415f9930; const P7: u64 = 0x141534ef0; const P8: u64 = 0x1415fba90; const P9: u64 = 0x1415b1830; const P10: u64 = 0x1415b1260;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; r12 = rcx.wrapping_add(0x8); rcx = r12; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rbx = r9; r13 = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r13.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbp = rbx.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = r15; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rbp = rbx.wrapping_add(0x8); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rdi = r15; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rbp; bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rcx = rbx.wrapping_add(0x8); bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = rdi; fl_r = rdi; rsi = rdi.wrapping_sub(0x10); rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 54; continue; } bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), ((rdi as u8)) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsi = rax; r12 = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { r8 = rsp.wrapping_add(0x80); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((r14 as u32)) as u32); rdx = rax.wrapping_add(0x10); rcx = rbx.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rdi = (((r13 as u8)) as u32) as u32 as u64; bb = 44; continue; }
            30 => { crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((r15 as u32)) as u32); rax = ((r12 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rdi = ((crate::simd_ops::ru8(rsp.wrapping_add(0x80u64))) as u32) as u32 as u64; rsi = r15; bb = 33; continue; }
            33 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 35; continue; }
            35 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { rdx = r15; bb = 40; continue; }
            40 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; rsi = rdx.wrapping_sub(0x10); bb = 43; continue; }
            42 => { rsi = r15; bb = 43; continue; }
            43 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 44; continue; }
            44 => { fl_l = (crate::simd_ops::lk_add_u32(rbx, (1) as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((r15 as u32)) as u32); bb = 46; continue; }
            46 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 48; continue; } bb = 47; continue; }
            47 => { r13 = ((rsi.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 52; continue; }
            48 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 52; continue; } bb = 49; continue; }
            49 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = rsi; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 52; continue; } bb = 50; continue; }
            50 => { rcx = rsi.wrapping_add(0x10); let xa_src: u32 = (r12 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r12 = (xa_old) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 52; continue; }
            52 => { rcx = rsp.wrapping_add(0x88); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 60; continue; } bb = 53; continue; }
            53 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 54; continue; }
            54 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = rsi.wrapping_add(0x10); fl_l = rsi; fl_r = rsi; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { r8 }) as u64; rdx = (crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 57; continue; } bb = 55; continue; }
            55 => { rax = ((rdx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; r9 = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 56; continue; }
            56 => { crate::simd_ops::wu64(r9, r8); bb = 57; continue; }
            57 => { rcx = r12; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 58; continue; }
            58 => { rax = ((r13 as u32)) as u32 as u64; bb = 59; continue; }
            59 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            60 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 58; continue; } bb = 61; continue; }
            61 => { rax = ((r13.wrapping_sub(0x5au64)) as u32) as u32 as u64; bb = 59; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b5770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b1130; const P1: u64 = 0x1426452b0; const P2: u64 = 0x1426452a8; const P3: u64 = 0x142645560; const P4: u64 = 0x1415b16f0; const P5: u64 = 0x1415bcd90; const P6: u64 = 0x141616340; const P7: u64 = 0x141534ef0; const P8: u64 = 0x1415b1830; const P9: u64 = 0x1415b1260; const P10: u64 = 0x1416192e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; rax = rcx.wrapping_add(0x8); rcx = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); rdi = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rbp = r8; r13 = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((r13.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r15 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rdi.wrapping_add(0x8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rdi.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { rcx = rsi; bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rcx = rdi.wrapping_add(0x8); bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 53; continue; } bb = 27; continue; }
            27 => { rcx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rcx = ((rbx.wrapping_add(0x2u64)) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), ((r14 as u8)) as u8); r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rbp = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 28; continue; }
            28 => { r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r15 as u32)) as u32); rdx = rax; rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { r12 = (((r13 as u8)) as u32) as u32 as u64; bb = 43; continue; }
            30 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x0u32) as u32); rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { r14 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 43; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 40; continue; }
            40 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 43; continue; }
            42 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 43; continue; }
            43 => { fl_l = (crate::simd_ops::lk_add_u32(rdi, (1) as u32)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if (((fl_l as u32)) != 0) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x0u32) as u32); bb = 45; continue; }
            45 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { rcx = rsp.wrapping_add(0x80); r13 = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 57; continue; }
            47 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 51; continue; } bb = 48; continue; }
            48 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 51; continue; } bb = 49; continue; }
            49 => { let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 51; continue; } bb = 50; continue; }
            50 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 56; continue; }
            51 => { rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 56; continue; } bb = 52; continue; }
            52 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 53; continue; }
            53 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 56; continue; } bb = 54; continue; }
            54 => { rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((rax as u32)) as u32); r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 55; continue; }
            55 => { crate::simd_ops::wu64(r8, rbx); bb = 56; continue; }
            56 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = ((r13 as u32)) as u32 as u64; bb = 58; continue; }
            58 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b5a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a1e0; const P1: u64 = 0x1415b0c70; const P2: u64 = 0x68; const P3: u64 = 0x1426452b8; const P4: u64 = 0xec; const P5: u64 = 0xf0; const P6: u64 = 0x90; const P7: u64 = 0x98; const P8: u64 = 0xa0; const P9: u64 = 0xa8; const P10: u64 = 0xb0; const P11: u64 = 0xb8; const P12: u64 = 0x141613820;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rbx.wrapping_add(P4), (0x1u32) as u32);
    rcx = ((rbx.wrapping_add(P5)) as u64) as u64;
    crate::simd_ops::wu64(rbx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P10), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(P11), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(0xc0u64), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(0xc8u64), rax);
    crate::simd_ops::wu32(rbx.wrapping_add(0xd0u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rbx.wrapping_add(0xd8u64), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(0xe0u64), rax);
    crate::simd_ops::wu8(rbx.wrapping_add(0xe8u64), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b5af0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a1e0; const P1: u64 = 0x141613840; const P2: u64 = 0xc0; const P3: u64 = 0xc8; const P4: u64 = 0x1415242e0; const P5: u64 = 0xcc; const P6: u64 = 0x90; const P7: u64 = 0x98; const P8: u64 = 0x9c; const P9: u64 = 0x68; const P10: u64 = 0x1426452a0; const P11: u64 = 0x8; const P12: u64 = 0x1415b0ca0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rcx = ((rcx).wrapping_add(0xf0u64)) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    rdi = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P2), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rdi as u32)) as u32); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P6), rdi); crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rdi as u32)) as u32); }
    rcx = ((rbx.wrapping_add(P9)) as u64) as u64;
    rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P11)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b5b80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14268a1e0; const P1: u64 = 0x141613840; const P2: u64 = 0xc0; const P3: u64 = 0xc8; const P4: u64 = 0x1415242e0; const P5: u64 = 0xcc; const P6: u64 = 0x90; const P7: u64 = 0x98; const P8: u64 = 0x9c; const P9: u64 = 0x68; const P10: u64 = 0x1426452a0; const P11: u64 = 0x8; const P12: u64 = 0x1415b0ca0; const P13: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rbx = rcx; crate::simd_ops::wu64(rcx, rax); rdi = ((rdx as u32)) as u32 as u64; rcx = ((rcx).wrapping_add(0xf0u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P3), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P2), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P5), ((rsi as u32)) as u32); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(P7), ((rsi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P6), rsi); crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rsi as u32)) as u32); bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_add(P9)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P11)) as u64) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = 0xf8; rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b5c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x58; const P1: u64 = 0x5c; const P2: u64 = 0x1; const P3: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64;
    if (((fl_l as u32)) >= ((fl_r as u32))) { return rax; }
    rax = ((rcx.wrapping_add(P2)) as u32) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(P0), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rdx, r8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b5c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452b0; const P1: u64 = 0x1415bbb10; const P2: u64 = 0x1415b74b0; const P3: u64 = 0x1415bc250; const P4: u64 = 0x1415b70e0; const P5: u64 = 0x1415bccb0; const P6: u64 = 0x1426452a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r13 = ((r8 as u32)) as u32 as u64; r15 = rdx; rsi = rcx; r12 = 0x1; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdx.wrapping_add(0x1fu64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 38; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbx); rbx = rcx.wrapping_add(0x68); rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); rdi = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x98u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { rax = 0; fl_l = 0; fl_r = 0; bb = 13; continue; }
            13 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rax.wrapping_add(0x10u64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            15 => { rax = rdx.wrapping_add(0x10); bb = 13; continue; }
            16 => { rdx = ((r8 as u32)) as u32 as u64; rcx = rsi.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rcx as u32)) as u32); bb = 18; continue; }
            18 => { rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 19; continue; }
            19 => { r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = ((rdi).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu64(rax, rbp); crate::simd_ops::wu8(rax.wrapping_add(0xcu64), (0x3u8) as u8); fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 20; continue; }
            20 => { bb = 36; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r12 = 0x34; bb = 24; continue; }
            22 => { r12 = 0x34; bb = 24; continue; }
            23 => { r12 = 0x1f; bb = 24; continue; }
            24 => { rbp = ((r14.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x98u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(r15)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 35; continue; } bb = 26; continue; }
            26 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x90u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 39; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 29; continue; }
            29 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { rdx = r15; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = rsi.wrapping_add(0x90); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { r15 = ((r15).wrapping_add(0x18u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 36; continue; }
            36 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 37; continue; }
            37 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 38; continue; }
            38 => { rax = ((r12 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            39 => { rdi = rax.wrapping_add(0x10); bb = 30; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b5e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158ec20; const P1: u64 = 0x14183acc8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; rdi = r8; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r11 = (crate::simd_ops::ru64(rcx)) as u64; r10 = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(1u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r9 as u32))) as u32 as u64; r8 = ((((rcx as u32)) as i32 as i64) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(r11))) as u64; rdx = ((rdx).wrapping_shr(32u32)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 3; continue; }
            3 => { if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { r9 = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = (((rax as u32)).wrapping_sub(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; bb = 5; continue; }
            5 => { fl_l = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; if (((fl_l as u32)) != 0) { bb = 2; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rdi, (0x0u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rdi = ((((r9 as u32)) as i32 as i64) as u64) as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; fl_l = ((rdi).wrapping_add(rax)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 8; continue; }
            8 => { rdi = ((rdi).wrapping_sub(rax)) as u64; rdi = ((((rdi) as i64).wrapping_shr(5u32) as u64)) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 10; continue; }
            10 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rsi = ((rdi as u32)) as u32 as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = (((rax as u32)).wrapping_sub((rdi as u32))) as u32 as u64; rcx = rdx.wrapping_add(0x20); rax = (((rax as u32)).wrapping_shl(5u32)) as u32 as u64; r8 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            13 => { crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x8u64), (1) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rax = ((rax).wrapping_add(rsi)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            14 => { crate::simd_ops::wu8(rdi, (0x1u8) as u8); rax = (crate::simd_ops::ru64(rbx)) as u64; rdi = ((r8.wrapping_add(rax)) as u64) as u64; bb = 7; continue; }
            15 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b5f80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b74b0; const P1: u64 = 0x10; const P2: u64 = 0x8; const P3: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rdx;
    rbx = ((r9 as u32)) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rdi = ((r8 as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0x34; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rax, rsi);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P1), ((rbx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P2), ((rdi as u32)) as u32);
    crate::simd_ops::wu8(rax.wrapping_add(P3), (0x1u8) as u8);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415b5fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x142645718;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = r8; r9 = ((rdx as u32)) as u32 as u64; r8 = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = 0; fl_l = 0; fl_r = 0; r9 = ((rbx.wrapping_add(P0)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = 0xfde9; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = (0xffffffffffffffffu64) as u64; fl_l = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; rdx = ((fl_l as u32)) as u32 as u64; rcx = rax; bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rcx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r8.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 6; continue; } bb = 7; continue; }
            7 => { bb = 9; continue; }
            8 => { rax = ((rdx as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = 0xfde9; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r9 as u32)) as u32); r9 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rbx), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
