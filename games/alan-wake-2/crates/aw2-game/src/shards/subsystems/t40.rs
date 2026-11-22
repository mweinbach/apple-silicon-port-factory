#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_140002a80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14183acc2; const P2: u64 = 0x14397ce90; const P3: u64 = 0x1426c6fe0; const P4: u64 = 0x140002970; const P5: u64 = 0x1426c6fc8; const P6: u64 = 0x142645758; const P7: u64 = 0x14397ce98; const P8: u64 = 0x142645350; const P9: u64 = 0x1400022c0; const P10: u64 = 0x1426c6fc4; const P11: u64 = 0x140002460;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 75>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x250u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x240u64), rax); rdi = 0; fl_l = 0; fl_r = 0; rbx = rdx; rsi = ((rcx as u32)) as u32 as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rcx = rsp.wrapping_add(0x30); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x20a; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(P2)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = P3; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P2, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = P5; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P7, rax); bb = 5; continue; }
            4 => { rax = (crate::simd_ops::ru64(P7)) as u64; bb = 5; continue; }
            5 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            7 => { rcx = rsp.wrapping_add(0x30); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = rbx; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rdi = rsp.wrapping_add(0x30); rcx = (!(rcx)) as u64; rdx = rcx.wrapping_sub(0x1); rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; rsi = ((rdx.wrapping_add(rcx).wrapping_add(0x1u64)) as u64) as u64; rcx = ((rax.wrapping_add(0x40u64)) as u32) as u32 as u64; rdx = ((rsi.wrapping_add(rsi)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 9; continue; }
            9 => { r8 = rsp.wrapping_add(0x30); rdx = rsi; rcx = rax; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); r8 = P10; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rax = rdi; bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x240u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x250); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140002bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d000; const P1: u64 = 0x142645480; const P2: u64 = 0x14397d004; const P3: u64 = 0x14397d008; const P4: u64 = 0x14397d114; const P5: u64 = 0x142645478; const P6: u64 = 0x143821ecc; const P7: u64 = 0x142645488; const P8: u64 = 0x142645350; const P9: u64 = 0x1426c6f0c; const P10: u64 = 0x140002a80; const P11: u64 = 0x14183ad34; const P12: u64 = 0x1426456f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r13 = 0; fl_l = 0; fl_r = 0; r12 = rcx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x24u32) as u32); rbx = r13; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x26u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2au32) as u32); rbp = r13; rdi = ((r13 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((r13.wrapping_add(0x2u64)) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | (((0x3u8) as u8 as u64)); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((r13.wrapping_add(0x1u64)) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | (((0x3u8) as u8 as u64)); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((r13.wrapping_add(0x20u64)) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | (((0x3u8) as u8 as u64)); rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdx = ((r13.wrapping_add(0x23u64)) as u32) as u32 as u64; rcx = P0; r8 = rax; crate::simd_ops::wu32(P0, (0x11cu32) as u32); crate::simd_ops::wu32(P2, (0x6u32) as u32); crate::simd_ops::wu32(P3, (0x1u32) as u32); crate::simd_ops::wu16(P4, ((r13 as u16)) as u16); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rcx = ((r13 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(P6, ((rcx as u32)) as u32); bb = 3; continue; }
            2 => { rcx = (crate::simd_ops::ru32(P6)) as u32 as u64; bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x1; bb = 15; continue; }
            5 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 6; continue; }
            6 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rdx = rsi; rcx = 0x40; rdx = ((rdx).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 8; continue; }
            8 => { r9 = 0; fl_l = 0; fl_r = 0; r8 = rax; rdx = ((rsi as u32)) as u32 as u64; rcx = r12; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { rsi = r13; bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(rsi.wrapping_mul(4)).wrapping_add(0x20u64))) as u32 as u64; rdx = P9; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 11; continue; }
            11 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = rbx; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rdx = rbx; rcx = (!(rcx)) as u64; r8 = rcx.wrapping_sub(0x1); rcx = rbp; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rdi = ((r13 as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rdi = (rdi & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(1)) as u64; rbx = r13; fl_l = rsi; fl_r = (0x3u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 10; continue; } bb = 13; continue; }
            13 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 14; continue; }
            14 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rax = ((rdi as u32)) as u32 as u64; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140002d90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x40; const P2: u64 = 0x142645350; const P3: u64 = 0x1; const P4: u64 = 0x1400022c0; const P5: u64 = 0x1426456f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = rbx; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; rsi = ((rcx.wrapping_add(P0)) as u64) as u64; rdx = ((rcx.wrapping_add(rcx)) as u64) as u64; rcx = ((rax.wrapping_add(P1)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = ((rsi.wrapping_add(P3)) as u64) as u64; r8 = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140002e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x142645350; const P2: u64 = 0x1400023d0; const P3: u64 = 0x1426456f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) < 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = rsi; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; fl_l = rdx; fl_r = rcx; rbp = rcx; rcx = ((rax.wrapping_add(P0)) as u32) as u32 as u64; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { rdx } else { rbp }) as u64; rdx = ((rbp.wrapping_add(rbp).wrapping_add(0x2u64)) as u64) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { rdx = rbp.wrapping_add(0x1); r9 = rbx; r8 = rsi; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { rax = rdi; bb = 3; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140002ec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397cef0; const P1: u64 = 0x1426c7030; const P2: u64 = 0x140002970; const P3: u64 = 0x14397cf08; const P4: u64 = 0x1426c7050; const P5: u64 = 0x142645758; const P6: u64 = 0x142645350; const P7: u64 = 0x142645708; const P8: u64 = 0x1426456f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r13 = 0; fl_l = 0; fl_r = 0; rbp = rcx; rcx = (crate::simd_ops::ru64(P0)) as u64; rdi = r9; r12 = r8; rsi = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((r13 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r13 as u32)) as u32); rbx = ((r13 as u32)) as u32 as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; crate::simd_ops::wu64(P0, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(P3)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = P4; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x30); r9 = rsp.wrapping_add(0x34); r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = rbp; rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r13); rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rbx = ((rax.wrapping_sub(0x3u64)) as u32) as u32 as u64; bb = 11; continue; }
            10 => { rbx = 0x2; bb = 11; continue; }
            11 => { rdx = ((rbx.wrapping_add(rcx)) as u32) as u32 as u64; rcx = 0x40; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            13 => { rcx = rsp.wrapping_add(0x30); r9 = r12; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); rcx = rbp; rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi, r13); bb = 15; continue; }
            15 => { rax = ((rbx as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140003000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14397cffc; const P2: u64 = 0x142645480; const P3: u64 = 0x14183acc2; const P4: u64 = 0x142645478; const P5: u64 = 0x14397cef0; const P6: u64 = 0x1426c7030; const P7: u64 = 0x140002970; const P8: u64 = 0x14397cef8; const P9: u64 = 0x1426c7078; const P10: u64 = 0x142645758; const P11: u64 = 0x14397cf10; const P12: u64 = 0x1426c7068; const P13: u64 = 0x1426c70b0; const P14: u64 = 0x1426c7088; const P15: u64 = 0x140002ec0; const P16: u64 = 0x14183ad16; const P17: u64 = 0x142645708; const P18: u64 = 0x1426456f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 51>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x180u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x170u64), rax); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rsi = 0; fl_l = 0; fl_r = 0; r12 = ((rcx as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 1; continue; }
            1 => { rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; r8 = (r8 & !0xffu64) | (((0x3u8) as u8 as u64)); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P2), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x50); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x11c; rdi = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x50); r8 = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((r12 as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rsi.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 24; continue; }
            3 => { rax = (crate::simd_ops::ru64(P5)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x1b0u64), rbp); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rsi as u32)) as u32); rbp = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rsi); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = P6; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P5, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(P8)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = P9; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; crate::simd_ops::wu64(P8, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(P5)) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(P11)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = P12; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P11, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(P8)) as u64; bb = 11; continue; }
            11 => { rax = rsp.wrapping_add(0x38); rdx = P13; r9 = 0x1; r8 = 0; fl_l = 0; fl_r = 0; rcx = (0xffffffff80000002u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(rdi, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r9 = rsp.wrapping_add(0x40); r8 = rsp.wrapping_add(0x30); rdx = P14; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rdi = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 13; continue; }
            13 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 22; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 15; continue; }
            15 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = rbp; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } crate::simd_ops::wu64(rsp.wrapping_add(0x1a8u64), rbx); rbx = rbp; rcx = (!(rcx)) as u64; rdi = ((rbp.wrapping_add(rcx.wrapping_mul(2)).wrapping_sub(0x2u64)) as u64) as u64; fl_l = rbp; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 16; continue; }
            16 => { rcx = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rbx = ((rbx).wrapping_add(0x2u64)) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rax = ((crate::simd_ops::ru16(rbx.wrapping_sub(0x2u64))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(2)).wrapping_sub(0x30u64)) as u32) as u32 as u64; crate::simd_ops::wu32(P1, ((rcx as u32)) as u32); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } bb = 18; continue; }
            18 => { bb = 21; continue; }
            19 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P17), rax), rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; bb = 13; continue; }
            20 => { crate::simd_ops::wu32(P1, ((rsi as u32)) as u32); bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x1a8u64))) as u64; bb = 22; continue; }
            22 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P18), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x1b0u64))) as u64; bb = 23; continue; }
            23 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; rsi = (rsi & !0xffu64) | (((if (((fl_l as u32)) >= ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rsi as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x170u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0x180u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140003230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397ce50; const P1: u64 = 0x1426c71b0; const P2: u64 = 0x140002970; const P3: u64 = 0x14397ce58; const P4: u64 = 0x1426c7198; const P5: u64 = 0x142645758; const P6: u64 = 0x14397ce60; const P7: u64 = 0x1426c7180; const P8: u64 = 0x14397ce70; const P9: u64 = 0x1426c7158; const P10: u64 = 0x14397ce68; const P11: u64 = 0x1426c7130; const P12: u64 = 0x14397ce80; const P13: u64 = 0x1426c7110; const P14: u64 = 0x140002160;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rsi = 0; fl_l = 0; fl_r = 0; rdi = r8; r12 = rdx; rbp = ((rcx as u32)) as u32 as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = P4; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P6)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = P7; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P6, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(P8)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = P9; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P8, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(P10)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdx = P11; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P10, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru64(P12)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rdx = P13; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P12, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            17 => { r9 = 0x6; r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 23; continue; } bb = 18; continue; }
            18 => { rax = 0; fl_l = 0; fl_r = 0; r8 = rdi; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); rcx = rbx; crate::simd_ops::wu32(rdi, (0x20u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { r8 = rdi; rdx = rbx; rcx = ((rbp as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 21; continue; }
            21 => { rax = 0; fl_l = 0; fl_r = 0; rsi = (((rsi as u32)).wrapping_add(1)) as u32 as u64; r8 = rdi; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x18u64), rax); rdx = ((rsi as u32)) as u32 as u64; rcx = rbx; crate::simd_ops::wu32(rdi, (0x20u32) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 20; continue; } bb = 22; continue; }
            22 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rax = 0; fl_l = 0; fl_r = 0; bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 25; continue; }
            25 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            26 => { crate::simd_ops::wu64(r12, rbx); rax = 0x1; bb = 24; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140003420(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426c6f00; const P1: u64 = 0x140002d90; const P2: u64 = 0x140002e20; const P3: u64 = 0x142645350; const P4: u64 = 0x140002340; const P5: u64 = 0x140002530; const P6: u64 = 0x1426456f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = P0; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; r9 = rcx.wrapping_sub(0x1); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 1; continue; }
            1 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rdi = rbx; r11 = 0x2e; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; rdi = ((rbx.wrapping_add(rcx.wrapping_mul(2)).wrapping_sub(0x2u64)) as u64) as u64; r8 = rcx.wrapping_sub(0x1); rax = rdi; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { r10 = 0x5c; bb = 3; continue; }
            3 => { rdx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((r10 as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = ((r11 as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((rax).wrapping_sub(0x2u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { fl_l = ((r11 as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rax)) as u16 as u64; rdi = (if (((fl_l as u16)) == ((fl_r as u16))) { rax } else { rdi }) as u64; rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = rcx; fl_r = (0x1u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            9 => { rdx = r8.wrapping_sub(0x1); rcx = rbx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdi = rax; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { rax = ((r8.wrapping_add(r9)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rcx = 0x40; rsi = ((rax.wrapping_add(rax).wrapping_add(0x2u64)) as u64) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { r8 = rbx; rdx = rsi; rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 13; continue; } bb = 12; continue; }
            12 => { r8 = P0; rdx = rsi; rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            14 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = rdi; bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140003550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397cef0; const P1: u64 = 0x1426c7030; const P2: u64 = 0x140002970; const P3: u64 = 0x14397cf18; const P4: u64 = 0x1426c7020; const P5: u64 = 0x142645758; const P6: u64 = 0x14397cf20; const P7: u64 = 0x1426c7010; const P8: u64 = 0x14397cf28; const P9: u64 = 0x1426c71d0; const P10: u64 = 0x14397cf38; const P11: u64 = 0x1426c6ff8; const P12: u64 = 0x142645350; const P13: u64 = 0x140001ff0; const P14: u64 = 0x140002d90; const P15: u64 = 0x1426456f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rbx = 0; fl_l = 0; fl_r = 0; r12 = rdx; r13 = rcx; rdi = rbx; rbp = rbx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = P4; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P6)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = P7; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P6, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(P8)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = P9; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P8, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(P10)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rdx = P11; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P10, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; bb = 27; continue; }
            14 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; r8 = 0x80000000; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((rbx as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { r8 = 0x80000000; rdx = r12; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 16; continue; }
            16 => { r9 = rsp.wrapping_add(0x50); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 17; continue; }
            17 => { rdx = r11; rcx = 0x40; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; r9 = rsp.wrapping_add(0x50); rdx = rax; rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rdx = r13; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rbp = rax; bb = 22; continue; }
            22 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P15), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 23; continue; }
            23 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            24 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); bb = 26; continue; }
            26 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rbp; bb = 27; continue; }
            27 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140003740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x14397ce50; const P2: u64 = 0x1426c71b0; const P3: u64 = 0x140002970; const P4: u64 = 0x14397ce70; const P5: u64 = 0x1426c7158; const P6: u64 = 0x142645758; const P7: u64 = 0x14397ce80; const P8: u64 = 0x1426c7110; const P9: u64 = 0x140003230; const P10: u64 = 0x142645350; const P11: u64 = 0x140003550; const P12: u64 = 0x1426456f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0xa8u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); rax = 0; fl_l = 0; fl_r = 0; rsi = 0; fl_l = 0; fl_r = 0; rdi = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = (crate::simd_ops::ru64(P1)) as u64; r12 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rsi); rbp = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x20u32) as u32); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P2; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P1, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P4)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = P5; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P4, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P1)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P7)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = P8; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P7, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 32; continue; }
            8 => { r8 = rsp.wrapping_add(0x60); rdx = rsp.wrapping_add(0x48); rcx = ((rdi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r15); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x4u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), (0x11u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x12u32) as u32); r13 = ((rsi as u32)) as u32 as u64; r14 = rsp.wrapping_add(0x50); bb = 10; continue; }
            10 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 30; continue; } bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru32(r14)) as u32 as u64; rax = rsp.wrapping_add(0x48); r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rsp.wrapping_add(0x60); rcx = r15; r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rsi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rsi as u32)) as u32); rdi = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 29; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 16; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rdi = ((rax.wrapping_sub(0x3u64)) as u32) as u32 as u64; bb = 17; continue; }
            16 => { rdi = 0x2; bb = 17; continue; }
            17 => { rdx = ((rdi.wrapping_add(r11)) as u32) as u32 as u64; rcx = 0x40; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x48u64))) as u32 as u64; rdi = 0; fl_l = 0; fl_r = 0; r9 = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rdx = rsp.wrapping_add(0x60); r8 = ((rbx as u32)) as u32 as u64; rcx = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 27; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; rbx = rsi; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } bb = 20; continue; }
            20 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 21; continue; }
            21 => { fl_l = ((rdi as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rsi)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 27; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rdx = rsi; rcx = r12; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 24; continue; }
            24 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rdi = rbx; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rdi = 0; fl_l = 0; fl_r = 0; rcx = (!(rcx)) as u64; rbx = ((rbx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = ((rdi as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rbx)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 23; continue; } bb = 25; continue; }
            25 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            26 => { rdx = rsi; rcx = r12; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbp = rax; bb = 27; continue; }
            27 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            28 => { rsi = 0; fl_l = 0; fl_r = 0; bb = 29; continue; }
            29 => { r13 = (((r13 as u32)).wrapping_add(1)) as u32 as u64; r14 = ((r14).wrapping_add(0x4u64)) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 30; continue; }
            30 => { rcx = r15; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 31; continue; }
            31 => { rax = rbp; bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = (((rcx) ^ (rsp))) as u64; rsp = ((rsp).wrapping_add(0xa8u64)) as u64; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400039f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397cef0; const P1: u64 = 0x1426c7030; const P2: u64 = 0x140002970; const P3: u64 = 0x14397cef8; const P4: u64 = 0x1426c7078; const P5: u64 = 0x142645758; const P6: u64 = 0x14397cf10; const P7: u64 = 0x1426c7068; const P8: u64 = 0x140002ec0; const P9: u64 = 0x1426452c0; const P10: u64 = 0x140001ff0; const P11: u64 = 0x140002d90; const P12: u64 = 0x1426456f0; const P13: u64 = 0x14183ad10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r14 = 0; fl_l = 0; fl_r = 0; r12 = r9; rbx = r8; rbp = rdx; r13 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); rsi = r14; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { rdi = (crate::simd_ops::ru64(P3)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = P4; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; crate::simd_ops::wu64(P3, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P6)) as u64; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = P7; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P6, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdi = (crate::simd_ops::ru64(P3)) as u64; bb = 8; continue; }
            8 => { rax = rsp.wrapping_add(0x30); r9 = 0x1; r8 = 0; fl_l = 0; fl_r = 0; rdx = rbx; rcx = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(rdi, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; bb = 34; continue; }
            10 => { bb = 11; continue; }
            11 => { rdx = (crate::simd_ops::ru64(r12)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r9 = rsp.wrapping_add(0x38); r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r14); crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((r14 as u32)) as u32); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 24; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x88u64))) as u32 as u64; rbx = rbp; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 14; continue; }
            14 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x6; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = 0xd; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 24; continue; }
            16 => { fl_l = ((r14 as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rbp)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 19; continue; } bb = 17; continue; }
            17 => { rdx = r13; rcx = rbx; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = rbx; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; rbx = ((rbx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; fl_l = ((r14 as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rbx)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 17; continue; } bb = 19; continue; }
            19 => { fl_l = ((r14 as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rbx)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rcx = rbx; bb = 23; continue; }
            21 => { rdx = r13; rcx = rbp; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 22; continue; }
            22 => { rcx = rbp; bb = 23; continue; }
            23 => { rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 24; continue; }
            24 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); r12 = ((r12).wrapping_add(0x8u64)) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 25; continue; }
            25 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 26; continue; }
            26 => { rax = ((crate::simd_ops::ru16(rsi)) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 33; continue; } bb = 27; continue; }
            27 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2fu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 33; continue; } bb = 28; continue; }
            28 => { rcx = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 32; continue; } bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru16(rsi.wrapping_add(0x2u64))) as u16 as u64; fl_r = (0x3au16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 32; continue; } bb = 30; continue; }
            30 => { rax = ((crate::simd_ops::ru16(rsi.wrapping_add(0x4u64))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 33; continue; } bb = 31; continue; }
            31 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2fu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P12), rax), rax, rcx, rdx, r8, r9, rsp); rsi = r14; bb = 33; continue; }
            33 => { rax = rsi; bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140003c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397cef0; const P1: u64 = 0x1426c7030; const P2: u64 = 0x140002970; const P3: u64 = 0x14397cef8; const P4: u64 = 0x1426c7078; const P5: u64 = 0x142645758; const P6: u64 = 0x14397cf00; const P7: u64 = 0x1426c71e8; const P8: u64 = 0x14397cf10; const P9: u64 = 0x1426c7068; const P10: u64 = 0x142645350; const P11: u64 = 0x14183acc2; const P12: u64 = 0x140001ff0; const P13: u64 = 0x140002d90; const P14: u64 = 0x1426456f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; r15 = 0; fl_l = 0; fl_r = 0; r12 = r8; r14 = rdx; r13 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r15); rdi = r15; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = P4; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P6)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = P7; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P6, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 8; continue; }
            8 => { fl_l = (crate::simd_ops::ru64(P8)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rdx = P9; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P8, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rsi); bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(r12)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } bb = 13; continue; }
            13 => { rax = rsp.wrapping_add(0x98); r9 = 0x20019; r8 = 0; fl_l = 0; fl_r = 0; rcx = r14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 14; continue; }
            14 => { rcx = ((rax.wrapping_add(0x40u64)) as u32) as u32 as u64; rdx = 0x10000; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 15; continue; }
            15 => { rbp = ((r15 as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x8000; rcx = rbx; crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), (0x7fffu32) as u32); rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); r9 = rsp.wrapping_add(0x90); r8 = rbx; rdx = ((rbp as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 17; continue; }
            17 => { rdx = r13; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 19; continue; }
            19 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 20; continue; }
            20 => { fl_l = ((rsi as u32)) as u32 as u64; fl_r = 0x103; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } bb = 21; continue; }
            21 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { r12 = ((r12).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 24; continue; }
            24 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rax = rdi; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140003e10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452c0; const P1: u64 = 0x14183ad10; const P2: u64 = 0x2; const P3: u64 = 0x4; const P4: u64 = 0x140002bd0; const P5: u64 = 0x1426456a8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = 0; fl_l = 0; fl_r = 0; rdi = ((rdx as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(rbx)) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } bb = 2; continue; }
            2 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2fu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = (((rax as u16)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 10; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(P2))) as u16 as u64; fl_r = (0x3au16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = ((crate::simd_ops::ru16(rbx.wrapping_add(P3))) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2fu16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rcx = 0xa1; bb = 11; continue; }
            9 => { rdi = ((((rdi as u32)) & (0xffffe8f7u32))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; r8 = ((rdi as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); }
            10 => { rcx = 0xa0; bb = 11; continue; }
            11 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140003eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140002d90; const P1: u64 = 0x142645350; const P2: u64 = 0x140002340; const P3: u64 = 0x1426c6fc4; const P4: u64 = 0x140002530; const P5: u64 = 0x1426456f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rsi = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = rdx; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); rdi = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r12); while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rdi = rbx; rcx = (!(rcx)) as u64; rdx = rcx.wrapping_sub(0x1); rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; rcx = ((rcx).wrapping_sub(1)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru16(rsi.wrapping_add(rdx.wrapping_mul(2)).wrapping_sub(0x2u64))) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru16(rbx)) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rbp = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; bb = 10; continue; }
            9 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); rax = ((rax).wrapping_add(rcx)) as u64; rcx = 0x40; rax = ((rax).wrapping_add(rdx)) as u64; r12 = ((rax.wrapping_add(rax).wrapping_add(0x2u64)) as u64) as u64; rdx = r12; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rdi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { r8 = rsi; rdx = r12; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 12; continue; }
            12 => { fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { r8 = P3; rdx = r12; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r8 = rbx; rdx = r12; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdi = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rdi; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140003fc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397cfa8; const P1: u64 = 0x1426c7270; const P2: u64 = 0x140002970; const P3: u64 = 0x14397cfe0; const P4: u64 = 0x1426c7258; const P5: u64 = 0x142645758; const P6: u64 = 0x14397cff0; const P7: u64 = 0x1426c7240; const P8: u64 = 0x14397cfe8; const P9: u64 = 0x1426c7228; const P10: u64 = 0x142645350; const P11: u64 = 0x1426456f0; const P12: u64 = 0x1400022c0; const P13: u64 = 0x140001f80; const P14: u64 = 0x1426c71f8; const P15: u64 = 0x14183ad34; const P16: u64 = 0x140003eb0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; rbx = 0; fl_l = 0; fl_r = 0; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), rax); rax = (crate::simd_ops::ru64(P0)) as u64; r14 = rcx; rsi = rbx; r13 = rbx; crate::simd_ops::wu16(rcx, ((rbx as u16)) as u16); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P1; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P0, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P3)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = P4; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P3, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P6)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rdx = P7; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P6, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 38; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(P0)) as u64; bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(P8)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), r12); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = P9; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rdx = rax; crate::simd_ops::wu64(P8, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 10; continue; }
            10 => { rcx = rsp.wrapping_add(0x48); rdi = (((rdi) | (0x3u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rax = crate::simd_ops::call_stk(rdx, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 16; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = ((rax as u32)) as u32 as u64; rcx = 0x40; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { rcx = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x50u64))) as u32 as u64; bb = 20; continue; }
            15 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rsi = rbx; bb = 16; continue; }
            16 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 37; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = 0x40; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdx = ((rdx).wrapping_shl(2u32)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 37; continue; } bb = 18; continue; }
            18 => { rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P3), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 37; continue; } bb = 19; continue; }
            19 => { r12 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; bb = 20; continue; }
            20 => { rdi = ((rbx as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 37; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), rbp); rbp = rsi; bb = 22; continue; }
            22 => { rax = 0; fl_l = 0; fl_r = 0; rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = rsp.wrapping_add(0x60); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x1fu32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x1cu32) as u32); rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 36; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x64u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x10de; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 24; continue; }
            24 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4144564e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 25; continue; }
            25 => { rdi = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rbp = ((rbp).wrapping_add(0x14u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 22; continue; } bb = 26; continue; }
            26 => { bb = 36; continue; }
            27 => { rax = ((rdi as u32)) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rdi = (crate::simd_ops::ru32(rsi.wrapping_add(rdx.wrapping_mul(4)))) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 36; continue; } bb = 28; continue; }
            28 => { rax = 0; fl_l = 0; fl_r = 0; rdx = 0x432; rcx = ((rax.wrapping_add(0x40u64)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 29; continue; }
            29 => { crate::simd_ops::wu32(rax, (0x2u32) as u32); rcx = rsp.wrapping_add(0x30); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x30u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x432u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 36; continue; } bb = 30; continue; }
            30 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x21cu64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 36; continue; } bb = 31; continue; }
            31 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; r8 = rbx.wrapping_add(0x220); rax = 0; fl_l = 0; fl_r = 0; rdi = r8; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; rdx = rcx; rcx = r14; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 32; continue; }
            32 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = P14; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rdx = P14; rcx = (!(rcx)) as u64; r8 = rcx.wrapping_sub(0x1); rdi = rcx.wrapping_sub(0x1); rcx = r14; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 33; continue; }
            33 => { rdx = ((r14.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); r13 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = r13; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } r8 = r13; rcx = (!(rcx)) as u64; rdx = rcx; rcx = r14; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); bb = 36; continue; }
            36 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; bb = 37; continue; }
            37 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; bb = 38; continue; }
            38 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rcx = r13; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140004360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1426452c0; const P2: u64 = 0x14397ce50; const P3: u64 = 0x1426c71b0; const P4: u64 = 0x140002970; const P5: u64 = 0x14397ce70; const P6: u64 = 0x1426c7158; const P7: u64 = 0x142645758; const P8: u64 = 0x14397ce80; const P9: u64 = 0x1426c7110; const P10: u64 = 0x140003230; const P11: u64 = 0x142645350; const P12: u64 = 0x1426c7290; const P13: u64 = 0x140003eb0; const P14: u64 = 0x1426456f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); rax = 0; fl_l = 0; fl_r = 0; rdi = ((rcx as u32)) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rax); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x40u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x20u32) as u32); rsi = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(P2)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = P3; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P2, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P5)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rdx = P6; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P5, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(P2)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(P8)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = P9; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P8, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            9 => { r8 = rsp.wrapping_add(0x50); rdx = rsp.wrapping_add(0x48); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r9 = 0; fl_l = 0; fl_r = 0; rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rdx = rsp.wrapping_add(0x50); r8 = ((r9.wrapping_add(0x9u64)) as u32) as u32 as u64; rcx = rdi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rbx as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 11; continue; }
            11 => { rdx = r11; rcx = 0x40; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; r9 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); r8 = ((r9.wrapping_add(0x9u64)) as u32) as u32 as u64; rdx = rsp.wrapping_add(0x50); rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = P12; rdx = rsi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 14; continue; }
            14 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            18 => { rax = rbx; bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140004520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x140003000; const P2: u64 = 0x14183acc2; const P3: u64 = 0x140003fc0; const P4: u64 = 0x1426454d0; const P5: u64 = 0x142645350; const P6: u64 = 0x1400022c0; const P7: u64 = 0x1426452c0; const P8: u64 = 0x14397ce50; const P9: u64 = 0x1426c71b0; const P10: u64 = 0x140002970; const P11: u64 = 0x14397ce78; const P12: u64 = 0x1426c7300; const P13: u64 = 0x142645758; const P14: u64 = 0x14397ce88; const P15: u64 = 0x1426c72e0; const P16: u64 = 0x14397ce80; const P17: u64 = 0x1426c7110; const P18: u64 = 0x140003230; const P19: u64 = 0x1426c6ee8; const P20: u64 = 0x14183acec; const P21: u64 = 0x140002e20; const P22: u64 = 0x1426456f0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 83>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x290u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x280u64), rax); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); rbp = rdi; rbx = rdi; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 1; continue; }
            1 => { rcx = 0x42ea; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = rsp.wrapping_add(0x72); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x208; crate::simd_ops::wu16(rsp.wrapping_add(0x70u64), ((rdi as u16)) as u16); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rdi as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rsp.wrapping_add(0x70u64))) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 7; continue; } bb = 3; continue; }
            3 => { rcx = rsp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P4), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = rsp.wrapping_add(0x70); while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; rdi = rcx.wrapping_sub(0x1); rdx = ((rcx.wrapping_add(rcx)) as u64) as u64; rcx = ((rbx.wrapping_add(0x40u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r8 = rsp.wrapping_add(0x70); rdx = rdi.wrapping_add(0x1); rcx = rax; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = rbx; bb = 36; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x20u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rcx = 0x1770; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = ((rax.wrapping_add(0x78u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; bb = 36; continue; }
            10 => { rax = (crate::simd_ops::ru64(P8)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = P9; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P8, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(P11)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rdx = P12; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P11, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(P8)) as u64; bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru64(P14)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { rdx = P15; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P14, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(P8)) as u64; bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(P16)) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = P17; rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P13), rax), rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(P16, rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 20; continue; }
            20 => { r8 = rsp.wrapping_add(0x50); rdx = rsp.wrapping_add(0x48); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x2a8u64), rsi); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r9 = rsp.wrapping_add(0x44); r8 = P19; rdx = rsp.wrapping_add(0x50); rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 22; continue; }
            22 => { rdx = r11; rcx = 0x40; rdx = ((rdx).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32); r9 = rsp.wrapping_add(0x44); r8 = P19; rdx = rsp.wrapping_add(0x50); rcx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P11), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { rax = rsp.wrapping_add(0x40); r9 = 0; fl_l = 0; fl_r = 0; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = 0; fl_l = 0; fl_r = 0; rcx = rbp; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 25; continue; }
            25 => { rdx = r11; rcx = 0x40; rdx = ((rdx).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x40u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rdi); r9 = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rcx as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rdx = 0x5c; rcx = rbx; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = rax.wrapping_add(0x2); bb = 30; continue; }
            29 => { rdx = rbx; bb = 30; continue; }
            30 => { rdx = ((rdx).wrapping_sub(rbx)) as u64; rcx = rbx; rdx = ((((rdx) as i64).wrapping_shr(1u32) as u64)) as u64; rdx = ((rdx).wrapping_sub(1)) as u64; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 31; continue; }
            31 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P22), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P22), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 33; continue; } bb = 32; continue; }
            32 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P16), rax), rax, rcx, rdx, r8, r9, rsp); bb = 33; continue; }
            33 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x2a8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); bb = 35; continue; }
            35 => { rax = rdi; bb = 36; continue; }
            36 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x280u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x290); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140004890(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143822fc0; const P1: u64 = 0x1426452c0; const P2: u64 = 0x140001dd0; const P3: u64 = 0x140003000; const P4: u64 = 0x14183acc2; const P5: u64 = 0x140001e90; const P6: u64 = 0x140003fc0; const P7: u64 = 0x140003eb0; const P8: u64 = 0x1426454d0; const P9: u64 = 0x1426456f0; const P10: u64 = 0x140004360; const P11: u64 = 0x140003420; const P12: u64 = 0x1426c6f00; const P13: u64 = 0x14183ad3a; const P14: u64 = 0x142645708; const P15: u64 = 0x140003740; const P16: u64 = 0x1426c7510; const P17: u64 = 0x1426c74e0; const P18: u64 = 0x1426c74b8; const P19: u64 = 0x1426c7480; const P20: u64 = 0x1426c7458; const P21: u64 = 0x1426c7430; const P22: u64 = 0x1426c7400; const P23: u64 = 0x1400039f0; const P24: u64 = 0x1426c73c0; const P25: u64 = 0x1426c7380; const P26: u64 = 0x140003c00; const P27: u64 = 0x1426c7350; const P28: u64 = 0x1426c7320;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 83>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = ((rsp).wrapping_sub(0x280u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rax = (((rax) ^ (rsp))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x270u64), rax); r12 = 0; fl_l = 0; fl_r = 0; rdi = rcx; rcx = 0; fl_l = 0; fl_r = 0; rbp = r12; rbx = r12; rsi = r12; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 32; continue; } bb = 2; continue; }
            2 => { rcx = 0x42ea; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = rsp.wrapping_add(0x62); rdx = 0; fl_l = 0; fl_r = 0; r8 = 0x208; crate::simd_ops::wu16(rsp.wrapping_add(0x60u64), ((r12 as u16)) as u16); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru16(rsp.wrapping_add(0x60u64))) as u16 as u64; fl_r = ((rbx as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rsp.wrapping_add(0x60); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rsp.wrapping_add(0x60); rdx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rsi = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x50u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 7; continue; }
            7 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rsi = r12; bb = 8; continue; }
            8 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 9; continue; }
            9 => { rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 10; continue; }
            10 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = rbx; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } r8 = ((rax.wrapping_add(0x2eu64)) as u32) as u32 as u64; rcx = (!(rcx)) as u64; rdi = ((rbx.wrapping_add(rcx.wrapping_mul(2)).wrapping_sub(0x2u64)) as u64) as u64; rcx = rdi; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { rdx = 0x5c; bb = 12; continue; }
            12 => { rax = ((crate::simd_ops::ru16(rcx)) as u32) as u32 as u64; fl_l = ((rdx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 15; continue; } bb = 13; continue; }
            13 => { fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = ((rcx).wrapping_sub(0x2u64)) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { fl_l = ((r8 as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rcx)) as u16 as u64; rdx = P12; rdi = (if (((fl_l as u16)) == ((fl_r as u16))) { rcx } else { rdi }) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 18; continue; } bb = 16; continue; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 18; continue; }
            18 => { rdx = P16; rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 24; continue; } bb = 19; continue; }
            19 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 20; continue; }
            20 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rax = P17; r9 = rsp.wrapping_add(0x20); r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = P18; rdx = (0xffffffff80000002u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = P19; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = P20; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = P21; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = P22; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 22; continue; }
            22 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 25; continue; } bb = 23; continue; }
            23 => { rax = P24; r8 = rsp.wrapping_add(0x20); rdx = (0xffffffff80000002u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = P25; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r12); rax = crate::simd_ops::call_stk(P26, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 24; continue; }
            24 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = P27; r9 = rsp.wrapping_add(0x40); r8 = rbp; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = P28; rdx = (0xffffffff80000002u64) as u64; rcx = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 27; continue; }
            27 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = rsi; bb = 33; continue; }
            29 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = ((rax.wrapping_add(0x7eu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { rax = rsi; bb = 33; continue; }
            32 => { rcx = 0xa0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P1), rax), rax, rcx, rdx, r8, r9, rsp); rax = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x270u64))) as u64; fl_l = (((rcx) ^ (rsp))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; r11 = rsp.wrapping_add(0x280); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140004b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452c0; const P1: u64 = 0x140001dd0; const P2: u64 = 0x140004890; const P3: u64 = 0x140001f80; const P4: u64 = 0x1426c7520; const P5: u64 = 0x1426c71f8; const P6: u64 = 0x14183ad34; const P7: u64 = 0x140003eb0; const P8: u64 = 0x140001e30; const P9: u64 = 0x1426456f0; const P10: u64 = 0x142645708; const P11: u64 = 0x140003420; const P12: u64 = 0x140003000; const P13: u64 = 0x140004520; const P14: u64 = 0x1426454d0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rcx = 0; fl_l = 0; fl_r = 0; rsi = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 35; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 35; continue; } bb = 2; continue; }
            2 => { rcx = rdi; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbp); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r12 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 4; continue; }
            4 => { rcx = rbp; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } bb = 5; continue; }
            5 => { rax = P4; rbx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = rdx; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rcx = (!(rcx)) as u64; r8 = rcx.wrapping_sub(0x1); rdi = rcx.wrapping_sub(0x1); rcx = rbp; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { rax = rsp.wrapping_add(0x30); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { bb = 10; continue; }
            9 => { rdx = ((rbp.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = r12; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 10; continue; }
            10 => { rax = rsp.wrapping_add(0x30); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = rbp; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 14; continue; } bb = 12; continue; }
            12 => { rsi = rbp; rbp = 0; fl_l = 0; fl_r = 0; bb = 14; continue; }
            13 => { rdx = rbp; rcx = r12; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsi = rax; bb = 14; continue; }
            14 => { rcx = r12; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rcx = rbp; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 30; continue; }
            16 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7e; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 32; continue; } bb = 17; continue; }
            17 => { rcx = rdi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 18; continue; }
            18 => { rcx = 0x37e4; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 20; continue; } bb = 19; continue; }
            19 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 21; continue; }
            20 => { rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdi = rax; bb = 21; continue; }
            21 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 22; continue; }
            22 => { rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rsi = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 28; continue; } bb = 23; continue; }
            23 => { rcx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P14), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x50u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 25; continue; }
            25 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            27 => { rcx = rsi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); rsi = 0; fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { rcx = rdi; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 29; continue; }
            29 => { rcx = rbx; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P9), rax), rax, rcx, rdx, r8, r9, rsp); bb = 30; continue; }
            30 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            32 => { rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P10), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = ((rax.wrapping_add(0x7eu64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); bb = 34; continue; }
            34 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            35 => { rcx = 0xa0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140004dc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426452c0; const P1: u64 = 0x140003000; const P2: u64 = 0x1426c7534; const P3: u64 = 0x14183ad34; const P4: u64 = 0x140004b90; const P5: u64 = 0x142645708; const P6: u64 = 0x140003e10; const P7: u64 = 0x1426456f0; const P8: u64 = 0x140002970;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rcx = 0; fl_l = 0; fl_r = 0; rbp = ((rdx as u32)) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = rsi; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } r8 = ((rbx as u32)) as u32 as u64; rcx = (!(rcx)) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rdx = rsi; bb = 3; continue; }
            3 => { rax = ((crate::simd_ops::ru16(rdx)) as u32) as u32 as u64; fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x5cu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u16)) as u16 as u64; fl_r = (0x2fu16) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 11; continue; } bb = 5; continue; }
            5 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; rdx = ((rdx).wrapping_add(0x2u64)) as u64; rax = ((r8 as u32)) as u32 as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 6; continue; }
            6 => { rcx = 0x37e4; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 7; continue; }
            7 => { fl_l = ((((rbp as u32)) >> (11u32)) & 1) as u64; if ((fl_l as u32) != 0) { bb = 14; continue; } bb = 8; continue; }
            8 => { rcx = (((rcx) | (0xffffffffffffffffu64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rdi = P2; while rcx != 0 { let v = crate::simd_ops::ru16(rdi); rdi = rdi.wrapping_add(2); rcx = rcx.wrapping_sub(1); if v == (rax as u16) { break; } } rdx = P2; rcx = (!(rcx)) as u64; r8 = rcx.wrapping_sub(0x1); rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } bb = 9; continue; }
            9 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P5), rax), rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rdi = rax; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P7), rax), rax, rcx, rdx, r8, r9, rsp); rax = rdi; bb = 16; continue; }
            11 => { rcx = 0xa0; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P0), rax), rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            12 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7e; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            14 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 15; continue; }
            15 => { rax = rbx; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140005540(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14397d77c; const P1: u64 = 0x1426cfa3d; const P2: u64 = 0x1426cfab6; const P3: u64 = 0x14397d7e0; const P4: u64 = 0x143a00288; const P5: u64 = 0x14397d7c0; const P6: u64 = 0x143c8ce68; const P7: u64 = 0x14397d7c8; const P8: u64 = 0x1401de410; const P9: u64 = 0x14397d7f0; const P10: u64 = 0x14397d7d0; const P11: u64 = 0x14397d7e8; const P12: u64 = 0x141834a78; const P13: u64 = 0x141834a0c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = ((crate::exe::tls_pointer() as u64)) as u64; rcx = 0x74; rax = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rax))) as u32 as u64; fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) > (((fl_r as u32)) as i32)) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1bu64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = P2; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(P3, rax); rax = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu64(P5, rax); rax = P6; crate::simd_ops::wu64(P7, rax); rax = P8; crate::simd_ops::wu64(P9, rax); rax = P5; crate::simd_ops::wu64(P4, rax); crate::simd_ops::wu128(P10, (xmm0) as u128); crate::simd_ops::wu64(P11, (0x4u64) as u64); crate::simd_ops::wu32(P6, (0xffffffffu32) as u32); rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(P0)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = P10; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); rcx = P0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 1; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14001d520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c48330; const P1: u64 = 0x141834d64; const P2: u64 = 0x8; const P3: u64 = 0x143c48338; const P4: u64 = 0x143c48348; const P5: u64 = 0x143c48350; const P6: u64 = 0x143c48360; const P7: u64 = 0x143c48368; const P8: u64 = 0x1434268d0; const P9: u64 = 0x140203960; const P10: u64 = 0x1425d5ba0; const P11: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14001d520, funcinfo_rva: 0x35a34ec, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x28, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = ((P0) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64);
    rcx = (0x48u32) as u32 as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x14001d535); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    crate::simd_ops::wu64(rax, (rax) as u64);
    crate::simd_ops::wu64(rax.wrapping_add(P2), (rax) as u64);
    crate::simd_ops::wu64(P3, (rax) as u64);
    crate::simd_ops::wu64(P4, (0x0u64) as u64);
    xmm0 = 0u128;
    crate::simd_ops::wu128(P5, (xmm0) as u128);
    crate::simd_ops::wu64(P6, (0x7u64) as u64);
    crate::simd_ops::wu64(P7, (0x8u64) as u64);
    xmm0 = ((crate::simd_ops::ru32(P8)) as u32) as u128;
    crate::simd_ops::wu32(P0, ((xmm0 as u32)) as u32);
    r8 = (rax) as u64;
    rcx = ((P4) as u64) as u64;
    let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64);
    eh.site(0x14001d58f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0;
    rcx = ((P10) as u64) as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140020580(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430853a8; const P1: u64 = 0x142409b50; const P2: u64 = 0x1430853c0; const P3: u64 = 0x1430853d8; const P4: u64 = 0x1430853f0; const P5: u64 = 0x143085408; const P6: u64 = 0x143085458; const P7: u64 = 0x143085470; const P8: u64 = 0x143085488; const P9: u64 = 0x1430854a0; const P10: u64 = 0x1430854b8; const P11: u64 = 0x1430854d0; const P12: u64 = 0x1430854e8; const P13: u64 = 0x143a0ff20; const P14: u64 = 0x143a0ff28; const P15: u64 = 0x14021b730; const P16: u64 = 0x14021e590; const P17: u64 = 0x14240aa00; const P18: u64 = 0x140208500; const P19: u64 = 0x1401de3c0; const P20: u64 = 0x141834ee8; const P21: u64 = 0x1425d6920; const P22: u64 = 0x141834d18; const P23: u64 = 0x140200130;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 231>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x610u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x710u64)) as u64; rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x63u64), (0x10u8) as u8); r12 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((r12 as u8)) as u8); rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x11u64) as u64); r9 = ((r12.wrapping_add(0x11u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0x1u32) as u32); rax = ((rbp.wrapping_sub(0x14u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x14u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x14u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x11u64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0xcu64), ((r12 as u8)) as u8); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x11u64) as u64); r9 = ((r12.wrapping_add(0x11u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x14u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x74u64), (0x2u32) as u32); rax = ((rbp.wrapping_add(0x78u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x78u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x7bu64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x7cu64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x80u64), ((r12 as u8)) as u8); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x13u64) as u64); r9 = ((r12.wrapping_add(0x13u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x78u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x100u64), (0x2u32) as u32); rax = ((rbp.wrapping_add(0x104u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x104u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x104u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x107u64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x108u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x10cu64), ((r12 as u8)) as u8); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x13u64) as u64); r9 = ((r12.wrapping_add(0x13u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x104u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x18cu64), (0x3u32) as u32); rax = ((rbp.wrapping_add(0x190u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x190u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x190u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x193u64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x194u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x198u64), ((r12 as u8)) as u8); rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x13u64) as u64); r9 = ((r12.wrapping_add(0x13u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x190u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x218u64), (0x2u32) as u32); rax = ((rbp.wrapping_add(0x21cu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x21cu64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x21cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x21fu64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x220u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x224u64), ((r12 as u8)) as u8); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x13u64) as u64); r9 = ((r12.wrapping_add(0x13u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x21cu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x2a4u64), (0x3u32) as u32); rax = ((rbp.wrapping_add(0x2a8u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x2a8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x2a8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x2abu64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x2acu64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x2b0u64), ((r12 as u8)) as u8); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); r9 = ((r12.wrapping_add(0x10u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x2a8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x330u64), (0x1u32) as u32); rax = ((rbp.wrapping_add(0x334u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x334u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x334u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x337u64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x338u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x33cu64), ((r12 as u8)) as u8); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); r9 = ((r12.wrapping_add(0x10u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x334u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x3bcu64), (0x2u32) as u32); rax = ((rbp.wrapping_add(0x3c0u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x3c0u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x3c0u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x3c3u64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x3c4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x3c8u64), ((r12 as u8)) as u8); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); r9 = ((r12.wrapping_add(0x10u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x3c0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x448u64), (0x3u32) as u32); rax = ((rbp.wrapping_add(0x44cu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x44cu64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x44cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x44fu64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x450u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x454u64), ((r12 as u8)) as u8); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x17u64) as u64); r9 = ((r12.wrapping_add(0x17u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x44cu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x4d4u64), (0x1u32) as u32); rax = ((rbp.wrapping_add(0x4d8u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4d8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x4d8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x4dbu64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x4dcu64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x4e0u64), ((r12 as u8)) as u8); rax = ((P11) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x17u64) as u64); r9 = ((r12.wrapping_add(0x17u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x4d8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x560u64), (0x2u32) as u32); rax = ((rbp.wrapping_add(0x564u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x564u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x564u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x567u64), (0x10u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x568u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x56cu64), ((r12 as u8)) as u8); rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x17u64) as u64); r9 = ((r12.wrapping_add(0x17u64)) as u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x564u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x5ecu64), (0x3u32) as u32); crate::simd_ops::wu64(P13, (r12) as u64); crate::simd_ops::wu64(P14, (r12) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r15 = (rax) as u64; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(P13, (rax) as u64); rbx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdi = (0x186186186186186u64) as u64; r13 = ((P13) as u64) as u64; bb = 1; continue; }
            1 => { r9 = (rbx) as u64; r8 = (r15) as u64; rdx = ((rbp.wrapping_add(0x5f0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(P14)) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(P13)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r13) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r12) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rdi = ((rax.wrapping_add(0x1cu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rdi) as u64); crate::simd_ops::lk_and_u32(rdi, (0xff000010u32) as u32); crate::simd_ops::lk_or_u32(rdi, (0x10u32) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x3u64), (0x10u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), ((r12 as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(0x8u64), (0x0u8) as u8); rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rax as u32)) as u32); crate::simd_ops::wu64(r14, (rsi) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x8u64), (rsi) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu16(r14.wrapping_add(0x18u64), (0x0u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r12) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r8 = (r14) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (r13) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (0x186186186186186u64) as u64; bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x8cu64)) as u64; rax = ((rbp.wrapping_add(0x5f0u64)) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 5; continue; }
            5 => { r9 = ((P19) as u64) as u64; rdx = (0x8cu32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x80u64)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P21) as u64) as u64; r11 = ((rsp.wrapping_add(0x710u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            6 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140025f60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x14308de2c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143c8f058; const P4: u64 = 0x143844040; const P5: u64 = 0x143c8f060; const P6: u64 = 0x1402344d0; const P7: u64 = 0x143c8f088; const P8: u64 = 0x143c8f068; const P9: u64 = 0x143c8f078;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x6dfu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140026770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x14308fa3c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143c8eb38; const P4: u64 = 0x143843fc8; const P5: u64 = 0x143c8eb40; const P6: u64 = 0x140234f10; const P7: u64 = 0x143c8eb68; const P8: u64 = 0x143c8eb48; const P9: u64 = 0x143c8eb58;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x169u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140034640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cf9bd; const P1: u64 = 0x14309dcfd; const P2: u64 = 0x143a00288; const P3: u64 = 0x143c906e8; const P4: u64 = 0x1438447c4; const P5: u64 = 0x143c906f0; const P6: u64 = 0x1401de410; const P7: u64 = 0x143c90718; const P8: u64 = 0x143c906f8; const P9: u64 = 0x143c90708;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x18u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x39u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400346d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cf9bd; const P1: u64 = 0x14309dd9d; const P2: u64 = 0x143a00288; const P3: u64 = 0x143c904a8; const P4: u64 = 0x14384484c; const P5: u64 = 0x143c904b0; const P6: u64 = 0x1401de410; const P7: u64 = 0x143c904d8; const P8: u64 = 0x143c904b8; const P9: u64 = 0x143c904c8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x18u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x3eu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140034b20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cfa3d; const P1: u64 = 0x14309dcfd; const P2: u64 = 0x143a00288; const P3: u64 = 0x143c908d8; const P4: u64 = 0x143844848; const P5: u64 = 0x143c908e0; const P6: u64 = 0x1401de410; const P7: u64 = 0x143c90908; const P8: u64 = 0x143c908e8; const P9: u64 = 0x143c908f8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1bu64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x39u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140034bb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cfa3d; const P1: u64 = 0x14309dd9d; const P2: u64 = 0x143a00288; const P3: u64 = 0x143c90760; const P4: u64 = 0x1438447c0; const P5: u64 = 0x143c90768; const P6: u64 = 0x1401de410; const P7: u64 = 0x143c90790; const P8: u64 = 0x143c90770; const P9: u64 = 0x143c90780;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1bu64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x3eu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14003f630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14309feb0; const P1: u64 = 0x14309fea8; const P2: u64 = 0x14309feb7; const P3: u64 = 0x14309feaf; const P4: u64 = 0x143a15218;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rcx = (crate::simd_ops::ru64(P0)) as u64; rbx = (0x9ae16a3b2f90407du64) as u64; r8 = (crate::simd_ops::ru64(P1)) as u64; rax = (0xb492b66fbe98f273u64) as u64; rdx = (crate::simd_ops::ru64(P2)) as u64; r11 = (0x9ae16a3b2f90404fu64) as u64; r8 = (((r8) as i64).wrapping_mul((rax) as i64) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(P3)) as u64; r9 = ((rcx.wrapping_add(r11)) as u64) as u64; r10 = ((rcx.wrapping_add(r8)) as u64) as u64; r9 = ((r9).rotate_right(18u32)) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; r10 = ((r10).rotate_right(43u32)) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; rdx = ((rdx).rotate_right(30u32)) as u64; r10 = ((r10).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r10 = (((r10) ^ (r9))) as u64; r10 = (((r10) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rcx = r10; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r9))) as u64; rcx = (((rcx) ^ (r10))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(P4, rax); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140042790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430a0100; const P1: u64 = 0x142409b50; const P2: u64 = 0x143a16750; const P3: u64 = 0x143a16753; const P4: u64 = 0x143a16754; const P5: u64 = 0x143a16758; const P6: u64 = 0x1401df680; const P7: u64 = 0x14183acc8; const P8: u64 = 0x142408f50; const P9: u64 = 0x14183acbc; const P10: u64 = 0x143a16778; const P11: u64 = 0x143a16780; const P12: u64 = 0x140267b90; const P13: u64 = 0x14240ab10; const P14: u64 = 0x1425da840; const P15: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbp as u8)) as u8); rax = P0; crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), (0x1eu64) as u64); r9 = ((rbp.wrapping_add(0x1eu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = r11.wrapping_sub(0x58); rcx = r11.wrapping_sub(0x48); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r14); rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); r9 = (r9 & !0xffu64) | (((0x4u8) as u8 as u64)); crate::simd_ops::wu8(P3, ((r9 as u8)) as u8); crate::simd_ops::wu32(P4, ((rbp as u32)) as u32); crate::simd_ops::wu8(P5, ((rbp as u8)) as u8); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rsi = rsp.wrapping_add(0x38); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rsi }) as u64; rbx = P5; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(P4, ((rbp as u32)) as u32); rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; rbx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P5) } else { rbx }) as u64; crate::simd_ops::wu8(rbx, ((rdi as u8)) as u8); bb = 21; continue; }
            5 => { rcx = (((r9 as u8)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; r10 = ((rax.wrapping_mul(8)) as u32) as u32 as u64; rax = rbx; r11 = (crate::simd_ops::ru64(P5)) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x20; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { r11 } else { rax }) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { r8 = rbx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r8 = (if (((fl_l as u32)) > ((fl_r as u32))) { r11 } else { r8 }) as u64; rax = ((rdi.wrapping_add(rsi)) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = r14; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rax; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            11 => { rax = ((r10 as u32)) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = rdi; rcx = r14; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru64(P5)) as u64; r9 = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; bb = 13; continue; }
            13 => { rax = (((r9 as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 16; continue; }
            15 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = rbx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r11 } else { rcx }) as u64; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 20; continue; }
            19 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P5) } else { rbx }) as u64; crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); crate::simd_ops::wu32(P4, ((rdi as u32)) as u32); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(P10, rbp); crate::simd_ops::wu64(P11, rbp); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rax); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(P10, rax); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = P14; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P15, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400429f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430a00d8; const P1: u64 = 0x142409b50; const P2: u64 = 0x143a16448; const P3: u64 = 0x143a1644b; const P4: u64 = 0x143a1644c; const P5: u64 = 0x143a16450; const P6: u64 = 0x1401df680; const P7: u64 = 0x14183acc8; const P8: u64 = 0x142408f50; const P9: u64 = 0x14183acbc; const P10: u64 = 0x143a16470; const P11: u64 = 0x143a16478; const P12: u64 = 0x140267b90; const P13: u64 = 0x14240ab10; const P14: u64 = 0x1425da880; const P15: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rbp = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rbp as u8)) as u8); rax = P0; crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x50u64), (0x20u64) as u64); r9 = ((rbp.wrapping_add(0x20u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = r11.wrapping_sub(0x58); rcx = r11.wrapping_sub(0x48); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); r14 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r14); rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); r9 = (r9 & !0xffu64) | (((0x4u8) as u8 as u64)); crate::simd_ops::wu8(P3, ((r9 as u8)) as u8); crate::simd_ops::wu32(P4, ((rbp as u32)) as u32); crate::simd_ops::wu8(P5, ((rbp as u8)) as u8); rdi = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rsi = rsp.wrapping_add(0x38); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rsi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rsi }) as u64; rbx = P5; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(P4, ((rbp as u32)) as u32); rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; rbx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P5) } else { rbx }) as u64; crate::simd_ops::wu8(rbx, ((rdi as u8)) as u8); bb = 21; continue; }
            5 => { rcx = (((r9 as u8)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; r10 = ((rax.wrapping_mul(8)) as u32) as u32 as u64; rax = rbx; r11 = (crate::simd_ops::ru64(P5)) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = 0x20; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { r11 } else { rax }) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { r8 = rbx; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r8 = (if (((fl_l as u32)) > ((fl_r as u32))) { r11 } else { r8 }) as u64; rax = ((rdi.wrapping_add(rsi)) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = r14; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = rax; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            11 => { rax = ((r10 as u32)) as u32 as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = rdi; rcx = r14; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r11 = (crate::simd_ops::ru64(P5)) as u64; r9 = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru32(P2)) as u32 as u64; bb = 13; continue; }
            13 => { rax = (((r9 as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 16; continue; }
            15 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x7c00; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rdx = ((((rdx as u32)) & (0xffffffu32))) as u32 as u64; rdx = (((rdx as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = rbx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r11 } else { rcx }) as u64; r8 = rdi; rdx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 17; continue; }
            17 => { rcx = ((crate::simd_ops::ru8(P3)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 20; continue; }
            19 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rbx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P5) } else { rbx }) as u64; crate::simd_ops::wu8(rbx.wrapping_add(rdi), (0x0u8) as u8); crate::simd_ops::wu32(P4, ((rdi as u32)) as u32); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(P10, rbp); crate::simd_ops::wu64(P11, rbp); rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rax); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(P10, rax); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); rcx = P14; r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P15, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14004c260(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14342703c; const P1: u64 = 0x14342660c; const P2: u64 = 0x143427080; const P3: u64 = 0x1434268d0; const P4: u64 = 0x143a001ac; const P5: u64 = 0x143845a1c; const P6: u64 = 0x143845a28; const P7: u64 = 0x143845a18; const P8: u64 = 0x143845a24; const P9: u64 = 0x143845a30; const P10: u64 = 0x143845a3c; const P11: u64 = 0x143845a10; const P12: u64 = 0x143845a34; const P13: u64 = 0x143a00290; const P14: u64 = 0x143845a00; const P15: u64 = 0x1425dc910; const P16: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm2 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    xmm4 = ((crate::simd_ops::ru32(P3)) as u32) as u128;
    rcx = (((crate::simd_ops::ru32(P4)) as i32 as i64) as u64) as u64;
    xmm3 = crate::simd_ops::vunpcklps(xmm0, xmm1);
    xmm0 = crate::simd_ops::vunpcklps(xmm2, xmm2);
    crate::simd_ops::wu64(P5, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vunpcklps(xmm4, xmm4);
    crate::simd_ops::wu64(P6, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(P7, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(P8, ((xmm2 as u32)) as u32);
    crate::simd_ops::wu32(P9, ((xmm4 as u32)) as u32);
    crate::simd_ops::wu32(P10, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu64(P11, ((xmm3 as u64)) as u64);
    crate::simd_ops::wu64(P12, ((xmm3 as u64)) as u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1000;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { r8 = P13; rdx = P14; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rdx); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P4, ((rcx as u32)) as u32); }
    rcx = P15;
    return crate::simd_ops::tail_fwd_xmm(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14004c510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143427080; const P2: u64 = 0x143845bc8; const P3: u64 = 0x143845bcb; const P4: u64 = 0x1430a5100; const P5: u64 = 0x142409b50; const P6: u64 = 0x143845c50; const P7: u64 = 0x14248a640; const P8: u64 = 0x1430a510d; const P9: u64 = 0x1413cf570; const P10: u64 = 0x143845bc0; const P11: u64 = 0x8; const P12: u64 = 0x143845c58; const P13: u64 = 0x143845c60; const P14: u64 = 0x143845c64; const P15: u64 = 0x143845c6c; const P16: u64 = 0x143845c70; const P17: u64 = 0x143845c78; const P18: u64 = 0x143845c7c; const P19: u64 = 0x143845c84; const P20: u64 = 0x143845c88; const P21: u64 = 0x143845c90; const P22: u64 = 0x1430a7188; const P23: u64 = 0x1425dce20; const P24: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14004c510, funcinfo_rva: 0x35b38f4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm7 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm8 = 0u128; rcx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); crate::simd_ops::wu8(P3, (0x10u8) as u8); rbx = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xdu64) as u64); r9 = (0xdu32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14004c587); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(P6, (0x2u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14004c598); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P8) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14004c5d2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P10) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P11), (rcx) as u64); xmm2 = crate::simd_ops::vunpcklps(xmm8, xmm8); crate::simd_ops::wu64(P12, ((xmm2 as u64)) as u64); crate::simd_ops::wu32(P13, ((xmm8 as u32)) as u32); crate::simd_ops::wu64(P14, ((xmm2 as u64)) as u64); crate::simd_ops::wu32(P15, ((xmm8 as u32)) as u32); xmm0 = crate::simd_ops::vunpcklps(xmm7, xmm7); crate::simd_ops::wu64(P16, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(P17, ((xmm7 as u32)) as u32); xmm0 = crate::simd_ops::vunpcklps(xmm6, xmm6); crate::simd_ops::wu64(P18, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(P19, ((xmm6 as u32)) as u32); crate::simd_ops::wu64(P20, ((xmm2 as u64)) as u64); crate::simd_ops::wu32(P21, ((xmm8 as u32)) as u32); rax = ((P22) as u64) as u64; crate::simd_ops::wu64(P10, (rax) as u64); rcx = ((P23) as u64) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14004c670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426ed8; const P1: u64 = 0x143427144; const P2: u64 = 0x143845928; const P3: u64 = 0x14384592b; const P4: u64 = 0x1430a50f0; const P5: u64 = 0x142409b50; const P6: u64 = 0x1438459b0; const P7: u64 = 0x14248a640; const P8: u64 = 0x1430a50ff; const P9: u64 = 0x1413cf570; const P10: u64 = 0x143845920; const P11: u64 = 0x8; const P12: u64 = 0x1438459b8; const P13: u64 = 0x1438459c0; const P14: u64 = 0x1438459c4; const P15: u64 = 0x1438459cc; const P16: u64 = 0x1438459d0; const P17: u64 = 0x1438459d8; const P18: u64 = 0x1438459dc; const P19: u64 = 0x1438459e4; const P20: u64 = 0x1438459e8; const P21: u64 = 0x1438459f0; const P22: u64 = 0x1430a7188; const P23: u64 = 0x1425dce40; const P24: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14004c670, funcinfo_rva: 0x35b392c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm8) as u128); xmm6 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm7 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm8 = 0u128; rcx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); crate::simd_ops::wu8(P3, (0x10u8) as u8); rbx = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xfu64) as u64); r9 = (0xfu32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14004c6e7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(P6, (0x2u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14004c6f8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P8) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14004c732); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P10) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P11), (rcx) as u64); xmm2 = crate::simd_ops::vunpcklps(xmm8, xmm8); crate::simd_ops::wu64(P12, ((xmm2 as u64)) as u64); crate::simd_ops::wu32(P13, ((xmm8 as u32)) as u32); crate::simd_ops::wu64(P14, ((xmm2 as u64)) as u64); crate::simd_ops::wu32(P15, ((xmm8 as u32)) as u32); xmm0 = crate::simd_ops::vunpcklps(xmm7, xmm7); crate::simd_ops::wu64(P16, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(P17, ((xmm7 as u32)) as u32); xmm0 = crate::simd_ops::vunpcklps(xmm6, xmm6); crate::simd_ops::wu64(P18, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(P19, ((xmm6 as u32)) as u32); crate::simd_ops::wu64(P20, ((xmm2 as u64)) as u64); crate::simd_ops::wu32(P21, ((xmm8 as u32)) as u32); rax = ((P22) as u64) as u64; crate::simd_ops::wu64(P10, (rax) as u64); rcx = ((P23) as u64) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140051be0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307aafc; const P1: u64 = 0x1430ad81d; const P2: u64 = 0x143a00288; const P3: u64 = 0x143c924d0; const P4: u64 = 0x143845bb8; const P5: u64 = 0x143c924d8; const P6: u64 = 0x140287c00; const P7: u64 = 0x143c92500; const P8: u64 = 0x143c924e0; const P9: u64 = 0x143c924f0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1bu64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x3cu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140051c70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x1430ad8cc; const P2: u64 = 0x143a00288; const P3: u64 = 0x143c92550; const P4: u64 = 0x1438459f8; const P5: u64 = 0x143c92558; const P6: u64 = 0x140287cf0; const P7: u64 = 0x143c92580; const P8: u64 = 0x143c92560; const P9: u64 = 0x143c92570;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x193u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140051d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x1430ad66c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143c92588; const P4: u64 = 0x1438459fc; const P5: u64 = 0x143c92590; const P6: u64 = 0x140287e60; const P7: u64 = 0x143c925b8; const P8: u64 = 0x143c92598; const P9: u64 = 0x143c925a8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x13du64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140055160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a196c8; const P1: u64 = 0x143a196cb; const P2: u64 = 0x1430ae9fc; const P3: u64 = 0x142409b50; const P4: u64 = 0x1425ddfa0; const P5: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rax = ((((rax as u32)) & (0xff000002u32))) as u32 as u64;
    rax = ((((rax as u32)) | (0x2u32))) as u32 as u64;
    crate::simd_ops::wu32(P0, ((rax as u32)) as u32);
    crate::simd_ops::wu8(P1, (0x2u8) as u8);
    rax = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x6u64) as u64);
    r9 = 0x6;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x20);
    rcx = P0;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = P4;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14005dd70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427048; const P1: u64 = 0x143426650; const P2: u64 = 0x14342703c; const P3: u64 = 0x143a001ac; const P4: u64 = 0x14384684c; const P5: u64 = 0x143846858; const P6: u64 = 0x143846848; const P7: u64 = 0x143846854; const P8: u64 = 0x143846860; const P9: u64 = 0x14384686c; const P10: u64 = 0x143846840; const P11: u64 = 0x143846864; const P12: u64 = 0x143a00290; const P13: u64 = 0x143846830; const P14: u64 = 0x1425df530; const P15: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm2 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm1 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    rcx = (((crate::simd_ops::ru32(P3)) as i32 as i64) as u64) as u64;
    xmm0 = 0u128;
    xmm3 = crate::simd_ops::vunpcklps(xmm0, xmm0);
    xmm0 = crate::simd_ops::vunpcklps(xmm2, xmm2);
    crate::simd_ops::wu64(P4, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vunpcklps(xmm4, xmm4);
    crate::simd_ops::wu64(P5, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(P6, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(P7, ((xmm2 as u32)) as u32);
    crate::simd_ops::wu32(P8, ((xmm4 as u32)) as u32);
    crate::simd_ops::wu32(P9, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu64(P10, ((xmm3 as u64)) as u64);
    crate::simd_ops::wu64(P11, ((xmm3 as u64)) as u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1000;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { r8 = P12; rdx = P13; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rdx); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P3, ((rcx as u32)) as u32); }
    rcx = P14;
    return crate::simd_ops::tail_fwd_xmm(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14005de10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426d48; const P1: u64 = 0x143427124; const P2: u64 = 0x143426dd4; const P3: u64 = 0x143a001ac; const P4: u64 = 0x1438467cc; const P5: u64 = 0x1438467d8; const P6: u64 = 0x1438467c8; const P7: u64 = 0x1438467d4; const P8: u64 = 0x1438467e0; const P9: u64 = 0x1438467ec; const P10: u64 = 0x1438467c0; const P11: u64 = 0x1438467e4; const P12: u64 = 0x143a00290; const P13: u64 = 0x1438467b0; const P14: u64 = 0x1425df5c0; const P15: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm4 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    rcx = (((crate::simd_ops::ru32(P3)) as i32 as i64) as u64) as u64;
    xmm0 = crate::simd_ops::vunpcklps(xmm2, xmm2);
    xmm3 = crate::simd_ops::vunpcklps(xmm1, xmm1);
    crate::simd_ops::wu64(P4, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vunpcklps(xmm4, xmm4);
    crate::simd_ops::wu64(P5, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(P6, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(P7, ((xmm2 as u32)) as u32);
    crate::simd_ops::wu32(P8, ((xmm4 as u32)) as u32);
    crate::simd_ops::wu32(P9, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu64(P10, ((xmm3 as u64)) as u64);
    crate::simd_ops::wu64(P11, ((xmm3 as u64)) as u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1000;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { r8 = P12; rdx = P13; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rdx); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P3, ((rcx as u32)) as u32); }
    rcx = P14;
    return crate::simd_ops::tail_fwd_xmm(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14006ce00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430e7e08; const P1: u64 = 0x1430e7e00; const P2: u64 = 0x1430e7e16; const P3: u64 = 0x1430e7e0e; const P4: u64 = 0x143a1b1d8;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rcx = (crate::simd_ops::ru64(P0)) as u64; rbx = (0x9ae16a3b2f90408bu64) as u64; r8 = (crate::simd_ops::ru64(P1)) as u64; rax = (0xb492b66fbe98f273u64) as u64; rdx = (crate::simd_ops::ru64(P2)) as u64; r11 = (0x9ae16a3b2f90404fu64) as u64; r8 = (((r8) as i64).wrapping_mul((rax) as i64) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(P3)) as u64; r9 = ((rcx.wrapping_add(r11)) as u64) as u64; r10 = ((rcx.wrapping_add(r8)) as u64) as u64; r9 = ((r9).rotate_right(18u32)) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; r10 = ((r10).rotate_right(43u32)) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; rdx = ((rdx).rotate_right(30u32)) as u64; r10 = ((r10).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r10 = (((r10) ^ (r9))) as u64; r10 = (((r10) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rcx = r10; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r9))) as u64; rcx = (((rcx) ^ (r10))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(P4, rax); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14006cea0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430e8288; const P1: u64 = 0x1430e8280; const P2: u64 = 0x1430e828e; const P3: u64 = 0x1430e8286; const P4: u64 = 0x143a1b1e8;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rcx = (crate::simd_ops::ru64(P0)) as u64; rbx = (0x9ae16a3b2f90407bu64) as u64; r8 = (crate::simd_ops::ru64(P1)) as u64; rax = (0xb492b66fbe98f273u64) as u64; rdx = (crate::simd_ops::ru64(P2)) as u64; r11 = (0x9ae16a3b2f90404fu64) as u64; r8 = (((r8) as i64).wrapping_mul((rax) as i64) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(P3)) as u64; r9 = ((rcx.wrapping_add(r11)) as u64) as u64; r10 = ((rcx.wrapping_add(r8)) as u64) as u64; r9 = ((r9).rotate_right(18u32)) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; r10 = ((r10).rotate_right(43u32)) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; rdx = ((rdx).rotate_right(30u32)) as u64; r10 = ((r10).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r10 = (((r10) ^ (r9))) as u64; r10 = (((r10) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rcx = r10; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r9))) as u64; rcx = (((rcx) ^ (r10))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(P4, rax); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14006cf40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430e82a0; const P1: u64 = 0x1430e8298; const P2: u64 = 0x1430e82ac; const P3: u64 = 0x1430e82a4; const P4: u64 = 0x143a1b188;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rcx = (crate::simd_ops::ru64(P0)) as u64; rbx = (0x9ae16a3b2f904087u64) as u64; r8 = (crate::simd_ops::ru64(P1)) as u64; rax = (0xb492b66fbe98f273u64) as u64; rdx = (crate::simd_ops::ru64(P2)) as u64; r11 = (0x9ae16a3b2f90404fu64) as u64; r8 = (((r8) as i64).wrapping_mul((rax) as i64) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(P3)) as u64; r9 = ((rcx.wrapping_add(r11)) as u64) as u64; r10 = ((rcx.wrapping_add(r8)) as u64) as u64; r9 = ((r9).rotate_right(18u32)) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; r10 = ((r10).rotate_right(43u32)) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; rdx = ((rdx).rotate_right(30u32)) as u64; r10 = ((r10).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r10 = (((r10) ^ (r9))) as u64; r10 = (((r10) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rcx = r10; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r9))) as u64; rcx = (((rcx) ^ (r10))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(P4, rax); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140072cf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a1bdd0; const P1: u64 = 0x143a1bdd3; const P2: u64 = 0x1430ee0a8; const P3: u64 = 0x142409b50; const P4: u64 = 0x1425e2d10; const P5: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64;
    rax = ((((rax as u32)) | (0x8u32))) as u32 as u64;
    crate::simd_ops::wu32(P0, ((rax as u32)) as u32);
    crate::simd_ops::wu8(P1, (0x8u8) as u8);
    rax = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xau64) as u64);
    r9 = 0xa;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x20);
    rcx = P0;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = P4;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140076b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430f4918; const P1: u64 = 0x1430f48f8; const P2: u64 = 0x1426cdcb0; const P3: u64 = 0x143079878; const P4: u64 = 0x1430f4950; const P5: u64 = 0x1430f4930; const P6: u64 = 0x1401fc690; const P7: u64 = 0x143847ad8; const P8: u64 = 0x143847ae0; const P9: u64 = 0x143847ae4; const P10: u64 = 0x14183acc8; const P11: u64 = 0x143847ae8; const P12: u64 = 0x143847aec; const P13: u64 = 0x143847af0; const P14: u64 = 0x1430f4980; const P15: u64 = 0x143847af8; const P16: u64 = 0x143847b00; const P17: u64 = 0x1430f4968; const P18: u64 = 0x1430f49b8; const P19: u64 = 0x1430f4998; const P20: u64 = 0x1430f49e8; const P21: u64 = 0x143847b08; const P22: u64 = 0x143847b10; const P23: u64 = 0x143847b14; const P24: u64 = 0x143847b18; const P25: u64 = 0x143847b1c; const P26: u64 = 0x143847b20; const P27: u64 = 0x1430f49d0; const P28: u64 = 0x143847b28; const P29: u64 = 0x143847b30; const P30: u64 = 0x1430f4a18; const P31: u64 = 0x1430f4a00; const P32: u64 = 0x1430f4a48; const P33: u64 = 0x1430f4a30; const P34: u64 = 0x1430f4ab8; const P35: u64 = 0x1430f4a80; const P36: u64 = 0x1430f4af0; const P37: u64 = 0x143847b38; const P38: u64 = 0x143847b40; const P39: u64 = 0x143847b44; const P40: u64 = 0x143847b48; const P41: u64 = 0x143847b4c; const P42: u64 = 0x1425e3270; const P43: u64 = 0x141834d18;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), (0xffffffffu32) as u32); rax = P0; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); rbx = P1; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rbx); rdi = P2; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rdi); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r14 as u32)) as u32); rsi = P3; crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rsi); crate::simd_ops::wu8(rbp.wrapping_add(0x17u64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0x3u32) as u32); rax = P4; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); rax = P5; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rdi); crate::simd_ops::wu32(rbp.wrapping_add(0x3fu64), (0x2u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), rsi); crate::simd_ops::wu8(rbp.wrapping_add(0x4fu64), ((r14 as u8)) as u8); rdx = ((r14.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P7, rax); crate::simd_ops::wu32(P8, ((r14 as u32)) as u32); crate::simd_ops::wu32(P9, (0x2u32) as u32); r8 = ((r14.wrapping_add(0x70u64)) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x19); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P8, (0x2u32) as u32); crate::simd_ops::wu32(P11, ((r14 as u32)) as u32); crate::simd_ops::wu8(P12, ((r14 as u8)) as u8); crate::simd_ops::wu32(P13, (0x1u32) as u32); rax = P14; crate::simd_ops::wu64(P15, rax); crate::simd_ops::wu32(P16, (0x1u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((r14 as u32)) as u32); rax = P17; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rdi); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rsi); crate::simd_ops::wu8(rbp.wrapping_add(0x17u64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), ((r14 as u32)) as u32); rax = P18; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); rax = P19; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rax); crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rdi); crate::simd_ops::wu32(rbp.wrapping_add(0x3fu64), (0x1u32) as u32); rax = P20; crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), rax); crate::simd_ops::wu8(rbp.wrapping_add(0x4fu64), ((r14 as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu128(P21, (xmm0) as u128); crate::simd_ops::wu64(P21, r14); crate::simd_ops::wu64(P22, r14); rdx = ((r14.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P21, rax); crate::simd_ops::wu32(P22, ((r14 as u32)) as u32); crate::simd_ops::wu32(P23, (0x2u32) as u32); r8 = ((r14.wrapping_add(0x70u64)) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x19); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P22, (0x2u32) as u32); crate::simd_ops::wu32(P24, ((r14 as u32)) as u32); crate::simd_ops::wu8(P25, ((r14 as u8)) as u8); crate::simd_ops::wu32(P26, (0x2u32) as u32); rax = P27; crate::simd_ops::wu64(P28, rax); crate::simd_ops::wu32(P29, (0x4u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((r14 as u32)) as u32); rax = P30; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); rax = P31; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); rax = P32; crate::simd_ops::wu64(rbp.wrapping_sub(0x1u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((r14 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rsi); crate::simd_ops::wu8(rbp.wrapping_add(0x17u64), ((r14 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), ((r14 as u32)) as u32); rax = P33; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); rax = P34; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rax); rax = P35; crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x3fu64), (0x2u32) as u32); rax = P36; crate::simd_ops::wu64(rbp.wrapping_add(0x47u64), rax); crate::simd_ops::wu8(rbp.wrapping_add(0x4fu64), ((r14 as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu128(P37, (xmm0) as u128); crate::simd_ops::wu64(P37, r14); crate::simd_ops::wu64(P38, r14); rdx = ((r14.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P37, rax); crate::simd_ops::wu32(P38, ((r14 as u32)) as u32); crate::simd_ops::wu32(P39, (0x2u32) as u32); r8 = ((r14.wrapping_add(0x70u64)) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x19); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P38, (0x2u32) as u32); crate::simd_ops::wu32(P40, ((r14 as u32)) as u32); crate::simd_ops::wu8(P41, ((r14 as u8)) as u8); rcx = P42; r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; r14 = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P43, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140076e20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430f4b20; const P1: u64 = 0x1430f4b00; const P2: u64 = 0x1430f4b50; const P3: u64 = 0x143079878; const P4: u64 = 0x1430f4b38; const P5: u64 = 0x1430f45a8; const P6: u64 = 0x1430f4578; const P7: u64 = 0x1430f45e0; const P8: u64 = 0x1430f45c8; const P9: u64 = 0x1430f4628; const P10: u64 = 0x1430f45f8; const P11: u64 = 0x1401fc690; const P12: u64 = 0x143847978; const P13: u64 = 0x143847980; const P14: u64 = 0x143847984; const P15: u64 = 0x14183acc8; const P16: u64 = 0x143847988; const P17: u64 = 0x14384798c; const P18: u64 = 0x143847990; const P19: u64 = 0x1430f4660; const P20: u64 = 0x143847998; const P21: u64 = 0x1438479a0; const P22: u64 = 0x1430f4648; const P23: u64 = 0x1430f4a00; const P24: u64 = 0x1430f4690; const P25: u64 = 0x1430f4678; const P26: u64 = 0x1430f46f0; const P27: u64 = 0x1430f46c0; const P28: u64 = 0x1430f4730; const P29: u64 = 0x1438479a8; const P30: u64 = 0x1438479b0; const P31: u64 = 0x1438479b4; const P32: u64 = 0x1438479b8; const P33: u64 = 0x1438479bc; const P34: u64 = 0x1438479c0; const P35: u64 = 0x1430f4708; const P36: u64 = 0x1438479c8; const P37: u64 = 0x1438479d0; const P38: u64 = 0x1430f4778; const P39: u64 = 0x1430f4740; const P40: u64 = 0x1430f47c8; const P41: u64 = 0x1430f4ab8; const P42: u64 = 0x1430f4790; const P43: u64 = 0x1430f4af0; const P44: u64 = 0x1438479d8; const P45: u64 = 0x1438479e0; const P46: u64 = 0x1438479e4; const P47: u64 = 0x1438479e8; const P48: u64 = 0x1438479ec; const P49: u64 = 0x1425e32a0; const P50: u64 = 0x141834d18;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), (0xffffffffu32) as u32); rax = P0; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rax); rax = P1; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); rax = P2; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rsi as u32)) as u32); rdi = P3; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rdi); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), ((rsi as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rsi as u32)) as u32); rax = P4; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rax = P5; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0x2u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); crate::simd_ops::wu8(rbp.wrapping_add(0xfu64), ((rsi as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x17u64), (0x7u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), rax); rax = P9; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); rax = P10; crate::simd_ops::wu64(rbp.wrapping_add(0x2fu64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x37u64), (0x2u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x3fu64), rdi); crate::simd_ops::wu8(rbp.wrapping_add(0x47u64), ((rsi as u8)) as u8); rdx = ((rsi.wrapping_add(0x3u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P12, rax); crate::simd_ops::wu32(P13, ((rsi as u32)) as u32); crate::simd_ops::wu32(P14, (0x3u32) as u32); r8 = 0xa8; rdx = rbp.wrapping_sub(0x59); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P13, (0x3u32) as u32); crate::simd_ops::wu32(P16, ((rsi as u32)) as u32); crate::simd_ops::wu8(P17, ((rsi as u8)) as u8); crate::simd_ops::wu32(P18, (0x4u32) as u32); rax = P19; crate::simd_ops::wu64(P20, rax); crate::simd_ops::wu32(P21, (0x3u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rsi as u32)) as u32); rax = P22; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rax); rbx = P23; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rbx); rax = P24; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rdi); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), ((rsi as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rsi as u32)) as u32); rax = P25; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rax = P26; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); rax = P27; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0x1u32) as u32); rax = P28; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); crate::simd_ops::wu8(rbp.wrapping_add(0xfu64), ((rsi as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu128(P29, (xmm0) as u128); crate::simd_ops::wu64(P29, rsi); crate::simd_ops::wu64(P30, rsi); rdx = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P29, rax); crate::simd_ops::wu32(P30, ((rsi as u32)) as u32); crate::simd_ops::wu32(P31, (0x2u32) as u32); r8 = ((rsi.wrapping_add(0x70u64)) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x59); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P30, (0x2u32) as u32); crate::simd_ops::wu32(P32, ((rsi as u32)) as u32); crate::simd_ops::wu8(P33, ((rsi as u8)) as u8); crate::simd_ops::wu32(P34, (0x5u32) as u32); rax = P35; crate::simd_ops::wu64(P36, rax); crate::simd_ops::wu32(P37, (0x4u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x59u64), ((rsi as u32)) as u32); rax = P38; crate::simd_ops::wu64(rbp.wrapping_sub(0x51u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rbx); rax = P39; crate::simd_ops::wu64(rbp.wrapping_sub(0x41u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x39u64), ((rsi as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), rdi); crate::simd_ops::wu8(rbp.wrapping_sub(0x29u64), ((rsi as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), ((rsi as u32)) as u32); rax = P40; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); rax = P41; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); rax = P42; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0x2u32) as u32); rax = P43; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); crate::simd_ops::wu8(rbp.wrapping_add(0xfu64), ((rsi as u8)) as u8); xmm0 = 0u128; crate::simd_ops::wu128(P44, (xmm0) as u128); crate::simd_ops::wu64(P44, rsi); crate::simd_ops::wu64(P45, rsi); rdx = ((rsi.wrapping_add(0x2u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(P44, rax); crate::simd_ops::wu32(P45, ((rsi as u32)) as u32); crate::simd_ops::wu32(P46, (0x2u32) as u32); r8 = ((rsi.wrapping_add(0x70u64)) as u32) as u32 as u64; rdx = rbp.wrapping_sub(0x59); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(P45, (0x2u32) as u32); crate::simd_ops::wu32(P47, ((rsi as u32)) as u32); crate::simd_ops::wu8(P48, ((rsi as u8)) as u8); rcx = P49; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P50, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14008de70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143106ea8; const P1: u64 = 0x143107088; const P2: u64 = 0x143107060; const P3: u64 = 0x143107040; const P4: u64 = 0x143107020; const P5: u64 = 0x143107000; const P6: u64 = 0x143106fe8; const P7: u64 = 0x143106fd0; const P8: u64 = 0x143106fb8; const P9: u64 = 0x143107150; const P10: u64 = 0x143107130; const P11: u64 = 0x143107110; const P12: u64 = 0x1431070f8; const P13: u64 = 0x1431070e0; const P14: u64 = 0x1431070c8; const P15: u64 = 0x1431070b8; const P16: u64 = 0x1431070a8; const P17: u64 = 0x143107708; const P18: u64 = 0x1431076f0; const P19: u64 = 0x1431076d8; const P20: u64 = 0x1431076c8; const P21: u64 = 0x1431076b8; const P22: u64 = 0x1431076a0; const P23: u64 = 0x143107690; const P24: u64 = 0x143107680; const P25: u64 = 0x1431077d8; const P26: u64 = 0x1431077b8; const P27: u64 = 0x1431077a0; const P28: u64 = 0x143107788; const P29: u64 = 0x143107770; const P30: u64 = 0x143107760; const P31: u64 = 0x143107740; const P32: u64 = 0x143107720; const P33: u64 = 0x1431078a0; const P34: u64 = 0x143107888; const P35: u64 = 0x143107878; const P36: u64 = 0x143107858; const P37: u64 = 0x143107838; const P38: u64 = 0x143107820; const P39: u64 = 0x143107808; const P40: u64 = 0x1431077f0; const P41: u64 = 0x143107970; const P42: u64 = 0x143107958; const P43: u64 = 0x143107940; const P44: u64 = 0x143107928; const P45: u64 = 0x143107908; const P46: u64 = 0x1431078f0; const P47: u64 = 0x1431078d8; const P48: u64 = 0x1431078c0; const P49: u64 = 0x143107378; const P50: u64 = 0x143107350; const P51: u64 = 0x143107338; const P52: u64 = 0x143107320; const P53: u64 = 0x143107308; const P54: u64 = 0x1431072f0; const P55: u64 = 0x1431072d0; const P56: u64 = 0x1431072b0; const P57: u64 = 0x143107480; const P58: u64 = 0x143107460; const P59: u64 = 0x143107438; const P60: u64 = 0x143107418; const P61: u64 = 0x1431073f8; const P62: u64 = 0x1431073d8; const P63: u64 = 0x1431073b8; const P64: u64 = 0x143107398; const P65: u64 = 0x143107588; const P66: u64 = 0x143107568; const P67: u64 = 0x143107540; const P68: u64 = 0x143107520; const P69: u64 = 0x143107500; const P70: u64 = 0x1431074e0; const P71: u64 = 0x1431074c0; const P72: u64 = 0x1431074a0; const P73: u64 = 0x143107668; const P74: u64 = 0x143107650; const P75: u64 = 0x143107638; const P76: u64 = 0x143107620; const P77: u64 = 0x143107608; const P78: u64 = 0x1431075e8; const P79: u64 = 0x1431075c8; const P80: u64 = 0x1431075a8; const P81: u64 = 0x143107d90; const P82: u64 = 0x143107d70; const P83: u64 = 0x143107d50; const P84: u64 = 0x143107d30; const P85: u64 = 0x143107d08; const P86: u64 = 0x143107ce0; const P87: u64 = 0x143107cb8; const P88: u64 = 0x143107ca0; const P89: u64 = 0x143107e68; const P90: u64 = 0x143107e50; const P91: u64 = 0x143a21c9c; const P92: u64 = 0x143a21c98; const P93: u64 = 0x1401dec00; const P94: u64 = 0x143a21c90; const P95: u64 = 0x14183acc8; const P96: u64 = 0x1425371a0; const P97: u64 = 0x142533200; const P98: u64 = 0x143a21cac; const P99: u64 = 0x143a21ca8; const P100: u64 = 0x1401decf0; const P101: u64 = 0x143a21ca0; const P102: u64 = 0x143a21cc4; const P103: u64 = 0x1401de8d0; const P104: u64 = 0x143a082e0; const P105: u64 = 0x143a21cc8; const P106: u64 = 0x1425e6330; const P107: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 189>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x4d0); rsp = ((rsp).wrapping_sub(0x5d0u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x3bu32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x3cu32) as u32); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x3du32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x3eu32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x3fu32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x40u32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x41u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x42u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x43u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x44u32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0x45u32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0x46u32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0x47u32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0x48u32) as u32); rax = P14; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0x49u32) as u32); rax = P15; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), (0x4au32) as u32); rax = P16; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), (0x4bu32) as u32); rax = P17; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0x4cu32) as u32); rax = P18; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), (0x4du32) as u32); rax = P19; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), (0x4eu32) as u32); rax = P20; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0x4fu32) as u32); rax = P21; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), (0x50u32) as u32); rax = P22; crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), (0x51u32) as u32); rax = P23; crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x98u64), (0x5au32) as u32); rax = P24; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xa8u64), (0x5du32) as u32); rax = P25; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xb8u64), (0x52u32) as u32); rax = P26; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), (0x53u32) as u32); rax = P27; crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xd8u64), (0x54u32) as u32); rax = P28; crate::simd_ops::wu64(rbp.wrapping_add(0xe0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xe8u64), (0x55u32) as u32); rax = P29; crate::simd_ops::wu64(rbp.wrapping_add(0xf0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xf8u64), (0x56u32) as u32); rax = P30; crate::simd_ops::wu64(rbp.wrapping_add(0x100u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x108u64), (0x57u32) as u32); rax = P31; crate::simd_ops::wu64(rbp.wrapping_add(0x110u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x118u64), (0x58u32) as u32); rax = P32; crate::simd_ops::wu64(rbp.wrapping_add(0x120u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x128u64), (0x59u32) as u32); rax = P33; crate::simd_ops::wu64(rbp.wrapping_add(0x130u64), rax); r15 = 0x5b; crate::simd_ops::wu32(rbp.wrapping_add(0x138u64), ((r15 as u32)) as u32); rax = P34; crate::simd_ops::wu64(rbp.wrapping_add(0x140u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x148u64), (0x5cu32) as u32); rax = P35; crate::simd_ops::wu64(rbp.wrapping_add(0x150u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x158u64), (0x5eu32) as u32); rax = P36; crate::simd_ops::wu64(rbp.wrapping_add(0x160u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x168u64), (0x5fu32) as u32); rax = P37; crate::simd_ops::wu64(rbp.wrapping_add(0x170u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x178u64), (0x60u32) as u32); rax = P38; crate::simd_ops::wu64(rbp.wrapping_add(0x180u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x188u64), (0x61u32) as u32); rax = P39; crate::simd_ops::wu64(rbp.wrapping_add(0x190u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x198u64), (0x62u32) as u32); rax = P40; crate::simd_ops::wu64(rbp.wrapping_add(0x1a0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1a8u64), (0x63u32) as u32); rax = P41; crate::simd_ops::wu64(rbp.wrapping_add(0x1b0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1b8u64), (0x64u32) as u32); rax = P42; crate::simd_ops::wu64(rbp.wrapping_add(0x1c0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1c8u64), (0x65u32) as u32); rax = P43; crate::simd_ops::wu64(rbp.wrapping_add(0x1d0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1d8u64), (0x66u32) as u32); rax = P44; crate::simd_ops::wu64(rbp.wrapping_add(0x1e0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1e8u64), (0x67u32) as u32); rax = P45; crate::simd_ops::wu64(rbp.wrapping_add(0x1f0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1f8u64), (0x68u32) as u32); rax = P46; crate::simd_ops::wu64(rbp.wrapping_add(0x200u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x208u64), (0x69u32) as u32); rax = P47; crate::simd_ops::wu64(rbp.wrapping_add(0x210u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x218u64), (0x6eu32) as u32); rax = P48; crate::simd_ops::wu64(rbp.wrapping_add(0x220u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x228u64), (0x6fu32) as u32); rax = P49; crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x238u64), (0x70u32) as u32); rax = P50; crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), (0x71u32) as u32); rax = P51; crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), (0x6au32) as u32); rax = P52; crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0x6bu32) as u32); rax = P53; crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), (0x6cu32) as u32); rax = P54; crate::simd_ops::wu64(rbp.wrapping_add(0x280u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x288u64), (0x6du32) as u32); rax = P55; crate::simd_ops::wu64(rbp.wrapping_add(0x290u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x298u64), (0x72u32) as u32); rax = P56; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2a8u64), (0x73u32) as u32); rax = P57; crate::simd_ops::wu64(rbp.wrapping_add(0x2b0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2b8u64), (0x74u32) as u32); rax = P58; crate::simd_ops::wu64(rbp.wrapping_add(0x2c0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2c8u64), (0x75u32) as u32); rax = P59; crate::simd_ops::wu64(rbp.wrapping_add(0x2d0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2d8u64), (0x76u32) as u32); rax = P60; crate::simd_ops::wu64(rbp.wrapping_add(0x2e0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2e8u64), (0x77u32) as u32); rax = P61; crate::simd_ops::wu64(rbp.wrapping_add(0x2f0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2f8u64), (0x78u32) as u32); rax = P62; crate::simd_ops::wu64(rbp.wrapping_add(0x300u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x308u64), (0x79u32) as u32); rax = P63; crate::simd_ops::wu64(rbp.wrapping_add(0x310u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x318u64), (0x7au32) as u32); rax = P64; crate::simd_ops::wu64(rbp.wrapping_add(0x320u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x328u64), (0x7bu32) as u32); rax = P65; crate::simd_ops::wu64(rbp.wrapping_add(0x330u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x338u64), (0x7cu32) as u32); rax = P66; crate::simd_ops::wu64(rbp.wrapping_add(0x340u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x348u64), (0x7du32) as u32); rax = P67; crate::simd_ops::wu64(rbp.wrapping_add(0x350u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x358u64), (0x7eu32) as u32); rax = P68; crate::simd_ops::wu64(rbp.wrapping_add(0x360u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x368u64), (0x7fu32) as u32); rax = P69; crate::simd_ops::wu64(rbp.wrapping_add(0x370u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x378u64), (0x80u32) as u32); rax = P70; crate::simd_ops::wu64(rbp.wrapping_add(0x380u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x388u64), (0x81u32) as u32); rax = P71; crate::simd_ops::wu64(rbp.wrapping_add(0x390u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x398u64), (0x82u32) as u32); rax = P72; crate::simd_ops::wu64(rbp.wrapping_add(0x3a0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x3a8u64), (0x83u32) as u32); rax = P73; crate::simd_ops::wu64(rbp.wrapping_add(0x3b0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x3b8u64), (0x84u32) as u32); rax = P74; crate::simd_ops::wu64(rbp.wrapping_add(0x3c0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x3c8u64), (0x85u32) as u32); rax = P75; crate::simd_ops::wu64(rbp.wrapping_add(0x3d0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x3d8u64), (0x86u32) as u32); rax = P76; crate::simd_ops::wu64(rbp.wrapping_add(0x3e0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x3e8u64), (0x87u32) as u32); rax = P77; crate::simd_ops::wu64(rbp.wrapping_add(0x3f0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x3f8u64), (0x88u32) as u32); rax = P78; crate::simd_ops::wu64(rbp.wrapping_add(0x400u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x408u64), (0x89u32) as u32); rax = P79; crate::simd_ops::wu64(rbp.wrapping_add(0x410u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x418u64), (0x8au32) as u32); rax = P80; crate::simd_ops::wu64(rbp.wrapping_add(0x420u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x428u64), (0x8bu32) as u32); rax = P81; crate::simd_ops::wu64(rbp.wrapping_add(0x430u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x438u64), (0x8cu32) as u32); rax = P82; crate::simd_ops::wu64(rbp.wrapping_add(0x440u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x448u64), (0x8du32) as u32); rax = P83; crate::simd_ops::wu64(rbp.wrapping_add(0x450u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x458u64), (0x8eu32) as u32); rax = P84; crate::simd_ops::wu64(rbp.wrapping_add(0x460u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x468u64), (0x8fu32) as u32); rax = P85; crate::simd_ops::wu64(rbp.wrapping_add(0x470u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x478u64), (0x90u32) as u32); rax = P86; crate::simd_ops::wu64(rbp.wrapping_add(0x480u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x488u64), (0x91u32) as u32); rax = P87; crate::simd_ops::wu64(rbp.wrapping_add(0x490u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x498u64), (0x92u32) as u32); rax = P88; crate::simd_ops::wu64(rbp.wrapping_add(0x4a0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x4a8u64), (0x93u32) as u32); rax = P89; crate::simd_ops::wu64(rbp.wrapping_add(0x4b0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x4b8u64), (0x94u32) as u32); rax = P90; crate::simd_ops::wu64(rbp.wrapping_add(0x4c0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x4c8u64), (0x95u32) as u32); rax = (crate::simd_ops::ru32(P91)) as u32 as u64; rsi = (crate::simd_ops::ru32(P92)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = r15; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rdi }) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P93, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P92)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P94)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P95, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P94)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P94)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P96, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P97, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P94, rbx); crate::simd_ops::wu32(P92, ((rsi as u32)) as u32); crate::simd_ops::wu32(P91, ((rdi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P94)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P98)) as u32 as u64; fl_l = rax; fl_r = (0x5bu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x5bu64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P99)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P100, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P99)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P101)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P95, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P101)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P101)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P96, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P97, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P101, rdi); crate::simd_ops::wu32(P99, ((rsi as u32)) as u32); crate::simd_ops::wu32(P98, ((rbx as u32)) as u32); rsi = (crate::simd_ops::ru32(P92)) as u32 as u64; rbx = (crate::simd_ops::ru64(P94)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P102, (0xffffffffu32) as u32); r14 = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rsi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P91)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rdi = rdx; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; fl_l = rdi; fl_r = rax; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; rsi = rcx; rdx = rdi; rax = crate::simd_ops::call_stk(P93, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P92)) as u32 as u64; rdx = (crate::simd_ops::ru64(P94)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P95, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P94)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P94)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P96, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P97, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P94, rbx); crate::simd_ops::wu32(P92, ((rsi as u32)) as u32); crate::simd_ops::wu32(P91, ((rdi as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rsi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P92, (1) as u32); rdx = r14.wrapping_add(0x8); rcx = P101; rax = crate::simd_ops::call_stk(P103, rax, rcx, rdx, r8, r9, rsp); r14 = ((r14).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rsi = (crate::simd_ops::ru32(P92)) as u32 as u64; rbx = (crate::simd_ops::ru64(P94)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P104)) as u64; crate::simd_ops::wu64(P105, rax); rax = P94; crate::simd_ops::wu64(P104, rax); rcx = P106; r11 = rsp.wrapping_add(0x5d0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P107, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14008ea30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143107e38; const P1: u64 = 0x143107e20; const P2: u64 = 0x143107e08; const P3: u64 = 0x143107df0; const P4: u64 = 0x143107dd0; const P5: u64 = 0x143107db0; const P6: u64 = 0x143107f88; const P7: u64 = 0x143107f58; const P8: u64 = 0x143107f38; const P9: u64 = 0x143107f10; const P10: u64 = 0x143107ef0; const P11: u64 = 0x143107ec8; const P12: u64 = 0x143107ea0; const P13: u64 = 0x143107e80; const P14: u64 = 0x1431080b8; const P15: u64 = 0x143108090; const P16: u64 = 0x143108068; const P17: u64 = 0x143108038; const P18: u64 = 0x143108018; const P19: u64 = 0x143107ff8; const P20: u64 = 0x143107fd8; const P21: u64 = 0x143107fb0; const P22: u64 = 0x143107a18; const P23: u64 = 0x1431079f8; const P24: u64 = 0x1431079e8; const P25: u64 = 0x1431079d8; const P26: u64 = 0x1431079c8; const P27: u64 = 0x1431079b8; const P28: u64 = 0x143107998; const P29: u64 = 0x143107988; const P30: u64 = 0x143107ae0; const P31: u64 = 0x143107ac8; const P32: u64 = 0x143107aa8; const P33: u64 = 0x143107a98; const P34: u64 = 0x143107a80; const P35: u64 = 0x143107a68; const P36: u64 = 0x143107a48; const P37: u64 = 0x143107a28; const P38: u64 = 0x143107b98; const P39: u64 = 0x143107b80; const P40: u64 = 0x143107b68; const P41: u64 = 0x143107b50; const P42: u64 = 0x143107b38; const P43: u64 = 0x143107b20; const P44: u64 = 0x143107b10; const P45: u64 = 0x143107af8; const P46: u64 = 0x143107c80; const P47: u64 = 0x143107c68; const P48: u64 = 0x143107c48; const P49: u64 = 0x143107c28; const P50: u64 = 0x143107c08; const P51: u64 = 0x143107be8; const P52: u64 = 0x143107bc8; const P53: u64 = 0x143107bb0; const P54: u64 = 0x143108568; const P55: u64 = 0x143108550; const P56: u64 = 0x143108540; const P57: u64 = 0x143108510; const P58: u64 = 0x143108500; const P59: u64 = 0x1431084e8; const P60: u64 = 0x1431084d0; const P61: u64 = 0x1431084c0; const P62: u64 = 0x143108618; const P63: u64 = 0x143108600; const P64: u64 = 0x1431085e8; const P65: u64 = 0x1431085d8; const P66: u64 = 0x1431077a0; const P67: u64 = 0x1431085c0; const P68: u64 = 0x1431085a8; const P69: u64 = 0x143108590; const P70: u64 = 0x143108578; const P71: u64 = 0x143108700; const P72: u64 = 0x1431086e8; const P73: u64 = 0x1431086c8; const P74: u64 = 0x1431086a8; const P75: u64 = 0x143108688; const P76: u64 = 0x143108668; const P77: u64 = 0x143108648; const P78: u64 = 0x143108628; const P79: u64 = 0x1431087d8; const P80: u64 = 0x1431087c0; const P81: u64 = 0x1431087a8; const P82: u64 = 0x143108788; const P83: u64 = 0x143108760; const P84: u64 = 0x143108750; const P85: u64 = 0x143108738; const P86: u64 = 0x143108718; const P87: u64 = 0x143108168; const P88: u64 = 0x143108150; const P89: u64 = 0x143108140; const P90: u64 = 0x143108130; const P91: u64 = 0x143108118; const P92: u64 = 0x143108108; const P93: u64 = 0x1431080f0; const P94: u64 = 0x1431080d8; const P95: u64 = 0x143108250; const P96: u64 = 0x143108230; const P97: u64 = 0x143108218; const P98: u64 = 0x1431081f0; const P99: u64 = 0x1431081d0; const P100: u64 = 0x1431081b8; const P101: u64 = 0x143108198; const P102: u64 = 0x143108180; const P103: u64 = 0x143108360; const P104: u64 = 0x143108348; const P105: u64 = 0x143108328; const P106: u64 = 0x143108308; const P107: u64 = 0x1431082e0; const P108: u64 = 0x1431082b0; const P109: u64 = 0x143108290; const P110: u64 = 0x143108270; const P111: u64 = 0x143108498; const P112: u64 = 0x143108470; const P113: u64 = 0x143108448; const P114: u64 = 0x143108420; const P115: u64 = 0x1431083f8; const P116: u64 = 0x1431083c8; const P117: u64 = 0x1431083a0; const P118: u64 = 0x143108380; const P119: u64 = 0x143108d78; const P120: u64 = 0x143108d48; const P121: u64 = 0x143108d20; const P122: u64 = 0x143108d00; const P123: u64 = 0x143108ce0; const P124: u64 = 0x143108cc0; const P125: u64 = 0x143108ca0; const P126: u64 = 0x143108c88; const P127: u64 = 0x143108e60; const P128: u64 = 0x143108e48; const P129: u64 = 0x143108e30; const P130: u64 = 0x143108e18; const P131: u64 = 0x143108df8; const P132: u64 = 0x143108dd8; const P133: u64 = 0x143108db0; const P134: u64 = 0x143108d98; const P135: u64 = 0x143108f60; const P136: u64 = 0x143108f38; const P137: u64 = 0x143108f10; const P138: u64 = 0x143108ee8; const P139: u64 = 0x143108ec8; const P140: u64 = 0x143108eb0; const P141: u64 = 0x143108e90; const P142: u64 = 0x143108e78; const P143: u64 = 0x143109028; const P144: u64 = 0x143109010; const P145: u64 = 0x143108ff8; const P146: u64 = 0x143108fe0; const P147: u64 = 0x143108fc8; const P148: u64 = 0x143108fb0; const P149: u64 = 0x143108f98; const P150: u64 = 0x143108f80; const P151: u64 = 0x143108900; const P152: u64 = 0x1431088e0; const P153: u64 = 0x1431088c0; const P154: u64 = 0x1431088a0; const P155: u64 = 0x143108880; const P156: u64 = 0x143108860; const P157: u64 = 0x143108828; const P158: u64 = 0x1431087f0; const P159: u64 = 0x143108a00; const P160: u64 = 0x1431089d0; const P161: u64 = 0x143a2235c; const P162: u64 = 0x143a22358; const P163: u64 = 0x1401dec00; const P164: u64 = 0x143a22350; const P165: u64 = 0x14183acc8; const P166: u64 = 0x1425371a0; const P167: u64 = 0x142533200; const P168: u64 = 0x143a2236c; const P169: u64 = 0x143a22368; const P170: u64 = 0x1401decf0; const P171: u64 = 0x143a22360; const P172: u64 = 0x143a22384; const P173: u64 = 0x1401de8d0; const P174: u64 = 0x143a082e0; const P175: u64 = 0x143a22388; const P176: u64 = 0x1425e6360; const P177: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<512, 329>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x930u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa30u64)) as u64; rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x17u32) as u32); rax = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x18u32) as u32); rax = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x19u32) as u32); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x1au32) as u32); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x1bu32) as u32); rax = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x1cu32) as u32); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x1du32) as u32); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x1eu32) as u32); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x1fu32) as u32); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x20u32) as u32); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0x21u32) as u32); rax = ((P11) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0x22u32) as u32); rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0x23u32) as u32); rax = ((P13) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0x24u32) as u32); rax = ((P14) as u64) as u64; crate::simd_ops::wu64(rbp, (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0x25u32) as u32); rax = ((P15) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), (0x26u32) as u32); rax = ((P16) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), (0x27u32) as u32); rax = ((P17) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0x28u32) as u32); rax = ((P18) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), (0x29u32) as u32); rax = ((P19) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), (0x2au32) as u32); rax = ((P20) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0x2bu32) as u32); rax = ((P21) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), (0x2cu32) as u32); rax = ((P22) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), (0x2du32) as u32); rax = ((P23) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x98u64), (0x32u32) as u32); rax = ((P24) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xa8u64), (0x2eu32) as u32); rax = ((P25) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xb8u64), (0x2fu32) as u32); rax = ((P26) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), (0x30u32) as u32); rax = ((P27) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xd8u64), (0x31u32) as u32); rax = ((P28) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xe0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xe8u64), (0x33u32) as u32); rax = ((P29) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xf0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xf8u64), (0x34u32) as u32); rax = ((P30) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x100u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x108u64), (0x35u32) as u32); rax = ((P31) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x110u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x118u64), (0x36u32) as u32); rax = ((P32) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x120u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x128u64), (0x37u32) as u32); rax = ((P33) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x130u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x138u64), (0x38u32) as u32); rax = ((P34) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x140u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x148u64), (0x58u32) as u32); rax = ((P35) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x150u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x158u64), (0x59u32) as u32); rax = ((P36) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x160u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x168u64), (0x5au32) as u32); rax = ((P37) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x170u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x178u64), (0x5bu32) as u32); rax = ((P38) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x180u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x188u64), (0x3fu32) as u32); rax = ((P39) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x190u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x198u64), (0x39u32) as u32); rax = ((P40) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1a0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x1a8u64), (0x3au32) as u32); rax = ((P41) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1b0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x1b8u64), (0x3bu32) as u32); rax = ((P42) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1c0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x1c8u64), (0x3cu32) as u32); rax = ((P43) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1d0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x1d8u64), (0x3du32) as u32); rax = ((P44) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1e0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x1e8u64), (0x3eu32) as u32); rax = ((P45) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1f0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x1f8u64), (0x40u32) as u32); rax = ((P46) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x200u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x208u64), (0x41u32) as u32); rax = ((P47) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x210u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x218u64), (0x42u32) as u32); rax = ((P48) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x220u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x228u64), (0x43u32) as u32); rax = ((P49) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x238u64), (0x44u32) as u32); rax = ((P50) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), (0x45u32) as u32); rax = ((P51) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), (0x46u32) as u32); rax = ((P52) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0x47u32) as u32); rax = ((P53) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), (0x48u32) as u32); rax = ((P54) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x280u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x288u64), (0x49u32) as u32); rax = ((P55) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x290u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x298u64), (0x4au32) as u32); rax = ((P56) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x2a8u64), (0x4bu32) as u32); rax = ((P57) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2b0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x2b8u64), (0x4cu32) as u32); rax = ((P58) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2c0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x2c8u64), (0x4du32) as u32); rax = ((P59) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2d0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x2d8u64), (0x4eu32) as u32); rax = ((P60) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2e0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x2e8u64), (0x4fu32) as u32); rax = ((P61) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2f0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x2f8u64), (0x50u32) as u32); rax = ((P62) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x300u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x308u64), (0x51u32) as u32); rax = ((P63) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x310u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x318u64), (0x52u32) as u32); rax = ((P64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x320u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x328u64), (0x53u32) as u32); rax = ((P65) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x330u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x338u64), (0x54u32) as u32); rax = ((P66) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x340u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x348u64), (0x55u32) as u32); rax = ((P67) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x350u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x358u64), (0x56u32) as u32); rax = ((P68) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x360u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x368u64), (0x57u32) as u32); rax = ((P69) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x370u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x378u64), (0x5cu32) as u32); rax = ((P70) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x380u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x388u64), (0x5du32) as u32); rax = ((P71) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x390u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x398u64), (0x5fu32) as u32); rax = ((P72) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3a0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x3a8u64), (0x60u32) as u32); rax = ((P73) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3b0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x3b8u64), (0x61u32) as u32); rax = ((P74) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3c0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x3c8u64), (0x65u32) as u32); rax = ((P75) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3d0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x3d8u64), (0x66u32) as u32); rax = ((P76) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3e0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x3e8u64), (0x62u32) as u32); rax = ((P77) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x3f0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x3f8u64), (0x63u32) as u32); rax = ((P78) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x400u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x408u64), (0x64u32) as u32); rax = ((P79) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x410u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x418u64), (0x5eu32) as u32); rax = ((P80) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x420u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x428u64), (0xa8u32) as u32); rax = ((P81) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x430u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x438u64), (0xa9u32) as u32); rax = ((P82) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x440u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x448u64), (0xaau32) as u32); rax = ((P83) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x450u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x458u64), (0xabu32) as u32); rax = ((P84) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x460u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x468u64), (0xacu32) as u32); rax = ((P85) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x470u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x478u64), (0xadu32) as u32); rax = ((P86) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x480u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x488u64), (0xaeu32) as u32); rax = ((P87) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x490u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x498u64), (0x6bu32) as u32); rax = ((P88) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4a0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x4a8u64), (0x67u32) as u32); rax = ((P89) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4b0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x4b8u64), (0x68u32) as u32); rax = ((P90) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4c0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x4c8u64), (0x69u32) as u32); rax = ((P91) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4d0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x4d8u64), (0x6au32) as u32); rax = ((P92) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4e0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x4e8u64), (0x6cu32) as u32); rax = ((P93) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x4f0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x4f8u64), (0x6du32) as u32); rax = ((P94) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x500u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x508u64), (0x6eu32) as u32); rax = ((P95) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x510u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x518u64), (0x6fu32) as u32); rax = ((P96) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x520u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x528u64), (0x70u32) as u32); rax = ((P97) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x530u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x538u64), (0x71u32) as u32); rax = ((P98) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x540u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x548u64), (0x72u32) as u32); rax = ((P99) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x550u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x558u64), (0x73u32) as u32); rax = ((P100) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x560u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x568u64), (0x74u32) as u32); rax = ((P101) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x570u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x578u64), (0x75u32) as u32); rax = ((P102) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x580u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x588u64), (0x76u32) as u32); rax = ((P103) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x590u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x598u64), (0x77u32) as u32); rax = ((P104) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x5a0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x5a8u64), (0x78u32) as u32); rax = ((P105) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x5b0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x5b8u64), (0x7au32) as u32); rax = ((P106) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x5c0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x5c8u64), (0x79u32) as u32); rax = ((P107) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x5d0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x5d8u64), (0x7bu32) as u32); rax = ((P108) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x5e0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x5e8u64), (0x7cu32) as u32); rax = ((P109) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x5f0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x5f8u64), (0x7du32) as u32); rax = ((P110) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x600u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x608u64), (0x7eu32) as u32); rax = ((P111) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x610u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x618u64), (0x7fu32) as u32); rax = ((P112) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x620u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x628u64), (0x80u32) as u32); rax = ((P113) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x630u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x638u64), (0x81u32) as u32); rax = ((P114) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x640u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x648u64), (0x82u32) as u32); rax = ((P115) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x650u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x658u64), (0x83u32) as u32); rax = ((P116) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x660u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x668u64), (0x84u32) as u32); rax = ((P117) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x670u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x678u64), (0x85u32) as u32); rax = ((P118) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x680u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x688u64), (0x86u32) as u32); rax = ((P119) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x690u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x698u64), (0x87u32) as u32); rax = ((P120) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6a0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x6a8u64), (0x88u32) as u32); rax = ((P121) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6b0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x6b8u64), (0x89u32) as u32); rax = ((P122) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6c0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x6c8u64), (0x8au32) as u32); rax = ((P123) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6d0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x6d8u64), (0x8bu32) as u32); rax = ((P124) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6e0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x6e8u64), (0x8cu32) as u32); rax = ((P125) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x6f0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x6f8u64), (0x8fu32) as u32); rax = ((P126) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x700u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x708u64), (0x8du32) as u32); rax = ((P127) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x710u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x718u64), (0x8eu32) as u32); rax = ((P128) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x720u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x728u64), (0x90u32) as u32); rax = ((P129) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x730u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x738u64), (0x91u32) as u32); rax = ((P130) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x740u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x748u64), (0x92u32) as u32); rax = ((P131) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x750u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x758u64), (0x93u32) as u32); rax = ((P132) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x760u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x768u64), (0x94u32) as u32); rax = ((P133) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x770u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x778u64), (0x95u32) as u32); rax = ((P134) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x780u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x788u64), (0x96u32) as u32); rax = ((P135) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x790u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x798u64), (0x97u32) as u32); rax = ((P136) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7a0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7a8u64), (0x98u32) as u32); rax = ((P137) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7b0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7b8u64), (0x99u32) as u32); rax = ((P138) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7c0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7c8u64), (0x9au32) as u32); rax = ((P139) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7d0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7d8u64), (0x9bu32) as u32); rax = ((P140) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7e0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7e8u64), (0x9cu32) as u32); rax = ((P141) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x7f0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7f8u64), (0x9du32) as u32); rax = ((P142) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x800u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x808u64), (0x9eu32) as u32); rax = ((P143) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x810u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x818u64), (0x9fu32) as u32); rax = ((P144) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x820u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x828u64), (0xa0u32) as u32); rax = ((P145) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x830u64), (rax) as u64); r15 = (0xa1u32) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x838u64), ((r15 as u32)) as u32); rax = ((P146) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x840u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x848u64), (0xa2u32) as u32); rax = ((P147) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x850u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x858u64), (0xa3u32) as u32); rax = ((P148) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x860u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x868u64), (0xa4u32) as u32); rax = ((P149) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x870u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x878u64), (0xa5u32) as u32); rax = ((P150) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x880u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x888u64), (0xa6u32) as u32); rax = ((P151) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x890u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x898u64), (0xa7u32) as u32); rax = ((P152) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8a0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x8a8u64), (0xafu32) as u32); rax = ((P153) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8b0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x8b8u64), (0xb0u32) as u32); rax = ((P154) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8c0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x8c8u64), (0xb1u32) as u32); rax = ((P155) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8d0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x8d8u64), (0xb2u32) as u32); rax = ((P156) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8e0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x8e8u64), (0xb3u32) as u32); rax = ((P157) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x8f0u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x8f8u64), (0xb4u32) as u32); rax = ((P158) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x900u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x908u64), (0xb5u32) as u32); rax = ((P159) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x910u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x918u64), (0xb6u32) as u32); rax = ((P160) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x920u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x928u64), (0xb7u32) as u32); rax = (crate::simd_ops::ru32(P161)) as u32 as u64; rsi = (crate::simd_ops::ru32(P162)) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (r15) as u64 as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rdi }) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P163) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; r8 = (crate::simd_ops::ru32(P162)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P164)) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P165) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (crate::simd_ops::ru64(P164)) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P164)) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P166) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P167) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P164, (rbx) as u64); crate::simd_ops::wu32(P162, ((rsi as u32)) as u32); crate::simd_ops::wu32(P161, ((rdi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P164)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P168)) as u32 as u64; fl_l = (rax) as u64 as u64; fl_r = (r15) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r15) as u64 as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P169)) as u32 as u64; rdx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P170) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rdi = (rax) as u64; r8 = (crate::simd_ops::ru32(P169)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P171)) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P165) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (crate::simd_ops::ru64(P171)) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P171)) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P166) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P167) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P171, (rdi) as u64); crate::simd_ops::wu32(P169, ((rsi as u32)) as u32); crate::simd_ops::wu32(P168, ((rbx as u32)) as u32); rsi = (crate::simd_ops::ru32(P162)) as u32 as u64; rbx = (crate::simd_ops::ru64(P164)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P172, (0xffffffffu32) as u32); r14 = ((rsp.wrapping_add(0x20u64)) as u64) as u64; bb = 22; continue; }
            22 => { rax = ((rsi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P161)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u64) as u64; rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rdi = (rdx) as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; rsi = (rcx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P163) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (rax) as u64; rax = (crate::simd_ops::ru32(P162)) as u32 as u64; rdx = (crate::simd_ops::ru64(P164)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P165) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r10 = (crate::simd_ops::ru64(P164)) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P164)) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = (rdx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P166) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P167) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P164, (rbx) as u64); crate::simd_ops::wu32(P162, ((rsi as u32)) as u32); crate::simd_ops::wu32(P161, ((rdi as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rsi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rcx, (rax) as u64); crate::simd_ops::lk_add_u32(P162, (1) as u32); rdx = ((r14.wrapping_add(0x8u64)) as u64) as u64; rcx = ((P171) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P173) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); r14 = ((r14).wrapping_add(0x10u64)) as u64; fl_l = (r15) as u64 as u64; fl_r = (0x1u64) as u64 as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rsi = (crate::simd_ops::ru32(P162)) as u32 as u64; rbx = (crate::simd_ops::ru64(P164)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P174)) as u64; crate::simd_ops::wu64(P175, (rax) as u64); rax = ((P164) as u64) as u64; crate::simd_ops::wu64(P174, (rax) as u64); rcx = ((P176) as u64) as u64; r11 = ((rsp.wrapping_add(0xa30u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((P177) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14008fc80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1431089b8; const P1: u64 = 0x143108998; const P2: u64 = 0x143a21840; const P3: u64 = 0x143108978; const P4: u64 = 0x1401de5b0; const P5: u64 = 0x1425e6390; const P6: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x3u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = P5;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14008fcf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14309f824; const P1: u64 = 0x143108938; const P2: u64 = 0x143a221d0; const P3: u64 = 0x143108928; const P4: u64 = 0x143108ad0; const P5: u64 = 0x14307898c; const P6: u64 = 0x143108ac0; const P7: u64 = 0x143108ab0; const P8: u64 = 0x143106a78; const P9: u64 = 0x143108a90; const P10: u64 = 0x14038be40; const P11: u64 = 0x1425e63c0; const P12: u64 = 0x141834d18;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = P2; rax = P3; crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rax); rax = P4; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax); rax = P5; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x4u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax); rax = P6; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x8u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); rax = P7; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x10u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax); rax = P8; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rax); rax = P9; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax); crate::simd_ops::wu32(r11.wrapping_sub(0x30u64), (0x20u32) as u32); crate::simd_ops::wu32(r11.wrapping_sub(0x20u64), (0x40u32) as u32); crate::simd_ops::wu32(r11.wrapping_sub(0x10u64), (0x80u32) as u32); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = P11; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14008fec0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143109b30; const P1: u64 = 0x143109b20; const P2: u64 = 0x143109b10; const P3: u64 = 0x143109b00; const P4: u64 = 0x143109af0; const P5: u64 = 0x143109ae0; const P6: u64 = 0x14309f914; const P7: u64 = 0x143109ad4; const P8: u64 = 0x14309f90c; const P9: u64 = 0x143109ac8; const P10: u64 = 0x143109b88; const P11: u64 = 0x143109b78; const P12: u64 = 0x143109b68; const P13: u64 = 0x143109b58; const P14: u64 = 0x143109b48; const P15: u64 = 0x143109b40; const P16: u64 = 0x143a225fc; const P17: u64 = 0x143a225f8; const P18: u64 = 0x1401dec00; const P19: u64 = 0x143a225f0; const P20: u64 = 0x14183acc8; const P21: u64 = 0x1425371a0; const P22: u64 = 0x142533200; const P23: u64 = 0x143a2260c; const P24: u64 = 0x143a22608; const P25: u64 = 0x1401decf0; const P26: u64 = 0x143a22600; const P27: u64 = 0x143a22624; const P28: u64 = 0x1401de8d0; const P29: u64 = 0x143a082e0; const P30: u64 = 0x143a22628; const P31: u64 = 0x1425e6450; const P32: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x20); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0xau32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0xbu32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0xcu32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x3u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x4u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x5u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0xdu32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0xeu32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xfu32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0x6u32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0x7u32) as u32); rax = P14; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0x8u32) as u32); rax = P15; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), (0x9u32) as u32); rax = (crate::simd_ops::ru32(P16)) as u32 as u64; r15 = 0x10; rdi = (crate::simd_ops::ru32(P17)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r15; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P17)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P19)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P19)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P19)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P19, rbx); crate::simd_ops::wu32(P17, ((rdi as u32)) as u32); crate::simd_ops::wu32(P16, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P19)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P23)) as u32 as u64; fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x10u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P24)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P25, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P24)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P26)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P26)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P26)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P26, rdi); crate::simd_ops::wu32(P24, ((rsi as u32)) as u32); crate::simd_ops::wu32(P23, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P17)) as u32 as u64; rbx = (crate::simd_ops::ru64(P19)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P27, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P16)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rdx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rdi = rcx; rdx = r14; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P17)) as u32 as u64; rdx = (crate::simd_ops::ru64(P19)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P19)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P19)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P19, rbx); crate::simd_ops::wu32(P17, ((rdi as u32)) as u32); crate::simd_ops::wu32(P16, ((r14 as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P17, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P26; rax = crate::simd_ops::call_stk(P28, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P17)) as u32 as u64; rbx = (crate::simd_ops::ru64(P19)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P29)) as u64; crate::simd_ops::wu64(P30, rax); rax = P19; crate::simd_ops::wu64(P29, rax); rcx = P31; r11 = rsp.wrapping_add(0x120); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P32, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140090430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143109bd4; const P1: u64 = 0x143109bc8; const P2: u64 = 0x143109bb0; const P3: u64 = 0x143109ba4; const P4: u64 = 0x143109b98; const P5: u64 = 0x143109740; const P6: u64 = 0x143109728; const P7: u64 = 0x143109720; const P8: u64 = 0x143109718; const P9: u64 = 0x143109708; const P10: u64 = 0x1431096f8; const P11: u64 = 0x1431096e0; const P12: u64 = 0x1431096c8; const P13: u64 = 0x143109840; const P14: u64 = 0x143109838; const P15: u64 = 0x143109834; const P16: u64 = 0x143109828; const P17: u64 = 0x143a2171c; const P18: u64 = 0x143a21718; const P19: u64 = 0x1401dec00; const P20: u64 = 0x143a21710; const P21: u64 = 0x14183acc8; const P22: u64 = 0x1425371a0; const P23: u64 = 0x142533200; const P24: u64 = 0x143a2172c; const P25: u64 = 0x143a21728; const P26: u64 = 0x1401decf0; const P27: u64 = 0x143a21720; const P28: u64 = 0x143a21744; const P29: u64 = 0x1401de8d0; const P30: u64 = 0x143a082e0; const P31: u64 = 0x143a21748; const P32: u64 = 0x1425e64b0; const P33: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x30); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x3u32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x4u32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x5u32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x6u32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x7u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x8u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x9u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0xau32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0xbu32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xcu32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0xdu32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0xeu32) as u32); rax = P14; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0xfu32) as u32); rax = P15; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), (0x10u32) as u32); rax = P16; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); r15 = 0x11; crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(P17)) as u32 as u64; rdi = (crate::simd_ops::ru32(P18)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r15; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P18)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P20)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P20)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P20)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P20, rbx); crate::simd_ops::wu32(P18, ((rdi as u32)) as u32); crate::simd_ops::wu32(P17, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P20)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P24)) as u32 as u64; fl_l = rax; fl_r = (0x11u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x11u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P25)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P26, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P25)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P27)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P27)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P27)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P27, rdi); crate::simd_ops::wu32(P25, ((rsi as u32)) as u32); crate::simd_ops::wu32(P24, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P18)) as u32 as u64; rbx = (crate::simd_ops::ru64(P20)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P28, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P17)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rdx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rdi = rcx; rdx = r14; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P18)) as u32 as u64; rdx = (crate::simd_ops::ru64(P20)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P20)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P20)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P20, rbx); crate::simd_ops::wu32(P18, ((rdi as u32)) as u32); crate::simd_ops::wu32(P17, ((r14 as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P18, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P27; rax = crate::simd_ops::call_stk(P29, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P18)) as u32 as u64; rbx = (crate::simd_ops::ru64(P20)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P30)) as u64; crate::simd_ops::wu64(P31, rax); rax = P20; crate::simd_ops::wu64(P30, rax); rcx = P32; r11 = rsp.wrapping_add(0x130); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P33, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140090980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143109940; const P1: u64 = 0x14310993c; const P2: u64 = 0x143a21640; const P3: u64 = 0x143109930; const P4: u64 = 0x143078678; const P5: u64 = 0x1403f8780; const P6: u64 = 0x1425e6510; const P7: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x4u32) as u32);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = P6;
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140090a00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143109b3c; const P1: u64 = 0x143109bf8; const P2: u64 = 0x143a21c20; const P3: u64 = 0x14309f914; const P4: u64 = 0x14309f90c; const P5: u64 = 0x1403f8780; const P6: u64 = 0x1425e6540; const P7: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x3u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x0u32) as u32);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = P6;
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140090c80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1430d8b50; const P1: u64 = 0x1430d8b58; const P2: u64 = 0x143a21b80; const P3: u64 = 0x1430d8b10; const P4: u64 = 0x1430d8b18; const P5: u64 = 0x1430d8b28; const P6: u64 = 0x143109de8; const P7: u64 = 0x1403f88a0; const P8: u64 = 0x1425e6660; const P9: u64 = 0x141834d18;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = rsp;
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax);
    rdx = r11.wrapping_sub(0x68);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32);
    rax = P1;
    crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax);
    rcx = P2;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32);
    rax = P3;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    rax = P4;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32);
    crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax);
    rax = P5;
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x3u32) as u32);
    crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rax);
    rax = P6;
    crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x4u32) as u32);
    crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x6u32) as u32);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rcx = P8;
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140090e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14310a098; const P1: u64 = 0x14310a090; const P2: u64 = 0x143a225b0; const P3: u64 = 0x14310a92c; const P4: u64 = 0x1401de5b0; const P5: u64 = 0x1425e6720; const P6: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x4u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = P5;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140090fa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14310a7d0; const P1: u64 = 0x14310a7c8; const P2: u64 = 0x143a22090; const P3: u64 = 0x14310a7c0; const P4: u64 = 0x1401de5b0; const P5: u64 = 0x1425e67e0; const P6: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x0u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = P5;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140091160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14310adc8; const P1: u64 = 0x14310adb8; const P2: u64 = 0x143a22120; const P3: u64 = 0x14310ada8; const P4: u64 = 0x14310ad98; const P5: u64 = 0x14310ad88; const P6: u64 = 0x14310ad80; const P7: u64 = 0x14310ad78; const P8: u64 = 0x14310ae48; const P9: u64 = 0x14310ae30; const P10: u64 = 0x14038be40; const P11: u64 = 0x1425e6870; const P12: u64 = 0x141834d18;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0xb8u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rsp.wrapping_add(0x20); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rcx = P2; rax = P3; crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rax); rax = P4; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x3u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax); rax = P5; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x4u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax); rax = P6; crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x5u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); rax = P7; crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x6u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax); rax = P8; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rax); rax = P9; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax); crate::simd_ops::wu32(r11.wrapping_sub(0x30u64), (0x7u32) as u32); crate::simd_ops::wu32(r11.wrapping_sub(0x20u64), (0x8u32) as u32); crate::simd_ops::wu32(r11.wrapping_sub(0x10u64), (0x9u32) as u32); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rcx = P11; rsp = ((rsp).wrapping_add(0xb8u64)) as u64; return crate::simd_ops::tail_stk(P12, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140091240(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14310a92c; const P1: u64 = 0x14310ae28; const P2: u64 = 0x14310ae24; const P3: u64 = 0x14310ae20; const P4: u64 = 0x14310ae1c; const P5: u64 = 0x14310ae10; const P6: u64 = 0x14310ae00; const P7: u64 = 0x14310b630; const P8: u64 = 0x14310b620; const P9: u64 = 0x14310b610; const P10: u64 = 0x14310b600; const P11: u64 = 0x14310b5f0; const P12: u64 = 0x14310b5e0; const P13: u64 = 0x14310b5d0; const P14: u64 = 0x143a21a7c; const P15: u64 = 0x143a21a78; const P16: u64 = 0x1401dec00; const P17: u64 = 0x143a21a70; const P18: u64 = 0x14183acc8; const P19: u64 = 0x1425371a0; const P20: u64 = 0x142533200; const P21: u64 = 0x143a21a8c; const P22: u64 = 0x143a21a88; const P23: u64 = 0x1401decf0; const P24: u64 = 0x143a21a80; const P25: u64 = 0x143a21aa4; const P26: u64 = 0x1401de8d0; const P27: u64 = 0x143a082e0; const P28: u64 = 0x143a21aa8; const P29: u64 = 0x1425e68a0; const P30: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 35>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x47); rsp = ((rsp).wrapping_sub(0x100u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32); rax = P2; crate::simd_ops::wu64(rbp.wrapping_sub(0x79u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x71u64), (0x3u32) as u32); rax = P3; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x61u64), (0x4u32) as u32); rax = P4; crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x51u64), (0x5u32) as u32); rax = P5; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x41u64), (0x6u32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x31u64), (0x7u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x21u64), (0x8u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), (0x9u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x1u64), (0xbu32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), (0xau32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), (0xcu32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_add(0x27u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2fu64), (0xdu32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_add(0x37u64), rax); r15 = 0xe; crate::simd_ops::wu32(rbp.wrapping_add(0x3fu64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(P14)) as u32 as u64; rdi = (crate::simd_ops::ru32(P15)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r15; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P15)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P17)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P17)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P17)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P17, rbx); crate::simd_ops::wu32(P15, ((rdi as u32)) as u32); crate::simd_ops::wu32(P14, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P17)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P21)) as u32 as u64; fl_l = rax; fl_r = (0xeu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0xeu64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P22)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P22)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P24)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P24)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P24)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P24, rdi); crate::simd_ops::wu32(P22, ((rsi as u32)) as u32); crate::simd_ops::wu32(P21, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P15)) as u32 as u64; rbx = (crate::simd_ops::ru64(P17)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P25, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P14)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rdx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rdi = rcx; rdx = r14; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P15)) as u32 as u64; rdx = (crate::simd_ops::ru64(P17)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P17)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P17)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P17, rbx); crate::simd_ops::wu32(P15, ((rdi as u32)) as u32); crate::simd_ops::wu32(P14, ((r14 as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P15, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P24; rax = crate::simd_ops::call_stk(P26, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P15)) as u32 as u64; rbx = (crate::simd_ops::ru64(P17)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P27)) as u64; crate::simd_ops::wu64(P28, rax); rax = P17; crate::simd_ops::wu64(P27, rax); rcx = P29; r11 = rsp.wrapping_add(0x100); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P30, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400916e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14310b6c0; const P1: u64 = 0x14310b6b0; const P2: u64 = 0x143a21d10; const P3: u64 = 0x14310b6a0; const P4: u64 = 0x14310b690; const P5: u64 = 0x14310b684; const P6: u64 = 0x14310b670; const P7: u64 = 0x1403f88a0; const P8: u64 = 0x1425e68d0; const P9: u64 = 0x141834d18;
    let (mut rax, mut r11) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    r11 = rsp;
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    rax = P0;
    crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax);
    rdx = r11.wrapping_sub(0x68);
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    rax = P1;
    crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax);
    rcx = P2;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32);
    rax = P3;
    crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax);
    rax = P4;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x3u32) as u32);
    crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax);
    rax = P5;
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x5u32) as u32);
    crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rax);
    rax = P6;
    crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x7u32) as u32);
    crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x8u32) as u32);
    rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp);
    rcx = P8;
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P9, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400a00c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1431202d8; const P1: u64 = 0x142409b50; const P2: u64 = 0x143a23d78; const P3: u64 = 0x140425090; const P4: u64 = 0x1401de3c0; const P5: u64 = 0xffffffffffffffb9; const P6: u64 = 0x141834ee8; const P7: u64 = 0x1425e7db0; const P8: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x88u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8000008u64) as u64);
    crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8);
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x8u64) as u64);
    r9 = 0x8;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x20);
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = rsp.wrapping_add(0x30);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rax = rsp.wrapping_add(0x78);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rcx = P2;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    r9 = P4;
    rdx = 0x48;
    r8 = ((rdx.wrapping_add(P5)) as u32) as u32 as u64;
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = P7;
    rsp = ((rsp).wrapping_add(0x88u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400a7b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a243a8; const P1: u64 = 0x143a243ab; const P2: u64 = 0x14312b590; const P3: u64 = 0x142409b50; const P4: u64 = 0x1425e9120; const P5: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64;
    rax = ((((rax as u32)) | (0x4u32))) as u32 as u64;
    crate::simd_ops::wu32(P0, ((rax as u32)) as u32);
    crate::simd_ops::wu8(P1, (0x4u8) as u8);
    rax = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1au64) as u64);
    r9 = 0x1a;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x20);
    rcx = P0;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rcx = P4;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400acbf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427048; const P1: u64 = 0x143427080; const P2: u64 = 0x1434268d0; const P3: u64 = 0x143a001ac; const P4: u64 = 0x143848e7c; const P5: u64 = 0x143848e88; const P6: u64 = 0x143848e78; const P7: u64 = 0x143848e84; const P8: u64 = 0x143848e90; const P9: u64 = 0x143848e9c; const P10: u64 = 0x143848e70; const P11: u64 = 0x143848e94; const P12: u64 = 0x143a00290; const P13: u64 = 0x143848e60; const P14: u64 = 0x1425e96a0; const P15: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm4 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    rcx = (((crate::simd_ops::ru32(P3)) as i32 as i64) as u64) as u64;
    xmm1 = 0u128;
    xmm3 = crate::simd_ops::vunpcklps(xmm1, xmm0);
    xmm0 = crate::simd_ops::vunpcklps(xmm2, xmm2);
    crate::simd_ops::wu64(P4, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vunpcklps(xmm4, xmm4);
    crate::simd_ops::wu64(P5, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(P6, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(P7, ((xmm2 as u32)) as u32);
    crate::simd_ops::wu32(P8, ((xmm4 as u32)) as u32);
    crate::simd_ops::wu32(P9, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu64(P10, ((xmm3 as u64)) as u64);
    crate::simd_ops::wu64(P11, ((xmm3 as u64)) as u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1000;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { r8 = P12; rdx = P13; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rdx); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P3, ((rcx as u32)) as u32); }
    rcx = P14;
    return crate::simd_ops::tail_fwd_xmm(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400acc90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427034; const P1: u64 = 0x143427080; const P2: u64 = 0x1434268d0; const P3: u64 = 0x143a001ac; const P4: u64 = 0x143848f5c; const P5: u64 = 0x143848f68; const P6: u64 = 0x143848f58; const P7: u64 = 0x143848f64; const P8: u64 = 0x143848f70; const P9: u64 = 0x143848f7c; const P10: u64 = 0x143848f50; const P11: u64 = 0x143848f74; const P12: u64 = 0x143a00290; const P13: u64 = 0x143848f40; const P14: u64 = 0x1425e9730; const P15: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm2 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm4 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    rcx = (((crate::simd_ops::ru32(P3)) as i32 as i64) as u64) as u64;
    xmm1 = 0u128;
    xmm3 = crate::simd_ops::vunpcklps(xmm0, xmm1);
    xmm0 = crate::simd_ops::vunpcklps(xmm2, xmm2);
    crate::simd_ops::wu64(P4, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vunpcklps(xmm4, xmm4);
    crate::simd_ops::wu64(P5, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(P6, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu32(P7, ((xmm2 as u32)) as u32);
    crate::simd_ops::wu32(P8, ((xmm4 as u32)) as u32);
    crate::simd_ops::wu32(P9, ((xmm1 as u32)) as u32);
    crate::simd_ops::wu64(P10, ((xmm3 as u64)) as u64);
    crate::simd_ops::wu64(P11, ((xmm3 as u64)) as u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1000;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { r8 = P12; rdx = P13; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rdx); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P3, ((rcx as u32)) as u32); }
    rcx = P14;
    return crate::simd_ops::tail_fwd_xmm(P15, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400b99c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143080e88; const P1: u64 = 0x143080e80; const P2: u64 = 0x143080e90; const P3: u64 = 0x143a27058;
    let (mut rax, mut r10, mut r11) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r9 = (crate::simd_ops::ru64(P0)) as u64;
    r11 = (0x9ae16a3b2f90407fu64) as u64;
    rdx = (crate::simd_ops::ru64(P1)) as u64;
    rax = (0xb492b66fbe98f273u64) as u64;
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    r10 = (0x9ae16a3b2f90404fu64) as u64;
    rdx = (((rdx) as i64).wrapping_mul((rax) as i64) as u64) as u64;
    rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64;
    rax = ((r9.wrapping_add(rdx)) as u64) as u64;
    rax = ((rax).rotate_right(43u32)) as u64;
    r8 = ((r9.wrapping_add(r10)) as u64) as u64;
    r8 = ((r8).rotate_right(18u32)) as u64;
    r9 = (((r9) as i64).wrapping_mul((r10) as i64) as u64) as u64;
    r8 = ((r8).wrapping_add(rcx)) as u64;
    rcx = ((rcx).rotate_right(30u32)) as u64;
    rax = ((rax).wrapping_add(rcx)) as u64;
    r8 = ((r8).wrapping_add(rdx)) as u64;
    rax = ((rax).wrapping_add(r9)) as u64;
    rax = (((rax) ^ (r8))) as u64;
    rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64;
    rcx = rax;
    rcx = ((rcx).wrapping_shr(47u32)) as u64;
    rcx = (((rcx) ^ (r8))) as u64;
    rcx = (((rcx) ^ (rax))) as u64;
    rcx = (((rcx) as i64).wrapping_mul((r11) as i64) as u64) as u64;
    rax = rcx;
    rax = ((rax).wrapping_shr(47u32)) as u64;
    rax = (((rax) ^ (rcx))) as u64;
    rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64;
    crate::simd_ops::wu64(P3, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400b9a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143080e68; const P1: u64 = 0x143080e60; const P2: u64 = 0x143080e77; const P3: u64 = 0x143080e6f; const P4: u64 = 0x143a27068;
    let (mut rax, mut r10, mut r11, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rcx = (crate::simd_ops::ru64(P0)) as u64; rbx = (0x9ae16a3b2f90408du64) as u64; r8 = (crate::simd_ops::ru64(P1)) as u64; rax = (0xb492b66fbe98f273u64) as u64; rdx = (crate::simd_ops::ru64(P2)) as u64; r11 = (0x9ae16a3b2f90404fu64) as u64; r8 = (((r8) as i64).wrapping_mul((rax) as i64) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(P3)) as u64; r9 = ((rcx.wrapping_add(r11)) as u64) as u64; r10 = ((rcx.wrapping_add(r8)) as u64) as u64; r9 = ((r9).rotate_right(18u32)) as u64; rax = (((rax) as i64).wrapping_mul((r11) as i64) as u64) as u64; r10 = ((r10).rotate_right(43u32)) as u64; r9 = ((r9).wrapping_add(rdx)) as u64; r9 = ((r9).wrapping_add(r8)) as u64; rdx = ((rdx).rotate_right(30u32)) as u64; r10 = ((r10).wrapping_add(rdx)) as u64; r10 = ((r10).wrapping_add(rax)) as u64; r10 = (((r10) ^ (r9))) as u64; r10 = (((r10) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rcx = r10; rcx = ((rcx).wrapping_shr(47u32)) as u64; rcx = (((rcx) ^ (r9))) as u64; rcx = (((rcx) ^ (r10))) as u64; rcx = (((rcx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rax = rcx; rax = ((rax).wrapping_shr(47u32)) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((rbx) as i64) as u64) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(P4, rax); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400bf550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a28840; const P1: u64 = 0x1401eeda0; const P2: u64 = 0x143a28868; const P3: u64 = 0x143a28869; const P4: u64 = 0x143a28870; const P5: u64 = 0x143a28878; const P6: u64 = 0x1425ec2b0; const P7: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = P0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu8(P2, (0x0u8) as u8);
    crate::simd_ops::wu8(P3, (0x1u8) as u8);
    xmm0 = 0u128;
    crate::simd_ops::wu64(P4, ((xmm0 as u64)) as u64);
    xmm0 = 0u128;
    crate::simd_ops::wu64(P5, ((xmm0 as u64)) as u64);
    rcx = P6;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400c9740(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426bf8; const P1: u64 = 0x143a29170;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128);
    crate::simd_ops::wu32(P1, ((xmm2 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400d1150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143175090; const P1: u64 = 0x1431750a0; const P2: u64 = 0x143175000; const P3: u64 = 0x143175010; const P4: u64 = 0x143175020; const P5: u64 = 0x143175038; const P6: u64 = 0x143175050; const P7: u64 = 0x143175068; const P8: u64 = 0x143175078; const P9: u64 = 0x143a295ec; const P10: u64 = 0x143a295e8; const P11: u64 = 0x1401dec00; const P12: u64 = 0x143a295e0; const P13: u64 = 0x14183acc8; const P14: u64 = 0x1425371a0; const P15: u64 = 0x142533200; const P16: u64 = 0x143a295fc; const P17: u64 = 0x143a295f8; const P18: u64 = 0x1401decf0; const P19: u64 = 0x143a295f0; const P20: u64 = 0x143a29614; const P21: u64 = 0x1401de8d0; const P22: u64 = 0x143a082e0; const P23: u64 = 0x143a29618; const P24: u64 = 0x1425eef90; const P25: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32); rcx = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x3u32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x4u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x5u32) as u32); crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x6u32) as u32); rax = P4; crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax); crate::simd_ops::wu32(r11.wrapping_sub(0x50u64), (0x7u32) as u32); rax = P5; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); crate::simd_ops::wu32(r11.wrapping_sub(0x40u64), (0x8u32) as u32); rax = P6; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax); crate::simd_ops::wu32(r11.wrapping_sub(0x30u64), (0x9u32) as u32); rax = P7; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rax); crate::simd_ops::wu32(r11.wrapping_sub(0x20u64), (0xau32) as u32); rax = P8; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax); r14 = 0xb; crate::simd_ops::wu32(r11.wrapping_sub(0x10u64), ((r14 as u32)) as u32); rax = (crate::simd_ops::ru32(P9)) as u32 as u64; rdi = (crate::simd_ops::ru32(P10)) as u32 as u64; fl_l = rax; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r14; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r14 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P10)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P12)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P12)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P12)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P12, rbx); crate::simd_ops::wu32(P10, ((rdi as u32)) as u32); crate::simd_ops::wu32(P9, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P12)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P16)) as u32 as u64; fl_l = rax; fl_r = (0xbu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0xbu64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r14 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P17)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P17)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P19)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P19)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P19)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P19, rdi); crate::simd_ops::wu32(P17, ((rsi as u32)) as u32); crate::simd_ops::wu32(P16, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P10)) as u32 as u64; rbx = (crate::simd_ops::ru64(P12)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P20, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P9)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rbp = rdx; rbp = ((rbp).wrapping_shr(1u32)) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; fl_l = rbp; fl_r = rax; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbp }) as u64; rdi = rcx; rdx = rbp; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P10)) as u32 as u64; rdx = (crate::simd_ops::ru64(P12)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P12)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P12)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P12, rbx); crate::simd_ops::wu32(P10, ((rdi as u32)) as u32); crate::simd_ops::wu32(P9, ((rbp as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P10, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P19; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P10)) as u32 as u64; rbx = (crate::simd_ops::ru64(P12)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P22)) as u64; crate::simd_ops::wu64(P23, rax); rax = P12; crate::simd_ops::wu64(P22, rax); rcx = P24; r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P25, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400d34c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143177a50; const P1: u64 = 0x143177a5b; const P2: u64 = 0x143177a40; const P3: u64 = 0x143177a4a; const P4: u64 = 0x143177a70; const P5: u64 = 0x143177a7a; const P6: u64 = 0x143177a60; const P7: u64 = 0x143177a6a; const P8: u64 = 0x143177a90; const P9: u64 = 0x143177a9a; const P10: u64 = 0x143177a80; const P11: u64 = 0x143177a8c; const P12: u64 = 0x143a29898; const P13: u64 = 0x143a298a0; const P14: u64 = 0x140283b60; const P15: u64 = 0x14183acc8; const P16: u64 = 0x1425ef550; const P17: u64 = 0x141834d18;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x58u64)) as u64; rdx = 0x811c9dc5; rcx = P0; r8 = ((rdx as u32)) as u32 as u64; r9 = P1; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r10 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r8 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((r10 as u32)) as u32); rcx = P2; r8 = ((rdx as u32)) as u32 as u64; r9 = P3; bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); rcx = P4; r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x4u32) as u32); r9 = P5; bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); rcx = P6; r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x5u32) as u32); r9 = P7; bb = 7; continue; }
            7 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32); rcx = P8; r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x1u32) as u32); r9 = P9; bb = 9; continue; }
            9 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r8 as u32)) as u32); rcx = P10; r8 = P11; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x2u32) as u32); bb = 11; continue; }
            11 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rdx as u32)) as u32); rcx = P12; rdx = 0x6; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x6u32) as u32); crate::simd_ops::wu64(P12, r10); crate::simd_ops::wu64(P13, r10); rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P12)) as u64; rdx = rsp.wrapping_add(0x20); r8 = 0x30; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rcx = P16; crate::simd_ops::wu32(P13, (0x6u32) as u32); rsp = ((rsp).wrapping_add(0x58u64)) as u64; return crate::simd_ops::tail_stk(P17, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400d3690(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143177ab0; const P1: u64 = 0x143177abe; const P2: u64 = 0x143177aa0; const P3: u64 = 0x143177aae; const P4: u64 = 0x14317d790; const P5: u64 = 0x14317d79d; const P6: u64 = 0x14317d770; const P7: u64 = 0x14317d789; const P8: u64 = 0x14317d7c0; const P9: u64 = 0x14317d7d9; const P10: u64 = 0x14317d7a0; const P11: u64 = 0x14317d7ba; const P12: u64 = 0x14317d7f0; const P13: u64 = 0x14317d80a; const P14: u64 = 0x14317d7e0; const P15: u64 = 0x14317d7ec; const P16: u64 = 0x143a298d0; const P17: u64 = 0x143a298d8; const P18: u64 = 0x140283b60; const P19: u64 = 0x14183acc8; const P20: u64 = 0x1425ef560; const P21: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rdx = 0x811c9dc5; rcx = P0; r8 = ((rdx as u32)) as u32 as u64; r9 = P1; bb = 1; continue; }
            1 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r8 as u32)) as u32); rcx = P2; r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), (0x3u32) as u32); r9 = P3; bb = 3; continue; }
            3 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r8 as u32)) as u32); rcx = P4; r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x2cu64), (0x3u32) as u32); r9 = P5; bb = 5; continue; }
            5 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r8 as u32)) as u32); rcx = P6; r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), (0x3u32) as u32); r9 = P7; bb = 7; continue; }
            7 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r8 as u32)) as u32); rcx = P8; r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), (0x3u32) as u32); r9 = P9; bb = 9; continue; }
            9 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((r8 as u32)) as u32); rcx = P10; r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), (0x3u32) as u32); r9 = P11; bb = 11; continue; }
            11 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((r8 as u32)) as u32); rcx = P12; r8 = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x4cu64), (0x3u32) as u32); r9 = P13; bb = 13; continue; }
            13 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; r8 = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((r8 as u32)) as u32); rcx = P14; r8 = P15; crate::simd_ops::wu32(rsp.wrapping_add(0x54u64), (0x3u32) as u32); bb = 15; continue; }
            15 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i32) as u32) as u32 as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = ((((rax as u32)) | (0x20u32))) as u32 as u64; rax = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rax as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 16; continue; }
            16 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), ((rdx as u32)) as u32); rcx = P16; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), ((rax as u32)) as u32); crate::simd_ops::wu64(P16, rax); crate::simd_ops::wu64(P17, rax); rdx = ((rax.wrapping_add(0x8u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P18, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P16)) as u64; rdx = rsp.wrapping_add(0x20); r8 = 0x40; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); rcx = P20; crate::simd_ops::wu32(P17, (0x8u32) as u32); rsp = ((rsp).wrapping_add(0x68u64)) as u64; return crate::simd_ops::tail_stk(P21, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400d64a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426cdcb0; const P1: u64 = 0x142409b50; const P2: u64 = 0x1401df680; const P3: u64 = 0x14183acc8; const P4: u64 = 0x142408f50; const P5: u64 = 0x14183acbc; const P6: u64 = 0x143181870; const P7: u64 = 0x143181860; const P8: u64 = 0x143181828; const P9: u64 = 0x143181810; const P10: u64 = 0x14240aa00; const P11: u64 = 0x1431817f8; const P12: u64 = 0x1431817e8; const P13: u64 = 0x143181850; const P14: u64 = 0x143181840; const P15: u64 = 0x143a29d30; const P16: u64 = 0x143a29d38; const P17: u64 = 0x140267680; const P18: u64 = 0x14050da10; const P19: u64 = 0x140208500; const P20: u64 = 0x1401de3f0; const P21: u64 = 0x141834ee8; const P22: u64 = 0x14240ab10; const P23: u64 = 0x1425efe70; const P24: u64 = 0x141834d18; const P25: u64 = 0x140200130;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 115>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x270u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x370u64)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x148u64), (0x4000004u64) as u64); r15 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_add(0x150u64), ((r15 as u8)) as u8); rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r15) as u64); r9 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x148u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((r15 as u32)) as u32); rax = ((rsp.wrapping_add(0x34u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rax as u32)) as u32); r10 = (r10 & !0xffu64) | (((0x4u8) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x37u64), ((r10 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x3cu64), ((r15 as u8)) as u8); rbx = (crate::simd_ops::ru32(rbp.wrapping_add(0x14cu64))) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x14bu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x148u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdi = ((rbp.wrapping_add(0x150u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x150u64)) } else { rdi }) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rcx = ((rsp.wrapping_add(0x3cu64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x3cu64)) } else { rcx }) as u64; crate::simd_ops::wu8(rcx, ((rbx as u8)) as u8); bb = 21; continue; }
            5 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; r11 = ((rax.wrapping_mul(8)) as u32) as u32 as u64; rax = ((rsp.wrapping_add(0x3cu64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x3cu64))) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rax }) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 6; continue; }
            6 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 9; continue; }
            8 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 9; continue; }
            9 => { rcx = ((rsp.wrapping_add(0x3cu64)) as u64) as u64; fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rcx }) as u64; rax = ((rbx.wrapping_add(rdi)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = ((rsp.wrapping_add(0x34u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; r8 = (rbx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            11 => { rax = ((r11 as u32)) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x34u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x3cu64))) as u64; r10 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x37u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; bb = 13; continue; }
            13 => { rax = (((r10 as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 15; continue; } bb = 14; continue; }
            14 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 16; continue; }
            15 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 16; continue; }
            16 => { r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x3cu64)) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rcx }) as u64; r8 = (rbx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 17; continue; }
            17 => { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x37u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 20; continue; }
            19 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rax = ((rsp.wrapping_add(0x3cu64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x3cu64)) } else { rax }) as u64; crate::simd_ops::wu8(rax.wrapping_add(rbx), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rbx as u32)) as u32); bb = 21; continue; }
            21 => { crate::simd_ops::wu64(rbp.wrapping_add(0x120u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x128u64), (0x0u8) as u8); rax = ((P6) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xcu64) as u64); r9 = (0xcu32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x120u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rsp.wrapping_add(0x5cu64), (0x1u32) as u32); rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); r11 = (r11 & !0xffu64) | (((0x4u8) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x63u64), ((r11 as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), (0x0u8) as u8); rbx = (crate::simd_ops::ru32(rbp.wrapping_add(0x124u64))) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0x123u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 24; continue; }
            23 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x120u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdi = ((rbp.wrapping_add(0x128u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x128u64)) } else { rdi }) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } bb = 25; continue; }
            25 => { crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x68u64)) } else { rcx }) as u64; crate::simd_ops::wu8(rcx, ((rbx as u8)) as u8); bb = 42; continue; }
            26 => { r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; r10 = ((rax.wrapping_mul(8)) as u32) as u32 as u64; rax = ((rsp.wrapping_add(0x68u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rax }) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 32; continue; } bb = 27; continue; }
            27 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 30; continue; }
            29 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 30; continue; }
            30 => { rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rcx }) as u64; rax = ((rdi.wrapping_add(rbx)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 32; continue; } bb = 31; continue; }
            31 => { rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; r8 = (rbx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 38; continue; }
            32 => { rax = ((r10 as u32)) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdx = (rbx) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; r11 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x63u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; bb = 34; continue; }
            34 => { rax = (((r11 as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 36; continue; } bb = 35; continue; }
            35 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 37; continue; }
            36 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 37; continue; }
            37 => { r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rcx }) as u64; r8 = (rbx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 38; continue; }
            38 => { rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x63u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 40; continue; } bb = 39; continue; }
            39 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 41; continue; }
            40 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 41; continue; }
            41 => { rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rax = ((rsp.wrapping_add(0x68u64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x68u64)) } else { rax }) as u64; crate::simd_ops::wu8(rax.wrapping_add(rbx), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((rbx as u32)) as u32); bb = 42; continue; }
            42 => { crate::simd_ops::wu64(rbp.wrapping_add(0xf8u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x100u64), (0x0u8) as u8); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xdu64) as u64); r9 = (0xdu32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xf8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x2u32) as u32); rax = ((rbp.wrapping_sub(0x74u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x74u64), ((rax as u32)) as u32); r11 = (r11 & !0xffu64) | (((0x4u8) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_sub(0x71u64), ((r11 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x6cu64), (0x0u8) as u8); rbx = (crate::simd_ops::ru32(rbp.wrapping_add(0xfcu64))) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0xfbu64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 44; continue; } bb = 43; continue; }
            43 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 45; continue; }
            44 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 45; continue; }
            45 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xf8u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdi = ((rbp.wrapping_add(0x100u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0x100u64)) } else { rdi }) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 47; continue; } bb = 46; continue; }
            46 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rcx = ((rbp.wrapping_sub(0x6cu64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x6cu64)) } else { rcx }) as u64; crate::simd_ops::wu8(rcx, ((rbx as u8)) as u8); bb = 63; continue; }
            47 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; r10 = ((rax.wrapping_mul(8)) as u32) as u32 as u64; rax = ((rbp.wrapping_sub(0x6cu64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x6cu64))) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rax }) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 53; continue; } bb = 48; continue; }
            48 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 50; continue; } bb = 49; continue; }
            49 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 51; continue; }
            50 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 51; continue; }
            51 => { rcx = ((rbp.wrapping_sub(0x6cu64)) as u64) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rcx }) as u64; rax = ((rbx.wrapping_add(rdi)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 53; continue; } bb = 52; continue; }
            52 => { rcx = ((rbp.wrapping_sub(0x74u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; r8 = (rbx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 59; continue; }
            53 => { rax = ((r10 as u32)) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 55; continue; } bb = 54; continue; }
            54 => { rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x74u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x6cu64))) as u64; r11 = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x71u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; bb = 55; continue; }
            55 => { rax = (((r11 as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 57; continue; } bb = 56; continue; }
            56 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 58; continue; }
            57 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 58; continue; }
            58 => { r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x6cu64)) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rcx }) as u64; r8 = (rbx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 59; continue; }
            59 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x71u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 61; continue; } bb = 60; continue; }
            60 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 62; continue; }
            61 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 62; continue; }
            62 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x74u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rax = ((rbp.wrapping_sub(0x6cu64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x6cu64)) } else { rax }) as u64; crate::simd_ops::wu8(rbx.wrapping_add(rax), (0x0u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x70u64), ((rbx as u32)) as u32); bb = 63; continue; }
            63 => { crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0xd8u64), (0x0u8) as u8); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x15u64) as u64); r9 = (0x15u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0xd0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_sub(0x4cu64), (0x3u32) as u32); rax = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), ((rax as u32)) as u32); r11 = (r11 & !0xffu64) | (((0x4u8) as u8 as u64)); crate::simd_ops::wu8(rbp.wrapping_sub(0x45u64), ((r11 as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x44u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x40u64), (0x0u8) as u8); rbx = (crate::simd_ops::ru32(rbp.wrapping_add(0xd4u64))) as u32 as u64; rcx = ((crate::simd_ops::ru8(rbp.wrapping_add(0xd3u64))) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 65; continue; } bb = 64; continue; }
            64 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 66; continue; }
            65 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 66; continue; }
            66 => { rax = (crate::simd_ops::ru32(rbp.wrapping_add(0xd0u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rdi = ((rbp.wrapping_add(0xd8u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_add(0xd8u64)) } else { rdi }) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 68; continue; } bb = 67; continue; }
            67 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x44u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64)) } else { rcx }) as u64; crate::simd_ops::wu8(rcx, ((rbx as u8)) as u8); bb = 84; continue; }
            68 => { r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(24u32)) as u32 as u64; rax = ((r8 as u32)) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; r10 = ((rax.wrapping_mul(8)) as u32) as u32 as u64; rax = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rax }) as u64; fl_l = (rdi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 74; continue; } bb = 69; continue; }
            69 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 71; continue; } bb = 70; continue; }
            70 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 72; continue; }
            71 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 72; continue; }
            72 => { rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rcx }) as u64; rax = ((rbx.wrapping_add(rdi)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 74; continue; } bb = 73; continue; }
            73 => { rcx = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; r8 = (rbx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 80; continue; }
            74 => { rax = ((r10 as u32)) as u32 as u64; fl_l = (rbx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 76; continue; } bb = 75; continue; }
            75 => { rdx = (rbx) as u64; rcx = ((rbp.wrapping_sub(0x48u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; r11 = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x45u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; bb = 76; continue; }
            76 => { rax = (((r11 as u8)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 78; continue; } bb = 77; continue; }
            77 => { rax = ((rax.wrapping_mul(8)) as u32) as u32 as u64; bb = 79; continue; }
            78 => { rax = (((rax as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 79; continue; }
            79 => { r8 = ((((r8 as u32)) & (0xffffffu32))) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { r9 } else { rcx }) as u64; r8 = (rbx) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 80; continue; }
            80 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x45u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 82; continue; } bb = 81; continue; }
            81 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 83; continue; }
            82 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 83; continue; }
            83 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x48u64))) as u32 as u64; rcx = ((((rcx as u32)) & (0xffffffu32))) as u32 as u64; rcx = (((rcx as u32)).wrapping_shl(3u32)) as u32 as u64; rax = ((rbp.wrapping_sub(0x40u64)) as u64) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rax = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64)) } else { rax }) as u64; crate::simd_ops::wu8(rax.wrapping_add(rbx), (0x0u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x44u64), ((rbx as u32)) as u32); bb = 84; continue; }
            84 => { crate::simd_ops::wu64(rbp.wrapping_add(0x210u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x218u64), (0x0u8) as u8); rax = ((P9) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x15u64) as u64); r9 = (0x15u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x210u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_sub(0x20u64), (0x4u32) as u32); rax = ((rbp.wrapping_sub(0x1cu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x1cu64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x1cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x19u64), (0x4u8) as u8); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x14u64), (0x0u8) as u8); rdx = ((rbp.wrapping_add(0x210u64)) as u64) as u64; rcx = ((rbp.wrapping_sub(0x1cu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x1e8u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x1f0u64), (0x0u8) as u8); rax = ((P11) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x16u64) as u64); r9 = (0x16u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x1e8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0xcu64), (0x5u32) as u32); rax = ((rbp.wrapping_add(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x10u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x10u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x13u64), (0x4u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x14u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x18u64), (0x0u8) as u8); rdx = ((rbp.wrapping_add(0x1e8u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x10u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x1c0u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x1c8u64), (0x0u8) as u8); rax = ((P12) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xfu64) as u64); r9 = (0xfu32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x1c0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0x6u32) as u32); rax = ((rbp.wrapping_add(0x3cu64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x3cu64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x3fu64), (0x4u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x40u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x44u64), (0x0u8) as u8); rdx = ((rbp.wrapping_add(0x1c0u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x3cu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x198u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x1a0u64), (0x0u8) as u8); rax = ((P13) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xdu64) as u64); r9 = (0xdu32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x198u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x64u64), (0x7u32) as u32); rax = ((rbp.wrapping_add(0x68u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x68u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x6bu64), (0x4u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x6cu64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x70u64), (0x0u8) as u8); rdx = ((rbp.wrapping_add(0x198u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x170u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x178u64), (0x0u8) as u8); rax = ((P14) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xbu64) as u64); r9 = (0xbu32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x170u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu32(rbp.wrapping_add(0x90u64), (0x8u32) as u32); rax = ((rbp.wrapping_add(0x94u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rax) as u64); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x94u64))) as u32 as u64; rax = ((((rax as u32)) & (0xff000004u32))) as u32 as u64; rax = ((((rax as u32)) | (0x4u32))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x94u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x97u64), (0x4u8) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x98u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x9cu64), (0x0u8) as u8); rdx = ((rbp.wrapping_add(0x170u64)) as u64) as u64; rcx = ((rbp.wrapping_add(0x94u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(P15, (r15) as u64); crate::simd_ops::wu64(P16, (r15) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (rax) as u64; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rax) as u64); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x101u16) as u16); crate::simd_ops::wu64(P15, (rax) as u64); rsi = ((rsp.wrapping_add(0x30u64)) as u64) as u64; r13 = (0x38e38e38e38e38eu64) as u64; r12 = ((P15) as u64) as u64; bb = 85; continue; }
            85 => { r9 = (rsi) as u64; r8 = (r14) as u64; rdx = ((rbp.wrapping_add(0x250u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xc0u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0x248u64), ((xmm0 as u64)) as u64); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x248u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 88; continue; } bb = 86; continue; }
            86 => { fl_l = (crate::simd_ops::ru64(P16)) as u64 as u64; fl_r = (r13) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 90; continue; } bb = 87; continue; }
            87 => { rbx = (crate::simd_ops::ru64(P15)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (r12) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r15) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P17) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (rax) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (rax) as u64); rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x1cu64), ((rax as u32)) as u32); rcx = ((rdi.wrapping_add(0x20u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (rcx) as u64); crate::simd_ops::lk_and_u32(rcx, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rcx, (0x4u32) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x8u64), (0x0u8) as u8); rdx = ((rsi.wrapping_add(0x4u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi, (rbx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rdi.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu16(rdi.wrapping_add(0x18u64), (0x0u16) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r15) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0xc0u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0xc0u64), (xmm0) as u128); r8 = (rdi) as u64; rdx = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; rcx = (r12) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 88; continue; }
            88 => { rsi = ((rsi).wrapping_add(0x2cu64)) as u64; rax = ((rbp.wrapping_add(0xbcu64)) as u64) as u64; fl_l = (rsi) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 85; continue; } bb = 89; continue; }
            89 => { r9 = ((P20) as u64) as u64; rdx = (0x2cu32) as u32 as u64; r8 = ((rdx.wrapping_sub(0x23u64)) as u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P21) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x170u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x198u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x1c0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x1e8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x210u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0xd0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0xf8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x120u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rbp.wrapping_add(0x148u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P22) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P23) as u64) as u64; r11 = ((rsp.wrapping_add(0x370u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P24) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            90 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P25) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400e4bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143195960; const P1: u64 = 0x1431a1d68; const P2: u64 = 0x1431a1d60; const P3: u64 = 0x1431a1d50; const P4: u64 = 0x1431a1d40; const P5: u64 = 0x1431a1d38; const P6: u64 = 0x1431a1d2c; const P7: u64 = 0x1431a1d20; const P8: u64 = 0x1431a1d10; const P9: u64 = 0x1431a1d00; const P10: u64 = 0x143175078; const P11: u64 = 0x1431a1cf0; const P12: u64 = 0x1431a1cd8; const P13: u64 = 0x1431a1cc0; const P14: u64 = 0x1431a1cb0; const P15: u64 = 0x1431a1ca0; const P16: u64 = 0x1431a1c88; const P17: u64 = 0x1431a1c70; const P18: u64 = 0x143a2b15c; const P19: u64 = 0x143a2b158; const P20: u64 = 0x1401dec00; const P21: u64 = 0x143a2b150; const P22: u64 = 0x14183acc8; const P23: u64 = 0x1425371a0; const P24: u64 = 0x142533200; const P25: u64 = 0x143a2b16c; const P26: u64 = 0x143a2b168; const P27: u64 = 0x1401decf0; const P28: u64 = 0x143a2b160; const P29: u64 = 0x143a2b184; const P30: u64 = 0x1401de8d0; const P31: u64 = 0x143a082e0; const P32: u64 = 0x143a2b188; const P33: u64 = 0x1425f1bf0; const P34: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x40); rsp = ((rsp).wrapping_sub(0x140u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0xau32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x3u32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x4u32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x5u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x6u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x7u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x8u32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0x9u32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0x68u32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0x74u32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0x79u32) as u32); rax = P14; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0x69u32) as u32); rax = P15; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), (0x12cu32) as u32); rax = P16; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), (0xcu32) as u32); rax = P17; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0x6bu32) as u32); rax = (crate::simd_ops::ru32(P18)) as u32 as u64; r15 = 0x12; rdi = (crate::simd_ops::ru32(P19)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r15; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P19)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P21)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P21)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P21)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P24, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P21, rbx); crate::simd_ops::wu32(P19, ((rdi as u32)) as u32); crate::simd_ops::wu32(P18, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P21)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P25)) as u32 as u64; fl_l = rax; fl_r = (0x12u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x12u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P26)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P27, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P26)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P28)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P28)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P28)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P24, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P28, rdi); crate::simd_ops::wu32(P26, ((rsi as u32)) as u32); crate::simd_ops::wu32(P25, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P19)) as u32 as u64; rbx = (crate::simd_ops::ru64(P21)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P29, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P18)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rdx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rdi = rcx; rdx = r14; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P19)) as u32 as u64; rdx = (crate::simd_ops::ru64(P21)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P22, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P21)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P21)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P24, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P21, rbx); crate::simd_ops::wu32(P19, ((rdi as u32)) as u32); crate::simd_ops::wu32(P18, ((r14 as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P19, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P28; rax = crate::simd_ops::call_stk(P30, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P19)) as u32 as u64; rbx = (crate::simd_ops::ru64(P21)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P31)) as u64; crate::simd_ops::wu64(P32, rax); rax = P21; crate::simd_ops::wu64(P31, rax); rcx = P33; r11 = rsp.wrapping_add(0x140); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P34, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400e7170(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1431a733c; const P1: u64 = 0x1431a7344; const P2: u64 = 0x1431a7350; const P3: u64 = 0x1431a7360; const P4: u64 = 0x1431a7368; const P5: u64 = 0x1431a7380; const P6: u64 = 0x1431a7390; const P7: u64 = 0x14310de60; const P8: u64 = 0x1431a73a0; const P9: u64 = 0x1431a73a8; const P10: u64 = 0x1431a73b8; const P11: u64 = 0x1431a73c8; const P12: u64 = 0x1431a73d8; const P13: u64 = 0x1431a73e8; const P14: u64 = 0x1431a73f8; const P15: u64 = 0x143a2b57c; const P16: u64 = 0x143a2b578; const P17: u64 = 0x1401dec00; const P18: u64 = 0x143a2b570; const P19: u64 = 0x14183acc8; const P20: u64 = 0x1425371a0; const P21: u64 = 0x142533200; const P22: u64 = 0x143a2b58c; const P23: u64 = 0x143a2b588; const P24: u64 = 0x1401decf0; const P25: u64 = 0x143a2b580; const P26: u64 = 0x143a2b5a4; const P27: u64 = 0x1401de8d0; const P28: u64 = 0x143a082e0; const P29: u64 = 0x143a2b5a8; const P30: u64 = 0x1425f21d0; const P31: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 37>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x10); rsp = ((rsp).wrapping_sub(0x110u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x3u32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x4u32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x5u32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x6u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x7u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x8u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x9u32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0xau32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xbu32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0xcu32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0xdu32) as u32); rax = P14; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0xeu32) as u32); rax = (crate::simd_ops::ru32(P15)) as u32 as u64; r15 = 0xf; rdi = (crate::simd_ops::ru32(P16)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r15; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P17, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P16)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P18)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(P18)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P18, rbx); crate::simd_ops::wu32(P16, ((rdi as u32)) as u32); crate::simd_ops::wu32(P15, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P18)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P22)) as u32 as u64; fl_l = rax; fl_r = (0xfu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0xfu64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P23)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P24, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P23)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P25)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(P25)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P25, rdi); crate::simd_ops::wu32(P23, ((rsi as u32)) as u32); crate::simd_ops::wu32(P22, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P16)) as u32 as u64; rbx = (crate::simd_ops::ru64(P18)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P26, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P15)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rdx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rdi = rcx; rdx = r14; rax = crate::simd_ops::call_stk(P17, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P16)) as u32 as u64; rdx = (crate::simd_ops::ru64(P18)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(P18)) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = r9.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P18, rbx); crate::simd_ops::wu32(P16, ((rdi as u32)) as u32); crate::simd_ops::wu32(P15, ((r14 as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P16, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P25; rax = crate::simd_ops::call_stk(P27, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P16)) as u32 as u64; rbx = (crate::simd_ops::ru64(P18)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P28)) as u64; crate::simd_ops::wu64(P29, rax); rax = P18; crate::simd_ops::wu64(P28, rax); rcx = P30; r11 = rsp.wrapping_add(0x110); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P31, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400e7620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1431a7410; const P1: u64 = 0x1431a7418; const P2: u64 = 0x1431a7428; const P3: u64 = 0x1431427a0; const P4: u64 = 0x1431a7438; const P5: u64 = 0x1431a7448; const P6: u64 = 0x143109c0c; const P7: u64 = 0x143a2b5bc; const P8: u64 = 0x143a2b5b8; const P9: u64 = 0x1401dec00; const P10: u64 = 0x143a2b5b0; const P11: u64 = 0x14183acc8; const P12: u64 = 0x1425371a0; const P13: u64 = 0x142533200; const P14: u64 = 0x143a2b5cc; const P15: u64 = 0x143a2b5c8; const P16: u64 = 0x1401decf0; const P17: u64 = 0x143a2b5c0; const P18: u64 = 0x143a2b5e4; const P19: u64 = 0x1401de8d0; const P20: u64 = 0x143a082e0; const P21: u64 = 0x143a2b5e8; const P22: u64 = 0x1425f2200; const P23: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rax = P0; crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rax = P1; crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = P2; crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rax = P3; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x3u32) as u32); rax = P4; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x4u32) as u32); rax = P5; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x5u32) as u32); rax = P6; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax); crate::simd_ops::wu32(r11.wrapping_sub(0x10u64), (0x6u32) as u32); rax = (crate::simd_ops::ru32(P7)) as u32 as u64; r14 = 0x7; rdi = (crate::simd_ops::ru32(P8)) as u32 as u64; fl_l = rax; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r14; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r14 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P8)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P10)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P10)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P10)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P10, rbx); crate::simd_ops::wu32(P8, ((rdi as u32)) as u32); crate::simd_ops::wu32(P7, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P10)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P14)) as u32 as u64; fl_l = rax; fl_r = (0x7u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x7u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r14 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P15)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P16, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P15)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P17)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P17)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P17)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P17, rdi); crate::simd_ops::wu32(P15, ((rsi as u32)) as u32); crate::simd_ops::wu32(P14, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P8)) as u32 as u64; rbx = (crate::simd_ops::ru64(P10)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P18, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P7)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rbp = rdx; rbp = ((rbp).wrapping_shr(1u32)) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; fl_l = rbp; fl_r = rax; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbp }) as u64; rdi = rcx; rdx = rbp; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P8)) as u32 as u64; rdx = (crate::simd_ops::ru64(P10)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P10)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P10)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P10, rbx); crate::simd_ops::wu32(P8, ((rdi as u32)) as u32); crate::simd_ops::wu32(P7, ((rbp as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P8, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P17; rax = crate::simd_ops::call_stk(P19, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P8)) as u32 as u64; rbx = (crate::simd_ops::ru64(P10)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P20)) as u64; crate::simd_ops::wu64(P21, rax); rax = P10; crate::simd_ops::wu64(P20, rax); rcx = P22; r11 = rsp.wrapping_add(0x90); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P23, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400ea160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14384bd18; const P1: u64 = 0x14384bd1b; const P2: u64 = 0x1431ad0d0; const P3: u64 = 0x142409b50; const P4: u64 = 0x14384bda0; const P5: u64 = 0x14248a640; const P6: u64 = 0x1431ad0f4; const P7: u64 = 0x1413cf570; const P8: u64 = 0x14384bd10; const P9: u64 = 0x8; const P10: u64 = 0x143426e28; const P11: u64 = 0x14384bda8; const P12: u64 = 0x14384bdac; const P13: u64 = 0x1434268d0; const P14: u64 = 0x14384bdb0; const P15: u64 = 0x143426e64; const P16: u64 = 0x14384bdb4; const P17: u64 = 0x14384bdb8; const P18: u64 = 0x1431ad110; const P19: u64 = 0x1425f2700; const P20: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1400ea160, funcinfo_rva: 0x35e9540, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); crate::simd_ops::wu8(P1, (0x10u8) as u8); rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x24u64) as u64); r9 = (0x24u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1400ea1b0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1400ea1bf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P6) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1400ea1f2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P8) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), (rcx) as u64); xmm2 = ((crate::simd_ops::ru32(P10)) as u32) as u128; crate::simd_ops::wu32(P11, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm2 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; crate::simd_ops::wu32(P14, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(P15)) as u32) as u128; crate::simd_ops::wu32(P16, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P17, ((xmm2 as u32)) as u32); rax = ((P18) as u64) as u64; crate::simd_ops::wu64(P8, (rax) as u64); rcx = ((P19) as u64) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400ea270(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14384bdc8; const P1: u64 = 0x14384bdcb; const P2: u64 = 0x1431ad0a8; const P3: u64 = 0x142409b50; const P4: u64 = 0x14384be50; const P5: u64 = 0x14248a640; const P6: u64 = 0x1431ad0cb; const P7: u64 = 0x1413cf570; const P8: u64 = 0x14384bdc0; const P9: u64 = 0x8; const P10: u64 = 0x143426e64; const P11: u64 = 0x14384be58; const P12: u64 = 0x14384be5c; const P13: u64 = 0x1434268d0; const P14: u64 = 0x14384be60; const P15: u64 = 0x143426ea4; const P16: u64 = 0x14384be64; const P17: u64 = 0x14384be68; const P18: u64 = 0x1431ad110; const P19: u64 = 0x1425f2720; const P20: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1400ea270, funcinfo_rva: 0x35e9568, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); crate::simd_ops::wu8(P1, (0x10u8) as u8); rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x23u64) as u64); r9 = (0x23u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1400ea2c0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1400ea2cf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P6) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1400ea302); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P8) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), (rcx) as u64); xmm2 = ((crate::simd_ops::ru32(P10)) as u32) as u128; crate::simd_ops::wu32(P11, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm2 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; crate::simd_ops::wu32(P14, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(P15)) as u32) as u128; crate::simd_ops::wu32(P16, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P17, ((xmm2 as u32)) as u32); rax = ((P18) as u64) as u64; crate::simd_ops::wu64(P8, (rax) as u64); rcx = ((P19) as u64) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400ec790(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14342702c; const P1: u64 = 0x143427080; const P2: u64 = 0x1434268d0; const P3: u64 = 0x143427048; const P4: u64 = 0x143a001ac; const P5: u64 = 0x14384bffc; const P6: u64 = 0x14384c008; const P7: u64 = 0x14384bff8; const P8: u64 = 0x14384c004; const P9: u64 = 0x14384c010; const P10: u64 = 0x14384c01c; const P11: u64 = 0x14384bff0; const P12: u64 = 0x14384c014; const P13: u64 = 0x143a00290; const P14: u64 = 0x14384bfe0; const P15: u64 = 0x1425f2db0; const P16: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm3 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm5 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128;
    rcx = (((crate::simd_ops::ru32(P4)) as i32 as i64) as u64) as u64;
    xmm4 = crate::simd_ops::vunpcklps(xmm1, xmm0);
    xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3);
    xmm2 = 0u128;
    crate::simd_ops::wu64(P5, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vunpcklps(xmm5, xmm5);
    crate::simd_ops::wu64(P6, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(P7, ((xmm2 as u32)) as u32);
    crate::simd_ops::wu32(P8, ((xmm3 as u32)) as u32);
    crate::simd_ops::wu32(P9, ((xmm5 as u32)) as u32);
    crate::simd_ops::wu32(P10, ((xmm2 as u32)) as u32);
    crate::simd_ops::wu64(P11, ((xmm4 as u64)) as u64);
    crate::simd_ops::wu64(P12, ((xmm4 as u64)) as u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1000;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { r8 = P13; rdx = P14; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rdx); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P4, ((rcx as u32)) as u32); }
    rcx = P15;
    return crate::simd_ops::tail_fwd_xmm(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400f36d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x1431ba4ec; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cb45e0; const P4: u64 = 0x14384bd0c; const P5: u64 = 0x143cb45e8; const P6: u64 = 0x140583920; const P7: u64 = 0x143cb4610; const P8: u64 = 0x143cb45f0; const P9: u64 = 0x143cb4600;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xb3bu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400f3760(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x1431bca8c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cb44b8; const P4: u64 = 0x14384bfd8; const P5: u64 = 0x143cb44c0; const P6: u64 = 0x1405839b0; const P7: u64 = 0x143cb44e8; const P8: u64 = 0x143cb44c8; const P9: u64 = 0x143cb44d8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400f3ac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x1431bd17c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cb46c0; const P4: u64 = 0x14384c560; const P5: u64 = 0x143cb46c8; const P6: u64 = 0x140583b60; const P7: u64 = 0x143cb46f0; const P8: u64 = 0x143cb46d0; const P9: u64 = 0x143cb46e0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x743u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1400f41a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426804; const P1: u64 = 0x143a001ac; const P2: u64 = 0x14384c608; const P3: u64 = 0x14384c620; const P4: u64 = 0x143a00290; const P5: u64 = 0x14384c5f8; const P6: u64 = 0x1425f4030; const P7: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    rcx = (((crate::simd_ops::ru32(P1)) as i32 as i64) as u64) as u64;
    xmm0 = 0u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x8u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0xcu64), ((xmm1 as u32)) as u32);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu64(P3, rax);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1000;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { r8 = P4; rdx = P5; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rdx); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P1, ((rcx as u32)) as u32); }
    rcx = P6;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140100b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143427030; const P1: u64 = 0x143427058; const P2: u64 = 0x1434266b8; const P3: u64 = 0x143426600; const P4: u64 = 0x143a001ac; const P5: u64 = 0x14384ca1c; const P6: u64 = 0x14384ca28; const P7: u64 = 0x14384ca18; const P8: u64 = 0x14384ca24; const P9: u64 = 0x14384ca30; const P10: u64 = 0x14384ca3c; const P11: u64 = 0x14384ca10; const P12: u64 = 0x14384ca34; const P13: u64 = 0x143a00290; const P14: u64 = 0x14384ca00; const P15: u64 = 0x1425f5410; const P16: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    xmm5 = ((crate::simd_ops::ru32(P2)) as u32) as u128;
    xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128;
    rcx = (((crate::simd_ops::ru32(P4)) as i32 as i64) as u64) as u64;
    xmm3 = crate::simd_ops::vunpcklps(xmm1, xmm0);
    xmm0 = crate::simd_ops::vunpcklps(xmm4, xmm4);
    xmm2 = 0u128;
    crate::simd_ops::wu64(P5, ((xmm0 as u64)) as u64);
    xmm0 = crate::simd_ops::vunpcklps(xmm5, xmm5);
    crate::simd_ops::wu64(P6, ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(P7, ((xmm2 as u32)) as u32);
    crate::simd_ops::wu32(P8, ((xmm4 as u32)) as u32);
    crate::simd_ops::wu32(P9, ((xmm5 as u32)) as u32);
    crate::simd_ops::wu32(P10, ((xmm2 as u32)) as u32);
    crate::simd_ops::wu64(P11, ((xmm3 as u64)) as u64);
    crate::simd_ops::wu64(P12, ((xmm3 as u64)) as u64);
    fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1000;
    if !(((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32))) { r8 = P13; rdx = P14; crate::simd_ops::wu64(r8.wrapping_add(rcx.wrapping_mul(8)), rdx); fl_l = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rcx = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(P4, ((rcx as u32)) as u32); }
    rcx = P15;
    return crate::simd_ops::tail_fwd_xmm(P16, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140102550(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14384cb78; const P1: u64 = 0x14384cb7b; const P2: u64 = 0x1431d6670; const P3: u64 = 0x142409b50; const P4: u64 = 0x14384cc00; const P5: u64 = 0x14248a640; const P6: u64 = 0x1431d667a; const P7: u64 = 0x1413cf570; const P8: u64 = 0x14384cb70; const P9: u64 = 0x8; const P10: u64 = 0x143426e8c; const P11: u64 = 0x14384cc08; const P12: u64 = 0x14384cc0c; const P13: u64 = 0x1434268d0; const P14: u64 = 0x14384cc10; const P15: u64 = 0x143426ecc; const P16: u64 = 0x14384cc14; const P17: u64 = 0x14384cc18; const P18: u64 = 0x1431ad110; const P19: u64 = 0x1425f55a0; const P20: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140102550, funcinfo_rva: 0x35f1260, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); crate::simd_ops::wu8(P1, (0x10u8) as u8); rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xau64) as u64); r9 = (0xau32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401025a0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401025af); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P6) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401025e2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P8) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), (rcx) as u64); xmm2 = ((crate::simd_ops::ru32(P10)) as u32) as u128; crate::simd_ops::wu32(P11, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm2 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; crate::simd_ops::wu32(P14, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(P15)) as u32) as u128; crate::simd_ops::wu32(P16, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P17, ((xmm2 as u32)) as u32); rax = ((P18) as u64) as u64; crate::simd_ops::wu64(P8, (rax) as u64); rcx = ((P19) as u64) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401078e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a2f3c3; const P1: u64 = 0x143a2f3c4; const P2: u64 = 0x143a2f3c0; const P3: u64 = 0x143a2f3c8; const P4: u64 = 0x143a291e8;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x8u64)) as u64; rcx = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64; r10 = (crate::simd_ops::ru32(P1)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rcx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rcx = (((rcx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x7c00; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(P2)) as u32 as u64; r9 = P3; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; r9 = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(P3) } else { r9 }) as u64; fl_l = r10; fl_r = (0x18u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 13; continue; } bb = 4; continue; }
            4 => { fl_l = r10; fl_r = (0xcu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 5; continue; }
            5 => { fl_l = r10; fl_r = (0x4u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; rdx = ((rax as u32)) as u32 as u64; r8 = ((rax.wrapping_add(0x9u64)) as u32) as u32 as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rcx = ((((rax as u32)) as i32).wrapping_mul((0x3361d2afu32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rax = (((crate::simd_ops::ru8(r9)) as i8 as i32) as u32) as u32 as u64; r9 = r9.wrapping_add(0x1); rax = (((rax as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rcx = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; fl_l = rcx; fl_r = r10; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { rcx = ((((r10 as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).rotate_right(17u32)) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rdx as u32)))) as u32 as u64; r8 = (((r8 as u32)).rotate_right(19u32)) as u32 as u64; rcx = (((rcx as u32)).rotate_right(17u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; r8 = ((r8.wrapping_add(r8.wrapping_mul(4))) as u32) as u32 as u64; fl_l = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 10; continue; }
            10 => { rax = (((rax as u32)).rotate_right(19u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(13u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(P4, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            11 => { rcx = (crate::simd_ops::ru32(r9)) as u32 as u64; r8 = ((r10.wrapping_add(r10.wrapping_mul(4))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((r10 as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rax = (crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_sub(0x4u64))) as u32 as u64; rax = (((rax as u32)).wrapping_add((r8 as u32))) as u32 as u64; r10 = ((r10).wrapping_shr(1u32)) as u64; r10 = ((((r10 as u32)) & (0x4u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((r8 as u32)))) as u32 as u64; rcx = (((rcx as u32)).rotate_right(19u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(r9))) as i32).wrapping_mul((0x3361d2afu32) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).rotate_right(17u32)) as u32 as u64; r8 = ((((rcx as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rcx = 0x318f97d9; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rcx = (((rcx as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rdx as u32)))) as u32 as u64; r8 = (((r8 as u32)).rotate_right(19u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; r8 = ((r8.wrapping_add(r8.wrapping_mul(4))) as u32) as u32 as u64; rax = ((((rax as u32)) ^ ((r8 as u32)))) as u32 as u64; rax = (((rax as u32)).rotate_right(19u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0x85ebca6bu32) as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(13u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0xc2b2ae35u32) as i32) as u32) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(16u32)) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; crate::simd_ops::wu32(P4, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            12 => { r8 = r10; r8 = ((r8).wrapping_shr(1u32)) as u64; rax = (((crate::simd_ops::ru32(r8.wrapping_add(r9).wrapping_sub(0x4u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(r9.wrapping_add(0x4u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) ^ ((r10 as u32)))) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).rotate_right(19u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_sub(0x8u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r8.wrapping_add(r9))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).rotate_right(19u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r9)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rcx = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_sub(0x4u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rcx = (((rcx as u32)).rotate_right(19u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; fl_l = ((((rax as u32)) ^ ((rdx as u32)))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 10; continue; }
            13 => { rax = (((crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_sub(0x4u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; r11 = r9.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); r8 = ((((r10 as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_sub(0x10u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; crate::simd_ops::wu64(rsp, rdi); rcx = ((((rcx as u32)) ^ ((r10 as u32)))) as u32 as u64; rcx = (((rcx as u32)).rotate_right(19u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_sub(0x8u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rsi = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_sub(0xcu64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) ^ ((r8 as u32)))) as u32 as u64; rcx = (((rcx as u32)).rotate_right(19u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rdx = ((((rdx as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(r9).wrapping_sub(0x14u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rbp = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u32) as u32 as u64; rax = (0xcccccccccccccccdu64) as u64; rcx = (((rcx as u32)).wrapping_add((r8 as u32))) as u32 as u64; rcx = (((rcx as u32)).rotate_right(19u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rbx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rcx = r10.wrapping_sub(0x1); let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; rdi = rdx; fl_l = ((rdi).wrapping_shr(4u32)) as u64; rdi = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rax = (((crate::simd_ops::ru32(r11.wrapping_sub(0x10u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; r8 = (crate::simd_ops::ru32(r11.wrapping_sub(0xcu64))) as u32 as u64; r11 = r11.wrapping_add(0x14); rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; r9 = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rax = ((r8.wrapping_add(rbx)) as u32) as u32 as u64; rax = (((rax as u32)).rotate_right(19u32)) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x3361d2afu32) as i32) as u32) as u32 as u64; rax = (((crate::simd_ops::ru32(r11.wrapping_sub(0x18u64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rdx = ((r9 as u32)) as u32 as u64; r9 = ((((r9 as u32)) ^ ((rsi as u32)))) as u32 as u64; r9 = (((r9 as u32)).rotate_right(18u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rsi = ((rdx as u32)) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; rax = ((r9.wrapping_sub(0x52250ecu64)) as u32) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_add((rcx as u32))) as u32 as u64; r8 = ((((r8 as u32)) ^ ((rax as u32)))) as u32 as u64; rax = (((crate::simd_ops::ru32(r11.wrapping_sub(0x1cu64))) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; r8 = (((r8 as u32)).rotate_right(19u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_add(crate::simd_ops::ru32(r11.wrapping_sub(0x14u64)))) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul((0x1b873593u32) as i32) as u32) as u32 as u64; r8 = ((r8.wrapping_add(r8.wrapping_mul(4))) as u32) as u32 as u64; r8 = (((r8 as u32)).swap_bytes()) as u32 as u64; rax = (((rax as u32)).wrapping_add((rbp as u32))) as u32 as u64; rbp = ((r8 as u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(18u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xfaddaf14u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rax = ((((rax as u32)) ^ (crate::simd_ops::ru32(r11.wrapping_sub(0x14u64))))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rsp)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x18u64))) as u64; rbx = (((rbx as u32)).rotate_right(11u32)) as u32 as u64; r8 = (((r8 as u32)).rotate_right(11u32)) as u32 as u64; rax = ((((r8 as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0x3361d2afu32) as i32) as u32) as u32 as u64; rax = ((((rbx as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rdx = (((rdx as u32)).wrapping_sub((rcx as u32))) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rdx = (((rdx as u32)).rotate_right(19u32)) as u32 as u64; rcx = ((rdx.wrapping_sub(0x52250ecu64)) as u32) as u32 as u64; rcx = ((rcx.wrapping_add(rcx.wrapping_mul(4))) as u32) as u32 as u64; rcx = (((rcx as u32)).rotate_right(17u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; rax = ((((rcx as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).rotate_right(19u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_add(0xe6546b64u32)) as u32 as u64; rax = (((rax as u32)).rotate_right(17u32)) as u32 as u64; rcx = ((((rax as u32)) as i32).wrapping_mul((0xcc9e2d51u32) as i32) as u32) as u32 as u64; crate::simd_ops::wu32(P4, ((rcx as u32)) as u32); rsp = ((rsp).wrapping_add(0x8u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1401088d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1431e8090; const P1: u64 = 0x142409b50; const P2: u64 = 0x143a2f480; const P3: u64 = 0x143a2f483; const P4: u64 = 0x143a2f484; const P5: u64 = 0x143a2f488; const P6: u64 = 0x143a2f4c8; const P7: u64 = 0x143a2f4e8; const P8: u64 = 0x143a2f4d8; const P9: u64 = 0x143a2f4d0; const P10: u64 = 0x143a2f4d4; const P11: u64 = 0x14052f030; const P12: u64 = 0x14240ab10; const P13: u64 = 0x1425f6960; const P14: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x1401088d0, funcinfo_rva: 0x35f2d7c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xeu64) as u64); r9 = (0xeu32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14010890c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x7c00u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = ((rsp.wrapping_add(0x38u64)) as u64) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rcx }) as u64; r9 = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rbx) as u64); rax = (crate::simd_ops::ru32(P2)) as u32 as u64; rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64; rax = ((((rax as u32)) | (0x8u32))) as u32 as u64; crate::simd_ops::wu32(P2, ((rax as u32)) as u32); crate::simd_ops::wu8(P3, (0x8u8) as u8); crate::simd_ops::wu32(P4, (0x0u32) as u32); crate::simd_ops::wu8(P5, (0x0u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r9) as u64); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14010899c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = P6; core::ptr::write_unaligned(y_addr as *mut u128, xmm0); core::ptr::write_unaligned(y_addr.wrapping_add(16) as *mut u128, ymm0_hi); crate::simd_ops::wu128(P7, (xmm0) as u128); rax = ((P8) as u64) as u64; crate::simd_ops::wu64(P6, (rax) as u64); crate::simd_ops::wu32(P9, (0x0u32) as u32); crate::simd_ops::wu32(P10, (0x4u32) as u32); rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401089de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x1401089e9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P12) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P13) as u64) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P14) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140111d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1431f5468; const P1: u64 = 0x142409b50; const P2: u64 = 0x1431f5490; const P3: u64 = 0x1431f54b8; const P4: u64 = 0x1431f54f8; const P5: u64 = 0x143a31110; const P6: u64 = 0x140425090; const P7: u64 = 0x1401de3c0; const P8: u64 = 0x141834ee8; const P9: u64 = 0x1425f7d00; const P10: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 43>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x50); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x20u64) as u64); r9 = 0x20; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x80u64), (0x0u8) as u8); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x20u64) as u64); r9 = 0x20; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x78); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x38u64), (0x0u8) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x20u64) as u64); r9 = 0x20; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_sub(0x40); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x10u64), (0x0u8) as u8); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x20u64) as u64); r9 = 0x20; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rax = rbp.wrapping_add(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdx = rsp.wrapping_add(0x20); rcx = P5; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); r9 = P7; rdx = 0x48; r8 = ((rdx.wrapping_sub(0x44u64)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = P9; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x160u64))) as u64; rsp = ((rsp).wrapping_add(0x150u64)) as u64; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140118080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307aafc; const P1: u64 = 0x143200fad; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cbcae0; const P4: u64 = 0x14384cf68; const P5: u64 = 0x143cbcae8; const P6: u64 = 0x1406123d0; const P7: u64 = 0x143cbcb10; const P8: u64 = 0x143cbcaf0; const P9: u64 = 0x143cbcb00;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1bu64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x2fu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140118200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x143200a7c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cbccd8; const P4: u64 = 0x14384c564; const P5: u64 = 0x143cbcce0; const P6: u64 = 0x140612c40; const P7: u64 = 0x143cbcd08; const P8: u64 = 0x143cbcce8; const P9: u64 = 0x143cbccf8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x438u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140118290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x14320090c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cbcbe8; const P4: u64 = 0x14384dc70; const P5: u64 = 0x143cbcbf0; const P6: u64 = 0x140612cd0; const P7: u64 = 0x143cbcc18; const P8: u64 = 0x143cbcbf8; const P9: u64 = 0x143cbcc08;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x103u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140118320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x14320104c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cbcb20; const P4: u64 = 0x14384cf6c; const P5: u64 = 0x143cbcb28; const P6: u64 = 0x140612d60; const P7: u64 = 0x143cbcb50; const P8: u64 = 0x143cbcb30; const P9: u64 = 0x143cbcb40;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x15au64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14011c6c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x14320c15c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cbe3c8; const P4: u64 = 0x14384df10; const P5: u64 = 0x143cbe3d0; const P6: u64 = 0x140623a10; const P7: u64 = 0x143cbe3f8; const P8: u64 = 0x143cbe3d8; const P9: u64 = 0x143cbe3e8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x3dfu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14011c750(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x14320ca9c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cbe2c8; const P4: u64 = 0x14384dc74; const P5: u64 = 0x143cbe2d0; const P6: u64 = 0x140623aa0; const P7: u64 = 0x143cbe2f8; const P8: u64 = 0x143cbe2d8; const P9: u64 = 0x143cbe2e8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x105u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14011c7e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x14320c8cc; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cbe390; const P4: u64 = 0x14384deec; const P5: u64 = 0x143cbe398; const P6: u64 = 0x140623b30; const P7: u64 = 0x143cbe3c0; const P8: u64 = 0x143cbe3a0; const P9: u64 = 0x143cbe3b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x15eu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14012f840(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a3a1a0; const P1: u64 = 0x1425fc670; const P2: u64 = 0x143a3a1e8; const P3: u64 = 0x143a3a1f0; const P4: u64 = 0x143a3a1f4; const P5: u64 = 0x143a3a1f8; const P6: u64 = 0x143a3a1a3; const P7: u64 = 0x143a3a208; const P8: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rcx = P1;
    xmm0 = 0u128; ymm0_hi = 0;
    let y_addr: u64 = P2; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64;
    crate::simd_ops::wu32(P3, (0x0u32) as u32);
    fl_l = ((((rax as u32)) | (0x8u32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(P4, (0x4u32) as u32);
    crate::simd_ops::wu32(P0, ((rax as u32)) as u32);
    rax = P5;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu8(P6, (0x8u8) as u8);
    crate::simd_ops::wu128(P7, (xmm0) as u128);
    return crate::simd_ops::tail_fwd_xmm(P8, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140134eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143101424; const P1: u64 = 0x142409b50; const P2: u64 = 0x143108a64; const P3: u64 = 0x1432b16c0; const P4: u64 = 0x1432b16a8; const P5: u64 = 0x1432b15a8; const P6: u64 = 0x1432b1598; const P7: u64 = 0x1432b1588; const P8: u64 = 0x1432b1580; const P9: u64 = 0x1432b15f0; const P10: u64 = 0x1432b15d8; const P11: u64 = 0x1432b15c8; const P12: u64 = 0x1432b15b8; const P13: u64 = 0x1432b1648; const P14: u64 = 0x1432b1630; const P15: u64 = 0x1432b1618; const P16: u64 = 0x1432b1600; const P17: u64 = 0x1432b1698; const P18: u64 = 0x1432b1678; const P19: u64 = 0x1430896d8; const P20: u64 = 0x143089770; const P21: u64 = 0x143089780; const P22: u64 = 0x143085420; const P23: u64 = 0x1432b1668; const P24: u64 = 0x143085430; const P25: u64 = 0x143108a58; const P26: u64 = 0x143a3e4c0; const P27: u64 = 0x143a3e8a8; const P28: u64 = 0x14240aa00; const P29: u64 = 0x1401de3c0; const P30: u64 = 0x141834ee8; const P31: u64 = 0x14260bde0; const P32: u64 = 0x141834d18;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 133>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x320); rsp = ((rsp).wrapping_sub(0x420u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x4000004u64) as u64); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rsi as u8)) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x6u64) as u64); r9 = ((rsi.wrapping_add(0x6u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), ((rsi as u8)) as u8); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x4u64) as u64); r9 = ((rsi.wrapping_add(0x4u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x78u64), ((rsi as u8)) as u8); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xcu64) as u64); r9 = ((rsi.wrapping_add(0xcu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_sub(0x80); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x50u64), ((rsi as u8)) as u8); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x12u64) as u64); r9 = ((rsi.wrapping_add(0x12u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_sub(0x58); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x28u64), ((rsi as u8)) as u8); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xbu64) as u64); r9 = ((rsi.wrapping_add(0xbu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_sub(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp, ((rsi as u8)) as u8); rax = P6; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xau64) as u64); r9 = ((rsi.wrapping_add(0xau64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_sub(0x8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x28u64), ((rsi as u8)) as u8); rax = P7; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xeu64) as u64); r9 = ((rsi.wrapping_add(0xeu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rsi as u8)) as u8); rax = P8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x7u64) as u64); r9 = ((rsi.wrapping_add(0x7u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x48); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x78u64), ((rsi as u8)) as u8); rax = P9; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xfu64) as u64); r9 = ((rsi.wrapping_add(0xfu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x70); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x98u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0xa0u64), ((rsi as u8)) as u8); rax = P10; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); r9 = ((rsi.wrapping_add(0x10u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x98); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0xc8u64), ((rsi as u8)) as u8); rax = P11; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xau64) as u64); r9 = ((rsi.wrapping_add(0xau64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0xc0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0xe8u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0xf0u64), ((rsi as u8)) as u8); rax = P12; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xau64) as u64); r9 = ((rsi.wrapping_add(0xau64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0xe8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x110u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x118u64), ((rsi as u8)) as u8); rax = P13; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xcu64) as u64); r9 = ((rsi.wrapping_add(0xcu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x110); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x138u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x140u64), ((rsi as u8)) as u8); rax = P14; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x16u64) as u64); r9 = ((rsi.wrapping_add(0x16u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x138); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x160u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x168u64), ((rsi as u8)) as u8); rax = P15; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x15u64) as u64); r9 = ((rsi.wrapping_add(0x15u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x160); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x188u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x190u64), ((rsi as u8)) as u8); rax = P16; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x13u64) as u64); r9 = ((rsi.wrapping_add(0x13u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x188); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x1b0u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x1b8u64), ((rsi as u8)) as u8); rax = P17; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xdu64) as u64); r9 = ((rsi.wrapping_add(0xdu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x1b0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x1d8u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x1e0u64), ((rsi as u8)) as u8); rax = P18; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x19u64) as u64); r9 = ((rsi.wrapping_add(0x19u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x1d8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x200u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x208u64), ((rsi as u8)) as u8); rax = P19; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xdu64) as u64); r9 = ((rsi.wrapping_add(0xdu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x200); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x228u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x230u64), ((rsi as u8)) as u8); rax = P20; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xau64) as u64); r9 = ((rsi.wrapping_add(0xau64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x228); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x258u64), ((rsi as u8)) as u8); rax = P21; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x11u64) as u64); r9 = ((rsi.wrapping_add(0x11u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x250); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x278u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x280u64), ((rsi as u8)) as u8); rax = P22; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xdu64) as u64); r9 = ((rsi.wrapping_add(0xdu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x278); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x2a8u64), ((rsi as u8)) as u8); rax = P23; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xeu64) as u64); r9 = ((rsi.wrapping_add(0xeu64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x2a0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x2c8u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x2d0u64), ((rsi as u8)) as u8); rax = P24; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x12u64) as u64); r9 = ((rsi.wrapping_add(0x12u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x2c8); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x2f0u64), (0x4000004u64) as u64); crate::simd_ops::wu8(rbp.wrapping_add(0x2f8u64), ((rsi as u8)) as u8); rax = P25; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x9u64) as u64); r9 = ((rsi.wrapping_add(0x9u64)) as u32) as u32 as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rbp.wrapping_add(0x2f0); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P26; crate::simd_ops::wu64(P27, rsi); rbx = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rdi = rax; bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbp.wrapping_add(0x330u64), rax); crate::simd_ops::lk_and_u32(rax, (0xff000004u32) as u32); crate::simd_ops::lk_or_u32(rax, (0x4u32) as u32); crate::simd_ops::wu8(rax.wrapping_add(0x3u64), (0x4u8) as u8); crate::simd_ops::wu32(rax.wrapping_add(0x4u64), ((rsi as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(0x8u64), (0x0u8) as u8); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk(P28, rax, rcx, rdx, r8, r9, rsp); rax = rdi.wrapping_add(0x28); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rbx = ((rbx).wrapping_add(0x28u64)) as u64; rdi = rax; rcx = rbp.wrapping_add(0x318); fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(P27, (0x19u64) as u64); r9 = P29; rdx = 0x28; r8 = ((rdx.wrapping_sub(0xfu64)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P30, rax, rcx, rdx, r8, r9, rsp); rcx = P31; r11 = rsp.wrapping_add(0x420); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P32, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14013fa40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307aafc; const P1: u64 = 0x143310c4d; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cc5d98; const P4: u64 = 0x14384e7d8; const P5: u64 = 0x143cc5da0; const P6: u64 = 0x140f53340; const P7: u64 = 0x143cc5dc8; const P8: u64 = 0x143cc5da8; const P9: u64 = 0x143cc5db8;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1bu64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x30u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14013ff80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x14331021c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cc5dd0; const P4: u64 = 0x14384df14; const P5: u64 = 0x143cc5dd8; const P6: u64 = 0x140f550b0; const P7: u64 = 0x143cc5e00; const P8: u64 = 0x143cc5de0; const P9: u64 = 0x143cc5df0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x176u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140142e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x1433138bc; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cc6390; const P4: u64 = 0x14384e838; const P5: u64 = 0x143cc6398; const P6: u64 = 0x140f5f130; const P7: u64 = 0x143cc63c0; const P8: u64 = 0x143cc63a0; const P9: u64 = 0x143cc63b0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x161u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140151080(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307aafc; const P1: u64 = 0x14332b63d; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cca650; const P4: u64 = 0x14384e83c; const P5: u64 = 0x143cca658; const P6: u64 = 0x140fa2b90; const P7: u64 = 0x143cca680; const P8: u64 = 0x143cca660; const P9: u64 = 0x143cca670;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1bu64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1eu64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140151200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x14332b4dc; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cca6f0; const P4: u64 = 0x14384e888; const P5: u64 = 0x143cca6f8; const P6: u64 = 0x140fa3970; const P7: u64 = 0x143cca720; const P8: u64 = 0x143cca700; const P9: u64 = 0x143cca710;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xf2u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016a5c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336a940; const P1: u64 = 0x14336a9a8; const P2: u64 = 0x143a46620; const P3: u64 = 0x14336a988; const P4: u64 = 0x1401de5b0; const P5: u64 = 0x142610ee0; const P6: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x4u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x8u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = P5;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016a7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336aa90; const P1: u64 = 0x143a467a0; const P2: u64 = 0x1403f9250; const P3: u64 = 0x142611000; const P4: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rcx = P1;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = P3;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016a910(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336abe4; const P1: u64 = 0x14336abd0; const P2: u64 = 0x143a468a0; const P3: u64 = 0x14336ac00; const P4: u64 = 0x14336abf0; const P5: u64 = 0x1403f8780; const P6: u64 = 0x1426110c0; const P7: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x3u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x4u32) as u32);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = P6;
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016aa00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336afb4; const P1: u64 = 0x14336adb0; const P2: u64 = 0x14336adcc; const P3: u64 = 0x14336adc0; const P4: u64 = 0x143314cdc; const P5: u64 = 0x143314cb4; const P6: u64 = 0x143314cc0; const P7: u64 = 0x143243034; const P8: u64 = 0x143a4692c; const P9: u64 = 0x143a46928; const P10: u64 = 0x1401dec00; const P11: u64 = 0x143a46920; const P12: u64 = 0x14183acc8; const P13: u64 = 0x1425371a0; const P14: u64 = 0x142533200; const P15: u64 = 0x143a4693c; const P16: u64 = 0x143a46938; const P17: u64 = 0x1401decf0; const P18: u64 = 0x143a46930; const P19: u64 = 0x143a46954; const P20: u64 = 0x1401de8d0; const P21: u64 = 0x143a082e0; const P22: u64 = 0x143a46958; const P23: u64 = 0x142611120; const P24: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rax = P1; crate::simd_ops::wu64(r11.wrapping_sub(0x78u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = P2; crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rax = P3; crate::simd_ops::wu64(r11.wrapping_sub(0x58u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x3u32) as u32); rax = P4; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x4u32) as u32); rax = P5; crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x5u32) as u32); rax = P6; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rax); crate::simd_ops::wu32(r11.wrapping_sub(0x20u64), (0x6u32) as u32); rax = P7; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rax); crate::simd_ops::wu32(r11.wrapping_sub(0x10u64), (0x7u32) as u32); rax = (crate::simd_ops::ru32(P8)) as u32 as u64; r14 = 0x8; rdi = (crate::simd_ops::ru32(P9)) as u32 as u64; fl_l = rax; fl_r = r14; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r14; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r14 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P9)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P11)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P11)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P11)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P11, rbx); crate::simd_ops::wu32(P9, ((rdi as u32)) as u32); crate::simd_ops::wu32(P8, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P11)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P15)) as u32 as u64; fl_l = rax; fl_r = (0x8u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x8u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r14 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P16)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P17, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P16)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P18)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P18)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P18)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P18, rdi); crate::simd_ops::wu32(P16, ((rsi as u32)) as u32); crate::simd_ops::wu32(P15, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P9)) as u32 as u64; rbx = (crate::simd_ops::ru64(P11)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P19, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P8)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rbp = rdx; rbp = ((rbp).wrapping_shr(1u32)) as u64; rbp = ((rbp).wrapping_add(rdx)) as u64; fl_l = rbp; fl_r = rax; rbp = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rbp }) as u64; rdi = rcx; rdx = rbp; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P9)) as u32 as u64; rdx = (crate::simd_ops::ru64(P11)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P11)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P11)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P11, rbx); crate::simd_ops::wu32(P9, ((rdi as u32)) as u32); crate::simd_ops::wu32(P8, ((rbp as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P9, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P18; rax = crate::simd_ops::call_stk(P20, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r14; fl_r = (0x1u64) as u64; r14 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P9)) as u32 as u64; rbx = (crate::simd_ops::ru64(P11)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P21)) as u64; crate::simd_ops::wu64(P22, rax); rax = P11; crate::simd_ops::wu64(P21, rax); rcx = P23; r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P24, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016bfa0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336cb4c; const P1: u64 = 0x14336cb38; const P2: u64 = 0x143a46a60; const P3: u64 = 0x1431a733c; const P4: u64 = 0x14336cb68; const P5: u64 = 0x1403f8780; const P6: u64 = 0x1426114a0; const P7: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x19u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x21u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x17u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x1bu32) as u32);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = P6;
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016d2c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336eb14; const P1: u64 = 0x14336eb94; const P2: u64 = 0x14309f914; const P3: u64 = 0x14309f90c; const P4: u64 = 0x14336eb90; const P5: u64 = 0x14336eba0; const P6: u64 = 0x14336eb9c; const P7: u64 = 0x1431998e8; const P8: u64 = 0x143314ce4; const P9: u64 = 0x14336ebb8; const P10: u64 = 0x143314ce8; const P11: u64 = 0x14336eba8; const P12: u64 = 0x14336ebd0; const P13: u64 = 0x14336ebc0; const P14: u64 = 0x14336eb28; const P15: u64 = 0x14336eb18; const P16: u64 = 0x14336eb40; const P17: u64 = 0x1426cf72c; const P18: u64 = 0x14336eb38; const P19: u64 = 0x143a46c9c; const P20: u64 = 0x143a46c98; const P21: u64 = 0x1401dec00; const P22: u64 = 0x143a46c90; const P23: u64 = 0x14183acc8; const P24: u64 = 0x1425371a0; const P25: u64 = 0x142533200; const P26: u64 = 0x143a46cac; const P27: u64 = 0x143a46ca8; const P28: u64 = 0x1401decf0; const P29: u64 = 0x143a46ca0; const P30: u64 = 0x143a46cc4; const P31: u64 = 0x1401de8d0; const P32: u64 = 0x143a082e0; const P33: u64 = 0x143a46cc8; const P34: u64 = 0x142611580; const P35: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 45>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x50); rsp = ((rsp).wrapping_sub(0x150u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x9bu32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x9cu32) as u32); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x9du32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x9eu32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x9fu32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x9fu32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0xa0u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0xa0u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0xa1u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0xa1u32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0xa2u32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xa4u32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0xa7u32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0xa3u32) as u32); rax = P14; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0xa6u32) as u32); rax = P15; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), (0xa5u32) as u32); rax = P16; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), (0xa8u32) as u32); rax = P17; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0xb0u32) as u32); rax = P18; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), (0xadu32) as u32); rax = (crate::simd_ops::ru32(P19)) as u32 as u64; r15 = 0x13; rdi = (crate::simd_ops::ru32(P20)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r15; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P20)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P22)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P22)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P22)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P24, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P25, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P22, rbx); crate::simd_ops::wu32(P20, ((rdi as u32)) as u32); crate::simd_ops::wu32(P19, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P22)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P26)) as u32 as u64; fl_l = rax; fl_r = (0x13u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x13u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P27)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P28, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P27)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P29)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P29)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P29)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P24, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P25, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P29, rdi); crate::simd_ops::wu32(P27, ((rsi as u32)) as u32); crate::simd_ops::wu32(P26, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P20)) as u32 as u64; rbx = (crate::simd_ops::ru64(P22)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P30, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P19)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rdx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rdi = rcx; rdx = r14; rax = crate::simd_ops::call_stk(P21, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P20)) as u32 as u64; rdx = (crate::simd_ops::ru64(P22)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P23, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P22)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P22)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P24, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P25, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P22, rbx); crate::simd_ops::wu32(P20, ((rdi as u32)) as u32); crate::simd_ops::wu32(P19, ((r14 as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P20, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P29; rax = crate::simd_ops::call_stk(P31, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P20)) as u32 as u64; rbx = (crate::simd_ops::ru64(P22)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P32)) as u64; crate::simd_ops::wu64(P33, rax); rax = P22; crate::simd_ops::wu64(P32, rax); rcx = P34; r11 = rsp.wrapping_add(0x150); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P35, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016d7d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336eb60; const P1: u64 = 0x14336eb50; const P2: u64 = 0x143a46c50; const P3: u64 = 0x14336eb80; const P4: u64 = 0x14336eb70; const P5: u64 = 0x1403f8780; const P6: u64 = 0x1426115b0; const P7: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x68u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x96u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x97u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x99u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x9au32) as u32);
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rcx = P6;
    rsp = ((rsp).wrapping_add(0x68u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016d850(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336ec10; const P1: u64 = 0x14336ec08; const P2: u64 = 0x143a46d10; const P3: u64 = 0x14336ec18; const P4: u64 = 0x1401de5b0; const P5: u64 = 0x1426115e0; const P6: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x3u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = P5;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016d8c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336ec30; const P1: u64 = 0x14336ebe8; const P2: u64 = 0x14336ebe0; const P3: u64 = 0x14336ebf0; const P4: u64 = 0x14309f914; const P5: u64 = 0x14309f90c; const P6: u64 = 0x14336eb14; const P7: u64 = 0x14336eb94; const P8: u64 = 0x14336eba0; const P9: u64 = 0x1431998e8; const P10: u64 = 0x14336ebb8; const P11: u64 = 0x14336eb90; const P12: u64 = 0x14336eb9c; const P13: u64 = 0x14336ebec; const P14: u64 = 0x14336ebfc; const P15: u64 = 0x14336ebf8; const P16: u64 = 0x14336ec04; const P17: u64 = 0x14336ec00; const P18: u64 = 0x14336ec80; const P19: u64 = 0x14336ec7c; const P20: u64 = 0x14336ec88; const P21: u64 = 0x14336ec84; const P22: u64 = 0x14336ec90; const P23: u64 = 0x14336ec8c; const P24: u64 = 0x14336ec98; const P25: u64 = 0x14336ec94; const P26: u64 = 0x14336ec50; const P27: u64 = 0x14336ec4c; const P28: u64 = 0x14336ec58; const P29: u64 = 0x14336ec54; const P30: u64 = 0x14336ec60; const P31: u64 = 0x14336ec5c; const P32: u64 = 0x1433653c0; const P33: u64 = 0x143314cc8; const P34: u64 = 0x143314ce4; const P35: u64 = 0x143314ce8; const P36: u64 = 0x143314cbc; const P37: u64 = 0x14336ec70; const P38: u64 = 0x143a46d5c; const P39: u64 = 0x143a46d58; const P40: u64 = 0x1401dec00; const P41: u64 = 0x143a46d50; const P42: u64 = 0x14183acc8; const P43: u64 = 0x1425371a0; const P44: u64 = 0x142533200; const P45: u64 = 0x143a46d6c; const P46: u64 = 0x143a46d68; const P47: u64 = 0x1401decf0; const P48: u64 = 0x143a46d60; const P49: u64 = 0x143a46d84; const P50: u64 = 0x1401de8d0; const P51: u64 = 0x143a082e0; const P52: u64 = 0x143a46d88; const P53: u64 = 0x142611610; const P54: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 83>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x180); rsp = ((rsp).wrapping_sub(0x280u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x6u32) as u32); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x7u32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x3u32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0xcu32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0xeu32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0xdu32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0xfu32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x3u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x1u32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0x22u32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0x1eu32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0x1fu32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0x20u32) as u32); rax = P14; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0x21u32) as u32); rax = P15; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), (0x22u32) as u32); rax = P16; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), (0x23u32) as u32); rax = P17; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0x24u32) as u32); rax = P18; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), (0x25u32) as u32); rax = P19; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); r15 = 0x26; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((r15 as u32)) as u32); rax = P20; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0x27u32) as u32); rax = P21; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), (0x28u32) as u32); rax = P22; crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), (0x29u32) as u32); rax = P23; crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x98u64), (0x2au32) as u32); rax = P24; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xa8u64), (0x2bu32) as u32); rax = P25; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xb8u64), (0x2cu32) as u32); rax = P26; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), (0x2du32) as u32); rax = P27; crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xd8u64), (0x2eu32) as u32); rax = P28; crate::simd_ops::wu64(rbp.wrapping_add(0xe0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xe8u64), (0x2fu32) as u32); rax = P29; crate::simd_ops::wu64(rbp.wrapping_add(0xf0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xf8u64), (0x30u32) as u32); rax = P30; crate::simd_ops::wu64(rbp.wrapping_add(0x100u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x108u64), (0x31u32) as u32); rax = P31; crate::simd_ops::wu64(rbp.wrapping_add(0x110u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x118u64), (0x32u32) as u32); rax = P32; crate::simd_ops::wu64(rbp.wrapping_add(0x120u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x128u64), (0x33u32) as u32); rax = P33; crate::simd_ops::wu64(rbp.wrapping_add(0x130u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x138u64), (0x34u32) as u32); rax = P34; crate::simd_ops::wu64(rbp.wrapping_add(0x140u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x148u64), (0x35u32) as u32); rax = P35; crate::simd_ops::wu64(rbp.wrapping_add(0x150u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x158u64), (0x36u32) as u32); rax = P36; crate::simd_ops::wu64(rbp.wrapping_add(0x160u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x168u64), (0x37u32) as u32); rax = P37; crate::simd_ops::wu64(rbp.wrapping_add(0x170u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x178u64), (0x11u32) as u32); rax = (crate::simd_ops::ru32(P38)) as u32 as u64; rdi = (crate::simd_ops::ru32(P39)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r15; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P40, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P39)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P41)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P42, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P41)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P41)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P43, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P44, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P41, rbx); crate::simd_ops::wu32(P39, ((rdi as u32)) as u32); crate::simd_ops::wu32(P38, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P41)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P45)) as u32 as u64; fl_l = rax; fl_r = (0x26u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x26u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P46)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P47, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P46)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P48)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P42, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P48)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P48)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P43, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P44, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P48, rdi); crate::simd_ops::wu32(P46, ((rsi as u32)) as u32); crate::simd_ops::wu32(P45, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P39)) as u32 as u64; rbx = (crate::simd_ops::ru64(P41)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P49, (0xffffffffu32) as u32); r14 = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P38)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rsi = rdx; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rdx)) as u64; fl_l = rsi; fl_r = rax; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rsi }) as u64; rdi = rcx; rdx = rsi; rax = crate::simd_ops::call_stk(P40, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P39)) as u32 as u64; rdx = (crate::simd_ops::ru64(P41)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P42, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P41)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P41)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P43, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P44, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P41, rbx); crate::simd_ops::wu32(P39, ((rdi as u32)) as u32); crate::simd_ops::wu32(P38, ((rsi as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P39, (1) as u32); rdx = r14.wrapping_add(0x8); rcx = P48; rax = crate::simd_ops::call_stk(P50, rax, rcx, rdx, r8, r9, rsp); r14 = ((r14).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P39)) as u32 as u64; rbx = (crate::simd_ops::ru64(P41)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P51)) as u64; crate::simd_ops::wu64(P52, rax); rax = P41; crate::simd_ops::wu64(P51, rax); rcx = P53; r11 = rsp.wrapping_add(0x280); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P54, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016df80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14309f914; const P1: u64 = 0x14336ec64; const P2: u64 = 0x143a46cd0; const P3: u64 = 0x14309f90c; const P4: u64 = 0x14336ed50; const P5: u64 = 0x14336ed48; const P6: u64 = 0x1401df440; const P7: u64 = 0x142611640; const P8: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x78u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x89u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x8bu32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x8au32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax);
    rax = P5;
    crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x8cu32) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x8du32) as u32);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = P7;
    rsp = ((rsp).wrapping_add(0x78u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016ebd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336f918; const P1: u64 = 0x14336fa08; const P2: u64 = 0x143a47170; const P3: u64 = 0x14336f9fc; const P4: u64 = 0x1401de5b0; const P5: u64 = 0x1426117c0; const P6: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x58u64)) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32);
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((rcx as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = P5;
    rsp = ((rsp).wrapping_add(0x58u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016ec30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14336f998; const P1: u64 = 0x14336fc78; const P2: u64 = 0x14336fc60; const P3: u64 = 0x14336fc48; const P4: u64 = 0x14336fc38; const P5: u64 = 0x14336fcd8; const P6: u64 = 0x14336fcc0; const P7: u64 = 0x14336fca0; const P8: u64 = 0x14336fc88; const P9: u64 = 0x14336fbc0; const P10: u64 = 0x14336fba8; const P11: u64 = 0x14336fb90; const P12: u64 = 0x14336fb78; const P13: u64 = 0x14336fc20; const P14: u64 = 0x14336fc08; const P15: u64 = 0x14336fbf0; const P16: u64 = 0x14336fbd8; const P17: u64 = 0x14336fdf0; const P18: u64 = 0x14336fde0; const P19: u64 = 0x14336fdc0; const P20: u64 = 0x14336fda8; const P21: u64 = 0x14336fe58; const P22: u64 = 0x14336fe40; const P23: u64 = 0x14336fe28; const P24: u64 = 0x14336fe10; const P25: u64 = 0x14336fd30; const P26: u64 = 0x14336fd20; const P27: u64 = 0x14336fd10; const P28: u64 = 0x14336fcf0; const P29: u64 = 0x14336fd90; const P30: u64 = 0x14336fd78; const P31: u64 = 0x14336fd60; const P32: u64 = 0x14336fd48; const P33: u64 = 0x143370c10; const P34: u64 = 0x143370bf0; const P35: u64 = 0x143370be0; const P36: u64 = 0x143370bd0; const P37: u64 = 0x143370c70; const P38: u64 = 0x143370c58; const P39: u64 = 0x143370c48; const P40: u64 = 0x143370c30; const P41: u64 = 0x143370b78; const P42: u64 = 0x143370b68; const P43: u64 = 0x143370b48; const P44: u64 = 0x143370b30; const P45: u64 = 0x143370bb0; const P46: u64 = 0x143370ba8; const P47: u64 = 0x143370b90; const P48: u64 = 0x143370b80; const P49: u64 = 0x143370d50; const P50: u64 = 0x143370d40; const P51: u64 = 0x143370d28; const P52: u64 = 0x143370d10; const P53: u64 = 0x143370dc0; const P54: u64 = 0x143370da8; const P55: u64 = 0x143370d88; const P56: u64 = 0x143370d68; const P57: u64 = 0x143370cb8; const P58: u64 = 0x143370ca8; const P59: u64 = 0x143a46fbc; const P60: u64 = 0x143a46fb8; const P61: u64 = 0x1401dec00; const P62: u64 = 0x143a46fb0; const P63: u64 = 0x14183acc8; const P64: u64 = 0x1425371a0; const P65: u64 = 0x142533200; const P66: u64 = 0x143a46fcc; const P67: u64 = 0x143a46fc8; const P68: u64 = 0x1401decf0; const P69: u64 = 0x143a46fc0; const P70: u64 = 0x143a46fe4; const P71: u64 = 0x1401de8d0; const P72: u64 = 0x143a082e0; const P73: u64 = 0x143a46fe8; const P74: u64 = 0x1426117f0; const P75: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 125>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x2d0); rsp = ((rsp).wrapping_sub(0x3d0u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x3u32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x4u32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x5u32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x6u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x7u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x8u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x9u32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0xau32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xbu32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0xcu32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0xdu32) as u32); rax = P14; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0xeu32) as u32); rax = P15; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), (0xfu32) as u32); rax = P16; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), (0x10u32) as u32); rax = P17; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0x11u32) as u32); rax = P18; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), (0x12u32) as u32); rax = P19; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), (0x13u32) as u32); rax = P20; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0x14u32) as u32); rax = P21; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), (0x15u32) as u32); rax = P22; crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), (0x16u32) as u32); rax = P23; crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x98u64), (0x17u32) as u32); rax = P24; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xa8u64), (0x18u32) as u32); rax = P25; crate::simd_ops::wu64(rbp.wrapping_add(0xb0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xb8u64), (0x19u32) as u32); rax = P26; crate::simd_ops::wu64(rbp.wrapping_add(0xc0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), (0x1au32) as u32); rax = P27; crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xd8u64), (0x1bu32) as u32); rax = P28; crate::simd_ops::wu64(rbp.wrapping_add(0xe0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xe8u64), (0x1cu32) as u32); rax = P29; crate::simd_ops::wu64(rbp.wrapping_add(0xf0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0xf8u64), (0x1du32) as u32); rax = P30; crate::simd_ops::wu64(rbp.wrapping_add(0x100u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x108u64), (0x1eu32) as u32); rax = P31; crate::simd_ops::wu64(rbp.wrapping_add(0x110u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x118u64), (0x1fu32) as u32); rax = P32; crate::simd_ops::wu64(rbp.wrapping_add(0x120u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x128u64), (0x20u32) as u32); rax = P33; crate::simd_ops::wu64(rbp.wrapping_add(0x130u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x138u64), (0x21u32) as u32); rax = P34; crate::simd_ops::wu64(rbp.wrapping_add(0x140u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x148u64), (0x22u32) as u32); rax = P35; crate::simd_ops::wu64(rbp.wrapping_add(0x150u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x158u64), (0x23u32) as u32); rax = P36; crate::simd_ops::wu64(rbp.wrapping_add(0x160u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x168u64), (0x24u32) as u32); rax = P37; crate::simd_ops::wu64(rbp.wrapping_add(0x170u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x178u64), (0x25u32) as u32); rax = P38; crate::simd_ops::wu64(rbp.wrapping_add(0x180u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x188u64), (0x26u32) as u32); rax = P39; crate::simd_ops::wu64(rbp.wrapping_add(0x190u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x198u64), (0x27u32) as u32); rax = P40; crate::simd_ops::wu64(rbp.wrapping_add(0x1a0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1a8u64), (0x28u32) as u32); rax = P41; crate::simd_ops::wu64(rbp.wrapping_add(0x1b0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1b8u64), (0x29u32) as u32); rax = P42; crate::simd_ops::wu64(rbp.wrapping_add(0x1c0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1c8u64), (0x2au32) as u32); rax = P43; crate::simd_ops::wu64(rbp.wrapping_add(0x1d0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1d8u64), (0x2bu32) as u32); rax = P44; crate::simd_ops::wu64(rbp.wrapping_add(0x1e0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1e8u64), (0x2cu32) as u32); rax = P45; crate::simd_ops::wu64(rbp.wrapping_add(0x1f0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x1f8u64), (0x2du32) as u32); rax = P46; crate::simd_ops::wu64(rbp.wrapping_add(0x200u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x208u64), (0x2eu32) as u32); rax = P47; crate::simd_ops::wu64(rbp.wrapping_add(0x210u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x218u64), (0x2fu32) as u32); rax = P48; crate::simd_ops::wu64(rbp.wrapping_add(0x220u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x228u64), (0x30u32) as u32); rax = P49; crate::simd_ops::wu64(rbp.wrapping_add(0x230u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x238u64), (0x31u32) as u32); rax = P50; crate::simd_ops::wu64(rbp.wrapping_add(0x240u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x248u64), (0x32u32) as u32); rax = P51; crate::simd_ops::wu64(rbp.wrapping_add(0x250u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x258u64), (0x33u32) as u32); rax = P52; crate::simd_ops::wu64(rbp.wrapping_add(0x260u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x268u64), (0x34u32) as u32); rax = P53; crate::simd_ops::wu64(rbp.wrapping_add(0x270u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x278u64), (0x35u32) as u32); rax = P54; crate::simd_ops::wu64(rbp.wrapping_add(0x280u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x288u64), (0x36u32) as u32); rax = P55; crate::simd_ops::wu64(rbp.wrapping_add(0x290u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x298u64), (0x37u32) as u32); rax = P56; crate::simd_ops::wu64(rbp.wrapping_add(0x2a0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2a8u64), (0x38u32) as u32); rax = P57; crate::simd_ops::wu64(rbp.wrapping_add(0x2b0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2b8u64), (0x39u32) as u32); rax = P58; crate::simd_ops::wu64(rbp.wrapping_add(0x2c0u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x2c8u64), (0x3au32) as u32); rax = (crate::simd_ops::ru32(P59)) as u32 as u64; r15 = 0x3b; rsi = (crate::simd_ops::ru32(P60)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rdi = ((rax as u32)) as u32 as u64; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; fl_l = rdi; fl_r = r15; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rdi }) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(P61, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P60)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P62)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P63, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P62)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P62)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P65, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P62, rbx); crate::simd_ops::wu32(P60, ((rsi as u32)) as u32); crate::simd_ops::wu32(P59, ((rdi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P62)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P66)) as u32 as u64; fl_l = rax; fl_r = (0x3bu64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x3bu64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P67)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P68, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P67)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P69)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P63, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P69)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P69)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P64, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P65, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P69, rdi); crate::simd_ops::wu32(P67, ((rsi as u32)) as u32); crate::simd_ops::wu32(P66, ((rbx as u32)) as u32); rsi = (crate::simd_ops::ru32(P60)) as u32 as u64; rbx = (crate::simd_ops::ru64(P62)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P70, (0xffffffffu32) as u32); r14 = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rsi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P59)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; rdi = rdx; rdi = ((rdi).wrapping_shr(1u32)) as u64; rdi = ((rdi).wrapping_add(rdx)) as u64; fl_l = rdi; fl_r = rax; rdi = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdi }) as u64; rsi = rcx; rdx = rdi; rax = crate::simd_ops::call_stk(P61, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P60)) as u32 as u64; rdx = (crate::simd_ops::ru64(P62)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P63, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P62)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P62)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P64, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P65, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P62, rbx); crate::simd_ops::wu32(P60, ((rsi as u32)) as u32); crate::simd_ops::wu32(P59, ((rdi as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rsi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P60, (1) as u32); rdx = r14.wrapping_add(0x8); rcx = P69; rax = crate::simd_ops::call_stk(P71, rax, rcx, rdx, r8, r9, rsp); r14 = ((r14).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rsi = (crate::simd_ops::ru32(P60)) as u32 as u64; rbx = (crate::simd_ops::ru64(P62)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P72)) as u64; crate::simd_ops::wu64(P73, rax); rax = P62; crate::simd_ops::wu64(P72, rax); rcx = P74; r11 = rsp.wrapping_add(0x3d0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P75, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016f4f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14310fe28; const P1: u64 = 0x143370c90; const P2: u64 = 0x143370c80; const P3: u64 = 0x143370b80; const P4: u64 = 0x143370dc0; const P5: u64 = 0x143370da8; const P6: u64 = 0x143370d88; const P7: u64 = 0x143370d68; const P8: u64 = 0x143370d08; const P9: u64 = 0x143370cf8; const P10: u64 = 0x143370ce0; const P11: u64 = 0x143370cc8; const P12: u64 = 0x143370f48; const P13: u64 = 0x143370f38; const P14: u64 = 0x143370f20; const P15: u64 = 0x143370f10; const P16: u64 = 0x143370fa0; const P17: u64 = 0x143370f90; const P18: u64 = 0x143370f78; const P19: u64 = 0x143370f60; const P20: u64 = 0x143370e30; const P21: u64 = 0x143370e18; const P22: u64 = 0x143370e00; const P23: u64 = 0x143a4713c; const P24: u64 = 0x143a47138; const P25: u64 = 0x1401dec00; const P26: u64 = 0x143a47130; const P27: u64 = 0x14183acc8; const P28: u64 = 0x1425371a0; const P29: u64 = 0x142533200; const P30: u64 = 0x143a4714c; const P31: u64 = 0x143a47148; const P32: u64 = 0x1401decf0; const P33: u64 = 0x143a47140; const P34: u64 = 0x143a47164; const P35: u64 = 0x1401de8d0; const P36: u64 = 0x143a082e0; const P37: u64 = 0x143a47168; const P38: u64 = 0x142611820; const P39: u64 = 0x141834d18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 53>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x90); rsp = ((rsp).wrapping_sub(0x190u64)) as u64; rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x2u32) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x3u32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0x4u32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x5u32) as u32); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x78u64), (0x6u32) as u32); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x68u64), (0x7u32) as u32); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), (0x8u32) as u32); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x48u64), (0x9u32) as u32); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x38u64), (0xau32) as u32); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), (0xbu32) as u32); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), (0xcu32) as u32); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x8u64), (0xdu32) as u32); rax = P14; crate::simd_ops::wu64(rbp, rax); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0xeu32) as u32); rax = P15; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x18u64), (0xfu32) as u32); rax = P16; crate::simd_ops::wu64(rbp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x28u64), (0x10u32) as u32); rax = P17; crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0x11u32) as u32); rax = P18; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), (0x12u32) as u32); rax = P19; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), (0x13u32) as u32); rax = P20; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0x14u32) as u32); rax = P21; crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x78u64), (0x15u32) as u32); rax = P22; crate::simd_ops::wu64(rbp.wrapping_add(0x80u64), rax); crate::simd_ops::wu32(rbp.wrapping_add(0x88u64), (0x16u32) as u32); rax = (crate::simd_ops::ru32(P23)) as u32 as u64; r15 = 0x17; rdi = (crate::simd_ops::ru32(P24)) as u32 as u64; fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 1; continue; }
            1 => { rsi = ((rax as u32)) as u32 as u64; rsi = ((rsi).wrapping_shr(1u32)) as u64; rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = rsi; fl_r = r15; rsi = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rsi }) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(P25, rax, rcx, rdx, r8, r9, rsp); rbx = rax; r8 = (crate::simd_ops::ru32(P24)) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; rdx = (crate::simd_ops::ru64(P26)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P27, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P26)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 2; continue; }
            2 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P26)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = rdx; rax = crate::simd_ops::call_stk(P28, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            6 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 8; continue; }
            7 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { rcx = r10; rax = crate::simd_ops::call_stk(P29, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { crate::simd_ops::wu64(P26, rbx); crate::simd_ops::wu32(P24, ((rdi as u32)) as u32); crate::simd_ops::wu32(P23, ((rsi as u32)) as u32); bb = 11; continue; }
            10 => { rbx = (crate::simd_ops::ru64(P26)) as u64; bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(P30)) as u32 as u64; fl_l = rax; fl_r = (0x17u64) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 21; continue; } bb = 12; continue; }
            12 => { rbx = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_shr(1u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; fl_l = rbx; fl_r = (0x17u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { r15 } else { rbx }) as u64; rsi = (crate::simd_ops::ru32(P31)) as u32 as u64; rdx = rbx; rax = crate::simd_ops::call_stk(P32, rax, rcx, rdx, r8, r9, rsp); rdi = rax; r8 = (crate::simd_ops::ru32(P31)) as u32 as u64; r8 = ((r8).wrapping_shl(2u32)) as u64; rdx = (crate::simd_ops::ru64(P33)) as u64; rcx = rax; rax = crate::simd_ops::call_stk(P27, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P33)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P33)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = rdx; rax = crate::simd_ops::call_stk(P28, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rcx = r10; rax = crate::simd_ops::call_stk(P29, rax, rcx, rdx, r8, r9, rsp); bb = 20; continue; }
            20 => { crate::simd_ops::wu64(P33, rdi); crate::simd_ops::wu32(P31, ((rsi as u32)) as u32); crate::simd_ops::wu32(P30, ((rbx as u32)) as u32); rdi = (crate::simd_ops::ru32(P24)) as u32 as u64; rbx = (crate::simd_ops::ru64(P26)) as u64; bb = 21; continue; }
            21 => { crate::simd_ops::wu32(P34, (0xffffffffu32) as u32); rsi = rsp.wrapping_add(0x20); bb = 22; continue; }
            22 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru32(P23)) as u32 as u64; rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_sub(rbx)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rcx.wrapping_add(0x1); rdx = ((rdx).wrapping_sub(rbx)) as u64; rdx = ((((rdx) as i64).wrapping_shr(3u32) as u64)) as u64; r14 = rdx; r14 = ((r14).wrapping_shr(1u32)) as u64; r14 = ((r14).wrapping_add(rdx)) as u64; fl_l = r14; fl_r = rax; r14 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r14 }) as u64; rdi = rcx; rdx = r14; rax = crate::simd_ops::call_stk(P25, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rax = (crate::simd_ops::ru32(P24)) as u32 as u64; rdx = (crate::simd_ops::ru64(P26)) as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P27, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(P26)) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 31; continue; } bb = 24; continue; }
            24 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(P26)) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 31; continue; } bb = 27; continue; }
            27 => { rcx = rdx; rax = crate::simd_ops::call_stk(P28, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            28 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 30; continue; }
            29 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 30; continue; }
            30 => { rcx = r10; rax = crate::simd_ops::call_stk(P29, rax, rcx, rdx, r8, r9, rsp); bb = 31; continue; }
            31 => { crate::simd_ops::wu64(P26, rbx); crate::simd_ops::wu32(P24, ((rdi as u32)) as u32); crate::simd_ops::wu32(P23, ((r14 as u32)) as u32); bb = 32; continue; }
            32 => { rax = ((rdi as u32)) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::lk_add_u32(P24, (1) as u32); rdx = rsi.wrapping_add(0x8); rcx = P33; rax = crate::simd_ops::call_stk(P35, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(0x10u64)) as u64; fl_l = r15; fl_r = (0x1u64) as u64; r15 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rdi = (crate::simd_ops::ru32(P24)) as u32 as u64; rbx = (crate::simd_ops::ru64(P26)) as u64; bb = 22; continue; }
            34 => { rax = (crate::simd_ops::ru64(P36)) as u64; crate::simd_ops::wu64(P37, rax); rax = P26; crate::simd_ops::wu64(P36, rax); rcx = P38; r11 = rsp.wrapping_add(0x190); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P39, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016fa50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143370de0; const P1: u64 = 0x143a47030; const P2: u64 = 0x1403f9250; const P3: u64 = 0x142611850; const P4: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x0u32) as u32);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rcx = P1;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rcx = P3;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14016fbc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143365384; const P1: u64 = 0x1433711d8; const P2: u64 = 0x143a46f30; const P3: u64 = 0x1433714bc; const P4: u64 = 0x1433714a8; const P5: u64 = 0x14310a92c; const P6: u64 = 0x1401df440; const P7: u64 = 0x142611910; const P8: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x78u64)) as u64;
    rax = P0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rdx = rsp.wrapping_add(0x20);
    rax = P1;
    crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x2u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    rcx = P2;
    rax = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x4u32) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax);
    rax = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax);
    rax = P5;
    crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax);
    crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0x8u32) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x68u64), (0xffffu32) as u32);
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    rcx = P7;
    rsp = ((rsp).wrapping_add(0x78u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P8, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140175370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a47380; const P1: u64 = 0x142611cf0; const P2: u64 = 0x143a47383; const P3: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    rcx = P1;
    rax = ((((rax as u32)) & (0xff000008u32))) as u32 as u64;
    fl_l = ((((rax as u32)) | (0x8u32))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(P0, ((rax as u32)) as u32);
    crate::simd_ops::wu8(P2, (0x8u8) as u8);
    return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14017d430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a32868;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0xffffffffffffffffu64) as u64);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x8u64))) as u64;
    crate::simd_ops::wu64(P0, rax);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14017e8b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143854488; const P2: u64 = 0x14385448b; const P3: u64 = 0x1433912f0; const P4: u64 = 0x142409b50; const P5: u64 = 0x143854510; const P6: u64 = 0x14248a640; const P7: u64 = 0x1433912f9; const P8: u64 = 0x1413cf570; const P9: u64 = 0x143854480; const P10: u64 = 0x8; const P11: u64 = 0x143854518; const P12: u64 = 0x143854520; const P13: u64 = 0x143854524; const P14: u64 = 0x14385452c; const P15: u64 = 0x143854530; const P16: u64 = 0x143854538; const P17: u64 = 0x14385453c; const P18: u64 = 0x143854544; const P19: u64 = 0x143854548; const P20: u64 = 0x143854550; const P21: u64 = 0x1430a7188; const P22: u64 = 0x1426139a0; const P23: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14017e8b0, funcinfo_rva: 0x366fbd4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; xmm6 = 0u128; rcx = ((P1) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P1, ((rax as u32)) as u32); crate::simd_ops::wu8(P2, (0x10u8) as u8); rbx = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x9u64) as u64); r9 = (0x9u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14017e918); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(P5, (0x2u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14017e929); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P7) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14017e962); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P9) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P10), (rcx) as u64); xmm2 = crate::simd_ops::vunpcklps(xmm6, xmm6); crate::simd_ops::wu64(P11, ((xmm2 as u64)) as u64); crate::simd_ops::wu32(P12, ((xmm6 as u32)) as u32); crate::simd_ops::wu64(P13, ((xmm2 as u64)) as u64); crate::simd_ops::wu32(P14, ((xmm6 as u32)) as u32); xmm0 = crate::simd_ops::vunpcklps(xmm6, xmm6); crate::simd_ops::wu64(P15, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(P16, ((xmm6 as u32)) as u32); xmm0 = crate::simd_ops::vunpcklps(xmm7, xmm7); crate::simd_ops::wu64(P17, ((xmm0 as u64)) as u64); crate::simd_ops::wu32(P18, ((xmm7 as u32)) as u32); crate::simd_ops::wu64(P19, ((xmm2 as u64)) as u64); crate::simd_ops::wu32(P20, ((xmm6 as u32)) as u32); rax = ((P21) as u64) as u64; crate::simd_ops::wu64(P9, (rax) as u64); rcx = ((P22) as u64) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P23) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14017fa30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307aafc; const P1: u64 = 0x143391d6d; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cdc108; const P4: u64 = 0x14384e988; const P5: u64 = 0x143cdc110; const P6: u64 = 0x1410db400; const P7: u64 = 0x143cdc138; const P8: u64 = 0x143cdc118; const P9: u64 = 0x143cdc128;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1bu64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x26u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14017fac0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307aafc; const P1: u64 = 0x143391dfd; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cdc140; const P4: u64 = 0x14384e88c; const P5: u64 = 0x143cdc148; const P6: u64 = 0x1410db4f0; const P7: u64 = 0x143cdc170; const P8: u64 = 0x143cdc150; const P9: u64 = 0x143cdc160;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1bu64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x27u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14017fc40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x1433923ec; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cdbeb0; const P4: u64 = 0x14384eab8; const P5: u64 = 0x143cdbeb8; const P6: u64 = 0x1410dbf20; const P7: u64 = 0x143cdbee0; const P8: u64 = 0x143cdbec0; const P9: u64 = 0x143cdbed0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x40au64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14017fcd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x143391f2c; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cdbe38; const P4: u64 = 0x14384eabc; const P5: u64 = 0x143cdbe40; const P6: u64 = 0x1410dbfb0; const P7: u64 = 0x143cdbe68; const P8: u64 = 0x143cdbe48; const P9: u64 = 0x143cdbe58;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1c5u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140183290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x1433998fc; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cdd3a0; const P4: u64 = 0x14384ee5c; const P5: u64 = 0x143cdd3a8; const P6: u64 = 0x1410f55b0; const P7: u64 = 0x143cdd3d0; const P8: u64 = 0x143cdd3b0; const P9: u64 = 0x143cdd3c0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x2e4u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140183320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14307ab7d; const P1: u64 = 0x1433995ac; const P2: u64 = 0x143a00288; const P3: u64 = 0x143cdd3e0; const P4: u64 = 0x14384ee58; const P5: u64 = 0x143cdd3e8; const P6: u64 = 0x1410f5640; const P7: u64 = 0x143cdd410; const P8: u64 = 0x143cdd3f0; const P9: u64 = 0x143cdd400;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 3>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x18u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x25u64) as u64);
    rax = P0;
    crate::simd_ops::wu64(rsp, rax);
    rax = P1;
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(rsp, rax);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    crate::simd_ops::wu64(P3, rax);
    rax = P4;
    crate::simd_ops::wu64(P5, rax);
    rax = P6;
    crate::simd_ops::wu64(P7, rax);
    rax = P3;
    crate::simd_ops::wu128(P8, (xmm0) as u128);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (0x1c9u64) as u64);
    xmm0 = (crate::simd_ops::ru128(rsp)) as u128;
    crate::simd_ops::wu64(P2, rax);
    crate::simd_ops::wu128(P9, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140183590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14339f790; const P1: u64 = 0x142409b50; const P2: u64 = 0x1424b3250; const P3: u64 = 0x14240ab10; const P4: u64 = 0x143a29fac;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); r9 = 0x10; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = rsp.wrapping_add(0x38); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P4, ((rbx as u32)) as u32); rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140183640(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14339f770; const P1: u64 = 0x142409b50; const P2: u64 = 0x1424b3250; const P3: u64 = 0x14240ab10; const P4: u64 = 0x143a29fa8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x8000008u64) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), (0x0u8) as u8); rax = P0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x19u64) as u64); r9 = 0x19; r8 = 0; fl_l = 0; fl_r = 0; rdx = rsp.wrapping_add(0x20); rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x33u64))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = ((rdx.wrapping_mul(8)) as u32) as u32 as u64; bb = 3; continue; }
            2 => { rdx = (((rdx as u32)).wrapping_shl(8u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x7c00; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x30u64))) as u32 as u64; rax = ((((rax as u32)) & (0xffffffu32))) as u32 as u64; rax = (((rax as u32)).wrapping_shl(3u32)) as u32 as u64; rcx = rsp.wrapping_add(0x38); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; rcx = (if (((fl_l as u32)) > ((fl_r as u32))) { crate::simd_ops::ru64(rsp.wrapping_add(0x38u64)) } else { rcx }) as u64; rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x34u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = ((rax as u32)) as u32 as u64; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(P4, ((rbx as u32)) as u32); rsp = ((rsp).wrapping_add(0x80u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140184ef0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438556f8; const P1: u64 = 0x1438556fb; const P2: u64 = 0x1433a0030; const P3: u64 = 0x142409b50; const P4: u64 = 0x143855780; const P5: u64 = 0x14248a640; const P6: u64 = 0x1433a0040; const P7: u64 = 0x1413cf570; const P8: u64 = 0x1438556f0; const P9: u64 = 0x8; const P10: u64 = 0x143426e84; const P11: u64 = 0x143855788; const P12: u64 = 0x14385578c; const P13: u64 = 0x1434264e4; const P14: u64 = 0x143855790; const P15: u64 = 0x143855794; const P16: u64 = 0x143855798; const P17: u64 = 0x1431ad110; const P18: u64 = 0x142615460; const P19: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140184ef0, funcinfo_rva: 0x3673c6c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); crate::simd_ops::wu8(P1, (0x10u8) as u8); rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); r9 = (0x10u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140184f40); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140184f4f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P6) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140184f82); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P8) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), (rcx) as u64); xmm1 = ((crate::simd_ops::ru32(P10)) as u32) as u128; crate::simd_ops::wu32(P11, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; crate::simd_ops::wu32(P14, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(P15, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P16, ((xmm1 as u32)) as u32); rax = ((P17) as u64) as u64; crate::simd_ops::wu64(P8, (rax) as u64); rcx = ((P18) as u64) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P19) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140185100(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143855878; const P1: u64 = 0x14385587b; const P2: u64 = 0x1433a0000; const P3: u64 = 0x142409b50; const P4: u64 = 0x143855900; const P5: u64 = 0x14248a640; const P6: u64 = 0x1433a0015; const P7: u64 = 0x1413cf570; const P8: u64 = 0x143855870; const P9: u64 = 0x8; const P10: u64 = 0x1434268d0; const P11: u64 = 0x143855908; const P12: u64 = 0x14385590c; const P13: u64 = 0x143855910; const P14: u64 = 0x143855914; const P15: u64 = 0x143855918; const P16: u64 = 0x1431ad110; const P17: u64 = 0x1426154a0; const P18: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x140185100, funcinfo_rva: 0x3673cbc, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); crate::simd_ops::wu8(P1, (0x10u8) as u8); rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x15u64) as u64); r9 = (0x15u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140185150); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018515f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P6) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x140185192); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P8) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), (rcx) as u64); xmm1 = ((crate::simd_ops::ru32(P10)) as u32) as u128; crate::simd_ops::wu32(P11, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm1 as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu32(P13, ((xmm0 as u32)) as u32); crate::simd_ops::wu32(P14, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P15, ((xmm1 as u32)) as u32); rax = ((P16) as u64) as u64; crate::simd_ops::wu64(P8, (rax) as u64); rcx = ((P17) as u64) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14018ee70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143856198; const P1: u64 = 0x14385619b; const P2: u64 = 0x1433b7088; const P3: u64 = 0x142409b50; const P4: u64 = 0x143856220; const P5: u64 = 0x14248a640; const P6: u64 = 0x1433b70a1; const P7: u64 = 0x1413cf570; const P8: u64 = 0x143856190; const P9: u64 = 0x8; const P10: u64 = 0x143426818; const P11: u64 = 0x143856228; const P12: u64 = 0x14385622c; const P13: u64 = 0x143426650; const P14: u64 = 0x143856230; const P15: u64 = 0x143426bf8; const P16: u64 = 0x143856234; const P17: u64 = 0x143856238; const P18: u64 = 0x1431ad110; const P19: u64 = 0x1426166a0; const P20: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14018ee70, funcinfo_rva: 0x36770a0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); crate::simd_ops::wu8(P1, (0x10u8) as u8); rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x19u64) as u64); r9 = (0x19u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018eec0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018eecf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P6) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018ef02); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P8) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), (rcx) as u64); xmm2 = ((crate::simd_ops::ru32(P10)) as u32) as u128; crate::simd_ops::wu32(P11, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm2 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; crate::simd_ops::wu32(P14, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(P15)) as u32) as u128; crate::simd_ops::wu32(P16, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P17, ((xmm2 as u32)) as u32); rax = ((P18) as u64) as u64; crate::simd_ops::wu64(P8, (rax) as u64); rcx = ((P19) as u64) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14018f000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143855f28; const P1: u64 = 0x143855f2b; const P2: u64 = 0x1430f1918; const P3: u64 = 0x142409b50; const P4: u64 = 0x143855fb0; const P5: u64 = 0x14248a640; const P6: u64 = 0x1430f1929; const P7: u64 = 0x1413cf570; const P8: u64 = 0x143855f20; const P9: u64 = 0x8; const P10: u64 = 0x143426650; const P11: u64 = 0x143855fb8; const P12: u64 = 0x143855fbc; const P13: u64 = 0x1434264e4; const P14: u64 = 0x143855fc0; const P15: u64 = 0x143426780; const P16: u64 = 0x143855fc4; const P17: u64 = 0x143855fc8; const P18: u64 = 0x1431ad110; const P19: u64 = 0x1426167e0; const P20: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14018f000, funcinfo_rva: 0x3677148, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); crate::simd_ops::wu8(P1, (0x10u8) as u8); rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x11u64) as u64); r9 = (0x11u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018f050); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018f05f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P6) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018f092); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P8) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), (rcx) as u64); xmm2 = ((crate::simd_ops::ru32(P10)) as u32) as u128; crate::simd_ops::wu32(P11, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm2 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; crate::simd_ops::wu32(P14, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(P15)) as u32) as u128; crate::simd_ops::wu32(P16, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P17, ((xmm2 as u32)) as u32); rax = ((P18) as u64) as u64; crate::simd_ops::wu64(P8, (rax) as u64); rcx = ((P19) as u64) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14018f190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143856038; const P1: u64 = 0x14385603b; const P2: u64 = 0x1433b6bb8; const P3: u64 = 0x142409b50; const P4: u64 = 0x1438560c0; const P5: u64 = 0x14248a640; const P6: u64 = 0x1433b6bc8; const P7: u64 = 0x1413cf570; const P8: u64 = 0x143856030; const P9: u64 = 0x8; const P10: u64 = 0x143426f70; const P11: u64 = 0x1438560c8; const P12: u64 = 0x1438560cc; const P13: u64 = 0x1434268d0; const P14: u64 = 0x1438560d0; const P15: u64 = 0x143426f78; const P16: u64 = 0x1438560d4; const P17: u64 = 0x1438560d8; const P18: u64 = 0x1431ad110; const P19: u64 = 0x142616920; const P20: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x14018f190, funcinfo_rva: 0x36771f0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x38, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbx) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rcx = ((P0) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rcx) as u64); rax = (crate::simd_ops::ru32(P0)) as u32 as u64; rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64; rax = ((((rax as u32)) | (0x10u32))) as u32 as u64; crate::simd_ops::wu32(P0, ((rax as u32)) as u32); crate::simd_ops::wu8(P1, (0x10u8) as u8); rbx = ((P2) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64); r9 = (0x10u32) as u32 as u64; r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = ((rsp.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018f1e0); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P4, (rax) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018f1ef); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (0x811c9dc5u32) as u32 as u64; r8 = ((P6) as u64) as u64; bb = 1; continue; }
            1 => { rcx = (((crate::simd_ops::ru8(rbx)) as i8 as i32) as u32) as u32 as u64; rcx = ((((rcx as u32)) | (0x20u32))) as u32 as u64; rcx = ((((rcx as u32)) ^ ((rdx as u32)))) as u32 as u64; rdx = ((((rcx as u32)) as i32).wrapping_mul((0x1000193u32) as i32) as u32) as u32 as u64; rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rdx as u32)) as u32); rdx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x14018f222); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((P8) as u64) as u64; crate::simd_ops::wu64(rax.wrapping_add(P9), (rcx) as u64); xmm2 = ((crate::simd_ops::ru32(P10)) as u32) as u128; crate::simd_ops::wu32(P11, ((xmm2 as u32)) as u32); crate::simd_ops::wu32(P12, ((xmm2 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; crate::simd_ops::wu32(P14, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(P15)) as u32) as u128; crate::simd_ops::wu32(P16, ((xmm1 as u32)) as u32); crate::simd_ops::wu32(P17, ((xmm2 as u32)) as u32); rax = ((P18) as u64) as u64; crate::simd_ops::wu64(P8, (rax) as u64); rcx = ((P19) as u64) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); drop(_eh); let s5 = crate::simd_ops::read_s5_tail(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x40) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14018f2a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143a485b0; const P1: u64 = 0x1423fd0b0; const P2: u64 = 0x1434285e0; const P3: u64 = 0x1430ee038; const P4: u64 = 0x143a48890; const P5: u64 = 0x14242cf70; const P6: u64 = 0x142616940; const P7: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = P0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(P2)) as u128;
    r8 = P3;
    rcx = P0;
    crate::simd_ops::wu64(P0, r8);
    crate::simd_ops::wu128(P4, (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = P6;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140190d60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141154170; const P1: u64 = 0x143a48980; const P2: u64 = 0x143a48988; const P3: u64 = 0x143a48990; const P4: u64 = 0x1401fc200; const P5: u64 = 0x143a48998; const P6: u64 = 0x143a489a0; const P7: u64 = 0x143a489b0; const P8: u64 = 0x143a489b8; const P9: u64 = 0x1434268d0; const P10: u64 = 0x1401e5730; const P11: u64 = 0x141153630; const P12: u64 = 0x141152f30; const P13: u64 = 0x141834ee8; const P14: u64 = 0x1425371a0; const P15: u64 = 0x142533200; const P16: u64 = 0x142616bb0; const P17: u64 = 0x141834d18;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0x3u32) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), (0x4u32) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); rdx = ((rbx.wrapping_add(0x2u64)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rcx, rax); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x38u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (0x2u32) as u32); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x2cu64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu32(P1, ((xmm0 as u32)) as u32); crate::simd_ops::wu64(P2, rbx); crate::simd_ops::wu64(P3, rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(P2, rax); crate::simd_ops::wu64(P5, rbx); xmm0 = 0u128; crate::simd_ops::wu128(P6, (xmm0) as u128); crate::simd_ops::wu64(P7, (0x7u64) as u64); crate::simd_ops::wu64(P8, (0x8u64) as u64); xmm0 = ((crate::simd_ops::ru32(P9)) as u32) as u128; crate::simd_ops::wu32(P1, ((xmm0 as u32)) as u32); r8 = rax; rdx = ((rbx.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = P5; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rsp.wrapping_add(0x48); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = P12; rdx = ((rbx.wrapping_add(0x18u64)) as u32) as u32 as u64; r8 = ((rbx.wrapping_add(0x1u64)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rcx = (rcx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rdx = r10.wrapping_add(0x108); rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rdx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rdx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { rcx = rdx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = P16; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140190f10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141fc2330; const P1: u64 = 0x141fc2320; const P2: u64 = 0x1401fc200; const P3: u64 = 0x8; const P4: u64 = 0x143c48778; const P5: u64 = 0x143c48788; const P6: u64 = 0x143c48790; const P7: u64 = 0x143c487a0; const P8: u64 = 0x143c487a8; const P9: u64 = 0x1434268d0; const P10: u64 = 0x143c48770; const P11: u64 = 0x1401e5730; const P12: u64 = 0x141153300; const P13: u64 = 0x142616bd0; const P14: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32);
    rax = P0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax);
    rax = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    crate::simd_ops::wu64(rax, rax);
    crate::simd_ops::wu64(rax.wrapping_add(P3), rax);
    crate::simd_ops::wu64(P4, rax);
    crate::simd_ops::wu64(P5, (0x0u64) as u64);
    xmm0 = 0u128;
    crate::simd_ops::wu128(P6, (xmm0) as u128);
    crate::simd_ops::wu64(P7, (0x7u64) as u64);
    crate::simd_ops::wu64(P8, (0x8u64) as u64);
    xmm0 = ((crate::simd_ops::ru32(P9)) as u32) as u128;
    crate::simd_ops::wu32(P10, ((xmm0 as u32)) as u32);
    r8 = rax;
    rdx = 0x10;
    rcx = P5;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    r8 = rsp.wrapping_add(0x38);
    rdx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = P13;
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140193a30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426564; const P1: u64 = 0x1433c07f0; const P2: u64 = 0x143c488d0; const P3: u64 = 0x143c488d3; const P4: u64 = 0x142409b50; const P5: u64 = 0x143c48958; const P6: u64 = 0x143c48960; const P7: u64 = 0x1401df790; const P8: u64 = 0x142617df0; const P9: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32);
    crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm0 as u32)) as u32);
    rax = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0xfu64) as u64);
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx);
    rax = (crate::simd_ops::ru32(P2)) as u32 as u64;
    rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64;
    rax = ((((rax as u32)) | (0x10u32))) as u32 as u64;
    crate::simd_ops::wu32(P2, ((rax as u32)) as u32);
    crate::simd_ops::wu8(P3, (0x10u8) as u8);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    r9 = 0xf;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(P5, rax);
    rcx = P6;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = P8;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140193ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x143426564; const P2: u64 = 0x1433c0808; const P3: u64 = 0x143c487c0; const P4: u64 = 0x143c487c3; const P5: u64 = 0x142409b50; const P6: u64 = 0x143c48848; const P7: u64 = 0x143c48850; const P8: u64 = 0x1401df790; const P9: u64 = 0x142617e00; const P10: u64 = 0x141834d18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32);
    xmm1 = ((crate::simd_ops::ru32(P1)) as u32) as u128;
    crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm1 as u32)) as u32);
    rax = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x10u64) as u64);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rcx);
    rax = (crate::simd_ops::ru32(P3)) as u32 as u64;
    rax = ((((rax as u32)) & (0xff000010u32))) as u32 as u64;
    rax = ((((rax as u32)) | (0x10u32))) as u32 as u64;
    crate::simd_ops::wu32(P3, ((rax as u32)) as u32);
    crate::simd_ops::wu8(P4, (0x10u8) as u8);
    xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128;
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    r9 = 0x10;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = rsp.wrapping_add(0x20);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu64(P6, rax);
    rcx = P7;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rcx = P9;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140199160(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426182b0; const P1: u64 = 0x141834d18; const P2: u64 = 0x1433d40c0; const P3: u64 = 0x141ffaea0; const P4: u64 = 0x1433d40d0; const P5: u64 = 0x141ffae50; const P6: u64 = 0x1433d40e8; const P7: u64 = 0x141ffada0; const P8: u64 = 0x1433d3e80; const P9: u64 = 0x141ffad60; const P10: u64 = 0x1433d3ea8; const P11: u64 = 0x141ffad30; const P12: u64 = 0x1433d3ec8; const P13: u64 = 0x141ffad00; const P14: u64 = 0x1433d3ee8; const P15: u64 = 0x141ffaca0; const P16: u64 = 0x1433d3e20; const P17: u64 = 0x141ffac40; const P18: u64 = 0x1430782a0; const P19: u64 = 0x143c48a00; const P20: u64 = 0x143c48a08; const P21: u64 = 0x143c48a10; const P22: u64 = 0x143c48a18; const P23: u64 = 0x143c48a28; const P24: u64 = 0x1401de990; const P25: u64 = 0x1424b16a0; const P26: u64 = 0x14119b210; const P27: u64 = 0x143a329d8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x30); rsp = ((rsp).wrapping_sub(0x130u64)) as u64; rcx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0xdu64) as u64); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x10u64) as u64); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rax = P6; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), (0x1fu64) as u64); rax = P7; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); rax = P8; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (0x21u64) as u64); rax = P9; crate::simd_ops::wu64(rbp.wrapping_sub(0x58u64), rax); rax = P10; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), (0x1au64) as u64); rax = P11; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); rax = P12; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), (0x1du64) as u64); rax = P13; crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rax); rax = P14; crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (0x23u64) as u64); rax = P15; crate::simd_ops::wu64(rbp.wrapping_sub(0x10u64), rax); rax = P16; crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), rax); crate::simd_ops::wu64(rbp, (0x25u64) as u64); rax = P17; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rax); rax = P18; crate::simd_ops::wu64(P19, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(P20, rax); crate::simd_ops::wu64(P21, rax); crate::simd_ops::wu64(P22, rax); crate::simd_ops::wu64(P23, rax); rbx = rsp.wrapping_add(0x50); rax = 0x9; let bit_in: u64 = rax; if bit_in != 0 { rax = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rcx = 0x3f; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = (0xffffffffffffffffu64) as u64; rax = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr((((rax) as u32) & 63))) as u64; rdi = P19; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P24, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rsi = (0xde5fb9d2630458e9u64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = rbx.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P25, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = rsi; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; rax = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = rbx; rdx = rbp.wrapping_add(0x10); rcx = rdi; rax = crate::simd_ops::call_stk(P26, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rax = rbp.wrapping_add(0x10); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(P27, rdi); r11 = rsp.wrapping_add(0x130); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_140199370(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1426182d0; const P1: u64 = 0x141834d18; const P2: u64 = 0x1433d3e48; const P3: u64 = 0x1433d3e58; const P4: u64 = 0x1433d3e70; const P5: u64 = 0x1433d3de8; const P6: u64 = 0x1433d3df8; const P7: u64 = 0x1430782a0; const P8: u64 = 0x143c489d0; const P9: u64 = 0x143c489d8; const P10: u64 = 0x143c489e0; const P11: u64 = 0x143c489e8; const P12: u64 = 0x143c489f8; const P13: u64 = 0x1401de990; const P14: u64 = 0x1424b16a0; const P15: u64 = 0x14119b060; const P16: u64 = 0x143a32b08;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 31>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; rcx = P0; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0xfu64) as u64); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rdx as u32)) as u32); rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x10u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), (0x1u32) as u32); rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (0xbu64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), (0x3u32) as u32); rax = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (0xcu64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0xa8u64), (0x4u32) as u32); rax = P6; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0xb8u64), (0xeu64) as u64); rcx = 0x5; crate::simd_ops::wu32(rsp.wrapping_add(0xc0u64), ((rcx as u32)) as u32); rax = P7; crate::simd_ops::wu64(P8, rax); crate::simd_ops::wu64(P9, rdx); crate::simd_ops::wu64(P10, rdx); crate::simd_ops::wu64(P11, rdx); crate::simd_ops::wu64(P12, rdx); rbx = rsp.wrapping_add(0x50); let bit_in: u64 = rcx; if bit_in != 0 { rax = (((63u32.wrapping_sub(bit_in.leading_zeros())) as u64)) as u64; } fl_l = bit_in as u64; rcx = 0x3f; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = (0xffffffffffffffffu64) as u64; rax = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr((((rax) as u32) & 63))) as u64; rdi = P8; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P13, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rsi = (0xde5fb9d2630458e9u64) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rax = rbx.wrapping_add(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P14, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = rsi; let prod = (rax as u128).wrapping_mul((rcx) as u128); rax = prod as u64; rdx = (prod >> 64) as u64; fl_l = (rdx != 0) as u64; r9 = ((rdx.wrapping_add(rax)) as u64) as u64; rax = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); r8 = rbx; rdx = rsp.wrapping_add(0xd0); rcx = rdi; rax = crate::simd_ops::call_stk(P15, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x18u64)) as u64; rax = rsp.wrapping_add(0xc8); fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu64(P16, rdi); r11 = rsp.wrapping_add(0xf0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14019b320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143a49132; const P1: u64 = 0xfffffffffffffffe; const P2: u64 = 0x6; const P3: u64 = 0xe; const P4: u64 = 0x16; const P5: u64 = 0x1e; const P6: u64 = 0x26; const P7: u64 = 0x2e; const P8: u64 = 0x36; const P9: u64 = 0x3e; const P10: u64 = 0x68; const P11: u64 = 0xffffffffffffffde;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rcx = 0x400;
    rax = P0;
    loop { crate::simd_ops::wu64(rax.wrapping_add(P1), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P2), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P3), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P4), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P5), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P6), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P7), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_add(P8), (0x0u64) as u64); crate::simd_ops::wu8(rax.wrapping_add(P9), (0x0u8) as u8); rax = ((rax.wrapping_add(P10)) as u64) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rax.wrapping_add(P11); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14019b390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e18a0; const P1: u64 = 0x140200580; const P2: u64 = 0x143a63130; const P3: u64 = 0xfffffffffffffffb; const P4: u64 = 0x141834fc0; const P5: u64 = 0x142619c00; const P6: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = 0x8;
    rax = P0;
    r9 = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rcx = P2;
    r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = P5;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14019b3d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401de3c0; const P1: u64 = 0x1401eeda0; const P2: u64 = 0x143a63150; const P3: u64 = 0xffffffffffffffdb; const P4: u64 = 0x141834fc0; const P5: u64 = 0x142619c20; const P6: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = 0x28;
    rax = P0;
    r9 = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rcx = P2;
    r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = P5;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14019b730(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1411d6ec0; const P1: u64 = 0x1402013d0; const P2: u64 = 0x143a63260; const P3: u64 = 0xfffffffffffffff6; const P4: u64 = 0x141834fc0; const P5: u64 = 0x14261a4a0; const P6: u64 = 0x141834d18;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rdx = 0x10;
    rax = P0;
    r9 = P1;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax);
    rcx = P2;
    r8 = ((rdx.wrapping_add(P3)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rcx = P5;
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14019bb90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143a632fa; const P1: u64 = 0xfffffffffffffff6; const P2: u64 = 0xfffffffffffffffe; const P3: u64 = 0x2; const P4: u64 = 0x6; const P5: u64 = 0xe; const P6: u64 = 0x16; const P7: u64 = 0x1e; const P8: u64 = 0x2e; const P9: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0x10;
    rax = P0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    loop { crate::simd_ops::wu64(rax.wrapping_add(P1), rdx); crate::simd_ops::wu32(rax.wrapping_add(P2), ((rdx as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_add(P3), ((rdx as u8)) as u8); crate::simd_ops::wu64(rax.wrapping_add(P4), rdx); crate::simd_ops::wu64(rax.wrapping_add(P5), rdx); crate::simd_ops::wu64(rax.wrapping_add(P6), rdx); crate::simd_ops::wu32(rax.wrapping_add(P7), ((rdx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(P8), rdx); rax = ((rax.wrapping_add(P9)) as u64) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14019bc20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xb28; const P1: u64 = 0xb2c;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::exe::tls_pointer() as u64)) as u64;
    rcx = 0x467e0;
    fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rax))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_14019c460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143a637b8; const P1: u64 = 0xfffffffffffffff8; const P2: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0x2400;
    rax = P0;
    loop { crate::simd_ops::wu64(rax.wrapping_add(P1), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rax, (0xffffffffffffffffu64) as u64); rax = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rcx; fl_r = (0x1u64) as u64; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    return rax;
}
