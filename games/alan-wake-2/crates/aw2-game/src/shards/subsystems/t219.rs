#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_1415837f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415810d0; const P1: u64 = 0x14157f8e0; const P2: u64 = 0x1415834a0; const P3: u64 = 0x142645718; const P4: u64 = 0x1415864b0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 51>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x180u64)) as u64; rsi = r9; rbp = ((r8 as u32)) as u32 as u64; r14 = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r13 = 0; fl_l = 0; fl_r = 0; r15 = ((rax as u32)) as u32 as u64; rcx = ((r13 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x1c8u64))) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d0u64))) as u64; r9 = rsp.wrapping_add(0x40); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x1c8u64))) as u32 as u64; r8 = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((r13 as u32)) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rcx = rsp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); bb = 5; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x1d8u64))) as u32 as u64; r9 = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); r8 = ((rbp as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x1c0u64))) as u64; rdx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rcx = (0xffffffffffffffffu64) as u64; rdx = rcx; bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(rdx.wrapping_mul(2)))) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x102; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(rcx.wrapping_mul(2)))) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 9; continue; } bb = 10; continue; }
            10 => { bb = 12; continue; }
            11 => { rcx = 0x102; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r13); rax = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r9 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x103u32) as u32); r8 = rbx; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = 0xfde9; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(rdx).wrapping_add(0x70u64), ((r13 as u8)) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x173u64), ((r13 as u8)) as u8); fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x1c8u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { rax = ((rsi as u32)) as u32 as u64; bb = 15; continue; }
            15 => { r11 = rsp.wrapping_add(0x180); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141583960(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x14157f8e0; const P6: u64 = 0x143930f18; const P7: u64 = 0x143930f90; const P8: u64 = 0x141599d60; const P9: u64 = 0x1415b16f0; const P10: u64 = 0x143930f78; const P11: u64 = 0x1415f3790; const P12: u64 = 0x1415a56a0; const P13: u64 = 0x14159b3d0; const P14: u64 = 0x1415864b0; const P15: u64 = 0x1415b1830;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x80u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = ((r9 as u32)) as u32 as u64; r13 = r8; r15 = ((rdx as u32)) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 33; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rsi); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsi = rbx.wrapping_add(0x110); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x148u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = r14; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x110); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 13; continue; }
            13 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            14 => { rdi = r14; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 23; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 17; continue; } bb = 19; continue; }
            19 => { bb = 24; continue; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            23 => { rsi = rbx.wrapping_add(0x110); bb = 24; continue; }
            24 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xb0u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), r14); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((r14 as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; r9 = rbp.wrapping_sub(0x3c); r8 = rbp.wrapping_sub(0x38); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); bb = 30; continue; }
            29 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), r14); bb = 30; continue; }
            30 => { rax = 0x1; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(P6, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rdx = rbp.wrapping_sub(0x30); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; rcx = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x24u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r13); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 35; continue; } bb = 31; continue; }
            31 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rax = 0; fl_l = 0; fl_r = 0; bb = 34; continue; }
            34 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            35 => { rcx = rbp.wrapping_sub(0x40); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(P10)) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rcx = rbp.wrapping_sub(0x20); r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; rdx = rax; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x24u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); rcx = rdi; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P7)) as u64; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 39; continue; }
            39 => { rbx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rcx = rbp.wrapping_sub(0x40); rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; bb = 34; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141583be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415ab0b0; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415aa7b0; const P8: u64 = 0x143930f90; const P9: u64 = 0x14159b7f0; const P10: u64 = 0x143930f18; const P11: u64 = 0x141599f20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x48u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = r8; r12 = (((r9 as u16)) as u32) as u32 as u64; r13 = rdx; r15 = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); fl_l = ((r12 as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 33; continue; } bb = 3; continue; }
            3 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbp); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rbp = rbx.wrapping_add(0x110); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x148u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { rdi = r14; bb = 13; continue; }
            13 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            15 => { rbp = rbx.wrapping_add(0x110); rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 8; continue; } bb = 16; continue; }
            16 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            17 => { rdi = r14; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 20; continue; } bb = 22; continue; }
            22 => { bb = 27; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            26 => { rbp = rbx.wrapping_add(0x110); bb = 27; continue; }
            27 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (((rax as u16)) as u32) as u32 as u64; rcx = ((r12.wrapping_add(r12.wrapping_mul(4))) as u32) as u32 as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = 0xffff; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 35; continue; } bb = 30; continue; }
            30 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rax = 0; fl_l = 0; fl_r = 0; bb = 34; continue; }
            34 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x48u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            35 => { rdx = 0x23; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 36; continue; }
            36 => { rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x90u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rax.wrapping_add(0x1cu64), ((rcx as u16)) as u16); crate::simd_ops::wu64(rax.wrapping_add(0xcu64), rdi); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x4u64), r13); crate::simd_ops::wu16(rax.wrapping_add(0x1eu64), ((r12 as u16)) as u16); fl_l = ((r14 as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 39; continue; } bb = 37; continue; }
            37 => { rdx = rax; rcx = r12; fl_l = rdx; fl_r = rsi; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 38; continue; }
            38 => { xmm0 = (crate::simd_ops::ru128(rsi)) as u128; rsi = rsi.wrapping_add(0x14); crate::simd_ops::wu128(rdx.wrapping_add(rsi).wrapping_add(0xcu64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rsi.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(rsi).wrapping_add(0x1cu64), ((rax as u32)) as u32); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 39; continue; }
            39 => { rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0xb0u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((r14 as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { rcx = (crate::simd_ops::ru64(P8)) as u64; r9 = ((r15 as u32)) as u32 as u64; r8 = r13; rdx = ((rsi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rsi as u32)) as u32); bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 45; continue; } bb = 43; continue; }
            43 => { rdx = 0x1; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(P10, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rdx as u32)) as u32); r8 = r13; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P8)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rax = 0x40; crate::simd_ops::wu16(rbx.wrapping_add(0x2u64), ((rax as u16)) as u16); rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; bb = 31; continue; }
            45 => { rcx = (crate::simd_ops::ru64(P6)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_l = (crate::simd_ops::lk_sub_u32(rcx.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; bb = 34; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141583eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415ab0b0; const P6: u64 = 0x143930f08; const P7: u64 = 0x143930f90; const P8: u64 = 0x14159b7f0; const P9: u64 = 0x143930f18; const P10: u64 = 0x141599f20; const P11: u64 = 0x1415b16f0; const P12: u64 = 0x143814e28; const P13: u64 = 0x143930f78; const P14: u64 = 0x1415f3730; const P15: u64 = 0x1415a7180; const P16: u64 = 0x14159b3d0; const P17: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbp = rsp.wrapping_add(0x40); fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = r9; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rbx); r12 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rdi); if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 54; continue; }
            2 => { r13 = ((crate::simd_ops::ru16(rbp.wrapping_add(0x60u64))) as u32) as u32 as u64; fl_l = ((r13 as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 53; continue; } bb = 3; continue; }
            3 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(P1)) as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rsi = rdi.wrapping_add(0x110); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x150u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x148u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { rbx = r15; bb = 13; continue; }
            13 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            15 => { rsi = rdi.wrapping_add(0x110); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 8; continue; } bb = 16; continue; }
            16 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            17 => { rbx = r15; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 20; continue; } bb = 22; continue; }
            22 => { bb = 27; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            26 => { rsi = rdi.wrapping_add(0x110); bb = 27; continue; }
            27 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            28 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 29; continue; }
            29 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (((rax as u16)) as u32) as u32 as u64; rcx = ((r13.wrapping_mul(4)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(P6)) as u64; rcx = (((rcx as u32)).wrapping_add((r13 as u32))) as u32 as u64; r8 = ((rdx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32 as u64; rax = 0xffff; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdx = (if (((fl_l as u32)) > ((fl_r as u32))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 34; continue; } bb = 30; continue; }
            30 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 31; continue; }
            31 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 33; continue; }
            33 => { rax = 0; fl_l = 0; fl_r = 0; bb = 54; continue; }
            34 => { rax = ((r8 as u32)) as u32 as u64; rcx = r8.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rcx = (0xffffffffffffff0u64) as u64; bb = 36; continue; }
            36 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_sub(rcx)) as u64; rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x68u64))) as u32) as u32 as u64; rdi = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rdi, r8); crate::simd_ops::wu16(rdi.wrapping_add(0x18u64), ((rax as u16)) as u16); crate::simd_ops::wu16(rdi.wrapping_add(0x1au64), ((r13 as u16)) as u16); fl_l = ((r15 as u16)) as u16 as u64; fl_r = ((r13 as u16)) as u16 as u64; if (((fl_l as u16)) >= ((fl_r as u16))) { bb = 39; continue; } bb = 37; continue; }
            37 => { rdx = rdi; rcx = r13; fl_l = rdx; fl_r = r14; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 38; continue; }
            38 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; r14 = r14.wrapping_add(0x14); crate::simd_ops::wu128(rdx.wrapping_add(r14).wrapping_add(0x8u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(r14.wrapping_sub(0x4u64))) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(r14).wrapping_add(0x18u64), ((rax as u32)) as u32); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 39; continue; }
            39 => { rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r15 as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { rcx = (crate::simd_ops::ru64(P7)) as u64; r9 = ((r12 as u32)) as u32 as u64; rdx = ((rsi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rsi as u32)) as u32); bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 46; continue; } bb = 43; continue; }
            43 => { rdx = 0x1; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(P9, xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P7)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 45; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu32(rdi.wrapping_add(0x14u64), ((r15 as u32)) as u32); bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 31; continue; } bb = 46; continue; }
            46 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P12)) as u64; r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 50; continue; } bb = 47; continue; }
            47 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r15 = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; r9 = rax; r8 = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 50; continue; }
            50 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(P7)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 52; continue; } bb = 51; continue; }
            51 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 52; continue; }
            52 => { rcx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = ((r15 as u32)) as u32 as u64; bb = 54; continue; }
            53 => { rax = 0x1f; bb = 54; continue; }
            54 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsp = rbp.wrapping_add(0x10); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141584200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141200330; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xc; const P6: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rdx;
    rdi = ((rcx as u32)) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0xa;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P5), ((rdi as u32)) as u32);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P6), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141584270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xffffffffffffffbf; const P2: u64 = 0x1; const P3: u64 = 0x141584200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rdx; r10 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r8 = 0x811c9dc5; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r8 = ((((r8 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r9.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r10 = ((r10.wrapping_add(P2)) as u64) as u64; r9 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = r11; rcx = ((r8 as u32)) as u32 as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rax = 0xe; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415842e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415810d0; const P1: u64 = 0x141584200;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = rbx; rcx = ((rax as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0xe;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141584310(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f88; const P1: u64 = 0x141524330; const P2: u64 = 0x14183acc2; const P3: u64 = 0x1426452b8; const P4: u64 = 0x142645290; const P5: u64 = 0x1415d2f80; const P6: u64 = 0x1415b1c60; const P7: u64 = 0x1415b19f0; const P8: u64 = 0x143930fa0; const P9: u64 = 0x1415c4670; const P10: u64 = 0x143930f48; const P11: u64 = 0x1415fce00; const P12: u64 = 0x143930f00; const P13: u64 = 0x143931154; const P14: u64 = 0x141580020; const P15: u64 = 0x143930f50; const P16: u64 = 0x1415f8c80; const P17: u64 = 0x1415b5a50; const P18: u64 = 0x143930f90; const P19: u64 = 0x1415f73d0; const P20: u64 = 0x14159a780; const P21: u64 = 0x143930f68; const P22: u64 = 0x1415d0c50; const P23: u64 = 0x143930f78; const P24: u64 = 0x1415f31f0; const P25: u64 = 0x1415f38d0; const P26: u64 = 0x143930f70; const P27: u64 = 0x1415d9400; const P28: u64 = 0x143930f80; const P29: u64 = 0x143930fa8; const P30: u64 = 0x14150d070; const P31: u64 = 0x1439310f8; const P32: u64 = 0x1415f8060; const P33: u64 = 0x143930f60; const P34: u64 = 0x1415af350; const P35: u64 = 0x1415afae0; const P36: u64 = 0x143931cf0; const P37: u64 = 0x1415fe7c0; const P38: u64 = 0x142646a40; const P39: u64 = 0x1439310e8; const P40: u64 = 0x1439310f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r14 = rcx; fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = rbx; rdi = 0x1; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = 0x4d0; rcx = ((rdi.wrapping_add(0xfu64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x4d0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((rdi.wrapping_add(0xdu64)) as u32) as u32 as u64; rsi = r15.wrapping_add(0x4c); bb = 3; continue; }
            3 => { rcx = rsi.wrapping_sub(0x44); crate::simd_ops::wu32(rsi.wrapping_sub(0x4cu64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsi.wrapping_sub(0x48u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi.wrapping_sub(0x1cu64), ((rbx as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_sub(0x14u64), rax); crate::simd_ops::wu64(rsi.wrapping_sub(0xcu64), rbx); crate::simd_ops::wu64(rsi.wrapping_sub(0x4u64), rbx); crate::simd_ops::wu32(rsi.wrapping_add(0x4u64), ((rbx as u32)) as u32); rsi = rsi.wrapping_add(0x58); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rcx = r15; crate::simd_ops::wu64(P0, r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = 0x34; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { crate::simd_ops::wu64(P0, rbx); rdi = 0x34; bb = 69; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = r14.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(P8)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = 0xf0; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0xf0; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rdi.wrapping_add(0x20u64), rbx); rcx = rdi; crate::simd_ops::wu32(rdi.wrapping_add(0x28u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rdi.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x38u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x40u64), ((rbx as u32)) as u32); crate::simd_ops::wu16(rdi.wrapping_add(0x48u64), ((rbx as u16)) as u16); crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x78u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xa8u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xb0u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xb8u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xc0u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xc8u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xd0u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xd8u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xe0u64), rbx); crate::simd_ops::wu64(rdi.wrapping_add(0xe8u64), rbx); crate::simd_ops::wu64(P8, rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(P10)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { rdx = 0x98; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 14; continue; }
            14 => { xmm0 = 0u128; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rdx, (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x10u64), (xmm0) as u128); rcx = ((rax.wrapping_add(0x6u64)) as u32) as u32 as u64; crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x40u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu128(rdx.wrapping_add(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rdx.wrapping_add(0x90u64), rax); rax = rdx.wrapping_add(0x8); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); rax = rax.wrapping_add(0x18); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rcx = rdx; crate::simd_ops::wu64(P10, rdx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru64(P12)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(P13)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; rcx = 0x10; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 19; continue; }
            19 => { rax = P14; rdx = 0x158; crate::simd_ops::wu64(P15, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 20; continue; }
            20 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            21 => { crate::simd_ops::wu64(P8, rbx); rdi = 0x34; bb = 69; continue; }
            22 => { crate::simd_ops::wu64(P10, rbx); rdi = 0x34; bb = 69; continue; }
            23 => { rcx = rbx; bb = 27; continue; }
            24 => { rdx = 0xf8; crate::simd_ops::wu64(P15, rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 56; continue; } bb = 25; continue; }
            25 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { rcx = rax; bb = 27; continue; }
            27 => { crate::simd_ops::wu64(P12, rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 57; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru64(P18)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rdx = 0xa0; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 58; continue; } bb = 31; continue; }
            31 => { xmm0 = 0u128; rcx = rdi.wrapping_add(0x18); crate::simd_ops::wu128(rdi.wrapping_add(0x14u64), (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu128(rdi.wrapping_add(0x24u64), (xmm0) as u128); crate::simd_ops::wu128(rdi.wrapping_add(0x34u64), (xmm0) as u128); crate::simd_ops::wu128(rdi.wrapping_add(0x44u64), (xmm0) as u128); crate::simd_ops::wu128(rdi.wrapping_add(0x54u64), (xmm0) as u128); crate::simd_ops::wu128(rdi.wrapping_add(0x64u64), (xmm0) as u128); crate::simd_ops::wu128(rdi.wrapping_add(0x74u64), (xmm0) as u128); crate::simd_ops::wu128(rdi.wrapping_add(0x84u64), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x94u64), rax); crate::simd_ops::wu32(rdi.wrapping_add(0x9cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(rdi, rbx); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rbx); crate::simd_ops::wu32(rdi.wrapping_add(0x10u64), ((rbx as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi.wrapping_add(0x48); crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; crate::simd_ops::wu64(P18, rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru64(P21)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 35; continue; } bb = 33; continue; }
            33 => { rdx = 0x50; rcx = ((rdx.wrapping_sub(0x40u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 34; continue; }
            34 => { crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rbx); rcx = rax.wrapping_add(0x20); crate::simd_ops::wu64(rax.wrapping_add(0x28u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x38u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x48u64), rbx); crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rdi; crate::simd_ops::wu64(P21, rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 35; continue; }
            35 => { fl_l = (crate::simd_ops::ru64(P23)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 36; continue; }
            36 => { rdx = 0x40; rcx = ((rdx.wrapping_sub(0x30u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 37; continue; }
            37 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P24, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P23, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 61; continue; } bb = 38; continue; }
            38 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P25, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 39; continue; }
            39 => { fl_l = (crate::simd_ops::ru64(P26)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 42; continue; } bb = 40; continue; }
            40 => { rdx = 0x8; rcx = ((rdx.wrapping_add(0x8u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 62; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu64(rax, rbx); rcx = rax; crate::simd_ops::wu64(P26, rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P27, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru64(P28)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 43; continue; }
            43 => { rdx = 0x20; rcx = ((rdx.wrapping_sub(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 44; continue; }
            44 => { crate::simd_ops::wu64(rax, rbx); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(P28, rax); bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru64(P29)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 49; continue; } bb = 46; continue; }
            46 => { rdx = 0x18; rcx = ((rdx.wrapping_sub(0x8u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 47; continue; }
            47 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P30, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P29, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 48; continue; }
            48 => { rdx = (crate::simd_ops::ru32(P31)) as u32 as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P32, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::ru64(P33)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 53; continue; } bb = 50; continue; }
            50 => { rdx = 0xb0; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 66; continue; } bb = 51; continue; }
            51 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P34, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P33, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 67; continue; } bb = 52; continue; }
            52 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P35, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 53; continue; }
            53 => { fl_l = (crate::simd_ops::ru64(P36)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 68; continue; } bb = 54; continue; }
            54 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P37, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(P36)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 68; continue; } bb = 55; continue; }
            55 => { rdi = 0x34; bb = 69; continue; }
            56 => { crate::simd_ops::wu64(P12, rbx); bb = 57; continue; }
            57 => { rdi = 0x34; bb = 69; continue; }
            58 => { crate::simd_ops::wu64(P18, rbx); rdi = 0x34; bb = 69; continue; }
            59 => { crate::simd_ops::wu64(P21, rbx); rdi = 0x34; bb = 69; continue; }
            60 => { crate::simd_ops::wu64(P23, rbx); bb = 61; continue; }
            61 => { rdi = 0x34; bb = 69; continue; }
            62 => { crate::simd_ops::wu64(P26, rbx); rdi = 0x34; bb = 69; continue; }
            63 => { crate::simd_ops::wu64(P28, rbx); rdi = 0x34; bb = 69; continue; }
            64 => { crate::simd_ops::wu64(P29, rbx); bb = 65; continue; }
            65 => { rdi = 0x34; bb = 69; continue; }
            66 => { crate::simd_ops::wu64(P33, rbx); bb = 67; continue; }
            67 => { rdi = 0x34; bb = 69; continue; }
            68 => { rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P38), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P39, rax); bb = 69; continue; }
            69 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 71; continue; } bb = 70; continue; }
            70 => { rax = P40; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(r14, (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x30u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x40u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x50u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x70u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x80u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x90u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0x90u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(0xa0u64))) as u128; crate::simd_ops::wu128(r14.wrapping_add(0xa0u64), (xmm1) as u128); bb = 71; continue; }
            71 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415849c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141584f40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9);
    r9 = rsp.wrapping_add(0x40);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64);
    crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141584a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f00; const P2: u64 = 0x143930f50;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = ((r9 as u32)) as u32 as u64; rsi = ((r8 as u32)) as u32 as u64; r14 = ((rdx as u32)) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 8; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbp.wrapping_sub(0x59); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rdx); crate::simd_ops::wu8(rbp.wrapping_sub(0x61u64), ((rdx as u8)) as u8); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x69u64))) as u128; rax = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rcx); rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0xfffffffd; crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((rdx as u32)) as u32); rdx = rbp.wrapping_add(0x7); rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm1) as u128); rax = (((rax as u32)).wrapping_add(0x6u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x41u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((rax as u32)) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; rax = rbp.wrapping_sub(0x59); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((rbx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x49u64))) as u128; crate::simd_ops::wu8(rbp.wrapping_sub(0xdu64), (if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x9u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), ((rsi as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x47u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; bb = 7; continue; }
            6 => { r8 = 0x64; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rbp.wrapping_sub(0x59); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141584f40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x143930f00; const P2: u64 = 0x18;
    let (mut rax, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0xc8u64)) as u64; rax = ((r8.wrapping_add(P0)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm1 = (crate::simd_ops::ru128(r9)) as u128; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0xfffffffd; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x0u32) as u32); rax = 0x0; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = (((rax as u32)).wrapping_add(0x6u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x80u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x5cu64), (if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdx as u32)) as u32); rdx = r11.wrapping_sub(0x58); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r8 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x38u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x48u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(r11.wrapping_sub(0x18u64), (xmm0) as u128); crate::simd_ops::wu128(r11.wrapping_sub(0x28u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0xc8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rax = 0x64; rsp = ((rsp).wrapping_add(0xc8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585010(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f00; const P2: u64 = 0x20; const P3: u64 = 0x143930f50; const P4: u64 = 0x141585220; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = rdx; rsi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(P3)) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); r9 = rdi; rcx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rsp.wrapping_add(0x30); r8 = ((rax as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415850a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141585220;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = r9;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
    r9 = rdx;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x1u8) as u8);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r8 as u32)) as u32);
    rdx = rax;
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585220(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14183acbc; const P2: u64 = 0x143930f00; const P3: u64 = 0x1415242e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; r14 = r9; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rdi.wrapping_add(0x1fu64)) as u32) as u32 as u64; bb = 14; continue; }
            2 => { r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), rbx); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xd8u64), rsi); rax = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x55u64), r9); rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r9); rax = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), ((r9 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x29u64), ((r9 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), r8); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rdx); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rdi as u32)) as u32); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(r14)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; bb = 9; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x7fu64))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rsi = rdi; rcx = 0; fl_l = 0; fl_r = 0; rsi = ((rsi).wrapping_shl(2u32)) as u64; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 13; continue; }
            7 => { r8 = rsi; rdx = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            8 => { rbx = r14; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rbx); bb = 9; continue; }
            9 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; rdx = rbp.wrapping_sub(0x9); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x49u64))) as u128; rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rax = ((rsi as u32)) as u32 as u64; bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 14; continue; }
            14 => { r11 = rsp.wrapping_add(0xc0); rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f00; const P2: u64 = 0x20; const P3: u64 = 0x143930f50; const P4: u64 = 0x141585d70; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r9; rdi = ((r8 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rbp = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 4; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; r9 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); r8 = (r8 & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rsp.wrapping_add(0x40); r8 = ((rax as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141585d70;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    r10 = ((r8 as u32)) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x48u64)) as u64; return rax; }
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64;
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x1u8) as u8);
    r8 = (r8 & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9);
    r9 = ((rdx as u32)) as u32 as u64;
    rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r10 as u32)) as u32);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141585350; const P2: u64 = 0x141524330; const P3: u64 = 0x143930f00; const P4: u64 = 0x143930f50; const P5: u64 = 0x1415242e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp.wrapping_sub(0x10); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = r9; r10 = r8; r11 = ((rdx as u32)) as u32 as u64; r14 = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x110u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x100u64), r12); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 3; continue; }
            3 => { r12 = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x140u64), rbx); rbx = 0x811c9dc5; crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), r15); r15 = 0; fl_l = 0; fl_r = 0; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rsi = ((r15 as u32)) as u32 as u64; bb = 8; continue; }
            6 => { r8 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rsi = ((rbx as u32)) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rsi = ((((rsi as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rsi = ((((rsi as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 15; continue; } bb = 9; continue; }
            9 => { r9 = (crate::simd_ops::ru64(r9)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { rbx = ((((rbx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r9 = r9.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rbx = ((((rbx as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 13; continue; }
            13 => { r15 = ((rbx as u32)) as u32 as u64; bb = 14; continue; }
            14 => { r9 = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); r8 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); rdx = ((r11 as u32)) as u32 as u64; rcx = ((r14 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            15 => { rdx = ((r12.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), r13); rcx = 0; fl_l = 0; fl_r = 0; r14 = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 16; continue; }
            16 => { fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { r11 = rax; bb = 18; continue; }
            18 => { r10 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { r8 = ((r15 as u32)) as u32 as u64; bb = 23; continue; }
            20 => { r9 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { bb = 22; continue; }
            22 => { r8 = ((((r8 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r9.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r9.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); r9 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 22; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu32(r11, ((r8 as u32)) as u32); rdi = ((rdi).wrapping_add(0x8u64)) as u64; r11 = ((r11).wrapping_add(0x4u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rbp.wrapping_sub(0x70); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(P4)) as u64; rdx = rbp.wrapping_sub(0x60); rcx = (crate::simd_ops::ru64(P3)) as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x30u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = rbp.wrapping_sub(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu8(rbp.wrapping_sub(0x80u64), (if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x44u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x4u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((r15 as u8)) as u8); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x50u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x7cu64), ((rsi as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x7cu64), ((r12 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), r13); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x30u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rdx = r13; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rbp.wrapping_sub(0x70); r8 = ((rbx as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            28 => { rax = 0x34; bb = 29; continue; }
            29 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; bb = 30; continue; }
            30 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x140u64))) as u64; bb = 32; continue; }
            31 => { rax = 0x1f; bb = 32; continue; }
            32 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; rsp = ((rsp).wrapping_add(0x110u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f00; const P2: u64 = 0x141524330; const P3: u64 = 0x141585d70;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rbp = rsp.wrapping_sub(0x2f); rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = r9; r10 = r8; r13 = ((rdx as u32)) as u32 as u64; r11 = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0xf0u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), r15); fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 3; continue; }
            3 => { r15 = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x118u64), rbx); rbx = 0x811c9dc5; crate::simd_ops::wu64(rsp.wrapping_add(0x120u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0xe8u64), r14); r14 = 0; fl_l = 0; fl_r = 0; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rsi = ((r14 as u32)) as u32 as u64; bb = 9; continue; }
            6 => { r8 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rsi = ((rbx as u32)) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rsi = ((((rsi as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rsi = ((((rsi as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 10; continue; }
            10 => { r9 = (crate::simd_ops::ru64(r9)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rbx = ((r14 as u32)) as u32 as u64; bb = 15; continue; }
            12 => { r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rbx = ((((rbx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r9 = r9.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rbx = ((((rbx as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 14; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; rdx = rbp.wrapping_sub(0x31); rcx = (crate::simd_ops::ru64(P1)) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x7du64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x4u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x61u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), r14); crate::simd_ops::wu8(rbp.wrapping_sub(0x69u64), ((r14 as u8)) as u8); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x71u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((rsi as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x51u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x3du64), (0x1u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rbx as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x41u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xfu64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 28; continue; }
            16 => { rdx = ((r15.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x128u64), r12); rcx = 0; fl_l = 0; fl_r = 0; r14 = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 17; continue; }
            17 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { r11 = rax; bb = 19; continue; }
            19 => { r10 = (crate::simd_ops::ru64(rdi)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 24; continue; }
            21 => { r9 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { r8 = ((((r8 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r9.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r9.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); r9 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 23; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu32(r11, ((r8 as u32)) as u32); rdi = ((rdi).wrapping_add(0x8u64)) as u64; r11 = ((r11).wrapping_add(0x4u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 19; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x4fu64))) as u32 as u64; fl_r = 0x0; r9 = ((r13 as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; r8 = (r8 & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rsi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            26 => { rax = 0x34; bb = 27; continue; }
            27 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; bb = 28; continue; }
            28 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe8u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; bb = 30; continue; }
            29 => { rax = 0x1f; bb = 30; continue; }
            30 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; rsp = ((rsp).wrapping_add(0xf0u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415859c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415810d0; const P2: u64 = 0x141585350; const P3: u64 = 0x141524330; const P4: u64 = 0x143930f00; const P5: u64 = 0x143930f50; const P6: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0xf8u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r9; r13 = ((rdx as u32)) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0xf8u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x138u64), r14); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x7fu64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 4; continue; }
            4 => { rcx = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x130u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = ((rax as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); r8 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); rdx = ((r13 as u32)) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; bb = 18; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x128u64), rsi); rdx = ((r14.wrapping_mul(4)) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), r15); rsi = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdi = rax; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = rdi.wrapping_add(0x4); rbx = rbx.wrapping_add(0x8); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x5fu64))) as u32 as u64; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = rbp.wrapping_sub(0x31); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = rbp.wrapping_sub(0x21); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x4u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x7du64), rax); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rax); crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8); crate::simd_ops::wu8(rbp.wrapping_sub(0x69u64), ((rax as u8)) as u8); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x71u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), rax); rax = rbp.wrapping_sub(0x31); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x61u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((r12 as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x51u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x3du64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r15); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x41u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x1fu64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0xfu64), (xmm1) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = r15; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(P4)) as u64; rdx = rbp.wrapping_sub(0x31); r8 = ((rbx as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            15 => { rax = 0x34; bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x130u64))) as u64; bb = 18; continue; }
            17 => { rax = 0x1f; bb = 18; continue; }
            18 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x138u64))) as u64; rsp = ((rsp).wrapping_add(0xf8u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415810d0; const P2: u64 = 0x143930f00; const P3: u64 = 0x141524330; const P4: u64 = 0x141585d70;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rbp = rsp.wrapping_sub(0x27); rsp = ((rsp).wrapping_sub(0xe8u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r9; r13 = ((rdx as u32)) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0xe8u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x128u64), r14); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru32(rbp.wrapping_add(0x6fu64))) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x120u64), r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = ((rax as u32)) as u32 as u64; fl_l = ((r14 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x7du64), rcx); rdx = rbp.wrapping_sub(0x31); crate::simd_ops::wu64(rbp.wrapping_sub(0x71u64), rcx); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_sub(0x69u64), ((rcx as u8)) as u8); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x71u64))) as u128; crate::simd_ops::wu8(rbp.wrapping_sub(0x41u64), (if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), ((rcx as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x61u64), rcx); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rcx); rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x21u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x4u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x61u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x11u64), (xmm0) as u128); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((r12 as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x51u64))) as u128; crate::simd_ops::wu32(rbp.wrapping_sub(0x3du64), ((r14 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x41u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0xfu64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; bb = 15; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x118u64), rsi); rdx = ((r14.wrapping_mul(4)) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), r15); rsi = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdi = rax; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi, ((rax as u32)) as u32); rdi = rdi.wrapping_add(0x4); rbx = rbx.wrapping_add(0x8); fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru32(rbp.wrapping_add(0x4fu64))) as u32 as u64; bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x7fu64))) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x77u64))) as u64; r9 = ((r13 as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); r8 = (r8 & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 13; continue; }
            12 => { rax = 0x34; bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x118u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x120u64))) as u64; bb = 15; continue; }
            14 => { rax = 0x1f; bb = 15; continue; }
            15 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x128u64))) as u64; rsp = ((rsp).wrapping_add(0xe8u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585d70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141524330; const P1: u64 = 0x14183acbc; const P2: u64 = 0x143930f00; const P3: u64 = 0x1415242e0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rax.wrapping_sub(0x3f); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x77u64))) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r14); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x65u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); crate::simd_ops::wu8(rbp.wrapping_sub(0x51u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x67u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x2du64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x69u64), (0x4u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rdx); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), ((r8 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x25u64), ((rsi as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rax as u32)) as u32); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x21u64))) as u64; bb = 11; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x7fu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 5; continue; }
            5 => { r14 = rsi; rcx = 0; fl_l = 0; fl_r = 0; r14 = ((r14).wrapping_shl(2u32)) as u64; rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rax); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { rax = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xd0u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xd8u64))) as u64; bb = 8; continue; }
            8 => { r11 = rsp.wrapping_add(0xc0); rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { r8 = r14; rdx = rdi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            10 => { rbx = rdi; crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rbx); bb = 11; continue; }
            11 => { xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x69u64))) as u128; rdx = rbp.wrapping_sub(0x19); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x59u64))) as u128; rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x19u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x49u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x9u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x39u64))) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rdx = rbx; rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { rax = ((rdi as u32)) as u32 as u64; bb = 7; continue; }
            15 => { rax = 0x1f; bb = 8; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1439310e8;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rcx = (0x5851f42d4c957f2du64) as u64;
    rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64;
    rax = ((rax).wrapping_add(1)) as u64;
    crate::simd_ops::wu64(P0, rax);
    fl_l = ((rax).wrapping_shr(33u32)) as u64;
    rax = ((fl_l as u64)) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585ee0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143931b40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(P0, rcx);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f98; const P2: u64 = 0x1415f78a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    r9 = (((r8 as u8)) as u32) as u32 as u64;
    r8 = rdx;
    rdx = ((rcx as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f98; const P2: u64 = 0x1415f7b20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; return rax; }
    r9 = r8;
    r8 = rdx;
    rdx = ((rcx as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141585f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1f; const P2: u64 = 0x1415b16f0; const P3: u64 = 0x1415e9510; const P4: u64 = 0x5d; const P5: u64 = 0x1415e9220; const P6: u64 = 0x1415e8060; const P7: u64 = 0x34; const P8: u64 = 0x1415e7e80; const P9: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = r8; rbx = rdx; rdi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((rdi.wrapping_add(P1)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = ((rax.wrapping_add(P4)) as u32) as u32 as u64; bb = 11; continue; }
            6 => { r8 = rsi; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { r8 = rsi; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbx = ((rax.wrapping_add(P7)) as u32) as u32 as u64; bb = 11; continue; }
            9 => { rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rbx = 0x1; bb = 11; continue; }
            11 => { rcx = rsp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586030(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158ee40;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    r10 = ((r8 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    r8 = rsp.wrapping_add(0x20);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9);
    rcx = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rdx = ((r10 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586070(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158ee40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r8);
    r8 = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415860a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141200330; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xc; const P6: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = rbx; fl_r = (0xffffffffffffffe0u64) as u64;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rax = 0x1f; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0xb;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rbx);
    crate::simd_ops::wu8(rax.wrapping_add(P5), (0x0u8) as u8);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P6), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586110(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x140000000; const P2: u64 = 0x141524330; const P3: u64 = 0x142688e08; const P4: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r14 = rcx; r15 = r8; rcx = rsp.wrapping_add(0x68); rbx = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x2000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rsi); rsi = 0x1; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { r12 = 0; fl_l = 0; fl_r = 0; r13 = P1; bb = 4; continue; }
            4 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { let bit_in: u32 = (rbx as u32); if bit_in != 0 { rbp = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(r13).wrapping_add(0x2688bf0u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 14; continue; } bb = 6; continue; }
            6 => { rdx = ((rsi.wrapping_add(0x1fu64)) as u32) as u32 as u64; rcx = ((rsi.wrapping_add(0xfu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = P3; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r14); crate::simd_ops::wu64(rax, rcx); rdx = r13.wrapping_add(0x39311a0); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r15); rcx = ((rbp.wrapping_mul(2)) as u64) as u64; rcx = ((rcx).wrapping_add(rbp)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r12); fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; fl_r = r12; rdx = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::lk_add_u32(rdx, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); bb = 11; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); fl_l = (crate::simd_ops::lk_add_u32(rdx, (1) as u32)) as u32 as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rax); bb = 11; continue; }
            10 => { rsi = 0x34; bb = 11; continue; }
            11 => { rcx = ((rbp as u32)) as u32 as u64; rax = 0x1; rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rax = (!((rax as u32))) as u32 as u64; fl_l = ((((rbx as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = ((rsi as u32)) as u32 as u64; bb = 13; continue; }
            13 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            14 => { rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0x1f; bb = 13; continue; }
            15 => { rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = 0x1f; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415862a0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rax = 0x1f; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
    rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r9 as u32)) as u32);
    r9 = (((rdx as u8)) as u32) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r8 as u32)) as u32);
    r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415862a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x142688e08; const P2: u64 = 0x141524330; const P3: u64 = 0x143930fb0; const P4: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; r14 = rcx; r15 = r8; rcx = rsp.wrapping_add(0x68); rbp = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x2000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 1; continue; }
            1 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rdi); rdi = ((rbp as u32)) as u32 as u64; rbx = 0x1; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r13 = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); bb = 4; continue; }
            4 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = ((rbx.wrapping_add(0x1fu64)) as u32) as u32 as u64; rcx = ((rbx.wrapping_add(0xfu64)) as u32) as u32 as u64; let bit_in: u32 = (rdi as u32); if bit_in != 0 { rsi = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rax, r13); rdx = ((rsi.wrapping_add(rsi.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r14); r9 = P3; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), r15); r8 = ((r9.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), r12); fl_l = (crate::simd_ops::ru64(r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(r9.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x10u64), rax); bb = 9; continue; }
            8 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rax); bb = 9; continue; }
            9 => { crate::simd_ops::lk_add_u32(r9.wrapping_add(rdx.wrapping_mul(8)), (1) as u32); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rax); fl_l = ((rbp as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = r15; rdx = 0x1; rcx = rax; rax = crate::simd_ops::call_stk(r14, rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            11 => { rbx = 0x34; bb = 12; continue; }
            12 => { rcx = ((rsi as u32)) as u32 as u64; rax = 0x1; rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rax = (!((rax as u32))) as u32 as u64; fl_l = ((((rdi as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; rdi = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; bb = 16; continue; }
            15 => { rbx = 0x1f; bb = 16; continue; }
            16 => { rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415863b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158be60; const P1: u64 = 0x1415b16f0; const P2: u64 = 0x1415e9510; const P3: u64 = 0x4d0; const P4: u64 = 0x30; const P5: u64 = 0x143930f98; const P6: u64 = 0x1415f76e0; const P7: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r9; rbp = (((r8 as u8)) as u32) as u32 as u64; r14 = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 10; continue; }
            2 => { rcx = rsp.wrapping_add(0x30); rbx = 0x66; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(P5)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { r9 = (((rbp as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); r8 = r14; rdx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158ef90;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    r10 = ((r9 as u32)) as u32 as u64;
    r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    r11 = ((r8 as u32)) as u32 as u64;
    rcx = (((rdx as u8)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x0u64) as u64);
    r8 = ((r10 as u32)) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rdx = ((r11 as u32)) as u32 as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415864a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158ef90;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415864b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x4; const P1: u64 = 0x10; const P2: u64 = 0x1415242e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::lk_add_u32(rcx, (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; rdi = rcx; if (((fl_l as u32)) != 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rax = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdi).wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rdx = rdi; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 7; continue; }
            7 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1; const P1: u64 = 0x14157ef40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x1;
    rcx = rsp.wrapping_add(0x30);
    r8 = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x14157f040;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    r9 = 0x2;
    rdx = rsp.wrapping_add(0x38);
    r8 = ((r9.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586570(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1403cd310; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x1e;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rbx);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P5), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415865d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f08; const P2: u64 = 0x1415aa630;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; return rax; }
    rdx = (((rcx as u8)) as u32) as u32 as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415b16f0; const P6: u64 = 0x1415e9510; const P7: u64 = 0x1415b1830; const P8: u64 = 0x143931328; const P9: u64 = 0x1415ecaf0; const P10: u64 = 0x1415ed340; const P11: u64 = 0x1415ab0c0; const P12: u64 = 0x143930f08; const P13: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r15 = r8; r12 = rdx; r14 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rbp = (crate::simd_ops::ru32(rcx)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rsi); rsi = 0xae0007; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(P1)) as u64; bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 5; continue; }
            5 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x420u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rsi = rbx.wrapping_add(0x428); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x468u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 8; continue; }
            8 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x460u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rbx.wrapping_add(0x428); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } bb = 15; continue; }
            15 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            16 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x420u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x450u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x458u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            24 => { rsi = rbx.wrapping_add(0x428); bb = 25; continue; }
            25 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 26; continue; }
            26 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = ((rdi.wrapping_add(0xfu64)) as u32) as u32 as u64; bb = 41; continue; }
            28 => { rsi = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 30; continue; }
            30 => { rcx = rsp.wrapping_add(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r12; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsp.wrapping_add(0x68); rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = ((rbx.wrapping_add(0x5du64)) as u32) as u32 as u64; bb = 41; continue; }
            32 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0xae0007; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 38; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(P8)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P8, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 36; continue; }
            36 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = 0x43; bb = 41; continue; }
            38 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rcx = (crate::simd_ops::ru32(r14.wrapping_add(0x4u64))) as u32 as u64; rax = (crate::simd_ops::ru32(r14)) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; fl_l = (((rcx) | (rax))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(r15, rcx); bb = 40; continue; }
            40 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P12)) as u64; rdx = 0x1f; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x4u64), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x14u64), r12); rax = (crate::simd_ops::ru64(P12)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; rax = 0x1; bb = 41; continue; }
            41 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1403cd310; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x11;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rbx);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P5), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab080; const P2: u64 = 0x143930f08; const P3: u64 = 0x5; const P4: u64 = 0x1415aa7b0; const P5: u64 = 0x1415ab0b0; const P6: u64 = 0x14; const P7: u64 = 0x18; const P8: u64 = 0x1c; const P9: u64 = 0x4; const P10: u64 = 0x10; const P11: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = ((r9 as u32)) as u32 as u64; rdi = ((r8 as u32)) as u32 as u64; rsi = rdx; rbp = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 7; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u32) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = ((rdi.wrapping_add(P3)) as u32) as u32 as u64; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            5 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x6; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax.wrapping_add(P6), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P7), ((r14 as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(P8), ((rbx as u8)) as u8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), rsi); crate::simd_ops::wu32(rax.wrapping_add(P10), (0x0u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P11), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(P2)) as u64; rax = ((rax).wrapping_add(0xb4u64)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax, (1) as u32)) as u32 as u64; rax = 0x1; bb = 7; continue; }
            7 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x20; const P2: u64 = 0xffffffffffffffbf; const P3: u64 = 0x141586890;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbx = ((r8 as u32)) as u32 as u64; rdi = rdx; r11 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r10 = 0x811c9dc5; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r10 = ((((r10 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rdx = (((rax as u8)) as u32) as u32 as u64; rcx = ((r8.wrapping_add(P2)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r11 = r11.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rdx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; r10 = ((((r10 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x30u64))) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; rdx = rdi; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); rcx = ((r10 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415869d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x1415810d0; const P2: u64 = 0x141586890;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = ((r9 as u32)) as u32 as u64;
    rdi = ((r8 as u32)) as u32 as u64;
    rsi = rdx;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x60u64))) as u32) as u32 as u64;
    r9 = ((rbx as u32)) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((rcx as u8)) as u8);
    r8 = ((rdi as u32)) as u32 as u64;
    rcx = ((rax as u32)) as u32 as u64;
    rdx = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415c5a40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = ((r9 as u32)) as u32 as u64; r15 = ((r8 as u32)) as u32 as u64; r12 = rdx; rbp = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 43; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(P1)) as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsi = rbx.wrapping_add(0x320); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x360u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x358u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            13 => { rsi = rbx.wrapping_add(0x320); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            15 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rsi = rbx.wrapping_add(0x320); bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(P1)) as u64; bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; rsi = rbx.wrapping_add(0x320); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 38; continue; } bb = 29; continue; }
            29 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rcx = rdi.wrapping_add(0x18); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; r9 = ((r15 as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r8 = r12; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbp = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 34; continue; } bb = 31; continue; }
            31 => { bb = 32; continue; }
            32 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 32; continue; } bb = 34; continue; }
            34 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 37; continue; }
            37 => { rax = ((rbp as u32)) as u32 as u64; bb = 44; continue; }
            38 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 30; continue; } bb = 39; continue; }
            39 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            40 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 35; continue; } bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            43 => { rax = 0; fl_l = 0; fl_r = 0; bb = 44; continue; }
            44 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415f6ee0; const P6: u64 = 0x1415c5a40;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = rsp.wrapping_add(0x40); fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r13 = rdx; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rbx); r10 = rcx; r12 = ((r8 as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 47; continue; }
            2 => { r15 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = ((r15 as u32)) as u32 as u64; bb = 7; continue; }
            4 => { r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; rsi = 0x811c9dc5; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rsi = ((((rsi as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rsi = ((((rsi as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(P1)) as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 9; continue; }
            9 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 10; continue; }
            10 => { r14 = rbx.wrapping_add(0x320); bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x360u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 12; continue; }
            12 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rsi as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x358u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { rdi = r15; bb = 16; continue; }
            16 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 21; continue; }
            18 => { r14 = rbx.wrapping_add(0x320); rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 19; continue; }
            19 => { rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            20 => { rdi = r15; bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 28; continue; } bb = 22; continue; }
            22 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 22; continue; } bb = 24; continue; }
            24 => { bb = 29; continue; }
            25 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 30; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            28 => { r14 = rbx.wrapping_add(0x320); bb = 29; continue; }
            29 => { rcx = r14; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = 0; fl_l = 0; fl_r = 0; bb = 47; continue; }
            32 => { rax = r12; rax = ((rax).wrapping_shl(2u32)) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = (0xffffffffffffff0u64) as u64; bb = 34; continue; }
            34 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; rsp = ((rsp).wrapping_sub(rcx)) as u64; rdx = r13; rcx = rdi; r12 = rsp.wrapping_add(0x40); r8 = r12; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 44; continue; } bb = 35; continue; }
            35 => { rbx = (crate::simd_ops::ru64(P1)) as u64; bb = 36; continue; }
            36 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; rsi = rbx.wrapping_add(0x320); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 48; continue; } bb = 37; continue; }
            37 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 38; continue; }
            38 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; rcx = rdi.wrapping_add(0x18); r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; r8 = r12; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r14 = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 42; continue; } bb = 39; continue; }
            39 => { bb = 40; continue; }
            40 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 40; continue; } bb = 42; continue; }
            42 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 43; continue; }
            43 => { r15 = ((r14 as u32)) as u32 as u64; bb = 44; continue; }
            44 => { rdx = 0xffffffff; let xa_src: u32 = (rdx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdx = (xa_old) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 46; continue; } bb = 45; continue; }
            45 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            46 => { rax = ((r15 as u32)) as u32 as u64; bb = 47; continue; }
            47 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; rsp = rbp; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            48 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 38; continue; } bb = 49; continue; }
            49 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            50 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 43; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 43; continue; } bb = 52; continue; }
            52 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 43; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141586f30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415810d0; const P2: u64 = 0x143930f88; const P3: u64 = 0x1426452b0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x142645560; const P6: u64 = 0x142645718; const P7: u64 = 0x1415f7050; const P8: u64 = 0x1415c5a40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rbp = rsp.wrapping_add(0x40); fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r13 = rdx; crate::simd_ops::wu64(rbp.wrapping_add(0x150u64), rbx); r14 = rcx; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 34; continue; } bb = 1; continue; }
            1 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(P2)) as u64; r12 = ((rax as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsi = rbx.wrapping_add(0x320); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x360u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x358u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rdi = r15; bb = 10; continue; }
            10 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            12 => { rsi = rbx.wrapping_add(0x320); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 5; continue; } bb = 13; continue; }
            13 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            14 => { rdi = r15; bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 23; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 17; continue; } bb = 19; continue; }
            19 => { bb = 24; continue; }
            20 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            23 => { rsi = rbx.wrapping_add(0x320); bb = 24; continue; }
            24 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 35; continue; } bb = 26; continue; }
            26 => { rax = (0xffffffffffffffffu64) as u64; rcx = rax; bb = 27; continue; }
            27 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(rcx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 27; continue; } bb = 28; continue; }
            28 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x102; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 32; continue; } bb = 29; continue; }
            29 => { bb = 30; continue; }
            30 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 30; continue; } bb = 31; continue; }
            31 => { bb = 33; continue; }
            32 => { rax = 0x102; bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r9 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x103u32) as u32); r8 = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rdx = 0; fl_l = 0; fl_r = 0; rcx = 0xfde9; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu8(rbp.wrapping_add(rcx), (0x0u8) as u8); bb = 34; continue; }
            34 => { rax = 0; fl_l = 0; fl_r = 0; bb = 50; continue; }
            35 => { rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0x160u64))) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(2u32)) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 37; continue; } bb = 36; continue; }
            36 => { rcx = (0xffffffffffffff0u64) as u64; bb = 37; continue; }
            37 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; r9 = ((rdx as u32)) as u32 as u64; rdx = r13; rcx = rdi; r12 = rsp.wrapping_add(0x40); r8 = r12; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 47; continue; } bb = 38; continue; }
            38 => { rbx = (crate::simd_ops::ru64(P2)) as u64; bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; rsi = rbx.wrapping_add(0x320); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 51; continue; } bb = 40; continue; }
            40 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x178u64))) as u64; rcx = rdi.wrapping_add(0x18); r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x160u64))) as u32 as u64; r8 = r12; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x170u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x168u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r14 = ((rax as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 45; continue; } bb = 42; continue; }
            42 => { bb = 43; continue; }
            43 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x318u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 53; continue; } bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 43; continue; } bb = 45; continue; }
            45 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            46 => { r15 = ((r14 as u32)) as u32 as u64; bb = 47; continue; }
            47 => { rax = (0xffffffffffffffffu64) as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rdx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 49; continue; }
            49 => { rax = ((r15 as u32)) as u32 as u64; bb = 50; continue; }
            50 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x150u64))) as u64; rsp = rbp.wrapping_add(0x110); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            51 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x318u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 41; continue; } bb = 52; continue; }
            52 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 39; continue; }
            53 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 46; continue; } bb = 54; continue; }
            54 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x348u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 46; continue; } bb = 55; continue; }
            55 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x350u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 46; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141581680; const P2: u64 = 0x1415ab070; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415aa7b0; const P5: u64 = 0x4; const P6: u64 = 0xc; const P7: u64 = 0x10; const P8: u64 = 0x14; const P9: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = ((r8 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = 0x19; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(P5), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P6), (0x2u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P7), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P8), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(P9), (1) as u32); rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x5e; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415872f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141581680; const P2: u64 = 0x1415ab080; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415aa7b0; const P5: u64 = 0x4; const P6: u64 = 0xc; const P7: u64 = 0x10; const P8: u64 = 0x11; const P9: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (((r8 as u8)) as u32) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = 0x3b; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(P5), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P6), ((rsi as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(P7), (0x0u8) as u8); crate::simd_ops::wu8(rax.wrapping_add(P8), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(P9), (1) as u32); rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x5e; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141581680; const P2: u64 = 0x1415ab080; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415aa7b0; const P5: u64 = 0x4; const P6: u64 = 0xc; const P7: u64 = 0x10; const P8: u64 = 0x11; const P9: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (((r8 as u8)) as u32) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm1) as u128; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = 0x3b; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax.wrapping_add(P5), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P6), ((xmm6 as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(P7), (0x1u8) as u8); crate::simd_ops::wu8(rax.wrapping_add(P8), ((rdi as u8)) as u8); rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(P9), (1) as u32); rax = 0x1; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x5e; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415ab0f0; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = ((r8 as u32)) as u32 as u64; r15 = (((r9 as u8)) as u32) as u32 as u64; r13 = rdx; rbp = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x110); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x148u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdi = r14; bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            13 => { rsi = rbx.wrapping_add(0x110); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            15 => { rdi = r14; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rsi = rbx.wrapping_add(0x110); bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = 0x28; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0xcu64), rdi); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x4u64), r13); crate::simd_ops::wu32(rax.wrapping_add(0x1cu64), ((rcx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x20u64), ((r14 as u16)) as u16); crate::simd_ops::wu32(rax.wrapping_add(0x18u64), ((r12 as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x22u64), ((r15 as u16)) as u16); rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; rax = 0x1; bb = 29; continue; }
            28 => { rax = 0xf; bb = 29; continue; }
            29 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415875e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415ab0f0; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r15 = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rbp = ((rcx as u32)) as u32 as u64; xmm6 = (xmm2) as u128; r14 = (((r9 as u8)) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x110); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x148u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rbx.wrapping_add(0x110); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            16 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 19; continue; } bb = 21; continue; }
            21 => { bb = 26; continue; }
            22 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            25 => { rsi = rbx.wrapping_add(0x110); bb = 26; continue; }
            26 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = 0x28; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; rdx = 0x1; crate::simd_ops::wu32(rax.wrapping_add(0x18u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0xcu64), rdi); crate::simd_ops::wu64(rax.wrapping_add(0x4u64), r15); crate::simd_ops::wu32(rax.wrapping_add(0x1cu64), ((rcx as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(0x20u64), ((rdx as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(0x22u64), ((r14 as u16)) as u16); rax = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P6)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rcx.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; bb = 30; continue; }
            29 => { rax = 0xf; bb = 30; continue; }
            30 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587780(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xffffffffffffffbf; const P2: u64 = 0x141587430;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rbx = ((r8 as u32)) as u32 as u64; rdi = rdx; r11 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = 0; fl_l = 0; fl_r = 0; bb = 5; continue; }
            2 => { r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r10 = 0x811c9dc5; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r10 = ((((r10 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r11 = r11.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r10 = ((((r10 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { r8 = ((rbx as u32)) as u32 as u64; rdx = rdi; rcx = ((r10 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587800(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xffffffffffffffbf; const P2: u64 = 0x1415875e0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbx = rdx; r11 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            2 => { r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r10 = 0x811c9dc5; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r10 = ((((r10 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r11 = r11.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r10 = ((((r10 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { rdx = rbx; rcx = ((r10 as u32)) as u32 as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415810d0; const P1: u64 = 0x141587430;
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = (((r9 as u8)) as u32) as u32 as u64;
    rdi = ((r8 as u32)) as u32 as u64;
    rsi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r9 = (((rbx as u8)) as u32) as u32 as u64;
    r8 = ((rdi as u32)) as u32 as u64;
    rcx = ((rax as u32)) as u32 as u64;
    rdx = rsi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415878a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415810d0; const P1: u64 = 0x1415875e0;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128);
    rbx = (((r9 as u8)) as u32) as u32 as u64;
    xmm6 = (xmm2) as u128;
    rdi = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    r9 = (((rbx as u8)) as u32) as u32 as u64;
    xmm2 = (xmm6) as u128;
    rcx = ((rax as u32)) as u32 as u64;
    rdx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415878e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141587930;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64;
    r10 = (((r9 as u8)) as u32) as u32 as u64;
    r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64;
    r11 = r8;
    crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32);
    rcx = ((rdx as u32)) as u32 as u64;
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64;
    r8 = (((r10 as u8)) as u32) as u32 as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = r11;
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab0a0; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x141524330; const P5: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((rcx as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r15 = ((r9 as u32)) as u32 as u64; r12 = (((r8 as u8)) as u32) as u32 as u64; r13 = rdx; rbx = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rsi); rsi = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); rdi = 0x1; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = ((rdi.wrapping_add(0x35u64)) as u32) as u32 as u64; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(0x18u64), ((rcx as u32)) as u32); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_add(0x1cu64), rcx); crate::simd_ops::wu32(rax.wrapping_add(0x24u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r13); crate::simd_ops::wu8(rax.wrapping_add(0x10u64), ((r12 as u8)) as u8); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; bb = 8; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rbp); rdx = rsi; rcx = 0x5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x36; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; r8 = rsi; rcx = rbp; rbx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), r13); crate::simd_ops::wu8(rbx.wrapping_add(0x10u64), ((r12 as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0x14u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x1cu64), rbp); crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; bb = 7; continue; }
            6 => { rdi = 0x34; bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587a70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab080; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xc; const P6: u64 = 0x8; const P7: u64 = 0x10; const P8: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rsi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x2c; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax.wrapping_add(P4), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P5), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P6), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P7), (0x0u32) as u32); rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(P8), (1) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f00; const P2: u64 = 0x1415b12b0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    r8 = (((rdx as u8)) as u32) as u32 as u64;
    fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    xmm1 = (xmm0) as u128;
    return crate::simd_ops::tail_fwd_xmm(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587b30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1403cd310; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x8; const P5: u64 = 0x4; const P6: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = ((rcx as u32)) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x2f;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rax.wrapping_add(P4), ((rbx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P5), ((rdi as u32)) as u32);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P6), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xffffffffffffffbf; const P2: u64 = 0x141587b30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r10 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r9 = 0x811c9dc5; fl_l = ((r10 as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r9 = ((((r9 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r10.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; r8 = ((r10.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r10 as u8)) as u32) as u32 as u64; r11 = r11.wrapping_add(0x1); r10 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r9 = ((((r9 as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; fl_l = ((r10 as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = ((r9 as u32)) as u32 as u64; return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rax = 0xe; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1403cd310; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = ((rdx as u32)) as u32 as u64; r15 = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x60); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = r14; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rbx.wrapping_add(0x60); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rdi = r14; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rsi = rbx.wrapping_add(0x60); bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = rdi; fl_r = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rax = rdi.wrapping_sub(0x10); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; rcx = rax.wrapping_add(0x10); rbp = 0xffffffff; rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rax = 0x1f; bb = 60; continue; }
            31 => { rbx = (crate::simd_ops::ru64(P1)) as u64; bb = 32; continue; }
            32 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 42; continue; } bb = 33; continue; }
            33 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x420u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 34; continue; }
            34 => { rsi = rbx.wrapping_add(0x428); bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x468u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 44; continue; } bb = 36; continue; }
            36 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x460u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 40; continue; } bb = 38; continue; }
            38 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 37; continue; } bb = 39; continue; }
            39 => { rdi = r14; bb = 40; continue; }
            40 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 45; continue; } bb = 41; continue; }
            41 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 45; continue; }
            42 => { rsi = rbx.wrapping_add(0x428); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 35; continue; } bb = 43; continue; }
            43 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            44 => { rdi = r14; bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 53; continue; } bb = 46; continue; }
            46 => { bb = 47; continue; }
            47 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x420u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 47; continue; } bb = 49; continue; }
            49 => { bb = 54; continue; }
            50 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 55; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x450u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 55; continue; } bb = 52; continue; }
            52 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x458u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            53 => { rsi = rbx.wrapping_add(0x428); bb = 54; continue; }
            54 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 55; continue; }
            55 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 56; continue; }
            56 => { let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 58; continue; } bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 58; continue; }
            58 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = 0x20; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r12 as u32)) as u32); rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; rax = 0x1; bb = 60; continue; }
            59 => { rax = 0xf; bb = 60; continue; }
            60 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141587ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1403cd310; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415aa7b0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r9 = rdx; r10 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rbp = 0x811c9dc5; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { r14 = ((r12 as u32)) as u32 as u64; bb = 7; continue; }
            4 => { r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r14 = ((rbp as u32)) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { r14 = ((((r14 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r10 = r10.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r14 = ((((r14 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbp = ((r12 as u32)) as u32 as u64; bb = 11; continue; }
            9 => { r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rbp = ((((rbp as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(0x20u64)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_sub(0x41u64)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r9 = r9.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r9)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; rbp = ((((rbp as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 13; continue; }
            13 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 14; continue; }
            14 => { rsi = rbx.wrapping_add(0x60); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 16; continue; }
            16 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 18; continue; }
            18 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 19; continue; }
            19 => { rdi = r12; bb = 20; continue; }
            20 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 25; continue; }
            22 => { rsi = rbx.wrapping_add(0x60); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 12; continue; }
            24 => { rdi = r12; bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 33; continue; } bb = 26; continue; }
            26 => { bb = 27; continue; }
            27 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 30; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 27; continue; } bb = 29; continue; }
            29 => { bb = 34; continue; }
            30 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            33 => { rsi = rbx.wrapping_add(0x60); bb = 34; continue; }
            34 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { fl_l = rdi; fl_r = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = rdi.wrapping_sub(0x10); rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r12 } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 36; continue; }
            36 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; rcx = rax.wrapping_add(0x10); r15 = 0xffffffff; rax = ((r15 as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 38; continue; }
            38 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 39; continue; }
            39 => { rax = 0x1f; bb = 69; continue; }
            40 => { rbx = (crate::simd_ops::ru64(P1)) as u64; bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 52; continue; } bb = 42; continue; }
            42 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x420u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 41; continue; } bb = 43; continue; }
            43 => { rsi = rbx.wrapping_add(0x428); bb = 44; continue; }
            44 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x468u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 54; continue; } bb = 45; continue; }
            45 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x460u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 46; continue; }
            46 => { bb = 47; continue; }
            47 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 50; continue; } bb = 48; continue; }
            48 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 49; continue; }
            49 => { rdi = r12; bb = 50; continue; }
            50 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 51; continue; }
            51 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 55; continue; }
            52 => { rsi = rbx.wrapping_add(0x428); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 44; continue; } bb = 53; continue; }
            53 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 41; continue; }
            54 => { rdi = r12; bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 62; continue; } bb = 56; continue; }
            56 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x420u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 59; continue; } bb = 57; continue; }
            57 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 56; continue; } bb = 58; continue; }
            58 => { bb = 63; continue; }
            59 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 64; continue; } bb = 60; continue; }
            60 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x450u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 64; continue; } bb = 61; continue; }
            61 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x458u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 64; continue; }
            62 => { rsi = rbx.wrapping_add(0x428); bb = 63; continue; }
            63 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 64; continue; }
            64 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 65; continue; }
            65 => { let xa_src: u32 = (r15 as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; r15 = (xa_old) as u32 as u64; fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 67; continue; } bb = 66; continue; }
            66 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 67; continue; }
            67 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = 0x20; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((rbp as u32)) as u32); rax = (crate::simd_ops::ru64(P6)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; rax = 0x1; bb = 69; continue; }
            68 => { rax = 0xf; bb = 69; continue; }
            69 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415810d0; const P2: u64 = 0x143930f88; const P3: u64 = 0x1426452b0; const P4: u64 = 0x1426452a8; const P5: u64 = 0x142645560; const P6: u64 = 0x142645718; const P7: u64 = 0x1403cd310; const P8: u64 = 0x143930f08; const P9: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 47>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x168u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = rdx; rbp = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x168u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x180u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x190u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x160u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x158u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x150u64), r15); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = r14; r12 = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(P2)) as u64; r13 = ((rax as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x188u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x60); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; r15 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r12 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = r15; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rbx.wrapping_add(0x60); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rdi = r15; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rsi = rbx.wrapping_add(0x60); bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { fl_l = rdi; fl_r = rdi; rax = rdi.wrapping_sub(0x10); rbx = (0xffffffffffffffffu64) as u64; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 34; continue; } bb = 27; continue; }
            27 => { rax = rbx; bb = 28; continue; }
            28 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rbp.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 28; continue; } bb = 29; continue; }
            29 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x102; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rbp.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 30; continue; } bb = 31; continue; }
            31 => { bb = 33; continue; }
            32 => { rbx = 0x102; bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x103u32) as u32); r8 = rbp; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = 0xfde9; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = 0xf; crate::simd_ops::wu8(rsp.wrapping_add(rcx).wrapping_add(0x40u64), (0x0u8) as u8); bb = 73; continue; }
            34 => { rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; rcx = rax.wrapping_add(0x10); rax = ((rbx as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            36 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rax = 0x1f; bb = 73; continue; }
            38 => { rdi = (crate::simd_ops::ru64(P2)) as u64; bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 50; continue; } bb = 40; continue; }
            40 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x420u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 41; continue; }
            41 => { rbp = rdi.wrapping_add(0x428); bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x468u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 52; continue; } bb = 43; continue; }
            43 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r13 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x460u64))) as u64; rsi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 44; continue; }
            44 => { bb = 45; continue; }
            45 => { fl_l = (crate::simd_ops::ru32(rsi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 48; continue; } bb = 46; continue; }
            46 => { rsi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 45; continue; } bb = 47; continue; }
            47 => { rsi = r15; bb = 48; continue; }
            48 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 53; continue; } bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 53; continue; }
            50 => { rbp = rdi.wrapping_add(0x428); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 42; continue; } bb = 51; continue; }
            51 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 39; continue; }
            52 => { rsi = r15; bb = 53; continue; }
            53 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 60; continue; } bb = 54; continue; }
            54 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x420u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 57; continue; } bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x420u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 54; continue; } bb = 56; continue; }
            56 => { bb = 61; continue; }
            57 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 62; continue; } bb = 58; continue; }
            58 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x450u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 62; continue; } bb = 59; continue; }
            59 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x458u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 62; continue; }
            60 => { rbp = rdi.wrapping_add(0x428); bb = 61; continue; }
            61 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 62; continue; }
            62 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 70; continue; } bb = 63; continue; }
            63 => { rax = rbx; bb = 64; continue; }
            64 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 64; continue; } bb = 65; continue; }
            65 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x102; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 68; continue; } bb = 66; continue; }
            66 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(r14.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 66; continue; } bb = 67; continue; }
            67 => { bb = 69; continue; }
            68 => { rbx = 0x102; bb = 69; continue; }
            69 => { crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x103u32) as u32); r8 = r14; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rcx = 0xfde9; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = 0xf; crate::simd_ops::wu8(rsp.wrapping_add(rcx).wrapping_add(0x40u64), (0x0u8) as u8); bb = 73; continue; }
            70 => { let xa_src: u32 = (rbx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbx = (xa_old) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 72; continue; } bb = 71; continue; }
            71 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 72; continue; }
            72 => { rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P8)) as u64; rdx = 0x20; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r13 as u32)) as u32); rax = (crate::simd_ops::ru64(P8)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; rax = 0x1; bb = 73; continue; }
            73 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x188u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x190u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x158u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x180u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x150u64))) as u64; rsp = ((rsp).wrapping_add(0x168u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588600(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415ab080; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r12 = ((r8 as u32)) as u32 as u64; r15 = ((rdx as u32)) as u32 as u64; r14 = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = ((r15 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x1f; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 6; continue; }
            6 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 7; continue; }
            7 => { rsi = rbx.wrapping_add(0x60); bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0xa0u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 9; continue; }
            9 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r14 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x98u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 12; continue; }
            12 => { rdi = rbp; bb = 13; continue; }
            13 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            15 => { rsi = rbx.wrapping_add(0x60); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 8; continue; } bb = 16; continue; }
            16 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            17 => { rdi = rbp; bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x58u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 20; continue; } bb = 22; continue; }
            22 => { bb = 27; continue; }
            23 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 28; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            26 => { rsi = rbx.wrapping_add(0x60); bb = 27; continue; }
            27 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rcx = rdi.wrapping_sub(0x10); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdi; fl_r = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { rcx }) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = ((rcx.wrapping_add(0xfu64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            30 => { rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = 0x2c; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((r14 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((r12 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), (0x1u32) as u32); rax = 0x1; rcx = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::lk_sub_u32(rcx.wrapping_add(0xb4u64), (1) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415887c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xffffffffffffffbf; const P2: u64 = 0x141588600;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rbx = ((rdx as u32)) as u32 as u64; r11 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r10 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r9 = 0x811c9dc5; fl_l = ((r10 as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r9 = ((((r9 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r10.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r10.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r10 as u8)) as u32) as u32 as u64; r11 = r11.wrapping_add(0x1); r10 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r9 = ((((r9 as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((r9 as u32)) as u32 as u64; fl_l = ((r10 as u8)) as u8 as u64; fl_r = ((r10 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((rbx as u32)) as u32 as u64; rcx = ((r9 as u32)) as u32 as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            6 => { rax = 0xe; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588830(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415810d0; const P1: u64 = 0x141588600;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((r8 as u32)) as u32 as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { r8 = ((rbx as u32)) as u32 as u64; rdx = ((rdi as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0xe;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x1415f9c40; const P5: u64 = 0x142645560; const P6: u64 = 0x141524330; const P7: u64 = 0x1415242e0; const P8: u64 = 0x1415b1830;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x58u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); r14 = rcx; rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 18; continue; } bb = 2; continue; }
            2 => { rdx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rdx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x8); bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14)) as u64; r9 = rsp.wrapping_add(0x88); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rdx = rsp.wrapping_add(0x80); crate::simd_ops::wu32(rsp.wrapping_add(0x80u64), ((r15 as u32)) as u32); rcx = r14; r8 = ((r15.wrapping_add(0x4u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 38; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r14)) as u64; r9 = rsp.wrapping_add(0x20); r8 = 0x4; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((r15 as u32)) as u32); rdx = rsp.wrapping_add(0x78); rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 45; continue; } bb = 7; continue; }
            7 => { rbp = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 20; continue; } bb = 9; continue; }
            9 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 11; continue; }
            11 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 15; continue; }
            15 => { rdi = r15; bb = 16; continue; }
            16 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 23; continue; }
            18 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 19; continue; }
            19 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            20 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 10; continue; } bb = 21; continue; }
            21 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            22 => { rdi = r15; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 26; continue; } bb = 24; continue; }
            24 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 24; continue; } bb = 26; continue; }
            26 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { fl_l = rdi; fl_r = rdi; rbp = rdi.wrapping_sub(0x10); rbp = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r15 } else { rbp }) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 28; continue; }
            28 => { rdx = r14; rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; rcx = rbp.wrapping_add(0x10); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 39; continue; } bb = 31; continue; }
            31 => { bb = 37; continue; }
            32 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 33; continue; }
            33 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 34; continue; }
            34 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            35 => { rdx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as i32 as i64) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 44; continue; } bb = 36; continue; }
            36 => { r10 = (crate::simd_ops::ru64(r14)) as u64; r9 = rsp.wrapping_add(0x24); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; rdx = rax; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbp; rcx = 0; fl_l = 0; fl_r = 0; rdi = (((rax as u8)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 45; continue; } bb = 37; continue; }
            37 => { rax = (crate::simd_ops::ru64(r14)) as u64; r9 = rsp.wrapping_add(0x88); r8 = 0x4; rdx = rsp.wrapping_add(0x80); rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 38; continue; }
            38 => { rdi = 0x1; bb = 39; continue; }
            39 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 46; continue; } bb = 41; continue; }
            41 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 40; continue; } bb = 42; continue; }
            42 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 43; continue; }
            43 => { rcx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = ((rdi as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x58u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            44 => { rdi = 0x34; bb = 39; continue; }
            45 => { rdi = 0x2; bb = 39; continue; }
            46 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 43; continue; } bb = 47; continue; }
            47 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 43; continue; } bb = 48; continue; }
            48 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); bb = 43; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588b00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f58; const P1: u64 = 0x141524330; const P2: u64 = 0x34; const P3: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdx = (crate::simd_ops::ru64(P0)) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(rdx))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = 0x53; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdx = (0xffffffffffffffffu64) as u64; bb = 7; continue; }
            7 => { rdx = ((rdx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(rdx))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { rdx = ((rdx).wrapping_add(1)) as u64; rcx = 0x10; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = ((rcx.wrapping_add(P2)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { bb = 11; continue; }
            11 => { rax = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rbx = ((rbx.wrapping_add(P3)) as u64) as u64; crate::simd_ops::wu8(rcx, ((rax as u8)) as u8); rcx = ((rcx.wrapping_add(P3)) as u64) as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 12; continue; }
            12 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab080; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0xc; const P5: u64 = 0x4; const P6: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rdx;
    rdi = rcx;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x3e;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(P5), rdi);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P6), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143426e08; const P2: u64 = 0x1415ab0b0; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415aa7b0; const P5: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = rdx; rsi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128; rcx = rdi.wrapping_add(0xc); xmm1 = 0u128; bb = 4; continue; }
            4 => { xmm0 = ((crate::simd_ops::ru32(rcx)) as u32) as u128; rax = ((xmm0 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm0 = (xmm1) as u128; bb = 8; continue; }
            7 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = 0x13; r8 = (((rax as u16)) as u32) as u32 as u64; rax = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shl(4u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rax as u32))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; crate::simd_ops::wu32(rax.wrapping_add(0xcu64), ((rbx as u32)) as u32); rcx = rax.wrapping_add(0x10); rbx = (((rbx as u32)).wrapping_shl(4u32)) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rax.wrapping_add(0x4u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x5f; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588d20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143426e08; const P2: u64 = 0x1415ab070; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415aa7b0; const P5: u64 = 0x14; const P6: u64 = 0x4; const P7: u64 = 0xc; const P8: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rdx; rdi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = ((xmm2 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = 0u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm6 = (xmm2) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) < f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); bb = 5; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = 0x14; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(P5), ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rax.wrapping_add(P6), rdi); crate::simd_ops::wu64(rax.wrapping_add(P7), rbx); rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(P8), (1) as u32); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x5f; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f44; const P1: u64 = 0x1415f1e90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    rbx = ((rcx as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu32(P0, ((rbx as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143931128; const P2: u64 = 0x1415b2780;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    fl_l = (crate::simd_ops::ru64(P1)) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) >= ((fl_r as u32))) { rax = 0x1f; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; } rax = 0x1f; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0x66;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588e50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab0c0; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x141524320; const P5: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = r9; rdi = ((r8 as u32)) as u32 as u64; r15 = rcx; r14 = (((rdx as u8)) as u32) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); r12 = 0; fl_l = 0; fl_r = 0; rbp = (((rdi as u8)) as u32) as u32 as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rcx = ((r12 as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 11; continue; } bb = 6; continue; }
            6 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = rcx; fl_r = rbp; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 5; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x12; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rdi as u32)) as u32); rdi = 0x1; crate::simd_ops::wu64(rax.wrapping_add(0x4u64), r15); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), ((r14 as u16)) as u16); crate::simd_ops::wu64(rax.wrapping_add(0xcu64), r12); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu16(rax.wrapping_add(0x1au64), ((rdi as u16)) as u16); fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rdx = ((rbp.wrapping_add(0x7u64)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; rdx = (((rdx as u32)).wrapping_shr(3u32)) as u32 as u64; r8 = ((rdi.wrapping_add(0xfu64)) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(5u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(0xcu64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rax = 0x40; crate::simd_ops::wu16(rbx.wrapping_add(0x2u64), ((rax as u16)) as u16); rdi = ((rax.wrapping_sub(0xcu64)) as u32) as u32 as u64; bb = 14; continue; }
            11 => { rax = 0x5f; bb = 15; continue; }
            12 => { r8 = rbp; rdx = rsi; r8 = ((r8).wrapping_shl(2u32)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            13 => { crate::simd_ops::wu16(rax.wrapping_add(0x1au64), ((r12 as u16)) as u16); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rcx.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14157f040;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143814d00; const P1: u64 = 0x1439315cc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32))) { rax = (((rcx as u16)) as u32) as u32 as u64; crate::simd_ops::wu32(P1, ((rax as u32)) as u32); crate::simd_ops::wu32(P0, ((rdx as u32)) as u32); }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143814cfc; const P1: u64 = 0x143931610;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64;
    if !(((((fl_l as u16)) & ((fl_r as u16))) != 0)) { rax = 0x1f; return rax; }
    fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0x0;
    if !(((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32))) { crate::simd_ops::wu32(P0, (0x0u32) as u32); crate::simd_ops::wu16(P1, ((rcx as u16)) as u16); }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141588ff0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143814cfc; const P1: u64 = 0x143931610;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64;
    if !(((((fl_l as u16)) & ((fl_r as u16))) != 0)) { rax = 0x1f; return rax; }
    fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64;
    if !(((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32))) { crate::simd_ops::wu32(P0, ((rdx as u32)) as u32); crate::simd_ops::wu16(P1, ((rcx as u16)) as u16); }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab080; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0xc; const P5: u64 = 0x4; const P6: u64 = 0x10; const P7: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = ((rcx as u32)) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x2e;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rax.wrapping_add(P4), ((rbx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P5), ((rdi as u32)) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P6), (0x1u32) as u32);
    rax = 0x1;
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rcx.wrapping_add(P7), (1) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415890c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xffffffffffffffbf; const P2: u64 = 0x1; const P3: u64 = 0x141589050;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = ((rdx as u32)) as u32 as u64; r10 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { r9 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r8 = 0x811c9dc5; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { r8 = ((((r8 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r9.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r10 = ((r10.wrapping_add(P2)) as u64) as u64; r9 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = ((r11 as u32)) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            6 => { rax = 0xe; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab070; const P2: u64 = 0x143930f08; const P3: u64 = 0x1434268d0; const P4: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = ((r9 as u32)) as u32 as u64; rdi = r8; rbp = rdx; r14 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 24; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = (crate::simd_ops::ru64(P2)) as u64; rcx = 0xffff; rax = (((rax as u16)) as u32) as u32 as u64; r8 = ((rax.wrapping_add(rsi.wrapping_mul(8))) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(r9.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x1f; bb = 24; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = 0; fl_l = 0; fl_r = 0; rcx = (((rbx as u16)) as u32) as u32 as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { xmm2 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm1 = 0u128; bb = 6; continue; }
            6 => { rdx = (((rcx as u16)) as u32) as u32 as u64; xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; rax = ((xmm0 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 7; continue; }
            7 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm0 = (xmm1) as u128; bb = 10; continue; }
            9 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rdi.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x4u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(rdx.wrapping_mul(8)))) as u32) as u128; rax = ((xmm0 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 11; continue; }
            11 => { fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { xmm0 = (xmm1) as u128; bb = 14; continue; }
            13 => { xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(xmm2 as u32) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(xmm2 as u32) }).to_bits() as u128); bb = 14; continue; }
            14 => { rcx = (rcx & !0xffffu64) | (((((rcx as u16)).wrapping_add(1)) as u16 as u64)); crate::simd_ops::wu32(rdi.wrapping_add(rdx.wrapping_mul(8)), ((xmm0 as u32)) as u32); rax = (((rcx as u16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 15; continue; }
            15 => { r9 = (crate::simd_ops::ru64(P2)) as u64; bb = 16; continue; }
            16 => { rdx = 0x16; rcx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = ((rcx.wrapping_add(0x51u64)) as u32) as u32 as u64; bb = 23; continue; }
            18 => { rax = 0x5f; bb = 23; continue; }
            19 => { crate::simd_ops::wu64(rax.wrapping_add(0x4u64), r14); crate::simd_ops::wu64(rax.wrapping_add(0xcu64), rbp); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rsi as u32)) as u32); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { bb = 21; continue; }
            21 => { rax = (((rbx as u16)) as u32) as u32 as u64; rbx = (rbx & !0xffffu64) | (((((rbx as u16)).wrapping_add(1)) as u16 as u64)); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(rax.wrapping_mul(8)))) as u64) as u128; crate::simd_ops::wu64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x18u64), ((xmm0 as u64)) as u64); rax = (((rbx as u16)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 21; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; rax = 0x1; bb = 23; continue; }
            23 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 24; continue; }
            24 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589330(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1434268d0; const P2: u64 = 0x1415ab0c0; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415aa7b0; const P5: u64 = 0x14; const P6: u64 = 0x18; const P7: u64 = 0x4; const P8: u64 = 0xc; const P9: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rdx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm7) as u128); rdi = rcx; xmm7 = (xmm2) as u128; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rax = ((xmm7 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 3; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm6 = 0u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm7 = 0u128; bb = 6; continue; }
            5 => { xmm7 = (xmm7 & !0xffff_ffffu128) | ((if f32::from_bits(xmm7 as u32) < f32::from_bits(xmm0 as u32) { f32::from_bits(xmm7 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); bb = 6; continue; }
            6 => { rax = ((xmm3 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 8; continue; }
            8 => { xmm6 = (xmm3) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) < f32::from_bits(xmm0 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm0 as u32) }).to_bits() as u128); bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = 0x15; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(P5), ((xmm7 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(P7), rdi); crate::simd_ops::wu64(rax.wrapping_add(P8), rbx); rax = (crate::simd_ops::ru64(P3)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P9), (1) as u32)) as u32 as u64; rax = 0x1; bb = 11; continue; }
            10 => { rax = 0x5f; bb = 11; continue; }
            11 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1403cd310; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x6; const P5: u64 = 0x4; const P6: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = (((rcx as u8)) as u32) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x3c;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = 0x1;
    crate::simd_ops::wu16(rax.wrapping_add(P4), ((rbx as u16)) as u16);
    crate::simd_ops::wu16(rax.wrapping_add(P5), ((rdx as u16)) as u16);
    rax = ((rdx as u32)) as u32 as u64;
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rcx.wrapping_add(P6), (1) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589470(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1403cd310; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x8; const P5: u64 = 0x4; const P6: u64 = 0xb4; const P7: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128);
    xmm6 = (xmm0) as u128;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x3c;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rax.wrapping_add(P4), ((xmm6 as u32)) as u32);
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu16(rax.wrapping_add(P5), ((rcx as u16)) as u16);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P6), (1) as u32);
    rax = ((rcx.wrapping_add(P7)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415894e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab080; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0x10; const P6: u64 = 0xc; const P7: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = ((r8 as u32)) as u32 as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rsi = rcx;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x2d;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rsi);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P5), ((rbx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P6), ((rdi as u32)) as u32);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P7), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141200330; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0xc; const P5: u64 = 0x4; const P6: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm1) as u128; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = ((xmm6 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x5f; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x17; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(P4), ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rax.wrapping_add(P5), rbx); rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(P6), (1) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415895f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141200330; const P1: u64 = 0x143930f08; const P2: u64 = 0x1415aa7b0; const P3: u64 = 0x4; const P4: u64 = 0xc; const P5: u64 = 0xb4;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = (((rcx as u8)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rdx = 0x30;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P3), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu8(rax.wrapping_add(P4), ((rdi as u8)) as u8);
    rax = (crate::simd_ops::ru64(P1)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P5), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141589660;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0xffffffffffffffe0u64) as u64;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rax = 0x1f; return rax; }
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589660(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143426890; const P2: u64 = 0x143426904; const P3: u64 = 0x1434265d4; const P4: u64 = 0x1415ab0d0; const P5: u64 = 0x143930f08; const P6: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = (((r8 as u8)) as u32) as u32 as u64; rbx = rdx; rsi = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rcx = (0x7fffffffffffffffu64) as u64; rax = (((rax) & (rcx))) as u64; rdx = (0x7ff0000000000000u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = (((rax) & (rcx))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rax = (((rax) & (rcx))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 12; continue; } bb = 6; continue; }
            6 => { xmm2 = ((crate::simd_ops::ru32(rbx)) as u32) as u128; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x4u64))) as u32) as u128; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 7; continue; }
            7 => { xmm4 = ((crate::simd_ops::ru32(P2)) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 8; continue; }
            8 => { xmm5 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32) as u128; xmm0 = (xmm5) as u128; xmm6 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x14u64))) as u32) as u128; xmm3 = (xmm2) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm6) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm1 as u32)).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx))).to_bits() as u128); xmm5 = (xmm5 & !0xffff_ffffu128) | ((f32::from_bits(xmm5 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x4u64)))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x8u64)))).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P3)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P5)) as u64; rdx = 0xd; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax.wrapping_add(0x4u64), rsi); xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rax.wrapping_add(0xcu64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x1cu64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu8(rax.wrapping_add(0x3cu64), ((rdi as u8)) as u8); crate::simd_ops::wu128(rax.wrapping_add(0x2cu64), (xmm0) as u128); rax = (crate::simd_ops::ru64(P5)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; rax = 0x1; bb = 13; continue; }
            12 => { rax = 0x1f; bb = 13; continue; }
            13 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415897f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158c120;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64;
    r8 = r9;
    crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    xmm1 = (xmm2) as u128;
    rcx = ((rdx as u32)) as u32 as u64;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589810(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158c120;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32);
    r9 = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x20; const P2: u64 = 0xffffffffffffffbf; const P3: u64 = 0x14158c120;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbx = r8; r11 = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r10 = 0x811c9dc5; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r10 = ((((r10 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_add(P2)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r11 = r11.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r10 = ((((r10 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u32) as u32 as u64; r8 = rbx; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rax as u8)) as u8); rcx = ((r10 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); r9 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415898d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x1415810d0; const P2: u64 = 0x14158c120;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x50u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128);
    xmm6 = (xmm1) as u128;
    rbx = ((r9 as u32)) as u32 as u64;
    rdi = r8;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64;
    r9 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rcx as u8)) as u8);
    r8 = rdi;
    rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64;
    xmm1 = (xmm6) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32);
    rcx = ((rax as u32)) as u32 as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbx as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128;
    rsp = ((rsp).wrapping_add(0x50u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f90; const P2: u64 = 0x14159a4d0; const P3: u64 = 0x14158c120;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rbx = ((r8 as u32)) as u32 as u64; xmm6 = (xmm1) as u128; rsi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 5; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = ((rbx as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; r9 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), ((rcx as u8)) as u8); r8 = rax; rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x80u64))) as u32 as u64; xmm1 = (xmm6) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); rcx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            4 => { rax = 0x5e; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1415899d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x20; const P2: u64 = 0xffffffffffffffbf; const P3: u64 = 0x141589940;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rbx = ((r8 as u32)) as u32 as u64;
    r11 = rcx;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64;
    r10 = 0x811c9dc5;
    fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64; r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8); rcx = ((r10 as u32)) as u32 as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    loop { r10 = ((((r10 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_add(P2)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r11 = r11.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r10 = ((((r10 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { break; } }
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x38u64))) as u32) as u32 as u64;
    r8 = ((rbx as u32)) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rax as u8)) as u8);
    rcx = ((r10 as u32)) as u32 as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589a40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xe; const P1: u64 = 0x1415810d0; const P2: u64 = 0x141589940;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128);
    xmm6 = (xmm1) as u128;
    rbx = ((r9 as u32)) as u32 as u64;
    rdi = ((r8 as u32)) as u32 as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = ((rcx.wrapping_add(P0)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x78u64))) as u32) as u32 as u64;
    r9 = ((rbx as u32)) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), ((rcx as u8)) as u8);
    r8 = ((rdi as u32)) as u32 as u64;
    xmm1 = (xmm6) as u128;
    rcx = ((rax as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589aa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f78; const P1: u64 = 0x1415f3790; const P2: u64 = 0x143930fa0; const P3: u64 = 0x1415c4f90;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm6) as u128); xmm6 = (xmm2) as u128; rbx = ((rdx as u32)) as u32 as u64; fl_l = r9; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = r9; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 3; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd0u64))) as u32 as u64; r8 = rsp.wrapping_add(0x58); crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((rax as u32)) as u32); xmm3 = (xmm6) as u128; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0xd8u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0xccu64), ((rax as u32)) as u32); rax = rsp.wrapping_add(0xc8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0xe0u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), ((rax as u8)) as u8); rax = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rcx); rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x30u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x44u64), (0x0u8) as u8); rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x51u64), (0x1u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x0u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x0u32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x70u64), (0xffu8) as u8); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), (0xffu8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x14062e040; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = ((rcx as u32)) as u32 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x34; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax.wrapping_add(P4), ((rbx as u32)) as u32); rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P5), (1) as u32)) as u32 as u64; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141200330; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0xc; const P5: u64 = 0x4; const P6: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); xmm6 = (xmm1) as u128; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { xmm0 = 0u128; fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = 0x1f; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { rax = ((xmm6 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0x5f; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x26; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rax.wrapping_add(P4), ((xmm6 as u32)) as u32); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rax.wrapping_add(P5), rbx); rax = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(P6), (1) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589c90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415e6d20; const P2: u64 = 0x1415ab0c0; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415aa7b0; const P5: u64 = 0x141524330; const P6: u64 = 0x14183acbc;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = r9; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rsi = rcx; xmm6 = (xmm2) as u128; rdi = ((rdx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 17; continue; }
            2 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 3; continue; }
            3 => { rcx = 0x633; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rdx = (rdx & !0xffu64) | (((((rdx as u8)).wrapping_add(1)) as u8 as u64)); rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((((rcx as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (((rdx as u8)) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 16; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = rsi; bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x4u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 10; continue; }
            10 => { r8 = 0; fl_l = 0; fl_r = 0; rdx = ((rdi as u32)) as u32 as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = 0x31; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = rdi; rcx = 0; fl_l = 0; fl_r = 0; rbp = ((rbp).wrapping_shl(2u32)) as u64; rbx = rax; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x4u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = 0x40; crate::simd_ops::wu16(rbx.wrapping_add(0x2u64), ((rax as u16)) as u16); rax = 0x34; bb = 15; continue; }
            13 => { rax = 0x5f; bb = 17; continue; }
            14 => { r8 = rbp; rdx = rsi; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x1; crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x14u64), r14); bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rcx.wrapping_add(0xb4u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            16 => { rax = 0x1f; bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141200330; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x8; const P5: u64 = 0x4; const P6: u64 = 0xc; const P7: u64 = 0xb4; const P8: u64 = 0x1;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rdi = ((rcx as u32)) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x7;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rax.wrapping_add(P4), ((rbx as u32)) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P5), ((rdi as u32)) as u32);
    crate::simd_ops::wu32(rax.wrapping_add(P6), ((rcx as u32)) as u32);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P7), (1) as u32);
    rax = ((rcx.wrapping_add(P8)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589e40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141200330; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0x8; const P6: u64 = 0xc; const P7: u64 = 0xe; const P8: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rsi = ((rdx as u32)) as u32 as u64;
    rbx = (((r9 as u8)) as u32) as u32 as u64;
    rbp = ((rcx as u32)) as u32 as u64;
    rdi = (((r8 as u8)) as u32) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; } else { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x7; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax.wrapping_add(P4), ((rbp as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rsi as u32)) as u32); crate::simd_ops::wu16(rax.wrapping_add(P6), ((rdi as u16)) as u16); crate::simd_ops::wu16(rax.wrapping_add(P7), ((rbx as u16)) as u16); rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P8), (1) as u32)) as u32 as u64; rax = 0x1; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xffffffffffffffbf; const P2: u64 = 0x1; const P3: u64 = 0x141589dd0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); r8 = 0; fl_l = 0; fl_r = 0; r11 = rdx; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = ((r8 as u32)) as u32 as u64; bb = 5; continue; }
            2 => { r9 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r10 = 0x811c9dc5; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r10 = ((((r10 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r9.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; rbx = ((rbx.wrapping_add(P2)) as u64) as u64; r9 = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r10 = ((((r10 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r9 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; r8 = 0x811c9dc5; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { r8 = ((((r8 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r9.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r11 = r11.wrapping_add(0x1); r9 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r8 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rdx = ((r8 as u32)) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            12 => { rax = 0xe; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589f90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415810d0; const P1: u64 = 0x141589dd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rdi = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0xe; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = ((rax as u32)) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0xe;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141589fe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab080; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xc; const P6: u64 = 0x10; const P7: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = r8;
    rdi = ((rdx as u32)) as u32 as u64;
    rsi = ((rcx as u32)) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x8;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rbx);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P5), ((rsi as u32)) as u32);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    crate::simd_ops::wu32(rax.wrapping_add(P6), ((rdi as u32)) as u32);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P7), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0xffffffffffffffbf; const P2: u64 = 0x1; const P3: u64 = 0x141589fe0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); r9 = 0; fl_l = 0; fl_r = 0; rdi = r8; r11 = rdx; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { r10 = ((r9 as u32)) as u32 as u64; bb = 5; continue; }
            2 => { r8 = ((crate::simd_ops::ru8(rcx)) as u32) as u32 as u64; r10 = 0x811c9dc5; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { r10 = ((((r10 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; rbx = ((rbx.wrapping_add(P2)) as u64) as u64; r8 = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r10 = ((((r10 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = r11; fl_r = r11; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r8 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; r9 = 0x811c9dc5; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { r9 = ((((r9 as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(P0)) as u32) as u32 as u64; rcx = (((rax as u8)) as u32) as u32 as u64; rdx = ((r8.wrapping_add(P1)) as u32) as u32 as u64; rax = (((r8 as u8)) as u32) as u32 as u64; r11 = r11.wrapping_add(0x1); r8 = ((crate::simd_ops::ru8(r11)) as u32) as u32 as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rcx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rcx as u32) }) as u32 as u64; rax = (((rcx as u8)) as u32) as u32 as u64; r9 = ((((r9 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 8; continue; } bb = 9; continue; }
            9 => { fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { r8 = rdi; rdx = ((r9 as u32)) as u32 as u64; rcx = ((r10 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = 0xe; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415810d0; const P1: u64 = 0x141589fe0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = r8;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rdi = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0xe; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { r8 = rsi; rdx = ((rax as u32)) as u32 as u64; rcx = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0xe;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a1a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142671398; const P1: u64 = 0x143814cf8; const P2: u64 = 0x143426dd4; const P3: u64 = 0x143426564; const P4: u64 = 0x14183ad46; const P5: u64 = 0x142673708; const P6: u64 = 0x1426736fc; const P7: u64 = 0x142673700; const P8: u64 = 0x1426736ec; const P9: u64 = 0x1426736e8; const P10: u64 = 0x1426736f0; const P11: u64 = 0x143815b3c; const P12: u64 = 0x143815b38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = (xmm0) as u128; rax = ((xmm7 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x5f; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { xmm6 = 0u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_r = 0x0; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 8; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(P2)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm8 = (xmm7) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); crate::simd_ops::wu32(P1, (0x0u32) as u32); xmm1 = (xmm8) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; xmm1 = (xmm0) as u128; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm8 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm6 = (xmm0) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(P9))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x68u64)))).to_bits() as u128); bb = 7; continue; }
            7 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); crate::simd_ops::wu32(P11, ((xmm7 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm6 as u32)) as u32); bb = 8; continue; }
            8 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rax = 0x1; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rax = 0x1f; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142671398; const P1: u64 = 0x143814cf8; const P2: u64 = 0x143426564; const P3: u64 = 0x143426dd4; const P4: u64 = 0x14183ad46; const P5: u64 = 0x142673708; const P6: u64 = 0x1426736fc; const P7: u64 = 0x142673700; const P8: u64 = 0x1426736ec; const P9: u64 = 0x1426736e8; const P10: u64 = 0x1426736f0; const P11: u64 = 0x143815b3c; const P12: u64 = 0x143815b38;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = (xmm0) as u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { xmm6 = 0u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 6; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm8 = (xmm0) as u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(P2))).to_bits() as u128); crate::simd_ops::wu32(P1, ((rdx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm1 = (xmm8) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P5)) as u32 as u64; xmm1 = (xmm0) as u128; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) * f32::from_bits(crate::simd_ops::ru32(P6))).to_bits() as u128); xmm8 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(P7))).to_bits() as u128); rcx = (crate::simd_ops::cvt_f32_i64((f32::from_bits(xmm8 as u32)))) as u64; rax = ((rcx as u32)) as u32 as u64; rcx = ((((rcx as u32)) & (0x7fffffu32))) as u32 as u64; rax = ((((rax as u32)) & (0xff800000u32))) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add(0x3f800000u32)) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rcx as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32) as u128; xmm6 = (xmm0) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(P8))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(P9))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(P10))).to_bits() as u128); xmm6 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) * f32::from_bits(crate::simd_ops::ru32(rsp.wrapping_add(0x70u64)))).to_bits() as u128); bb = 5; continue; }
            5 => { xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; xmm6 = (xmm6 & !0xffff_ffffu128) | ((if f32::from_bits(xmm6 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm6 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); crate::simd_ops::wu32(P11, ((xmm7 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm6 as u32)) as u32); bb = 6; continue; }
            6 => { rax = 0x1; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rax = 0x1f; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x58u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a3e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f89a0;
    let (mut rax, mut r10, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = ((r9 as u32)) as u32 as u64;
    rdi = ((r8 as u32)) as u32 as u64;
    rsi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r9 = ((rbx as u32)) as u32 as u64;
    r8 = ((rdi as u32)) as u32 as u64;
    rdx = rsi;
    r10 = (crate::simd_ops::ru64(rax)) as u64;
    rcx = rax;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(({ let t = (crate::simd_ops::ru64(r10)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x2; const P2: u64 = 0x141524330; const P3: u64 = 0x34; const P4: u64 = 0x1403cd310; const P5: u64 = 0x143930f08; const P6: u64 = 0x1415aa7b0; const P7: u64 = 0x4; const P8: u64 = 0x14183acbc; const P9: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 3; continue; }
            3 => { rax = (0xffffffffffffffffu64) as u64; bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(rax.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rcx = 0; fl_l = 0; fl_r = 0; rsi = ((rax.wrapping_mul(2).wrapping_add(P1)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rdi.wrapping_add(P3)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            8 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P5)) as u64; rdx = 0x2a; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r8 = rsi; rdx = rbx; rcx = rdi; crate::simd_ops::wu64(rax.wrapping_add(P7), rdi); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P5)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(P9), (1) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rax = 0x1f; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a500(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141581680; const P2: u64 = 0x1415ab070; const P3: u64 = 0x143930f08; const P4: u64 = 0x1415aa7b0; const P5: u64 = 0x4; const P6: u64 = 0xc; const P7: u64 = 0x10; const P8: u64 = 0x14; const P9: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = ((r8 as u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = 0x19; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(P5), rbx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P6), (0x4u32) as u32); crate::simd_ops::wu32(rax.wrapping_add(P7), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P8), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(P9), (1) as u32); rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x5e; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a5a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1403cd310; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x1c; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(P4), rbx); rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P5), (1) as u32)) as u32 as u64; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a5f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415f8bb0;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rcx = (crate::simd_ops::ru64(rdx)) as u64;
    rbx = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415ab0c0; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r14 = ((r8 as u32)) as u32 as u64; r15 = rdx; rbp = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x110); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x148u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 11; continue; }
            11 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 17; continue; }
            14 => { rsi = rbx.wrapping_add(0x110); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 15; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            16 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { bb = 19; continue; }
            19 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 19; continue; } bb = 21; continue; }
            21 => { bb = 26; continue; }
            22 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            25 => { rsi = rbx.wrapping_add(0x110); bb = 26; continue; }
            26 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 30; continue; } bb = 28; continue; }
            28 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = ((rdi.wrapping_add(0xfu64)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            30 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = 0x24; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(0xcu64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x4u64), r15); crate::simd_ops::wu32(rax.wrapping_add(0x18u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a7a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415b16f0; const P6: u64 = 0x143814e30; const P7: u64 = 0x143930f78; const P8: u64 = 0x1415f3730; const P9: u64 = 0x143930f08; const P10: u64 = 0x1415a73a0; const P11: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r15 = ((r9 as u32)) as u32 as u64; r14 = r8; rbp = ((rdx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rdi); rdi = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rdi.wrapping_add(0x110); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x150u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x148u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            13 => { rsi = rdi.wrapping_add(0x110); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            15 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rsi = rdi.wrapping_add(0x110); bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = ((rbx.wrapping_add(0x2u64)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            29 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P6)) as u64; r9 = ((rbp as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P7)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { fl_l = r14; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(P9)) as u64; r8 = rsp.wrapping_add(0x38); r9 = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rdx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), ((r15 as u32)) as u32); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 33; continue; }
            33 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 35; continue; } bb = 34; continue; }
            34 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1403cd310; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0x2a;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), (0x0u64) as u64);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P5), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158a9e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x141200330; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0x8; const P6: u64 = 0xc; const P7: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rdi = ((r8 as u32)) as u32 as u64;
    rsi = ((rdx as u32)) as u32 as u64;
    rbx = ((rcx as u32)) as u32 as u64;
    if (((fl_l as u8)) == ((fl_r as u8))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x21; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rax.wrapping_add(P4), ((rbx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P5), ((rsi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P6), ((rdi as u32)) as u32); rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax.wrapping_add(P7), (1) as u32)) as u32 as u64; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158aa50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415ab090; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x28; const P5: u64 = 0xc; const P6: u64 = 0x1c; const P7: u64 = 0x24; const P8: u64 = 0x14; const P9: u64 = 0x2c; const P10: u64 = 0x30; const P11: u64 = 0x4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = ((rdx as u32)) as u32 as u64; rsi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P2)) as u64; rdx = 0x29; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_add(P4), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P5), ((rbx as u32)) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P6), rcx); crate::simd_ops::wu32(rax.wrapping_add(P7), ((rcx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(P8), rcx); crate::simd_ops::wu32(rax.wrapping_add(P9), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P10), (0x1u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P11), (0xffffffffffffffffu64) as u64); rax = 0x1; rcx = (crate::simd_ops::ru64(P2)) as u64; crate::simd_ops::lk_sub_u32(rcx.wrapping_add(0xb4u64), (1) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158aaf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f08; const P2: u64 = 0x1438152a8; const P3: u64 = 0x1415abc00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; return rax; }
    r9 = 0;
    fl_l = 0; fl_r = 0;
    r8 = (((rcx as u8)) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64;
    rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64));
    r9 = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { crate::simd_ops::ru32(P2) } else { (r9 as u32) }) as u32 as u64;
    return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158ab20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x142647278; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415ab180; const P4: u64 = 0x1415eba50; const P5: u64 = 0x143930f00; const P6: u64 = 0x1415995a0; const P7: u64 = 0x143930f70; const P8: u64 = 0x1415d9610; const P9: u64 = 0x143931cf0; const P10: u64 = 0x1415fe850; const P11: u64 = 0x1415f27a0; const P12: u64 = 0x14158c0a0; const P13: u64 = 0x143930fc0; const P14: u64 = 0x1415242e0; const P15: u64 = 0x1439311b0; const P16: u64 = 0x143930fa8; const P17: u64 = 0x1415f84c0; const P18: u64 = 0x1401de410; const P19: u64 = 0x143930f80; const P20: u64 = 0x1415adfb0; const P21: u64 = 0x141599810; const P22: u64 = 0x1415abd30; const P23: u64 = 0x1415a4150; const P24: u64 = 0x143930f88; const P25: u64 = 0x1415d30f0; const P26: u64 = 0x1415d3010; const P27: u64 = 0x143930f58; const P28: u64 = 0x143930f78; const P29: u64 = 0x1415f4ab0; const P30: u64 = 0x1415ad0a0; const P31: u64 = 0x143930f90; const P32: u64 = 0x14159b730; const P33: u64 = 0x1415f7410; const P34: u64 = 0x1426455b0; const P35: u64 = 0x1426452a0; const P36: u64 = 0x143930f68; const P37: u64 = 0x1415d0e90; const P38: u64 = 0x143930f48; const P39: u64 = 0x1415fd100; const P40: u64 = 0x143930f60; const P41: u64 = 0x1415b0770; const P42: u64 = 0x1415af4b0; const P43: u64 = 0x1415d95c0; const P44: u64 = 0x143930fa0; const P45: u64 = 0x1415c5080; const P46: u64 = 0x1415b1c60; const P47: u64 = 0x1415b27d0; const P48: u64 = 0x1415d3290; const P49: u64 = 0x1415d8920; const P50: u64 = 0x1415aee50; const P51: u64 = 0x1415cf5e0; const P52: u64 = 0x143930f1d; const P53: u64 = 0x143930f40; const P54: u64 = 0x1426472d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdx = 0x2; crate::simd_ops::wu8(P0, (0x0u8) as u8); rcx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P2)) as u64; rbx = ((rax as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            2 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(P5)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (0x4u64) as u64); rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), ((r12 as u8)) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r12 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P7)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(P9)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = 0x80; let bit_in: u32 = (rcx as u32); if bit_in != 0 { rdx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = 0xd; rdi = P13; rbp = ((r15 as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 11; continue; }
            11 => { rax = rsi; bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rcx = rdi; rdx = (0xfffffffffffffff0u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), r12); rax = r12; bb = 14; continue; }
            14 => { crate::simd_ops::lk_sub_u32(rdx.wrapping_add(rcx), (1) as u32); r14 = ((rdx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rdi, rax); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; rax = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 17; continue; }
            17 => { bb = 19; continue; }
            18 => { r14 = rdi.wrapping_sub(0x10); bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), r12); rdi = ((rdi).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu32(r14, ((r12 as u32)) as u32); fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 20; continue; }
            20 => { rdi = P15; bb = 21; continue; }
            21 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 22; continue; }
            22 => { rax = rsi; bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; rcx = rdi; rdx = (0xfffffffffffffff0u64) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), r12); rax = r12; bb = 25; continue; }
            25 => { crate::simd_ops::lk_sub_u32(rdx.wrapping_add(rcx), (1) as u32); r14 = ((rdx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(rdi, rax); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { rsi = (crate::simd_ops::ru64(rdi)) as u64; rax = rsi; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 23; continue; } bb = 28; continue; }
            28 => { bb = 30; continue; }
            29 => { r14 = rdi.wrapping_sub(0x10); bb = 30; continue; }
            30 => { crate::simd_ops::wu64(rdi, r12); crate::simd_ops::wu64(rdi.wrapping_sub(0x8u64), r12); rdi = ((rdi).wrapping_add(0x18u64)) as u64; crate::simd_ops::wu32(r14, ((r12 as u32)) as u32); fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(P16)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 32; continue; }
            32 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(P16)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = ((r15.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 34; continue; }
            34 => { crate::simd_ops::wu64(P16, r12); bb = 35; continue; }
            35 => { rcx = (crate::simd_ops::ru64(P19)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 36; continue; }
            36 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(P19)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 37; continue; }
            37 => { rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 38; continue; }
            38 => { crate::simd_ops::wu64(P19, r12); bb = 39; continue; }
            39 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P2)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 43; continue; } bb = 40; continue; }
            40 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P22, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(P2)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 41; continue; }
            41 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P23, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            42 => { crate::simd_ops::wu64(P2, r12); bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(P5)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 47; continue; } bb = 44; continue; }
            44 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(P5)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 46; continue; }
            46 => { crate::simd_ops::wu64(P5, r12); bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(P24)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 49; continue; } bb = 48; continue; }
            48 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P25, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P24)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P26, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 49; continue; }
            49 => { rdx = (crate::simd_ops::ru64(P27)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 51; continue; } bb = 50; continue; }
            50 => { rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P27, r12); bb = 51; continue; }
            51 => { rcx = (crate::simd_ops::ru64(P28)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 52; continue; }
            52 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P29, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(P28)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P30, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            54 => { crate::simd_ops::wu64(P28, r12); bb = 55; continue; }
            55 => { rcx = (crate::simd_ops::ru64(P31)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 61; continue; } bb = 56; continue; }
            56 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P32, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(P31)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 60; continue; } bb = 57; continue; }
            57 => { rcx = rdi.wrapping_add(0x48); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P33, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 59; continue; } bb = 58; continue; }
            58 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P34), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 59; continue; }
            59 => { rcx = rdi.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P35), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 60; continue; }
            60 => { crate::simd_ops::wu64(P31, r12); bb = 61; continue; }
            61 => { rcx = (crate::simd_ops::ru64(P36)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 62; continue; }
            62 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P37, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(P36)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 64; continue; } bb = 63; continue; }
            63 => { rcx = rdi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P35), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 64; continue; }
            64 => { crate::simd_ops::wu64(P36, r12); bb = 65; continue; }
            65 => { rcx = (crate::simd_ops::ru64(P38)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 66; continue; }
            66 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P39, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(P38)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 68; continue; }
            68 => { crate::simd_ops::wu64(P38, r12); bb = 69; continue; }
            69 => { rcx = (crate::simd_ops::ru64(P40)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 73; continue; } bb = 70; continue; }
            70 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P41, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(P40)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 72; continue; } bb = 71; continue; }
            71 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P42, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 72; continue; }
            72 => { crate::simd_ops::wu64(P40, r12); bb = 73; continue; }
            73 => { rcx = (crate::simd_ops::ru64(P7)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 77; continue; } bb = 74; continue; }
            74 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P43, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(P7)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 76; continue; } bb = 75; continue; }
            75 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 76; continue; }
            76 => { crate::simd_ops::wu64(P7, r12); bb = 77; continue; }
            77 => { rcx = (crate::simd_ops::ru64(P44)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 81; continue; } bb = 78; continue; }
            78 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P45, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(P44)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 80; continue; } bb = 79; continue; }
            79 => { rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P45, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = 0x10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 80; continue; }
            80 => { crate::simd_ops::wu64(P44, r12); bb = 81; continue; }
            81 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P46, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 83; continue; } bb = 82; continue; }
            82 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P47, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 83; continue; }
            83 => { rcx = (crate::simd_ops::ru64(P24)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 91; continue; } bb = 84; continue; }
            84 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P48, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = (crate::simd_ops::ru64(P24)) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 90; continue; } bb = 85; continue; }
            85 => { rsi = 0xe; rdi = rbp.wrapping_add(0x508); bb = 86; continue; }
            86 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_sub(0x58u64))) as u64; rdi = rdi.wrapping_sub(0x58); rsi = ((rsi).wrapping_sub(1)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 88; continue; } bb = 87; continue; }
            87 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P34), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 88; continue; }
            88 => { rcx = rdi.wrapping_sub(0x30); crate::simd_ops::wu64(rdi, r12); (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P35), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 86; continue; } bb = 89; continue; }
            89 => { rdx = rbp; rcx = ((rsi.wrapping_add(0x10u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 90; continue; }
            90 => { crate::simd_ops::wu64(P24, r12); bb = 91; continue; }
            91 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P49, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P50, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P51, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(P52, ((r12 as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu32(P53, ((xmm0 as u32)) as u32); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 93; continue; } bb = 92; continue; }
            92 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P54), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 93; continue; }
            93 => { r11 = rsp.wrapping_add(0x50); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158b060(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f00; const P2: u64 = 0x1415bc070;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; return rax; }
    r9 = r8;
    r8 = ((rdx as u32)) as u32 as u64;
    rdx = rcx;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158b090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x14158b560;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = ((r8 as u32)) as u32 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x38u64)) as u64; return rax; }
    r9 = rdx;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64);
    rdx = ((rax as u32)) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8);
    r8 = rsp.wrapping_add(0x20);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158b0d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0xffffffffffffffff; const P2: u64 = 0x143930f00; const P3: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0xd8u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r10 = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0xd8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; r11 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r11); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((r11 as u8)) as u8); rax = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdx); fl_l = 0; fl_r = rdx; rdx = (0u64.wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rcx as u32)) as u32); rdx = rsp.wrapping_add(0x80); rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; rax = ((((rax as u32)) & (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r8); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r9); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((r11 as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0xd8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { rax = 0x64; rsp = ((rsp).wrapping_add(0xd8u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158b560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f00; const P1: u64 = 0x20; const P2: u64 = 0x143930f50; const P3: u64 = 0xffffffffffffffff; const P4: u64 = 0x18; const P5: u64 = 0x28;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rbp = ((rcx as u32)) as u32 as u64; rbx = ((rdx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rsp.wrapping_add(0x20); rdi = r9; rsi = r8; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = ((rbx.wrapping_add(P3)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm1 = (crate::simd_ops::ru128(rsi)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rcx); rax = rsp.wrapping_add(0x20); rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbp as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x0u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x2u32) as u32); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; bb = 5; continue; }
            4 => { r8 = 0x64; bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rdx = rsp.wrapping_add(0x20); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158b680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415f6550;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2c0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x2c0u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x2c8); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x308u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x300u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            13 => { rsi = rbx.wrapping_add(0x2c8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2c0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            15 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2c0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x2c0u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2c0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x2f0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x2f8u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rsi = rbx.wrapping_add(0x2c8); bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = ((rdi.wrapping_add(0x5eu64)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            28 => { rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdi.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158b7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415ab0c0; const P6: u64 = 0x143930f08; const P7: u64 = 0x1415aa7b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbp = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rsi = rbx.wrapping_add(0x110); bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x150u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x148u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdi = r14; bb = 11; continue; }
            11 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            13 => { rsi = rbx.wrapping_add(0x110); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 6; continue; } bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            15 => { rdi = r14; bb = 16; continue; }
            16 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 24; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx.wrapping_add(0x108u64), fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x108u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 18; continue; } bb = 20; continue; }
            20 => { bb = 25; continue; }
            21 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 26; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x138u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 26; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x140u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            24 => { rsi = rbx.wrapping_add(0x110); bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P6)) as u64; rdx = 0x25; r8 = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax.wrapping_add(0xcu64), rdi); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu16(rax.wrapping_add(0x1au64), ((r14 as u16)) as u16); r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rbp as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x4u64), (0xffffffffffffffffu64) as u64); rax = (crate::simd_ops::ru64(P6)) as u64; crate::simd_ops::lk_sub_u32(rax.wrapping_add(0xb4u64), (1) as u32); rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            28 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = 0xf; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158b970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158b7e0; const P1: u64 = 0x20; const P2: u64 = 0xffffffffffffffbf; const P3: u64 = 0x1;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (rcx) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); } r9 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; rcx = (0x811c9dc5u32) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rax = ((r9.wrapping_add(P1)) as u32) as u32 as u64; rdx = (((rax as u8)) as u32) as u32 as u64; r8 = ((r9.wrapping_add(P2)) as u32) as u32 as u64; rax = (((r9 as u8)) as u32) as u32 as u64; r10 = ((r10.wrapping_add(P3)) as u64) as u64; r9 = ((crate::simd_ops::ru8(r10)) as u32) as u32 as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = (0x19u8) as u8 as u64; rdx = (if (((fl_l as u8)) > ((fl_r as u8))) { (rax as u32) } else { (rdx as u32) }) as u32 as u64; rax = (((rdx as u8)) as u32) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = ((r9 as u8)) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::forward_incoming_to_outgoing(); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158b9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415810d0; const P1: u64 = 0x14158b7e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rax as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158b9f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1403cd310; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0xc;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), (0xffffffffffffffffu64) as u64);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P5), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158ba50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143931b40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(P0, (0x0u64) as u64);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158ba70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1415b16f0; const P2: u64 = 0x1415e9510; const P3: u64 = 0x1415ea230; const P4: u64 = 0x14158eaa0; const P5: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; r15 = r8; rbx = rdx; r14 = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = ((r14.wrapping_add(0x1fu64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rcx = rsp.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rbx = ((rax.wrapping_add(0x5du64)) as u32) as u32 as u64; bb = 19; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x88u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbp); rbp = 0; fl_l = 0; fl_r = 0; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); bb = 8; continue; }
            8 => { r8 = r15; rdx = r14; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rbp = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { bb = 17; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x88u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), rax); bb = 14; continue; }
            13 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); bb = 14; continue; }
            14 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 16; continue; } bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rbp); bb = 16; continue; }
            16 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbx = 0x1; bb = 19; continue; }
            19 => { rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158bb70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x1403cd310; const P2: u64 = 0x143930f08; const P3: u64 = 0x1415aa7b0; const P4: u64 = 0x4; const P5: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rbx = rcx;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = rbx; fl_r = (0xffffffffffffffe0u64) as u64;
    if !((((fl_l as u64)) < ((fl_r as u64)))) { rax = 0x1f; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    rdx = 0xc;
    r8 = (((rax as u16)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu64(rax.wrapping_add(P4), rbx);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::lk_sub_u32(rax.wrapping_add(P5), (1) as u32);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158bbe0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x140000000; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rsi = ((rdx as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = 0x1f; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x2000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 18; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rdx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { r12 = 0; fl_l = 0; fl_r = 0; r15 = P1; bb = 3; continue; }
            3 => { let bit_in: u32 = (rsi as u32); if bit_in != 0 { r14 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(r15).wrapping_add(0x2688bf0u64))) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 4; continue; }
            4 => { rax = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; rcx = r12; r8 = ((rax.wrapping_mul(8)) as u64) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(r15).wrapping_add(0x39311b0u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { bb = 15; continue; }
            9 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(r15).wrapping_add(0x39311b0u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r8.wrapping_add(r15).wrapping_add(0x39311b0u64), rdx); bb = 12; continue; }
            11 => { crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdx); bb = 12; continue; }
            12 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(r15).wrapping_add(0x39311a8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu64(r8.wrapping_add(r15).wrapping_add(0x39311a8u64), rcx); bb = 14; continue; }
            14 => { crate::simd_ops::lk_sub_u32(r8.wrapping_add(r15).wrapping_add(0x39311a0u64), (1) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x1; bb = 15; continue; }
            15 => { rcx = ((r14 as u32)) as u32 as u64; rax = 0x1; rax = (((rax as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rax = (!((rax as u32))) as u32 as u64; fl_l = ((((rsi as u32)) & ((rax as u32)))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 16; continue; }
            16 => { rbp = ((rdx as u32)) as u32 as u64; bb = 17; continue; }
            17 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 18; continue; }
            18 => { rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = ((rbp as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158bd10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158bd20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = rdx;
    rdx = ((r8 as u32)) as u32 as u64;
    return crate::simd_ops::call_fwd(P0, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158bd20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930fb0; const P2: u64 = 0x1415242e0; const P3: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rsi = ((rdx as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbp = 0x1f; fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x2000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 16; continue; } bb = 1; continue; }
            1 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r12); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = P1; bb = 3; continue; }
            3 => { let bit_in: u32 = (rsi as u32); if bit_in != 0 { r14 = ((bit_in.trailing_zeros() as u32)) as u32 as u64; } fl_l = bit_in as u64; rcx = ((r14.wrapping_add(r14.wrapping_mul(2))) as u64) as u64; r8 = ((rcx.wrapping_mul(8)) as u64) as u64; rcx = r12; rbx = (crate::simd_ops::ru64(r8.wrapping_add(r15).wrapping_add(0x10u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 7; continue; }
            7 => { bb = 14; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_l = rbx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(r15).wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(r8.wrapping_add(r15).wrapping_add(0x10u64), rdx); bb = 11; continue; }
            10 => { crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rdx); bb = 11; continue; }
            11 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(r15).wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r8.wrapping_add(r15).wrapping_add(0x8u64), rcx); bb = 13; continue; }
            13 => { crate::simd_ops::lk_sub_u32(r8.wrapping_add(r15), (1) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = 0x10; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = 0x1; bb = 14; continue; }
            14 => { rcx = ((r14 as u32)) as u32 as u64; rdx = 0x1; rdx = (((rdx as u32)).wrapping_shl(((rcx as u32) & 31))) as u32 as u64; rdx = (!((rdx as u32))) as u32 as u64; fl_l = ((((rsi as u32)) & ((rdx as u32)))) as u32 as u64; fl_r = fl_l; rsi = ((fl_l as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 3; continue; } bb = 15; continue; }
            15 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 16; continue; }
            16 => { rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = ((rbp as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158be60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f08; const P1: u64 = 0x143930f1c; const P2: u64 = 0x143931b48; const P3: u64 = 0x143931b50; const P4: u64 = 0x1415d2f70; const P5: u64 = 0x142645618; const P6: u64 = 0x1415aa630;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru64(P0)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(P3)) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rcx).wrapping_add(0x90u64)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = 0x5; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru8(P1)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(P0)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(P0)) as u64; rcx = ((rcx).wrapping_add(0x90u64)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = 0x5; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(P2)) as u32 as u64; rbx = (((rbx as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xc8; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 7; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 12; continue; }
            12 => { rdx = 0x1; rax = 0x5c; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            13 => { rax = 0x66; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158bf40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143930f08; const P2: u64 = 0x1415abc00;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; return rax; }
    r9 = ((rcx as u32)) as u32 as u64;
    r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64));
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    return crate::simd_ops::call_fwd(P2, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158bf70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14158c0a0;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let bit_in: u32 = (rcx as u32);
    if bit_in != 0 { rdx = ((bit_in.trailing_zeros() as u32)) as u32 as u64; }
    fl_l = bit_in as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x10u64), (0x0u32) as u32);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c0a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930fc0; const P1: u64 = 0x142688e08; const P2: u64 = 0x8; const P3: u64 = 0x10; const P4: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rdi = ((rcx as u32)) as u32 as u64;
    rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = P0;
    rbx = (crate::simd_ops::ru64(rax.wrapping_add(rbx.wrapping_mul(8)))) as u64;
    fl_l = rbx; fl_r = rbx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi);
    rsi = P1;
    loop { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rcx = rsp.wrapping_add(0x20); r9 = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r9); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r8); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(r9, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143930f1c; const P1: u64 = 0x143426fd0; const P2: u64 = 0x1415ab070; const P3: u64 = 0x143930f08; const P4: u64 = 0x2; const P5: u64 = 0x1415aa7b0; const P6: u64 = 0x1415ab0f0; const P7: u64 = 0x18; const P8: u64 = 0x1c; const P9: u64 = 0x20; const P10: u64 = 0xc; const P11: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = ((r9 as u32)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm6) as u128); rbp = r8; xmm6 = (xmm1) as u128; r14 = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0x66; bb = 10; continue; }
            2 => { rax = ((xmm6 as u32)) as u32 as u64; fl_l = ((((rax as u32)) >> (31u32)) & 1) as u64; rax = (((rax as u32)) & !((1u32) << (31u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7f800000; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } bb = 3; continue; }
            3 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 9; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x70u64))) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = ((rdi.wrapping_add(P4)) as u32) as u32 as u64; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = 0x3; r8 = (((rax as u16)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x68u64))) as u32 as u64; crate::simd_ops::wu32(rax.wrapping_add(P7), ((rdi as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P8), ((rcx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(P9), ((rbx as u8)) as u8); bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(P10), ((r14 as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(P11), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x4u64), rbp); crate::simd_ops::wu32(rax.wrapping_add(0x14u64), ((rsi as u32)) as u32); rax = (crate::simd_ops::ru64(P3)) as u64; rax = ((rax).wrapping_add(0xb4u64)) as u64; fl_l = (crate::simd_ops::lk_sub_u32(rax, (1) as u32)) as u32 as u64; rax = 0x1; bb = 10; continue; }
            9 => { rax = 0x5f; bb = 10; continue; }
            10 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c2b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142688fa8; const P1: u64 = 0x141604950; const P2: u64 = 0x70; const P3: u64 = 0x1416049d0; const P4: u64 = 0x20; const P5: u64 = 0x10; const P6: u64 = 0x142688f98; const P7: u64 = 0x18; const P8: u64 = 0x14392d458; const P9: u64 = 0x14392d450; const P10: u64 = 0x14392d448; const P11: u64 = 0x142688f88; const P12: u64 = 0x141834d30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rcx, rax); rbx = rcx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rax = P6; crate::simd_ops::wu64(rbx, rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(P7), rax); bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P8)) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(P8, rax); bb = 5; continue; }
            4 => { crate::simd_ops::wu64(rdx.wrapping_add(P5), rax); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(P9)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rax }) as u64; crate::simd_ops::lk_sub_u32(P10, (1) as u32); crate::simd_ops::wu64(P9, rax); rax = P11; crate::simd_ops::wu64(rbx, rax); fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = 0xf0; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f78; const P2: u64 = 0x1415f3640; const P3: u64 = 0x1415b1830;
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    rdx = rbx;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x38);
    rbx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rbx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14158c400(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1415b16f0; const P1: u64 = 0x143930f88; const P2: u64 = 0x1426452b0; const P3: u64 = 0x1426452a8; const P4: u64 = 0x142645560; const P5: u64 = 0x1415b1830;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); r12 = r8; rbp = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); rcx = rsp.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); r15 = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rsi = rbx.wrapping_add(0x8); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } bb = 8; continue; }
            8 => { rdi = r14; bb = 9; continue; }
            9 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x14u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            11 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 4; continue; } bb = 12; continue; }
            12 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            13 => { rdi = r14; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { bb = 16; continue; }
            16 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 23; continue; }
            19 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 24; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            22 => { rsi = rbx.wrapping_add(0x8); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = rdi.wrapping_sub(0x10); fl_l = rdi; fl_r = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rbx }) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rbx = 0xf; bb = 33; continue; }
            26 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((r15 as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x1a0u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx.wrapping_add(0x10); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 30; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu32(r12, ((rax as u32)) as u32); rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rbx = 0x1; bb = 33; continue; }
            30 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { rbx = 0x3; bb = 33; continue; }
            33 => { rcx = rsp.wrapping_add(0x68); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = ((rbx as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}
