#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc24a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1427bc100; const P1: u64 = 0x140242540; const P2: u64 = 0x143c8d08c; const P3: u64 = 0x1401f9650; const P4: u64 = 0x1406927a0; const P5: u64 = 0x140692200; const P6: u64 = 0x143427080; const P7: u64 = 0x141ba1e90; const P8: u64 = 0x141ba1f80; const P9: u64 = 0x141bc1e00; const P10: u64 = 0x141bc20a0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = rdx; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (0x100u64) as u64); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; rax = r11.wrapping_add(0x10); rbp = (((r8 as u8)) as u32) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x68u64), rax); rdi = rcx; r9 = P0; r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0xe8u64))) as u64; rcx = r11.wrapping_add(0x20); rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; rax = 0x3fff; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 14; continue; } bb = 1; continue; }
            1 => { rax = (0x3fffffffffffc000u64) as u64; fl_l = rax; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xd0u64))) as u64; rdx = rsp.wrapping_add(0x98); r9 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), r14); rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r9.wrapping_mul(8)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 3; continue; }
            3 => { rdx = rbx; rcx = rsi.wrapping_add(0xd0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 4; continue; }
            4 => { rdx = rbx; rcx = rsi.wrapping_add(0xd0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x98u64))) as u32 as u64; rbx = rax; fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x0; rcx = (rcx & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rdi.wrapping_add(0x66u64), ((rcx as u8)) as u8); fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 12; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x100u64) as u64); rcx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x100u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), (0xffffffffu32) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), (0x0u8) as u8); xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x30u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru32(P6)) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), ((xmm0 as u32)) as u32); fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 6; continue; }
            6 => { r8 = ((crate::simd_ops::ru8(rdi.wrapping_add(0x65u64))) as u32) as u32 as u64; r9 = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x60u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x98u64), ((rax as u32)) as u32); fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x68u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x65u64), ((rax as u8)) as u8); bb = 10; continue; }
            9 => { r8 = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { r8 = rsp.wrapping_add(0x30); rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { r8 = rbx; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            13 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 11; continue; }
            14 => { crate::simd_ops::wu64(rdi.wrapping_add(0x68u64), (0x100u64) as u64); rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x70u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc2670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1427bc100; const P1: u64 = 0x1402b8940; const P2: u64 = 0x14040f5f0; const P3: u64 = 0x143426b74; const P4: u64 = 0x1421240f0; const P5: u64 = 0x141bc28e0; const P6: u64 = 0x141ba21f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = rcx; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; r8 = P0; rbx = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xe8u64))) as u64; xmm6 = (xmm2) as u128; rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rdi); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; rdi = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x60u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbp = ((crate::simd_ops::ru8(rsi.wrapping_add(0xa8u64))) as u32) as u32 as u64; fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x53u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { xmm1 = ((crate::simd_ops::ru32(P3)) as u32) as u128; rcx = (crate::simd_ops::ru64(rdi)) as u64; r8 = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { xmm2 = (xmm6) as u128; rdx = rbx; crate::simd_ops::wu8(rsi.wrapping_add(0xa8u64), ((rbp as u8)) as u8); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rcx = ((rax as u32)) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = 0x2; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rdx = 0; fl_l = 0; fl_r = 0; rax = 0x3; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x2; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc27a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba3170; const P1: u64 = 0x143c9a628; const P2: u64 = 0x141b5c760; const P3: u64 = 0x141bac780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; rbp = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu32(rsi.wrapping_add(0x60u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64)))) as u64; rax = 0x3fff; fl_l = (crate::simd_ops::ru16(rcx.wrapping_add(0x20u64))) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (0x3fffffffffffc000u64) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rcx.wrapping_add(0x28u64), (0x1u8) as u8); bb = 7; continue; }
            7 => { crate::simd_ops::wu64(rsi.wrapping_add(0xc0u64), (0x100u64) as u64); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { r8 = rsi.wrapping_add(0x78); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; r9 = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r9.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rcx = (((crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x191u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x70u64)))) as u64; rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0xa0u64))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x70u64))) as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu32(rsi.wrapping_add(0x98u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x68u64), (0x100u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc28e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1427bc100; const P1: u64 = 0x140242540; const P2: u64 = 0x1427bc120; const P3: u64 = 0x1402422e0; const P4: u64 = 0x141bc24a0; const P5: u64 = 0x1427bc128; const P6: u64 = 0x141b52810; const P7: u64 = 0x141bc3120; const P8: u64 = 0x141bac2d0; const P9: u64 = 0x14170f800; const P10: u64 = 0x141ba21f0; const P11: u64 = 0x143427080; const P12: u64 = 0x1427bc110; const P13: u64 = 0x140242410; const P14: u64 = 0x14323c9d8; const P15: u64 = 0x141bac240; const P16: u64 = 0x1403cd090; const P17: u64 = 0x1430f6d80; const P18: u64 = 0x1401de830; const P19: u64 = 0x143c9a628; const P20: u64 = 0x141b6be50;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 41>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bc28e0, funcinfo_rva: 0x373e0f0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x148, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r12) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r13) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rax.wrapping_sub(0x48u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0x110u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm9) as u128); xmm6 = (xmm2) as u128; rdi = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), (0x100u64) as u64); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; rax = ((rbp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((P0) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rbp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2948); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x68u64))) as u64 as u64; fl_r = (rax) as u64 as u64; r14 = (r14 & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), (rax) as u64); if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu32(rbx.wrapping_add(0x98u64), (0xffffffffu32) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; r9 = ((rbp.wrapping_add(0x50u64)) as u64) as u64; r8 = ((P2) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2989); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbx.wrapping_add(0x64u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x65u64), ((rax as u8)) as u8); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 89; continue; } bb = 3; continue; }
            3 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 89; continue; } bb = 4; continue; }
            4 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 15; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 106; continue; } bb = 6; continue; }
            6 => { r15 = (0x3fffu32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0xc0u64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 86; continue; } bb = 7; continue; }
            7 => { r12 = (0x3fffffffffffc000u64) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 86; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(4u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64)))) as u64; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rsi.wrapping_add(0x28u64), ((rcx as u8)) as u8); r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2a0f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 105; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru16(rsi.wrapping_add(0x20u64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 88; continue; } bb = 12; continue; }
            12 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2a36); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 105; continue; } bb = 13; continue; }
            13 => { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x66u64))) as u32) as u32 as u64; fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(0x2u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 14; continue; }
            14 => { r11 = ((rsp.wrapping_add(0x110u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x50u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r13 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r12 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            15 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x70u64)))) as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; r9 = ((rbp.wrapping_add(0x50u64)) as u64) as u64; r8 = ((P5) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2ab4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r11 = (((rax as u8)) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), ((rax as u8)) as u8); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xd4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 16; continue; }
            16 => { rcx = ((rbx.wrapping_add(0xccu64)) as u64) as u64; xmm1 = (xmm6) as u128; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2ad4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 20; continue; } bb = 17; continue; }
            17 => { rdx = (rdi) as u64; rcx = (rbx) as u64; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0xc8u64))) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2aef); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rdi) as u64; rcx = (rbx) as u64; bb = 19; continue; }
            19 => { r8 = (0x3u32) as u32 as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2b00); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r11 = ((crate::simd_ops::ru8(rbp.wrapping_add(0x50u64))) as u32) as u32 as u64; bb = 20; continue; }
            20 => { r15 = (0x3fffu32) as u32 as u64; r12 = (0x3fffffffffffc000u64) as u64; xmm6 = 0u128; fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 30; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2b2e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 32; continue; } bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x67u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 32; continue; } bb = 23; continue; }
            23 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 29; continue; } bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xd4u64))) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 88; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0xc8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 27; continue; } bb = 26; continue; }
            26 => { rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xccu64), (0x3f000000u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xd0u64), (0x3f000000u32) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xd4u64), ((r14 as u8)) as u8); rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            27 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x9c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 86; continue; } bb = 28; continue; }
            28 => { rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x9c8u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x98u64)) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx.wrapping_mul(8)), (0x17u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64), (r8) as u64); rax = (0x2u32) as u32 as u64; bb = 14; continue; }
            29 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0xfffffffdu32) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 88; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), (rax) as u64); r13 = (((crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r13 = ((r13).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x60u64)))) as u64; xmm7 = ((crate::simd_ops::ru32(rax.wrapping_add(0xc0u64))) as u32) as u128; xmm8 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); rax = (crate::simd_ops::ru64(r13)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; xmm0 = 0u128; xmm0 = crate::simd_ops::vblendps(xmm0, crate::simd_ops::ru128(rcx.wrapping_add(0x214u64)), 0x1u8); xmm0 = crate::simd_ops::vinsertps(xmm0, ((crate::simd_ops::ru32(rcx.wrapping_add(0x21cu64))) as u128), 0x10u8); xmm0 = crate::simd_ops::vinsertps(xmm0, ((crate::simd_ops::ru32(rcx.wrapping_add(0x218u64))) as u128), 0x20u8); xmm0 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(rbx.wrapping_add(0x80u64))); xmm9 = crate::simd_ops::vdpps(xmm0, xmm0, 0x7fu8); fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 46; continue; } bb = 31; continue; }
            31 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2c47); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 47; continue; }
            32 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x9c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x9c8u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x98u64)) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx.wrapping_mul(8)), (0x16u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64), (r8) as u64); bb = 34; continue; }
            34 => { fl_l = ((r11 as u8)) as u8 as u64; fl_r = ((r11 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 36; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), (0x1u32) as u32); bb = 30; continue; }
            36 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0xc0u64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 39; continue; } bb = 37; continue; }
            37 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 39; continue; } bb = 38; continue; }
            38 => { r8 = (0x4u32) as u32 as u64; rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2cba); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            39 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xb8u64))) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm6 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 41; continue; } bb = 40; continue; }
            40 => { crate::simd_ops::wu32(rbx.wrapping_add(0x60u64), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xb4u64), (0x0u8) as u8); bb = 30; continue; }
            41 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2cfe); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 105; continue; } bb = 42; continue; }
            42 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x66u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 86; continue; } bb = 43; continue; }
            43 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2d25); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x9c0u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x20u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 45; continue; } bb = 44; continue; }
            44 => { rcx = ((rax as u32)) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x9c0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rdx.wrapping_add(rcx.wrapping_mul(4)), (0x15u32) as u32); bb = 45; continue; }
            45 => { rax = (0x1u32) as u32 as u64; bb = 14; continue; }
            46 => { xmm0 = ((crate::simd_ops::ru32(P11)) as u32) as u128; bb = 47; continue; }
            47 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm9 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 54; continue; } bb = 48; continue; }
            48 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 54; continue; } bb = 49; continue; }
            49 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0xb8u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 54; continue; } bb = 50; continue; }
            50 => { fl_l = (crate::simd_ops::ru16(rbx.wrapping_add(0xc0u64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 52; continue; } bb = 51; continue; }
            51 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xc0u64))) as u64 as u64; fl_r = (r12) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 54; continue; } bb = 52; continue; }
            52 => { fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x50u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 54; continue; } bb = 53; continue; }
            53 => { r15 = (r15 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 55; continue; }
            54 => { r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 55; continue; }
            55 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; r9 = ((rbx.wrapping_add(0x64u64)) as u64) as u64; r8 = ((P2) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2db3); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x64u64))) as u32) as u32 as u64; fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 58; continue; } bb = 56; continue; }
            56 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 58; continue; } bb = 57; continue; }
            57 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 88; continue; } bb = 58; continue; }
            58 => { rcx = (crate::simd_ops::ru64(r13)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x190u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 88; continue; } bb = 59; continue; }
            59 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 61; continue; } bb = 60; continue; }
            60 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x64u64), ((rax as u8)) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x65u64), ((rax as u8)) as u8); bb = 61; continue; }
            61 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 64; continue; } bb = 62; continue; }
            62 => { r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0x90u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xf8u64))) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x9c8u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 64; continue; } bb = 63; continue; }
            63 => { rcx = ((rax as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x9c8u64), ((rax as u32)) as u32); rcx = ((rcx).wrapping_add(0x98u64)) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64 as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu32(rdx.wrapping_add(rcx.wrapping_mul(8)), (0x16u32) as u32); crate::simd_ops::wu64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x8u64), (r8) as u64); bb = 64; continue; }
            64 => { r14 = (r14 & !0xffu64) | ((((((r14 as u8)) ^ (0x1u8))) as u8 as u64)); r8 = (((r14 as u8)) as u32) as u32 as u64; rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2e3a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 85; continue; } bb = 65; continue; }
            65 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rax = ((rsp.wrapping_add(0x50u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((P12) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2e78); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P13) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; fl_l = (crate::simd_ops::ru8(r13.wrapping_add(0x52u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 73; continue; } bb = 66; continue; }
            66 => { rdx = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(8)))) as u32) as u128; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 71; continue; } bb = 67; continue; }
            67 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 70; continue; } bb = 68; continue; }
            68 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 72; continue; } bb = 69; continue; }
            69 => { xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(core::ptr::read_unaligned((rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x10u64)) as *const u32))).to_bits() as u128); bb = 72; continue; }
            70 => { xmm1 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(core::ptr::read_unaligned((rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64)) as *const u32))).to_bits() as u128); bb = 72; continue; }
            71 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); bb = 72; continue; }
            72 => { rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rcx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), (rcx) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x78u64), ((rcx as u8)) as u8); rax = ((P14) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (0x100u64) as u64); crate::simd_ops::wu32(rbp.wrapping_sub(0x58u64), ((xmm1 as u32)) as u32); r8 = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2f09); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P15) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 83; continue; }
            73 => { xmm2 = ((crate::simd_ops::ru32(r14.wrapping_add(0x3cu64))) as u32) as u128; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm1) as u128); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x2u32) as u32); r9 = (crate::simd_ops::ru32(r14.wrapping_add(0x38u64))) as u32 as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2f5f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P16) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu8(rbp.wrapping_sub(0x33u64), (0x1u8) as u8); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 77; continue; } bb = 74; continue; }
            74 => { bb = 75; continue; }
            75 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 81; continue; } bb = 76; continue; }
            76 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 75; continue; } bb = 77; continue; }
            77 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 82; continue; } bb = 78; continue; }
            78 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 81; continue; } bb = 79; continue; }
            79 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 78; continue; } bb = 80; continue; }
            80 => { bb = 82; continue; }
            81 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2fbd); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 82; continue; }
            82 => { rax = ((P17) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rax) as u64); rcx = ((rbp.wrapping_sub(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc2fd1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P18) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 83; continue; }
            83 => { fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x40u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 88; continue; } bb = 84; continue; }
            84 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm0) as u128); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x48u64))) as u64; r9 = ((rbx.wrapping_add(0x78u64)) as u64) as u64; r10 = ((crate::simd_ops::ru16(P19)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); r8 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(r10.wrapping_mul(8)))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc3021); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P20) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            85 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x66u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 87; continue; } bb = 86; continue; }
            86 => { rax = (0x2u32) as u32 as u64; bb = 14; continue; }
            87 => { rax = (crate::simd_ops::ru64(r13)) as u64; crate::simd_ops::wu8(rax.wrapping_add(0x191u64), (0x1u8) as u8); bb = 88; continue; }
            88 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            89 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 92; continue; } bb = 90; continue; }
            90 => { crate::simd_ops::wu8(rbp.wrapping_add(0x50u64), (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; r9 = ((rbp.wrapping_add(0x50u64)) as u64) as u64; r8 = ((P5) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc3074); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 92; continue; } bb = 91; continue; }
            91 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 93; continue; }
            92 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 93; continue; }
            93 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x60u64))) as u32 as u64; fl_r = (0x2u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 100; continue; } bb = 94; continue; }
            94 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xb0u64))) as u32) as u128; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xb4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 96; continue; } bb = 95; continue; }
            95 => { xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; bb = 98; continue; }
            96 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm4 = 0u128; xmm2 = 0u128; xmm1 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0xb0u64), ((xmm1 as u32)) as u32); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 98; continue; } bb = 97; continue; }
            97 => { crate::simd_ops::wu8(rbx.wrapping_add(0xb4u64), (0x1u8) as u8); bb = 98; continue; }
            98 => { rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 100; continue; } bb = 99; continue; }
            99 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 101; continue; }
            100 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 101; continue; }
            101 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 104; continue; } bb = 102; continue; }
            102 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 104; continue; } bb = 103; continue; }
            103 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 106; continue; } bb = 104; continue; }
            104 => { r14 = (r14 & !0xffu64) | ((((((r14 as u8)) ^ (0x1u8))) as u8 as u64)); r8 = (((r14 as u8)) as u32) as u32 as u64; rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc30f1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 13; continue; } bb = 105; continue; }
            105 => { r8 = (0x3u32) as u32 as u64; rdx = (rdi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc310a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 106; continue; }
            106 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 14; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc3120(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8d08c; const P1: u64 = 0x1401f9650; const P2: u64 = 0x14054cab0; const P3: u64 = 0x1403cd370; const P4: u64 = 0x141b482e0; const P5: u64 = 0x1430f6d50; const P6: u64 = 0x141bac380; const P7: u64 = 0x143c9a628; const P8: u64 = 0x14040f5f0; const P9: u64 = 0x1434264ec; const P10: u64 = 0x14323c9d8; const P11: u64 = 0x141b6bc40;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bc3120, funcinfo_rva: 0x373e144, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xe8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = (rsp) as u64; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x60u64), ((r8 as u32)) as u32); fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x2u32) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } bb = 1; continue; }
            1 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; r8 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } bb = 2; continue; }
            2 => { fl_l = ((r8 as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 3; continue; }
            3 => { rsi = ((rcx.wrapping_add(0xc0u64)) as u64) as u64; rax = (0x3fffu32) as u32 as u64; fl_l = (crate::simd_ops::ru16(rsi)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 26; continue; } bb = 4; continue; }
            4 => { rax = (0x3fffffffffffc000u64) as u64; fl_l = (crate::simd_ops::ru64(rsi)) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 26; continue; } bb = 5; continue; }
            5 => { rdx = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; r8 = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((r11.wrapping_add(0x8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc31ab); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = (0xffffffffu32) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 6; continue; }
            6 => { rdx = (rbx) as u64; rcx = ((rdi.wrapping_add(0xd0u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc31cb); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 7; continue; }
            7 => { rdx = (rbx) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd0u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc31e2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (rax) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xa8u64))) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rdx = ((r8.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rdx = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; r8 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0xa0u64)))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (rsi) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc3221); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x70u64)))) as u64; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0xa8u64), ((rax as u8)) as u8); rdx = ((P5) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (rdx) as u64); rdx = ((rsp.wrapping_add(0x90u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc326d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 26; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rbx.wrapping_add(0x20u64), (0x2u32) as u32); bb = 26; continue; }
            10 => { crate::simd_ops::wu64(rsi, (0x100u64) as u64); bb = 26; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0xc8u64))) as u64; rbp = ((r9.wrapping_add(r9.wrapping_mul(2))) as u64) as u64; rbp = ((rbp).wrapping_shl(5u32)) as u64; rbp = ((rbp).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x70u64)))) as u64; r14 = (((r9) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x60u64)))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; r8 = ((crate::simd_ops::ru16(P7)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x580e0u64))) as u64; r15 = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xc0u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc32e7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = (crate::simd_ops::ru8(r14.wrapping_add(0x52u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xb8u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rcx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rdx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } bb = 15; continue; }
            15 => { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0xc0u64))) as u128; xmm1 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(rbx.wrapping_add(0x80u64))); xmm4 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); xmm2 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32) as u128; xmm3 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(core::ptr::read_unaligned((P9) as *const u32)) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(core::ptr::read_unaligned((P9) as *const u32)) }).to_bits() as u128); fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu8(rbx.wrapping_add(0x67u64), (0x1u8) as u8); bb = 26; continue; }
            17 => { rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; xmm1 = ((crate::simd_ops::ru32(rax.wrapping_add(r8.wrapping_mul(8)))) as u32) as u128; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 22; continue; } bb = 18; continue; }
            18 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 21; continue; } bb = 19; continue; }
            19 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 20; continue; }
            20 => { xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x10u64)) as *const u32))).to_bits() as u128); bb = 23; continue; }
            21 => { xmm0 = 0u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(core::ptr::read_unaligned((rax.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x8u64)) as *const u32))).to_bits() as u128); bb = 23; continue; }
            22 => { xmm0 = 0u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); bb = 23; continue; }
            23 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0xa0u64), (rax) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0xa8u64), ((rax as u8)) as u8); rcx = ((P10) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), (rcx) as u64); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), (0x100u64) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0xc8u64), ((xmm1 as u32)) as u32); rdx = ((rsp.wrapping_add(0x90u64)) as u64) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc33ee); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x70u64), (rax) as u64); bb = 26; continue; }
            24 => { xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); rax = ((rbx.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rbx.wrapping_add(0x78u64)) as u64) as u64; rdx = ((rsi.wrapping_add(0x48u64)) as u64) as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x80u64), (0x1u8) as u8); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdx) as u64); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rsi.wrapping_add(0x40u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x58u64), ((rax as u8)) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), (0x2u32) as u32); xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x3cu64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((rax as u32)) as u32); rax = ((rsp.wrapping_add(0x90u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rax) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (r15) as u64); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (r14) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc3489); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((((rax as u8)) ^ (0x1u8))) as u8 as u64; fl_r = fl_l; rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(0x67u64), ((rax as u8)) as u8); bb = 26; continue; }
            25 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0xb8u64))) as u32) as u128; crate::simd_ops::wu32(rcx.wrapping_add(0xacu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0xb0u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0xb4u64), (0x0u8) as u8); bb = 26; continue; }
            26 => { r11 = ((rsp.wrapping_add(0xd0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = (r11) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc34e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba31e0; const P1: u64 = 0x1427bc120; const P2: u64 = 0x1402422e0; const P3: u64 = 0x141bc2200; const P4: u64 = 0x141ba21f0; const P5: u64 = 0x1427bc100; const P6: u64 = 0x1402b8940; const P7: u64 = 0x140242540; const P8: u64 = 0x141bc3120;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; r9 = rsp.wrapping_add(0x50); r8 = P1; crate::simd_ops::wu8(rsp.wrapping_add(0x50u64), (0x1u8) as u8); rcx = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x68u64)))) as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(0x3eu64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbp.wrapping_add(0xa8u64), ((rax as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_add(0x67u64), (0x0u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(0x34u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(0x3cu64), ((rax as u8)) as u8); crate::simd_ops::wu8(rcx.wrapping_add(0x3fu64), ((rax as u8)) as u8); crate::simd_ops::wu32(rbp.wrapping_add(0x98u64), (0xffffffffu32) as u32); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rdi; crate::simd_ops::wu8(rbp.wrapping_add(0x64u64), ((rax as u8)) as u8); rcx = rbp; crate::simd_ops::wu8(rbp.wrapping_add(0x65u64), ((rax as u8)) as u8); rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; r8 = P5; rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = rbx; rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = rsp.wrapping_add(0x50); r9 = P5; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rbx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x100u64) as u64); rcx = rsp.wrapping_add(0x58); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r8 = 0x3; rdx = rdi; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rcx); rcx = rbp; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc3ba0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0x41; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x60; const P10: u64 = 0x64; const P11: u64 = 0x65;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P7), rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128; crate::simd_ops::wu128(r8.wrapping_add(P8), (xmm0) as u128); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P9))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P9), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P10))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(P10), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P11))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x65u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x66u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x66u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x67u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0x67u64), ((rax as u8)) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x70u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x78u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x78u64), rax); xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(0x80u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(0x80u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x90u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0x98u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0xa0u64), rax); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0xa8u64))) as u32) as u32 as u64; crate::simd_ops::wu8(r8.wrapping_add(0xa8u64), ((rax as u8)) as u8); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0xacu64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0xacu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xb4u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0xb4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xb8u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0xb8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0xc0u64), rax); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xc8u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0xc8u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(0xccu64))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(0xccu64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xd4u64))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(0xd4u64), ((rax as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc3e70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x38; const P2: u64 = 0x60; const P3: u64 = 0x18; const P4: u64 = 0x10;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (r10 as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((r9 as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0; fl_l = 0; fl_r = 0; rdx = 0x3; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rax = (if (((fl_l as u32)) != ((fl_r as u32))) { (rdx as u32) } else { (rax as u32) }) as u32 as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc4870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba31e0; const P1: u64 = 0x1c; const P2: u64 = 0x8; const P3: u64 = 0xc8; const P4: u64 = 0x60; const P5: u64 = 0x70; const P6: u64 = 0x52; const P7: u64 = 0x38; const P8: u64 = 0x143222d20; const P9: u64 = 0x141bac380;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = rdx; rbp = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rcx = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P4)))) as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P5)))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P6))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P7))) as u32) as u128; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); crate::simd_ops::wu8(rsp.wrapping_add(0x38u64), ((rcx as u8)) as u8); rax = P8; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); rdx = rsp.wrapping_add(0x20); rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rax); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc50d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xd0; const P1: u64 = 0x70; const P2: u64 = 0x143c9a628; const P3: u64 = 0xd8; const P4: u64 = 0x580e0; const P5: u64 = 0x141b5c760; const P6: u64 = 0x143427080; const P7: u64 = 0x141b5b7d0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdi = ((rcx.wrapping_add(P1)) as u64) as u64; r11 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(P3)) as u64) as u64; rbp = ((r9 as u32)) as u32 as u64; r14 = r8; r10 = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; rax = 0xffffffff; rsi = (crate::simd_ops::ru64(r10.wrapping_add(r11.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru64(rdi)) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rdi; rcx = rsi; rdx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { xmm0 = (crate::simd_ops::ru128(r14)) as u128; rdx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsp.wrapping_add(0x30); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru32(P6)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), (0x1u32) as u32); r9 = ((rbp as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = rsi; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rdi, rax); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc5190(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bac2d0; const P1: u64 = 0x1405e37f0; const P2: u64 = 0x1405e3070; const P3: u64 = 0x1406939e0; const P4: u64 = 0x14323c9d8; const P5: u64 = 0x141bac380; const P6: u64 = 0x141bc50d0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bc5190, funcinfo_rva: 0x373e2f0, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm6) as u128); rbx = (rdx) as u64; rdi = (rcx) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xc8u64))) as u64; rsi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rsi).wrapping_shl(5u32)) as u64; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x70u64)))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x60u64))) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc51ce); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r11 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu8(rdi.wrapping_add(0x68u64), (0x0u8) as u8); rbp = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r8 = ((rbp.wrapping_add(0x40u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc5206); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rbp.wrapping_add(0x40u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc5241); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsp.wrapping_add(0xc0u64), (0x100u64) as u64); r9 = ((rbp.wrapping_add(0xc0u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; rax = ((rsp.wrapping_add(0xc0u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0xc8u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc527f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x3cu64))) as u32) as u128; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rcx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rcx) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rcx as u8)) as u8); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x88u64), ((xmm0 as u32)) as u32); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc52d2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rdi.wrapping_add(0x60u64), (rax) as u64); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0xb8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0xbcu64))) as u32 as u64; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = (rbx) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc52fc); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 5; continue; }
            5 => { rax = (0x3u32) as u32 as u64; bb = 8; continue; }
            6 => { rax = (0x2u32) as u32 as u64; bb = 8; continue; }
            7 => { rax = (0x3u32) as u32 as u64; rcx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_l = ((r11 as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; rax = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (rax as u32) }) as u32 as u64; bb = 8; continue; }
            8 => { r11 = ((rsp.wrapping_add(0xa0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc5350(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba3170; const P1: u64 = 0x1c; const P2: u64 = 0x10; const P3: u64 = 0xb8; const P4: u64 = 0xd0; const P5: u64 = 0xd8; const P6: u64 = 0x143c9a628; const P7: u64 = 0x580e0; const P8: u64 = 0x141b5c760; const P9: u64 = 0xc8; const P10: u64 = 0x70; const P11: u64 = 0x141bac780;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbp = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rdi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; r8 = rbp.wrapping_add(0x70); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; r9 = ((crate::simd_ops::ru16(P6)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(r9.wrapping_mul(8)))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P10)))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc5410(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba31e0; const P1: u64 = 0x1406939e0; const P2: u64 = 0x1405e37f0; const P3: u64 = 0x1405e3070; const P4: u64 = 0x14323c9d8; const P5: u64 = 0x141bac380; const P6: u64 = 0x141bc50d0; const P7: u64 = 0x143222d20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bc5410, funcinfo_rva: 0x373e340, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xc8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rbp = ((rsp.wrapping_sub(0x37u64)) as u64) as u64; rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm6) as u128); rdi = (rdx) as u64; rsi = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc5437); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x58u64))) as u64; r14 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc545f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (r14) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rcx = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x60u64)))) as u64; r14 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((r14).wrapping_shl(5u32)) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x70u64)))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x52u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu8(rsi.wrapping_add(0x68u64), ((rax as u8)) as u8); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x67u64), (0x100u64) as u64); r9 = ((rbx.wrapping_add(0xc0u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rax = ((rbp.wrapping_add(0x67u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rbp.wrapping_add(0x6fu64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc54c4); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; r8 = ((rbx.wrapping_add(0x40u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc54de); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x68u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 7; continue; }
            7 => { xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rax = ((rbp.wrapping_sub(0x39u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rbx.wrapping_add(0x40u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc5520); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x3cu64))) as u32) as u128; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rdx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (rdx) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x1u64), ((rdx as u8)) as u8); rax = ((P4) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rax) as u64); xmm6 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm6) as u128); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x6fu64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x1fu64), ((xmm0 as u32)) as u32); rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc5560); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), (rax) as u64); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xb8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm6) as u128); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(0xbcu64))) as u32 as u64; r8 = ((rbp.wrapping_sub(0x29u64)) as u64) as u64; rdx = (rdi) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc5588); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 10; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32) as u128; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), (rdx) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), (rdx) as u64); crate::simd_ops::wu8(rbp.wrapping_sub(0x1u64), ((rdx as u8)) as u8); rax = ((P7) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x19u64), (rax) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0x7u64), ((xmm0 as u32)) as u32); rdx = ((rbp.wrapping_sub(0x19u64)) as u64) as u64; rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bc55b8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi.wrapping_add(0x60u64), (rax) as u64); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; rsp = ((rsp).wrapping_add(0xa0u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc5b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0x41; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x60; const P10: u64 = 0x68; const P11: u64 = 0x70;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P8), (xmm0) as u128);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P9), rax);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P10))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P10), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P11))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(0x70u64), rax);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc5c00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14323d330; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x70; const P9: u64 = 0x40; const P10: u64 = 0x48; const P11: u64 = 0x50;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = 0xffffffff;
    crate::simd_ops::wu64(rdx.wrapping_add(P8), rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P9), ((rcx as u16)) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x58u64), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rcx);
    crate::simd_ops::wu8(rdx.wrapping_add(0x68u64), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc5da0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x6c; const P1: u64 = 0x68; const P2: u64 = 0x64;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = (((rax as u32)).wrapping_add(0x3u32)) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128);
    xmm2 = 0u128;
    xmm1 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128);
    xmm4 = 0u128;
    fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32);
    if !(!(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8); }
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    fl_l = 0; fl_r = ((rax as u8)) as u8 as u64;
    rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64));
    rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(0x3u32)) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc5e00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba31e0; const P1: u64 = 0x50; const P2: u64 = 0x58; const P3: u64 = 0x1c; const P4: u64 = 0x8; const P5: u64 = 0x48; const P6: u64 = 0x6c; const P7: u64 = 0x3c; const P8: u64 = 0x60; const P9: u64 = 0x64; const P10: u64 = 0x68;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = rdx; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rsi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbp; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P6), (0x1u8) as u8); xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P7))) as u32) as u128; crate::simd_ops::wu32(rdi.wrapping_add(P8), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rdi.wrapping_add(P9), ((xmm0 as u32)) as u32); crate::simd_ops::wu8(rdi.wrapping_add(P10), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc63f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0x41; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x60; const P10: u64 = 0x68; const P11: u64 = 0x6c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P8), (xmm0) as u128);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P9))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P9), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P10))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P10), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P11))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(0x6cu64), ((rax as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc64a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14323d838; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P8), ((rcx as u16)) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(0x60u64), rcx);
    crate::simd_ops::wu8(rdx.wrapping_add(0x68u64), (0x1u8) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(0x6cu64), ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc66b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x2c; const P1: u64 = 0x48; const P2: u64 = 0xd8; const P3: u64 = 0xe8; const P4: u64 = 0xc8; const P5: u64 = 0x140631990; const P6: u64 = 0x10; const P7: u64 = 0x50; const P8: u64 = 0x14062eaf0; const P9: u64 = 0x1b8; const P10: u64 = 0x140673040; const P11: u64 = 0x14062d1c0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64; r9 = rsp.wrapping_add(0x30); rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = r8; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rdi = rdx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; r8 = ((rbx.wrapping_add(P4)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; r9 = ((rsi.wrapping_add(P6)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; r8 = ((rbx.wrapping_add(P7)) as u64) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rsi)) as u64; fl_r = (0xffffffffffffffffu64) as u64; r8 = ((rbx.wrapping_add(P9)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { r9 = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P11, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc6770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c9a628; const P1: u64 = 0x140631990; const P2: u64 = 0x14062eaf0; const P3: u64 = 0x140673040; const P4: u64 = 0x14062d1c0; const P5: u64 = 0x141b5c810; const P6: u64 = 0x14062e8e0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xd0u64))) as u64; rbx = rdx; r9 = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; r8 = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xc8u64))) as u64; r14 = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x88u64)))) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x7cu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xacu64))) as u32 as u64; r8 = rsi.wrapping_add(0xc8); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rbp); rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; r9 = rsp.wrapping_add(0x60); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xe8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r9 = rdi.wrapping_add(0x90); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; r8 = rsi.wrapping_add(0x50); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x80u64))) as u64; fl_r = (0xffffffffffffffffu64) as u64; r8 = rsi.wrapping_add(0x1b8); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = rdi.wrapping_add(0x80); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            3 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r9 = rsi.wrapping_add(0x50); xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x48u64))) as u32) as u128; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0xb0u64))) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rcx = r14; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = (rax & !0xffu64) | (((if (((fl_l as u8)) == ((fl_r as u8))) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x44u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r8 = rsi.wrapping_add(0x230); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; r9 = rsp.wrapping_add(0x60); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0x1u8) as u8); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rax = 0x2; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = 0x3; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            7 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; r8 = rsi.wrapping_add(0x230); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xe8u64))) as u64; r9 = rsp.wrapping_add(0x60); crate::simd_ops::wu8(rsp.wrapping_add(0x60u64), (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc6930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba3170; const P1: u64 = 0x50; const P2: u64 = 0x58; const P3: u64 = 0x1c; const P4: u64 = 0x10; const P5: u64 = 0xc8; const P6: u64 = 0x88; const P7: u64 = 0x70; const P8: u64 = 0x7c;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rdx; rdi = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P6)))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rcx, (0xffffffffffffffffu64) as u64); crate::simd_ops::wu64(rcx.wrapping_add(P7), (0x0u64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(P8), (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc69c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba31e0; const P1: u64 = 0x143c8d910; const P2: u64 = 0x140265ba0; const P3: u64 = 0x143426d28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r14 = rdx; rbp = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0xd0u64))) as u64; rdx = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rbp = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x580e0u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0xc8u64))) as u64; rbx = (((rax) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)))) as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x60u64)))) as u64; rdi = ((rdi).wrapping_shl(6u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(r14.wrapping_add(0x88u64)))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x68u64), (0x1u8) as u8); crate::simd_ops::wu32(rdi.wrapping_add(0x58u64), (0x0u32) as u32); rax = (crate::simd_ops::ru64(r14.wrapping_add(0xc0u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x8u64), rcx); rcx = rdi.wrapping_add(0x18); rbx = (crate::simd_ops::ru64(rbx)) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x58u64))) as u32) as u32 as u64; rdx = rbx.wrapping_add(0x60); crate::simd_ops::wu8(rdi.wrapping_add(0x10u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x59u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x11u64), ((rax as u8)) as u8); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x5au64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x12u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x70u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x28u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax); xmm0 = ((crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64) as u128; xmm0 = (xmm0 & !0xffff_ffff_ffff_ffffu128) | ((f64::from_bits(xmm0 as u64) * f64::from_bits(crate::simd_ops::ru64(P3))).to_bits() as u128); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (crate::simd_ops::cvt_f64_i64((f64::from_bits(xmm0 as u64)))) as u64; crate::simd_ops::wu64(rdi, rax); crate::simd_ops::wu32(rdi.wrapping_add(0x7cu64), (0x2u32) as u32); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc72f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x48; const P2: u64 = 0x14388e6a8; const P3: u64 = 0x14388e6e0; const P4: u64 = 0x14254768e; const P5: u64 = 0x10; const P6: u64 = 0x60; const P7: u64 = 0x41;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r9 = P2; r8 = P3; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdi as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_r = rdi; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu8(rbx.wrapping_add(P6), (0x1u8) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::wu8(rbx.wrapping_add(P7), (0x1u8) as u8); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc7360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x48; const P2: u64 = 0x14388e6a8; const P3: u64 = 0x14388e6e0; const P4: u64 = 0x14254768e; const P5: u64 = 0x38; const P6: u64 = 0xf0; const P7: u64 = 0x141bcc870; const P8: u64 = 0x10; const P9: u64 = 0x60; const P10: u64 = 0x41;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rdx; rdi = rcx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbp = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; r9 = P2; r8 = P3; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rbp as u32)) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; fl_l = ((((rax as u32)) & (0x2u32))) as u32 as u64; fl_r = fl_l; rax = ((fl_l as u32)) as u32 as u64; bb = 10; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P8))) as u64; fl_r = rbp; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu8(rbx.wrapping_add(P9), (0x1u8) as u8); rax = 0x2; crate::simd_ops::wu8(rbx.wrapping_add(P10), (0x1u8) as u8); bb = 10; continue; }
            9 => { rax = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc8230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1430aa288; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P2), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc83a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; r9 = ((rdx.wrapping_mul(8)) as u64) as u64; rax = rbx; r10 = rdx; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; rdx = 0x8; rdx = ((rdx).wrapping_sub(rax)) as u64; r11 = rcx; rcx = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; rdx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { rdx }) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = rcx; rax = ((rax).wrapping_sub(rdx)) as u64; fl_l = rax; fl_r = r9; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_sub(r9)) as u64; rcx = ((rdx.wrapping_add(rbx)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); rax = ((rcx.wrapping_add(r9)) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), rax); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp, r10); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); xmm0 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), r10); xmm1 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu128(r11.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(r11.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = rcx; fl_r = r9; rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), r9); crate::simd_ops::wu64(r8.wrapping_add(P1), rcx); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc89f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rdx;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc8a50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbp = rdx;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi);
    loop { rsi = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc8ad0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = r8;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rbp = rdx;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsi; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc8b50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x20; const P2: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    rsi = rdx;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc8c10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = ((rdx as u32)) as u32 as u64;
    rax = (((xmm1 >> 64) as u64)) as u64;
    rbx = (((xmm1 >> 64) as u64)) as u64;
    rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdi;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rsi as u32)) as u32 as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rsi as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = ((rsi as u32)) as u32 as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc9630(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x60; const P2: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) <= ((fl_r as u64))) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r9 = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; return crate::simd_ops::call_fwd(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc9bc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x20; const P3: u64 = 0x28; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0x41; const P7: u64 = 0x48; const P8: u64 = 0x50; const P9: u64 = 0x60; const P10: u64 = 0x68;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P3))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P3), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P4), ((xmm1 as u64)) as u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P5))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P5), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P7))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P7), rax);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P8))) as u128;
    crate::simd_ops::wu128(r8.wrapping_add(P8), (xmm0) as u128);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P9))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P9), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P10))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P10), rax);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bc9c60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14323e570; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x40; const P9: u64 = 0x48; const P10: u64 = 0x50; const P11: u64 = 0x58;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rcx);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P8), ((rcx as u16)) as u16);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rcx);
    crate::simd_ops::wu8(rdx.wrapping_add(0x60u64), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bca200(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140695f90; const P1: u64 = 0x1425371a0; const P2: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; rsp = ((rsp).wrapping_sub(0x68u64)) as u64; rax = rdx; rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rdx); crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdx); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rdx); crate::simd_ops::wu64(r11.wrapping_sub(0x10u64), rdx); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x48u64), rdx); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rcx as u32)) as u32); r8 = r11.wrapping_sub(0x28); rdx = r11.wrapping_sub(0x38); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            13 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 15; continue; }
            14 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 15; continue; }
            15 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rsp = ((rsp).wrapping_add(0x68u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bca3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e7c10; const P1: u64 = 0x140695480; const P2: u64 = 0x141bcc870; const P3: u64 = 0x14183acc8; const P4: u64 = 0x141ba33f0; const P5: u64 = 0x141bcab00; const P6: u64 = 0x141bcbf60; const P7: u64 = 0x14388e6a8; const P8: u64 = 0x14388e6e0; const P9: u64 = 0x14254768e; const P10: u64 = 0x143a082b8; const P11: u64 = 0x143a49050; const P12: u64 = 0x142421ed0; const P13: u64 = 0x140695810; const P14: u64 = 0x141bcc280;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 57>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r9); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x98); rsp = ((rsp).wrapping_sub(0x198u64)) as u64; r11 = (crate::simd_ops::ru64(rcx)) as u64; r14 = r8; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; r11 = ((r11).wrapping_add(0x40u64)) as u64; r8 = rsp.wrapping_add(0x70); rsi = rdx; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), r11); rdx = r11; rcx = rbp.wrapping_sub(0x58); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x58u64))) as u128; rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64; r12 = 0; fl_l = 0; fl_r = 0; xmm0 = ((crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64))) as u64) as u128; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); r9 = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); crate::simd_ops::wu128(rbp.wrapping_add(0x28u64), (xmm1) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru32(r11.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; r12 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); bb = 5; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); r12 = ((rcx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); bb = 5; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x1d8u64), rdi); fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 6; continue; }
            6 => { r9 = ((crate::simd_ops::ru16(r9)) as u32) as u32 as u64; rcx = rbp.wrapping_sub(0x40); r8 = r11; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 7; continue; }
            7 => { fl_l = ((rdi).wrapping_add(rbx)) as u64; fl_r = fl_l; rdi = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); bb = 10; continue; }
            10 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 13; continue; }
            13 => { fl_l = ((rcx).wrapping_add(rbx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 15; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); bb = 15; continue; }
            15 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); bb = 16; continue; }
            16 => { fl_l = r12; fl_r = r12; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 67; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x190u64), r13); r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); bb = 18; continue; }
            18 => { rax = ((r8.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; r12 = r15; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = ((rdi.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x78u64), rax); rax = (((r15) as i64).wrapping_mul((0x70u64) as i64) as u64) as u64; r12 = ((r12).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; r12 = ((r12).wrapping_add(rcx)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xd0u64), rax); rbx = (crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 28; continue; } bb = 19; continue; }
            19 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; r15 = rbx.wrapping_add(0x20); rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0xe8u64))) as u64; bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(r15.wrapping_sub(0x18u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { r8 = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; rdx = r15; r8 = ((r8).wrapping_shl(5u32)) as u64; rcx = rbx; r8 = ((r8).wrapping_sub(r15)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r12.wrapping_add(0x10u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(r12.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; bb = 26; continue; }
            23 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), r14); rcx = rsp.wrapping_add(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), r13); rdx = (crate::simd_ops::ru64(r15.wrapping_sub(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = r15.wrapping_sub(0x10); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; rdx = r15; r8 = ((r8).wrapping_shl(5u32)) as u64; rcx = rbx; r8 = ((r8).wrapping_sub(r15)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r12.wrapping_add(0x10u64)))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(r12.wrapping_add(0x18u64), (1) as u32)) as u32 as u64; bb = 26; continue; }
            25 => { rbx = ((rbx).wrapping_add(0x20u64)) as u64; fl_l = ((r15).wrapping_add(0x20u64)) as u64; fl_r = fl_l; r15 = ((fl_l as u64)) as u64; bb = 26; continue; }
            26 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r12.wrapping_add(0x10u64)))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 27; continue; }
            27 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 28; continue; }
            28 => { rbx = (crate::simd_ops::ru64(r12)) as u64; rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 58; continue; } bb = 29; continue; }
            29 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x78u64))) as u64; bb = 30; continue; }
            30 => { rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 31; continue; }
            31 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 32; continue; }
            32 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; r9 = P7; r8 = P8; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 36; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 36; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru8(r15.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 36; continue; } bb = 35; continue; }
            35 => { fl_l = ((rbx).wrapping_add(0x10u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 56; continue; }
            36 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = rdx.wrapping_sub(0x1); fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 55; continue; } bb = 37; continue; }
            37 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 38; continue; }
            38 => { bb = 39; continue; }
            39 => { r15 = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 68; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 39; continue; } bb = 41; continue; }
            41 => { r15 = (crate::simd_ops::ru64(rbp.wrapping_add(0xe8u64))) as u64; bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru64(P10)) as u64; r8 = rdx; rdx = P11; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 69; continue; } bb = 43; continue; }
            43 => { r8 = rsp.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rdx = rbp.wrapping_sub(0x58); rcx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), (0x1u64) as u64); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 44; continue; }
            44 => { fl_l = r15; fl_r = r15; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 45; continue; }
            45 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rsi); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), r13); rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x48u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 55; continue; } bb = 46; continue; }
            46 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 47; continue; }
            47 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; r9 = P7; r8 = P8; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 48; continue; }
            48 => { fl_l = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 54; continue; } bb = 49; continue; }
            49 => { rax = (crate::simd_ops::ru64(r15.wrapping_add(0xb8u64))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 50; continue; }
            50 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(0x30u64))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 54; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 52; continue; }
            52 => { r9 = 0x1; rcx = rbp.wrapping_sub(0x40); rdx = r15; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), r13); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r13.wrapping_add(0x10u64), (0x1u64) as u64); crate::simd_ops::wu64(r13.wrapping_add(0x18u64), rcx); crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r13.wrapping_add(0x68u64), rax); crate::simd_ops::wu8(r13.wrapping_add(0x41u64), (0x1u8) as u8); bb = 54; continue; }
            54 => { r13 = (crate::simd_ops::ru64(rbp.wrapping_add(0xd0u64))) as u64; bb = 55; continue; }
            55 => { r8 = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rdx = rbx.wrapping_add(0x10); r8 = ((r8).wrapping_shl(4u32)) as u64; rcx = rbx; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r12))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(r12.wrapping_add(0x8u64), (1) as u32)) as u32 as u64; bb = 56; continue; }
            56 => { rax = (crate::simd_ops::ru32(r12.wrapping_add(0x8u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(r12))) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 30; continue; } bb = 57; continue; }
            57 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0xe0u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 58; continue; }
            58 => { r15 = ((r15).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); fl_l = r15; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 64; continue; } bb = 59; continue; }
            59 => { r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64))) as u64; r13 = ((r13).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); fl_l = r13; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 64; continue; } bb = 60; continue; }
            60 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; bb = 61; continue; }
            61 => { rax = ((crate::simd_ops::ru16(r13)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 63; continue; } bb = 62; continue; }
            62 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(rax.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 70; continue; } bb = 63; continue; }
            63 => { r13 = ((r13).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), r13); fl_l = r13; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 61; continue; } bb = 64; continue; }
            64 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; bb = 65; continue; }
            65 => { fl_l = r15; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 66; continue; }
            66 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x190u64))) as u64; bb = 67; continue; }
            67 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x1d8u64))) as u64; rsp = ((rsp).wrapping_add(0x198u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            68 => { fl_l = (crate::simd_ops::lk_add_u64(rax.wrapping_add(0x18u64), (1) as u64)) as u64; bb = 44; continue; }
            69 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x0u64) as u64); bb = 55; continue; }
            70 => { fl_l = r13; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 64; continue; } bb = 71; continue; }
            71 => { r9 = ((crate::simd_ops::ru16(r13)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x60u64))) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 73; continue; } bb = 72; continue; }
            72 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 74; continue; }
            73 => { rax = 0; fl_l = 0; fl_r = 0; bb = 74; continue; }
            74 => { r8 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); r15 = 0; fl_l = 0; fl_r = 0; rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 76; continue; } bb = 75; continue; }
            75 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 77; continue; }
            76 => { rdi = ((rbx.wrapping_add(rax)) as u64) as u64; bb = 77; continue; }
            77 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 79; continue; } bb = 78; continue; }
            78 => { fl_l = ((r8).wrapping_add(rbx)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r8); bb = 79; continue; }
            79 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 81; continue; } bb = 80; continue; }
            80 => { fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx); bb = 81; continue; }
            81 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 65; continue; } bb = 82; continue; }
            82 => { fl_l = ((rcx).wrapping_add(rbx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); bb = 65; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bca9a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x140696a30; const P4: u64 = 0x18;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rsi = r9; rbx = r8; rdi = ((rcx.wrapping_add(P0)) as u64) as u64; r10 = (crate::simd_ops::ru64(rdi)) as u64; rax = r10; r11 = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; r8 = ((r11 as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(r10)) as u64; fl_l = r10; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x20u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(r10)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = r11.wrapping_add(0x1); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::lk_add_u32(rdi.wrapping_add(P1), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(0xffffffffffffffe0u64)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rdi))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rax.wrapping_add(P1), (0x2u32) as u32); crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P4), rsi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcaa60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438cdfb0; const P1: u64 = 0x1438c4a10; const P2: u64 = 0x14254768e; const P3: u64 = 0x68; const P4: u64 = 0xffffffffffffffff; const P5: u64 = 0x141bcab00; const P6: u64 = 0x18; const P7: u64 = 0x10; const P8: u64 = 0x141bcaa60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rsi = rcx; r9 = P0; rcx = rbx; r8 = P1; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rdx = ((rax.wrapping_add(P4)) as u64) as u64; fl_l = rdx; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rsp.wrapping_add(0x48); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rcx = rsi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(P7))) as u64; rbx = rcx; rdi = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { bb = 5; continue; }
            5 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcab00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14241d6f0; const P1: u64 = 0x1401e66f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rdx)) as u64; rdi = rcx; rax = r8.wrapping_sub(0x1); fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 26; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rbx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x1u64) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); if (((fl_l as u64)) != ((fl_r as u64))) { bb = 25; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rdx.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); bb = 11; continue; }
            11 => { rdx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = (0x100000001b3u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rax = (0xcbf29ce484222325u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x11u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x12u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x13u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x14u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x15u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x16u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x17u64))) as u32) as u32 as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rdx = (((rdx) ^ (rax))) as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r8 = (((r8) & (rdx))) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x18u64)))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 15; continue; }
            15 => { bb = 17; continue; }
            16 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(r8)) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 22; continue; } bb = 19; continue; }
            19 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8, rax); crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rax); bb = 24; continue; }
            21 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r8.wrapping_add(0x8u64), rax); bb = 24; continue; }
            22 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 23; continue; }
            23 => { rax = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(r8, rax); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::lk_sub_u64(rdi.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 26; continue; }
            26 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcac90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xffffffffffffffff; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x143a082b8; const P4: u64 = 0x143a49050; const P5: u64 = 0x142421ed0; const P6: u64 = 0x140695810; const P7: u64 = 0x18;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r8 = (crate::simd_ops::ru64(rdx)) as u64; rbx = rcx; rax = ((r8.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = P4; crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), (0x0u8) as u8); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { r8 = rsp.wrapping_add(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rdx = rsp.wrapping_add(0x30); rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P7), (0x1u64) as u64); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { crate::simd_ops::lk_add_u64(rax.wrapping_add(P7), (1) as u64); rax = rcx; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            7 => { crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (0x0u64) as u64); bb = 8; continue; }
            8 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcad40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x141bcaa60;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    return crate::simd_ops::call_fwd(P1, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcad50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x8; const P2: u64 = 0xc; const P3: u64 = 0x140696a30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x40u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdx);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax);
    rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rcx = (crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32 as u64;
    rcx = ((rcx).wrapping_shl(5u32)) as u64;
    rax = ((rdx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    fl_l = rax; fl_r = rcx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((rdx).wrapping_add(1)) as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64;
    rax = ((rax).wrapping_shl(5u32)) as u64;
    rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx))) as u64;
    let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::lk_add_u32(rbx.wrapping_add(P1), (1) as u32);
    rsp = ((rsp).wrapping_add(0x40u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcadc0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x1; const P3: u64 = 0x1401fc320;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rsi = rcx;
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    rdx = (crate::simd_ops::ru32(rsi.wrapping_add(P1))) as u32 as u64;
    rdx = ((rdx).wrapping_shl(4u32)) as u64;
    rax = ((rcx as u32)) as u32 as u64;
    rax = ((rax).wrapping_shl(4u32)) as u64;
    fl_l = rax; fl_r = rdx;
    if !((((fl_l as u64)) != ((fl_r as u64)))) { rdx = ((rcx.wrapping_add(P2)) as u64) as u64; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rcx = (crate::simd_ops::ru32(rsi.wrapping_add(P0))) as u32 as u64;
    rcx = ((rcx).wrapping_add(rcx)) as u64;
    rax = (crate::simd_ops::ru64(rsi)) as u64;
    crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rdi);
    crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P0), rbx);
    crate::simd_ops::lk_add_u32(rsi.wrapping_add(P0), (1) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcae30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1427c3050; const P1: u64 = 0x1410d6ca0; const P2: u64 = 0x140696270; const P3: u64 = 0x143823778; const P4: u64 = 0x14020e980; const P5: u64 = 0x14020afa0; const P6: u64 = 0x1420a6b70; const P7: u64 = 0x140696560; const P8: u64 = 0x143823788; const P9: u64 = 0x1427c3040; const P10: u64 = 0x1406955f0; const P11: u64 = 0x143823510; const P12: u64 = 0x1401e5930; const P13: u64 = 0x1434268d0; const P14: u64 = 0x1401e5730; const P15: u64 = 0x143ca787c; const P16: u64 = 0x143cc2b28; const P17: u64 = 0x1406970c0; const P18: u64 = 0x14066e7e0; const P19: u64 = 0x1425371a0; const P20: u64 = 0x142533200; const P21: u64 = 0x1401e66f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x5f); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm6) as u128); r13 = rcx; r15 = rcx.wrapping_add(0x8); xmm0 = (crate::simd_ops::ru128(P0)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x59); rdx = rbp.wrapping_sub(0x39); rcx = r15.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x59u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x51u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x19); rcx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rdx = (0x2aaaaaaaaaaaaaabu64) as u64; r9 = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r9 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm6 = (crate::simd_ops::ru128(P3)) as u128; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = rdx; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x30u64), (xmm6) as u128); crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0xfu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x19); rcx = rbp.wrapping_sub(0x59); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P8)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = 0xffff; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((rax as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm6 = (crate::simd_ops::ru128(P8)) as u128; rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x30u64), (xmm6) as u128); crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 8; continue; }
            8 => { r9 = 0x2; r8 = (((rbx as u16)) as u32) as u32 as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0xfu64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(P9)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x39); rdx = rbp.wrapping_sub(0x59); rcx = r15.wrapping_add(0x18); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_sub(0x39u64), r15); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x51u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x31u64), (xmm0) as u128); rdx = rbp.wrapping_sub(0x19); rcx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; r15 = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x59u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P11)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm1) as u128); rdx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x8); r12 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; xmm6 = (crate::simd_ops::ru128(P11)) as u128; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rdx = rdi.wrapping_add(0x1); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x30u64), (xmm6) as u128); crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x59u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 12; continue; }
            12 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((r15 as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x9u64), rsi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x11u64), rax); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x1u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_add(0xfu64), rsi); crate::simd_ops::wu64(rbp.wrapping_add(0x17u64), (0x7u64) as u64); crate::simd_ops::wu64(rbp.wrapping_add(0x1fu64), (0x8u64) as u64); xmm0 = ((crate::simd_ops::ru32(P13)) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), ((xmm0 as u32)) as u32); r8 = rax; rdx = ((rsi.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = rbp.wrapping_sub(0x1); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((crate::simd_ops::ru16(P15)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r13.wrapping_add(0x90u64)))) as u64; rax = (crate::simd_ops::ru32(P16)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = P17; crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(rcx.wrapping_add(0x38u64), ((rsi as u8)) as u8); rdx = rbp.wrapping_sub(0x19); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P18, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r13.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rbx.wrapping_mul(8)), rax); r10 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 13; continue; }
            13 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((rsi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 20; continue; } bb = 16; continue; }
            16 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            17 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 19; continue; }
            18 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 19; continue; }
            19 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rdi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rsi); rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P21, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 21; continue; } bb = 22; continue; }
            22 => { r10 = rdi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 27; continue; } bb = 23; continue; }
            23 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 26; continue; } bb = 24; continue; }
            24 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 29; continue; } bb = 25; continue; }
            25 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P19, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            26 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 28; continue; }
            27 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 28; continue; }
            28 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 29; continue; }
            29 => { r11 = rsp.wrapping_add(0xb0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcb3a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bac2d0; const P1: u64 = 0x1405e3070; const P2: u64 = 0x143427a60; const P3: u64 = 0x14323f990; const P4: u64 = 0x141bac240;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bcb3a0, funcinfo_rva: 0x373e9b8, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rbx = (rdx) as u64; rdi = (rcx) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0xc8u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x70u64)))) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcb3cf); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x3u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (0x2u32) as u32 as u64; bb = 7; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x4u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rdi.wrapping_add(0x48u64))) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x134u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsi.wrapping_add(0x40u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcb42e); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rsi.wrapping_add(0xb8u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcb468); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xc8u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x70u64)))) as u64; rdx = (crate::simd_ops::ru32(rsi.wrapping_add(0x38u64))) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x130u64))) as u32) as u128; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), ((rax as u8)) as u8); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rdx as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xa4u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), (0x3u32) as u32); r8 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcb4e5); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 6; continue; }
            6 => { rax = (0x3u32) as u32 as u64; bb = 7; continue; }
            7 => { r11 = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcb510(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba31e0; const P1: u64 = 0x1405e3070; const P2: u64 = 0x143427a60; const P3: u64 = 0x14323f990; const P4: u64 = 0x141bac380;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bcb510, funcinfo_rva: 0x373e9f4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0xb8, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rdi = (rdx) as u64; rbp = (rcx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcb52d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rsi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcb552); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rbx.wrapping_add(0x40u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcb590); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(P2)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r9 = ((rbx.wrapping_add(0xb8u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcb5ca); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x70u64)))) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x130u64))) as u32) as u128; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (rdx) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((rdx as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x78u64), ((rdx as u8)) as u8); rdx = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rdx) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0xa4u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), (0x3u32) as u32); rdx = ((rsp.wrapping_add(0x60u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcb643); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), (rax) as u64); r11 = ((rsp.wrapping_add(0xb0u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcbe00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x60; const P2: u64 = 0x78; const P3: u64 = 0x141bcd1d0; const P4: u64 = 0x141ba22d0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rdi = ((r9 as u32)) as u32 as u64; r10 = r8; rbp = rdx; rsi = rcx; rbx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 1; continue; }
            1 => { r14 = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r10)) as u64; r8 = ((rdi as u32)) as u32 as u64; rdx = rsi; rcx = r10; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((rdi as u32)) as u32 as u64; r8 = rbp; rdx = rsi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x1u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); r9 = ((rax as u32)) as u32 as u64; r8 = rsp.wrapping_add(0x20); rdx = rbp; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = ((rdi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; r10 = rbx; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcbed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xb8; const P1: u64 = 0x30; const P2: u64 = 0x28; const P3: u64 = 0x141bcc280; const P4: u64 = 0x8; const P5: u64 = 0x18; const P6: u64 = 0x10; const P7: u64 = 0x68; const P8: u64 = 0x41;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdi = r8;
    rbx = rdx;
    rax = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    r8 = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    r9 = 0x1;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu64(rax.wrapping_add(P4), rbx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P6), (0x1u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(P5), rcx); crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(P7), rax); crate::simd_ops::wu8(rbx.wrapping_add(P8), (0x1u8) as u8); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcbf60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14388e6a8; const P1: u64 = 0x14388e6e0; const P2: u64 = 0x48; const P3: u64 = 0x14254768e; const P4: u64 = 0x38; const P5: u64 = 0x10; const P6: u64 = 0x18; const P7: u64 = 0x141bcbf60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rdx; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdi = rcx; r9 = P0; r8 = P1; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = rsi; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P5))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P6))) as u64; rsi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { rax = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x50; const P2: u64 = 0x58; const P3: u64 = 0x8; const P4: u64 = 0x141bcc250; const P5: u64 = 0x140698600; const P6: u64 = 0x1406984c0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; r8 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; r10 = ((rax.wrapping_add(P1)) as u64) as u64; rdi = rcx; rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; r9 = ((r8.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { r8 = ((r8).wrapping_add(0x8u64)) as u64; fl_l = r8; fl_r = r9; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 7; continue; }
            7 => { rdx = rsp.wrapping_add(0x38); rcx = r10; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            10 => { r9 = rsp.wrapping_add(0x38); rcx = r10; rdx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc0a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140696a30;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(rax)) as u64; r11 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax, r11); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), r11); crate::simd_ops::wu64(rdx.wrapping_add(0x18u64), rax); crate::simd_ops::wu8(rdx.wrapping_add(0x60u64), ((r11 as u8)) as u8); crate::simd_ops::wu8(rdx.wrapping_add(0x41u64), (0x1u8) as u8); rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0xc8u64))) as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rdi = ((rdi).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x58u64)))) as u64; rbx = (crate::simd_ops::ru64(rdx.wrapping_add(0x68u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rcx = r9; r10 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((r10 as u32)) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; fl_l = r9; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 5; continue; }
            5 => { rcx = ((rcx).wrapping_add(0x20u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((r11 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r11); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), r11); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rdx = r10.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(0xffffffffffffffe0u64)) as u64; fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), (0x2u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rcx.wrapping_add(0x18u64), rsi); bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc1b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xf0; const P1: u64 = 0x50; const P2: u64 = 0x58; const P3: u64 = 0x8; const P4: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r8 = rdx; rsi = rcx; rbx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P2))) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); bb = 2; continue; }
            2 => { rdi = ((rbx.wrapping_add(P3)) as u64) as u64; fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = rdi; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = r8; rdx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = (crate::simd_ops::ru64(rsi.wrapping_add(P0))) as u64; rdx = rdi; rcx = rbx; r8 = (crate::simd_ops::ru32(rsi.wrapping_add(P2))) as u32 as u64; r8 = ((r8).wrapping_shl(3u32)) as u64; r8 = ((r8).wrapping_sub(rdi)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(P1)))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(P2), (1) as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc250(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            6 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc280(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b6e540; const P1: u64 = 0x140698200; const P2: u64 = 0x141bcc390; const P3: u64 = 0x1406981a0; const P4: u64 = 0x1406980c0; const P5: u64 = 0x140698380;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bcc280, funcinfo_rva: 0x373eb34, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x68, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (rsp) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), (rbx) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), (rbp) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdi = (r8) as u64; rsi = (rdx) as u64; rbp = (rcx) as u64; crate::simd_ops::wu64(rax.wrapping_sub(0x28u64), (0x0u64) as u64); crate::simd_ops::wu64(rax.wrapping_sub(0x20u64), (0x0u64) as u64); crate::simd_ops::wu32(rax.wrapping_sub(0x40u64), ((r9 as u32)) as u32); crate::simd_ops::wu8(rax.wrapping_sub(0x48u64), (0x1u8) as u8); r9 = ((rax.wrapping_sub(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcc2b9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rcx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcc2c8); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; r9 = (rbx) as u64; r8 = (rdi) as u64; rdx = (rsi) as u64; rcx = (rbp) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcc2df); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), (rbx) as u64); rcx = ((rdi.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcc302); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = ((rdi.wrapping_add(0x20u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcc315); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; xmm0 = (crate::simd_ops::ru128(rax)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r8 = ((xmm0 as u64)) as u64; fl_l = (r8) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(r8)) as u64 as u64; fl_r = (rsi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { r9 = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rdi.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcc34c); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r9 = ((rsp.wrapping_add(0x38u64)) as u64) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; r8 = (((xmm0 >> 64) as u64)) as u64; rdx = ((rsp.wrapping_add(0x70u64)) as u64) as u64; rcx = ((rdi.wrapping_add(0x38u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcc36b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 3; continue; }
            3 => { rax = (rsi) as u64; r11 = ((rsp.wrapping_add(0x60u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc390(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14387d04c; const P1: u64 = 0x14387d124; const P2: u64 = 0x141ba2b40; const P3: u64 = 0x140660160; const P4: u64 = 0x141bccbf0; const P5: u64 = 0x141b7d0d0; const P6: u64 = 0x141bcc390;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rax = (crate::simd_ops::ru64(r8)) as u64; r12 = rcx; rcx = r8; r15 = r9; rsi = r8; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0xb8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rdi = rax; r10 = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rbx = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = rbx.wrapping_sub(0x1); rcx = (((rcx) & (rdx))) as u64; r8 = rax; rbx = ((rbx).wrapping_sub(rcx)) as u64; fl_l = rcx; fl_r = rcx; rbx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rbx }) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; r14 = 0; fl_l = 0; fl_r = 0; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rax = rcx; rax = ((rax).wrapping_sub(rbx)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = ((rdx).wrapping_add(rbx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rdx = r14; rax = rcx; bb = 4; continue; }
            4 => { rax = ((rax).wrapping_sub(r8)) as u64; rcx = rdi; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rax); rax = ((r8.wrapping_add(rdx)) as u64) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = r15; rdx = r12; rdi = rax; rcx = rdi; crate::simd_ops::wu64(rax.wrapping_add(0x48u64), rsi); rax = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x68u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rbp = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rbx = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rdi); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x98u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rcx = rsi; rcx = (if (((fl_l as u32)) != ((fl_r as u32))) { r14 } else { rcx }) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx); fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbp).wrapping_add(1)) as u64; rbp = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = rsi; rbx = (crate::simd_ops::ru32(P1)) as u32 as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x98u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; r11 = rbp.wrapping_add(0x1); r9 = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; rax = r10; rcx = rsi; r8 = 0x8; rcx = (if (((fl_l as u32)) != ((fl_r as u32))) { r14 } else { rcx }) as u64; fl_l = rcx; fl_r = rcx; rcx = ((r9.wrapping_mul(8)) as u64) as u64; r11 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rbp } else { r11 }) as u64; rax = ((((rax as u32)) & (0x7u32))) as u32 as u64; r8 = ((r8).wrapping_sub(rax)) as u64; fl_l = rax; fl_r = rax; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax } else { r8 }) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = rdx; rax = ((rax).wrapping_sub(r8)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = ((rax).wrapping_sub(rcx)) as u64; rdx = ((r10.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rax); rax = ((rdx.wrapping_add(rcx)) as u64) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rax); fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rdi.wrapping_add(0x50u64), rdx); crate::simd_ops::wu64(rdi.wrapping_add(0x58u64), r9); bb = 11; continue; }
            10 => { fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rdx); bb = 11; continue; }
            11 => { r8 = r15; rdx = r11; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rdx = rsp.wrapping_add(0x30); rcx = r12; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r14); r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r13 = r14; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x18u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x20u64))) as u32 as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 12; continue; }
            12 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; r13 = rax; rdx = (crate::simd_ops::ru64(rax)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r13)) as u64; rcx = r13; rsi = rax; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(r15.wrapping_add(0x8u64))) as u64; rcx = rsi.wrapping_sub(0x1); rcx = (((rcx) & (rdx))) as u64; r8 = rax; rsi = ((rsi).wrapping_sub(rcx)) as u64; fl_l = rcx; fl_r = rcx; rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rsi }) as u64; rcx = (crate::simd_ops::ru64(r15.wrapping_add(0x10u64))) as u64; fl_l = rcx; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = rcx; rax = ((rax).wrapping_sub(rsi)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 14; continue; }
            14 => { fl_l = ((rdx).wrapping_add(rsi)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; bb = 16; continue; }
            15 => { rdx = r14; rax = rcx; bb = 16; continue; }
            16 => { rax = ((rax).wrapping_sub(r8)) as u64; rcx = r13; crate::simd_ops::wu64(r15.wrapping_add(0x10u64), rax); rax = ((rdx.wrapping_add(r8)) as u64) as u64; crate::simd_ops::wu64(r15.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(r13)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = r15; rdx = r12; rsi = rax; r9 = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rcx); rcx = rax; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(r9.wrapping_add(0x18u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rdx = r13; r13 = ((r13).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), r13); crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)), rsi); fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 18; continue; }
            18 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 19; continue; }
            19 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rdx = rcx.wrapping_add(0x38); rax = ((rax).wrapping_sub(1)) as u64; fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 21; continue; } bb = 20; continue; }
            20 => { r8 = r15; rcx = r12; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            21 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x40u64))) as u64; rdx = rcx.wrapping_add(0x40); rax = ((rax).wrapping_sub(1)) as u64; fl_l = rax; fl_r = (0xfffffffffffffffdu64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 25; continue; } bb = 22; continue; }
            22 => { r8 = r15; rcx = r12; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 23; continue; }
            23 => { rcx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); r14 = 0x1; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rax, rcx); bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x28u64))) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x30u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 26; continue; }
            26 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 27; continue; }
            27 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 29; continue; } bb = 28; continue; }
            28 => { r9 = r15; rdx = r13; rcx = r12; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = r14; fl_l = ((r14).wrapping_add(1)) as u64; r14 = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)), rax); bb = 29; continue; }
            29 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 27; continue; } bb = 30; continue; }
            30 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; bb = 31; continue; }
            31 => { crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), r13); rax = rbp; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r14); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), rcx); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc700(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x141bcbf60; const P2: u64 = 0x14388e6a8; const P3: u64 = 0x14388e6e0; const P4: u64 = 0x14254768e; const P5: u64 = 0x10; const P6: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = rdx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = rax;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64;
    r9 = P2;
    r8 = P3;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64;
    if !((((fl_l as u64)) <= ((fl_r as u64)))) { rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (rax & !0xffu64) | (((0u8) as u8 as u64));
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x48; const P2: u64 = 0x141bcbf60; const P3: u64 = 0x14388e6a8; const P4: u64 = 0x14388e6e0; const P5: u64 = 0x14254768e; const P6: u64 = 0x10; const P7: u64 = 0xb8; const P8: u64 = 0x30; const P9: u64 = 0x28; const P10: u64 = 0x141bcc280; const P11: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = rdx; rdi = r8; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; r9 = P3; r8 = P4; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } bb = 4; continue; }
            4 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rax.wrapping_add(P8))) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P9))) as u64; r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 8; continue; }
            8 => { r9 = 0x1; rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rax.wrapping_add(P11), rbx); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x10u64), (0x1u64) as u64); crate::simd_ops::wu64(rbx.wrapping_add(0x18u64), rcx); crate::simd_ops::wu64(rcx, rax); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x68u64), rax); crate::simd_ops::wu8(rbx.wrapping_add(0x41u64), (0x1u8) as u8); bb = 10; continue; }
            10 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc870(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x48; const P1: u64 = 0x141bcbf60; const P2: u64 = 0x14388e6a8; const P3: u64 = 0x14388e6e0; const P4: u64 = 0x14254768e; const P5: u64 = 0x10; const P6: u64 = 0x60; const P7: u64 = 0x41; const P8: u64 = 0x68; const P9: u64 = 0x6c; const P10: u64 = 0x1; const P11: u64 = 0x14020bcb0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rax = rdx; rdi = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); r9 = P2; r8 = P3; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { crate::simd_ops::wu8(rbx.wrapping_add(P6), (0x1u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(P7), (0x1u8) as u8); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(P8))) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P6))) as u64; rdx = ((rcx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rax = rcx; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 5; continue; }
            5 => { bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(P9))) as u32 as u64; rcx = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rdx = ((r8.wrapping_add(P10)) as u64) as u64; rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x68u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbx); fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x68u64), (1) as u32)) as u32 as u64; bb = 11; continue; }
            11 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc950(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 4; continue; }
            4 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; return rax; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc990(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0xb8; const P2: u64 = 0x141bcc280; const P3: u64 = 0x18; const P4: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    r8 = (crate::simd_ops::ru64(rax)) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r9 = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcc9d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x48; const P2: u64 = 0x141ba33f0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P1), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcca10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba33f0; const P1: u64 = 0x1406980c0; const P2: u64 = 0x1406981a0; const P3: u64 = 0x14066b8a0; const P4: u64 = 0x142442de0; const P5: u64 = 0x14183acc8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rdx; rbp = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rbx = rdx; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; rdi = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x10u64), (0x0u64) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rax); rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; rbx = rcx; rdi = ((rcx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rcx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rsi = (crate::simd_ops::ru64(rbx)) as u64; rdx = rbp; rcx = rsi; rax = (crate::simd_ops::ru64(rsi)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rsi; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x28u64), (0x0u64) as u64); crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rax); rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x58u64))) as u64; rsi = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbp; rcx = rbx; rax = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::wu64(r14.wrapping_add(0x58u64), (0x0u64) as u64); rdx = rbp; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x70u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(r14)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; r8 = rsp.wrapping_add(0x58); rdx = rsp.wrapping_add(0x20); rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rsi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; fl_l = rax; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; bb = 12; continue; }
            11 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rsi = rax; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x28u64))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 14; continue; }
            14 => { rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; rdx = rsi.wrapping_add(0x8); r8 = ((r8).wrapping_shl(3u32)) as u64; rcx = rsi; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x28u64)))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x30u64), (1) as u32); rdx = rdi.wrapping_add(0x8); r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rcx = rdi; r8 = ((r8).wrapping_shl(3u32)) as u64; r8 = ((r8).wrapping_sub(rdx)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x38u64)))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(0x40u64), (1) as u32)) as u32 as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bccbf0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140656520; const P1: u64 = 0xb8; const P2: u64 = 0x28; const P3: u64 = 0x30; const P4: u64 = 0x14065ff50; const P5: u64 = 0x141bcc390;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r11 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 1; continue; }
            1 => { r10 = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; rdx = (crate::simd_ops::ru64(r10)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdx = ((r8.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(r8)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r10 = (crate::simd_ops::ru64(r10)) as u64; r9 = rdi; rdx = r11; rcx = rbx; r8 = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; r8 = (crate::simd_ops::ru64(r8)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bccc70(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xe8u64))) as u64; r14 = rcx; rsi = (crate::simd_ops::ru64(rax.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0xc8u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbp = ((rsi.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0xf0u64))) as u64; rcx = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 7; continue; }
            6 => { rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0xf0u64))) as u64; bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x19u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx)) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rcx.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 13; continue; }
            13 => { bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = rsi; rdx = r14; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 15; continue; }
            15 => { rsi = ((rsi).wrapping_add(0x18u64)) as u64; fl_l = rsi; fl_r = rbp; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 16; continue; }
            16 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(0xe8u64))) as u64; bb = 18; continue; }
            17 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xe8u64))) as u64; bb = 18; continue; }
            18 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu32(rax.wrapping_add(0xc8u64), (0x0u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bccd80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183acc8; const P1: u64 = 0x14388e6a8; const P2: u64 = 0x14388e6e0; const P3: u64 = 0x14254768e; const P4: u64 = 0x14241c0a0; const P5: u64 = 0x14241c190; const P6: u64 = 0x141bcc0a0; const P7: u64 = 0x141bcc250; const P8: u64 = 0x140698600; const P9: u64 = 0x1406984c0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; r13 = rcx; rdi = (crate::simd_ops::ru64(rdx)) as u64; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r14 = ((rdi.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 75; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(r13.wrapping_add(0xf0u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; rcx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 3; continue; }
            3 => { rbp = rbx.wrapping_add(0x8); fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rbx = rbp; fl_l = rbp; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 12; continue; }
            6 => { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = r13; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x58u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(r13.wrapping_add(0xf0u64))) as u64; rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x50u64))) as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; r8 = ((r8).wrapping_sub(rbp)) as u64; rdx = rbp; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::lk_sub_u32(rsi.wrapping_add(0x58u64), (1) as u32)) as u32 as u64; bb = 12; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            10 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rdi = ((rdi).wrapping_add(0x18u64)) as u64; fl_l = rdi; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 13; continue; }
            13 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); r14 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r14); rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdi); r12 = 0; fl_l = 0; fl_r = 0; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; rbx = (crate::simd_ops::ru64(rbp)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r15 = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), r15); fl_l = rbx; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 14; continue; }
            14 => { bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 16; continue; }
            16 => { crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32); r9 = P1; r8 = P2; rdx = 0; fl_l = 0; fl_r = 0; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x48u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 17; continue; }
            17 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x60u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 24; continue; } bb = 19; continue; }
            19 => { rbp = ((r14.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; rax = ((r12 as u32)) as u32 as u64; rcx = ((r14.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbp; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 20; continue; }
            20 => { rbp = ((rbp).wrapping_sub(r14)) as u64; rdi = rbp; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi.wrapping_add(0x1); rcx = ((rcx).wrapping_sub(r14)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; r12 = rcx; r12 = ((r12).wrapping_shr(1u32)) as u64; r12 = ((r12).wrapping_add(rcx)) as u64; fl_l = r12; fl_r = rax; r12 = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { r12 }) as u64; rcx = ((r12.wrapping_mul(8)) as u64) as u64; rdx = 0x8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = rax; r8 = rbp; rdx = r14; rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } bb = 21; continue; }
            21 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 22; continue; }
            22 => { r14 = r15; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); crate::simd_ops::wu32(rsp.wrapping_add(0x3cu64), ((r12 as u32)) as u32); r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 23; continue; }
            23 => { rax = ((rdi as u32)) as u32 as u64; crate::simd_ops::wu64(r14.wrapping_add(rax.wrapping_mul(8)), rsi); fl_l = (((rdi as u32)).wrapping_add(1)) as u32 as u64; rdi = ((fl_l as u32)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); bb = 40; continue; }
            24 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 40; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x60u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 40; continue; } bb = 26; continue; }
            26 => { r8 = ((r14.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; rax = r14; fl_l = r14; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 27; continue; }
            27 => { rcx = rax; bb = 28; continue; }
            28 => { rax = rcx; fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 30; continue; } bb = 29; continue; }
            29 => { rax = rcx.wrapping_add(0x8); rcx = rax; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 28; continue; } bb = 30; continue; }
            30 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 31; continue; }
            31 => { rcx = rax.wrapping_add(0x8); fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 32; continue; }
            32 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rdx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 34; continue; } bb = 33; continue; }
            33 => { crate::simd_ops::wu64(rax, rdx); fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 34; continue; }
            34 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 35; continue; }
            35 => { rcx = ((r14.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 40; continue; } bb = 36; continue; }
            36 => { fl_l = rax; fl_r = r14; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 37; continue; }
            37 => { rdi = 0; fl_l = 0; fl_r = 0; bb = 39; continue; }
            38 => { rcx = ((rcx).wrapping_sub(rax)) as u64; rcx = ((((rcx) as i64).wrapping_shr(3u32) as u64)) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; rdi = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; bb = 39; continue; }
            39 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); bb = 40; continue; }
            40 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 41; continue; }
            41 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 42; continue; }
            42 => { rbx = r14; rsi = ((r14.wrapping_add(rdi.wrapping_mul(8))) as u64) as u64; fl_l = r14; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 53; continue; } bb = 43; continue; }
            43 => { bb = 44; continue; }
            44 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rdx = rdi; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(r13.wrapping_add(0xf0u64))) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x60u64))) as u64; r9 = (crate::simd_ops::ru32(r8.wrapping_add(0x68u64))) as u32 as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 52; continue; } bb = 45; continue; }
            45 => { r10 = rax; bb = 46; continue; }
            46 => { rcx = ((r9 as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 50; continue; } bb = 47; continue; }
            47 => { rcx = ((rcx).wrapping_sub(1)) as u64; rcx = ((r10.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 49; continue; } bb = 48; continue; }
            48 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 49; continue; }
            49 => { fl_l = (crate::simd_ops::lk_sub_u32(r8.wrapping_add(0x68u64), (1) as u32)) as u32 as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(0x68u64))) as u32 as u64; bb = 51; continue; }
            50 => { fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 51; continue; }
            51 => { r10 = (crate::simd_ops::ru64(r8.wrapping_add(0x60u64))) as u64; r9 = ((rcx as u32)) as u32 as u64; rdx = ((r10.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 46; continue; } bb = 52; continue; }
            52 => { rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 44; continue; } bb = 53; continue; }
            53 => { rbx = (crate::simd_ops::ru64(rbp)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rsi = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 70; continue; } bb = 54; continue; }
            54 => { bb = 55; continue; }
            55 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 66; continue; } bb = 56; continue; }
            56 => { rdi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rdi); r9 = (crate::simd_ops::ru64(r13.wrapping_add(0xf0u64))) as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(0x50u64))) as u64; r10 = (crate::simd_ops::ru32(r9.wrapping_add(0x58u64))) as u32 as u64; rax = ((r8.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 64; continue; } bb = 57; continue; }
            57 => { r11 = r8; bb = 58; continue; }
            58 => { rdx = (crate::simd_ops::ru64(r8)) as u64; fl_l = rdi; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 69; continue; } bb = 59; continue; }
            59 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 60; continue; }
            60 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 63; continue; } bb = 61; continue; }
            61 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 65; continue; } bb = 62; continue; }
            62 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 65; continue; } bb = 63; continue; }
            63 => { r8 = ((r8).wrapping_add(0x8u64)) as u64; rcx = ((r11.wrapping_add(r10.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 58; continue; } bb = 64; continue; }
            64 => { rcx = r9.wrapping_add(0x50); rdx = rsp.wrapping_add(0x88); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = r13; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 69; continue; }
            65 => { rcx = r9.wrapping_add(0x50); r9 = rsp.wrapping_add(0x88); rdx = rsp.wrapping_add(0x90); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = r13; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 69; continue; }
            66 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 69; continue; } bb = 67; continue; }
            67 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x10u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 69; continue; } bb = 68; continue; }
            68 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 69; continue; }
            69 => { rbx = ((rbx).wrapping_add(0x18u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 55; continue; } bb = 70; continue; }
            70 => { rdx = (crate::simd_ops::ru64(rbp)) as u64; rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = ((rdx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 73; continue; } bb = 71; continue; }
            71 => { bb = 72; continue; }
            72 => { crate::simd_ops::wu64(rdx.wrapping_add(0x8u64), rcx); crate::simd_ops::wu64(rdx.wrapping_add(0x10u64), rcx); rdx = ((rdx).wrapping_add(0x18u64)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 72; continue; } bb = 73; continue; }
            73 => { crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), ((rcx as u32)) as u32); fl_l = r14; fl_r = r14; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 75; continue; } bb = 74; continue; }
            74 => { rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 75; continue; }
            75 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcd1d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bcc250; const P1: u64 = 0x1406749f0; const P2: u64 = 0x1406982b0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = 0x1; r14 = ((r9 as u32)) as u32 as u64; rdi = r8; rsi = rdx; rbp = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 22; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rbx); rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; rax = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), r15); r15 = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(rbx)) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 5; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 6; continue; }
            6 => { fl_l = r8; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; r9 = (crate::simd_ops::ru32(rdi)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((r9 as u32)) as u32); rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r8); rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r15); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 10; continue; }
            10 => { bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 15; continue; }
            15 => { rdx = rsp.wrapping_add(0x20); rcx = rsi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rbx)) as u64; r8 = ((r14 as u32)) as u32 as u64; rdx = rbp; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rbx)) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 17; continue; }
            17 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rcx = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 18; continue; }
            18 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; bb = 20; continue; }
            20 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            21 => { rdx = rdi; rcx = rsi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 18; continue; }
            22 => { rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r8 = (crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 26; continue; } bb = 25; continue; }
            25 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 26; continue; }
            26 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xf0u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(0x58u64))) as u32 as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 31; continue; } bb = 28; continue; }
            28 => { bb = 29; continue; }
            29 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 30; continue; }
            30 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 31; continue; }
            31 => { rdx = rdi; rcx = rsi; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcd380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x140267680; const P1: u64 = 0x140208500; const P2: u64 = 0x140698ea0; const P3: u64 = 0x140200130;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rdi = r8; r14 = rdx; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; rbp = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r15 as u32)) as u32); rbx = rbp; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r15 as u32)) as u32); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; bb = 5; continue; }
            4 => { crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), (0x1u32) as u32); rbx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x19u64))) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(0x10u64))) as u64; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 8; continue; }
            8 => { rax = (0x38e38e38e38e38eu64) as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r15); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = (crate::simd_ops::ru128(r14)) as u128; crate::simd_ops::wu128(rax.wrapping_add(0x20u64), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(0x30u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x38u64), r15); crate::simd_ops::wu64(rax.wrapping_add(0x40u64), r15); crate::simd_ops::wu64(rax, rbp); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu16(rax.wrapping_add(0x18u64), (0x0u16) as u16); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); r8 = rax; rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = rax; bb = 10; continue; }
            10 => { r10 = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rdx = ((r10 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(r9)) as u64; rax = r9; fl_l = r9; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 11; continue; }
            11 => { r8 = (crate::simd_ops::ru64(rdi)) as u64; bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 14; continue; }
            14 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 15; continue; }
            15 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x44u64))) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r9)) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rax = r10.wrapping_add(0x1); rdx = ((rcx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(rcx)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rbx.wrapping_add(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x40u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x38u64)))) as u64; crate::simd_ops::wu64(rcx, r15); crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r15); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rcx, rax); fl_l = (crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x40u64), (1) as u32)) as u32 as u64; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141bcd521, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcd530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x19; const P2: u64 = 0x10; const P3: u64 = 0x30; const P4: u64 = 0x38; const P5: u64 = 0x40; const P6: u64 = 0xfffffffffffffff0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = (crate::simd_ops::ru64(rcx)) as u64; r11 = r8; r9 = r10; rax = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; bb = 5; continue; }
            4 => { r9 = rax; rax = (crate::simd_ops::ru64(rax)) as u64; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(r9.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 18; continue; } bb = 8; continue; }
            8 => { fl_l = r9; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(r9.wrapping_add(P4))) as u64; rcx = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 10; continue; }
            10 => { r8 = (crate::simd_ops::ru64(r8)) as u64; bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(r11.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(0x10u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 14; continue; }
            14 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 15; continue; }
            15 => { rdx = ((rcx.wrapping_add(P6)) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(rax.wrapping_add(P0), rcx); rcx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu64(rax, rcx); bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::lk_sub_u32(r9.wrapping_add(P5), (1) as u32)) as u32 as u64; bb = 18; continue; }
            18 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcd5d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bccc70; const P1: u64 = 0x141bccd80; const P2: u64 = 0x141bcd1d0; const P3: u64 = 0x141ba22d0; const P4: u64 = 0x141bcbe00; const P5: u64 = 0x141bcc0a0; const P6: u64 = 0x14241c190;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); xmm6 = (xmm1) as u128; rsi = rcx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xf0u64))) as u64; fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 54; continue; } bb = 1; continue; }
            1 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm0 = 0u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x40u64), (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), rax); r8 = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rdx = (crate::simd_ops::ru64(r8.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(0x58u64))) as u32 as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r9 = 0; fl_l = 0; fl_r = 0; r8 = rbp.wrapping_sub(0x40); rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; r15 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; xmm2 = (xmm6) as u128; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x40u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; xmm2 = (xmm6) as u128; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x80u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { 1u8 } else { 0u8 }) as u8 as u64)); r14 = (r14 & !0xffu64) | ((((((r14 as u8)) | ((rax as u8)))) as u8 as u64)); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 12; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rbx = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 7; continue; }
            7 => { bb = 8; continue; }
            8 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rcx = ((((rbx as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = rbp.wrapping_sub(0x40); rdx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x1; rbx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 8; continue; } bb = 10; continue; }
            10 => { bb = 12; continue; }
            11 => { rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; r12 = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; r12 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 28; continue; } bb = 13; continue; }
            13 => { r13 = ((((r12 as u32)) as i32 as i64) as u64) as u64; fl_l = ((r13).wrapping_shl(3u32)) as u64; r13 = ((fl_l as u64)) as u64; bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(r13))) as u64; fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x41u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 27; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 20; continue; } bb = 16; continue; }
            16 => { bb = 17; continue; }
            17 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 27; continue; } bb = 19; continue; }
            19 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 17; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r9 = (r9 & !0xffu64) | (((0x1u8) as u8 as u64)); r8 = rbp.wrapping_sub(0x40); rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 27; continue; } bb = 21; continue; }
            21 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rdi); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (0x0u64) as u64); r9 = ((rax as u32)) as u32 as u64; r8 = rbp.wrapping_sub(0x30); rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r15 = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 24; continue; } bb = 22; continue; }
            22 => { bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r14 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = r15; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 24; continue; }
            24 => { rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } bb = 25; continue; }
            25 => { rax = (crate::simd_ops::ru64(rdi)) as u64; r8 = ((r14 as u32)) as u32 as u64; rdx = rsi; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((r14 as u32)) as u32 as u64; r8 = rbp.wrapping_sub(0x40); rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 27; continue; } bb = 26; continue; }
            26 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (0x0u64) as u64); r9 = ((rax as u32)) as u32 as u64; r8 = rbp.wrapping_sub(0x30); rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((rdi as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((rax as u32)) as u32 as u64; r8 = rbx; rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 27; continue; }
            27 => { r13 = ((r13).wrapping_sub(0x8u64)) as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; r12 = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) >= 0) { bb = 14; continue; } bb = 28; continue; }
            28 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(0x50u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x58u64))) as u32 as u64; r13 = ((rdi.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = r13; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 43; continue; } bb = 29; continue; }
            29 => { r14 = (crate::simd_ops::ru64(rdi)) as u64; rax = (crate::simd_ops::ru64(r14)) as u64; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x38u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 42; continue; } bb = 30; continue; }
            30 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rcx = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; rdx = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; rcx = ((rax.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 35; continue; } bb = 31; continue; }
            31 => { bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 34; continue; } bb = 33; continue; }
            33 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_r = r14; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 42; continue; } bb = 34; continue; }
            34 => { rax = ((rax).wrapping_add(0x18u64)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 32; continue; } bb = 35; continue; }
            35 => { rax = (crate::simd_ops::ru64(r14)) as u64; xmm2 = (xmm6) as u128; rdx = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x30u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 36; continue; }
            36 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), r14); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (0x0u64) as u64); r9 = ((rax as u32)) as u32 as u64; r8 = rbp.wrapping_sub(0x30); rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rcx = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; r12 = ((rbx.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 39; continue; } bb = 37; continue; }
            37 => { bb = 38; continue; }
            38 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = ((r15 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = ((rax as u32)) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = r12; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 38; continue; } bb = 39; continue; }
            39 => { rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 42; continue; } bb = 40; continue; }
            40 => { rax = (crate::simd_ops::ru64(r14)) as u64; r8 = ((r15 as u32)) as u32 as u64; rdx = rsi; rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((r15 as u32)) as u32 as u64; r8 = rbp.wrapping_sub(0x40); rdx = rsi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x78u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 42; continue; } bb = 41; continue; }
            41 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), (0x1u32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), rbx); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (0x0u64) as u64); r9 = ((rax as u32)) as u32 as u64; r8 = rbp.wrapping_sub(0x30); rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((r14 as u32)) as u32 as u64; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((rax as u32)) as u32 as u64; r8 = rbx; rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 42; continue; }
            42 => { rdi = ((rdi).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 29; continue; } bb = 43; continue; }
            43 => { rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x40); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(0x60u64))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(0x68u64))) as u32 as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 47; continue; } bb = 44; continue; }
            44 => { bb = 45; continue; }
            45 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 45; continue; } bb = 46; continue; }
            46 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(0xf0u64))) as u64; r8 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rax.wrapping_add(0x68u64), ((r8 as u32)) as u32); bb = 48; continue; }
            47 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 48; continue; }
            48 => { rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x38u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 52; continue; } bb = 49; continue; }
            49 => { rcx = rax.wrapping_add(0x10); bb = 50; continue; }
            50 => { crate::simd_ops::wu64(rcx.wrapping_sub(0x8u64), r8); crate::simd_ops::wu64(rcx, r8); rcx = rcx.wrapping_add(0x18); rax = rcx.wrapping_sub(0x10); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 50; continue; } bb = 51; continue; }
            51 => { rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; bb = 52; continue; }
            52 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 54; continue; } bb = 53; continue; }
            53 => { rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 54; continue; }
            54 => { r11 = rsp.wrapping_add(0x60); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcdab0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1406939e0; const P1: u64 = 0x1401deba0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bcdab0, funcinfo_rva: 0x373ed9c, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x58, conts: &[] };
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rbx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r15) as u64); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbp = (((r8 as u8)) as u32) as u32 as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; r14 = (((crate::simd_ops::ru64(rdx.wrapping_add(0xc8u64))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r14 = ((r14).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(0x60u64)))) as u64; rbx = (0x100u32) as u32 as u64; rdi = ((rdx.wrapping_add(0xe8u64)) as u64) as u64; rsi = ((rdx.wrapping_add(0xd8u64)) as u64) as u64; fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (rbx) as u64); r9 = ((r15.wrapping_add(0x10u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcdb10); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rbx = (crate::simd_ops::ru64(rax)) as u64; bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), (0x100u64) as u64); r9 = ((r15.wrapping_add(0x88u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rsi)) as u64; rax = ((rsp.wrapping_add(0x60u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r8 = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x68u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcdb40); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); rax = (crate::simd_ops::ru64(r14)) as u64; crate::simd_ops::wu64(rax.wrapping_add(0x1b0u64), (rbx) as u64); r8 = (crate::simd_ops::ru64(r14)) as u64; r9 = (crate::simd_ops::ru32(r8.wrapping_add(0x88u64))) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x80u64))) as u64; rdx = ((rcx.wrapping_add(r9.wrapping_mul(8))) as u64) as u64; rax = (rcx) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = (rcx) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax)) as u64 as u64; fl_r = (rbx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { fl_l = ((rbp as u8)) as u8 as u64; fl_r = ((rbp as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } bb = 6; continue; }
            6 => { fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 7; continue; }
            7 => { rdi = ((r8.wrapping_add(0x80u64)) as u64) as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xcu64))) as u32 as u64; rax = ((rcx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((r9.wrapping_add(0x1u64)) as u64) as u64; rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rax) as u64 as u64; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = (rdi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bcdbb4); crate::simd_ops::replace_rax_in(rax); rax = crate::exe::call_va8((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8); bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x8u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), (rbx) as u64); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x8u64), (1) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            10 => { fl_l = (rax) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 11; continue; }
            11 => { rcx = ((rdx.wrapping_sub(0x8u64)) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (rcx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, (rdx) as u64); bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::lk_sub_u32(r8.wrapping_add(0x88u64), (1) as u32)) as u32 as u64; bb = 14; continue; }
            14 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r15 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rsi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcdc10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0xe8; const P2: u64 = 0x60; const P3: u64 = 0x8; const P4: u64 = 0xd8; const P5: u64 = 0x1406939e0; const P6: u64 = 0x1b0; const P7: u64 = 0x80; const P8: u64 = 0x88; const P9: u64 = 0xfffffffffffffff8;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = (((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rax = rsp.wrapping_add(0x40); rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P2)))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rcx = rsp.wrapping_add(0x48); rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; r9 = ((r9).wrapping_add(0x88u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x100u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(P6), (0x100u64) as u64); r8 = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(P7))) as u64; rcx = (crate::simd_ops::ru32(r8.wrapping_add(P8))) as u32 as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rax)) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rcx = ((rdx.wrapping_add(P9)) as u64) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rax, rdx); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_sub_u32(r8.wrapping_add(P8), (1) as u32)) as u32 as u64; bb = 8; continue; }
            8 => { rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcdcd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x60; const P2: u64 = 0x53; const P3: u64 = 0x141bcdab0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = (((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    fl_l = (crate::simd_ops::ru8(r8.wrapping_add(rax).wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); return crate::simd_ops::call_fwd(P3, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bce430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0xc8; const P4: u64 = 0x60; const P5: u64 = 0xe8; const P6: u64 = 0xe0; const P7: u64 = 0xec; const P8: u64 = 0xd8; const P9: u64 = 0xe4; const P10: u64 = 0xd0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = (((crate::simd_ops::ru64(rsi.wrapping_add(P3))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64; fl_l = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 3; continue; }
            3 => { r11 = (crate::simd_ops::ru32(r9.wrapping_add(P5))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru32(r9.wrapping_add(P6))) as u32 as u64; r10 = (crate::simd_ops::ru32(r9.wrapping_add(P7))) as u32 as u64; r8 = (crate::simd_ops::ru64(r9.wrapping_add(P8))) as u64; rbx = ((r11.wrapping_sub(0x1u64)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rax = ((rax).wrapping_add(rcx)) as u64; let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = (crate::simd_ops::ru64(r8.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((r11 as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru32(r9.wrapping_add(P9))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((rax).wrapping_sub(1)) as u64; crate::simd_ops::wu32(r9.wrapping_add(P5), ((rbx as u32)) as u32); rax = ((rax).wrapping_add(r10)) as u64; let d = (r10) as u128; let n = ((rdx as u128) << 64) | (rax as u128); if d != 0 { rax = (n / d) as u64; rdx = (n % d) as u64; } crate::simd_ops::wu32(r9.wrapping_add(P9), ((rdx as u32)) as u32); bb = 6; continue; }
            6 => { crate::simd_ops::wu8(r9.wrapping_add(P10), (0x1u8) as u8); bb = 7; continue; }
            7 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bce520(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0xc8; const P4: u64 = 0x60; const P5: u64 = 0xd8; const P6: u64 = 0x14; const P7: u64 = 0x1403497a0; const P8: u64 = 0xc; const P9: u64 = 0xd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 3; continue; }
            3 => { rcx = (((crate::simd_ops::ru64(rsi.wrapping_add(P3))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(rax))) as u64; rdi = ((rsi.wrapping_add(P5)) as u64) as u64; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P0))) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P8))) as u32 as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; crate::simd_ops::wu64(rax.wrapping_add(rcx.wrapping_mul(8)), rbx); crate::simd_ops::lk_add_u32(rdi.wrapping_add(P8), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P8))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 7; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu32(rdi.wrapping_add(P8), (0x0u32) as u32); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(P0), (1) as u32)) as u32 as u64; crate::simd_ops::wu8(rsi.wrapping_add(P9), (0x1u8) as u8); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bced30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc8; const P1: u64 = 0x60; const P2: u64 = 0x20; const P3: u64 = 0x88;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64;
    rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(rdx).wrapping_add(P3), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bced60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bced80(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1c; const P2: u64 = 0x18; const P3: u64 = 0x10; const P4: u64 = 0x20; const P5: u64 = 0xc8; const P6: u64 = 0x60; const P7: u64 = 0x88;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x1u8) as u8);
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    r9 = (crate::simd_ops::ru32(r8.wrapping_add(P3))) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((r9 as u32)) as u32);
    rcx = (((crate::simd_ops::ru64(rdx.wrapping_add(P5))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P6))) as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(rax).wrapping_add(P7), ((r9 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcf320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcf380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14323ff80; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), ((rcx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcf4f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xc8; const P2: u64 = 0x20; const P3: u64 = 0x80; const P4: u64 = 0x10; const P5: u64 = 0x8; const P6: u64 = 0x142489d30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (((r8 as u8)) as u32) as u32 as u64;
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = r8; fl_r = r8;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    r8 = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64;
    rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P4))) as u32 as u64;
    r11 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5))) as u64;
    rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff;
    if (((fl_l as u32)) == ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rcx = 0xffff;
    fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64;
    if !((((fl_l as u16)) == ((fl_r as u16)))) { rcx = (((rax as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11)) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5))) as u64; crate::simd_ops::wu8(rcx, ((rbx as u8)) as u8); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcf560(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0xc8; const P4: u64 = 0x80; const P5: u64 = 0x8; const P6: u64 = 0x142489d30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P0))) as u32 as u64; r11 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = 0xffff; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (((rax as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11)) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5))) as u64; crate::simd_ops::wu8(rcx, (0x0u8) as u8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcf620(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0xc8; const P4: u64 = 0x80; const P5: u64 = 0x8; const P6: u64 = 0x142489d30;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; r8 = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P0))) as u32 as u64; r11 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5))) as u64; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rcx = 0xffff; fl_l = ((rax as u16)) as u16 as u64; fl_r = ((rcx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rcx = (((rax as u16)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r11)) as u64; fl_l = ((rcx).wrapping_add(rcx)) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(P5))) as u64; crate::simd_ops::wu8(rcx, (0x1u8) as u8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcfe90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x88; const P2: u64 = 0x24; const P3: u64 = 0x80; const P4: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    r8 = ((r8).wrapping_shl(4u32)) as u64;
    r8 = ((r8).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P1)))) as u64;
    rax = (crate::simd_ops::ru32(r8.wrapping_add(P2))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { return rax; }
    fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r8.wrapping_add(P4), rax); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bcfed0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183ad94; const P1: u64 = 0x143428120; const P2: u64 = 0x1426fd060; const P3: u64 = 0x1426fcaa0; const P4: u64 = 0x1426fce00; const P5: u64 = 0x1426fcc90; const P6: u64 = 0x1426fcfc0; const P7: u64 = 0x1426fd020; const P8: u64 = 0x1426fccc0; const P9: u64 = 0x1426fcf60; const P10: u64 = 0x1426fcb90; const P11: u64 = 0x1426fccd0; const P12: u64 = 0x1426fcb00; const P13: u64 = 0x1426fcd70; const P14: u64 = 0x1426fd070; const P15: u64 = 0x1426fcd60; const P16: u64 = 0x1426fcce0; const P17: u64 = 0x1426fcbf0; const P18: u64 = 0x1426fcb70; const P19: u64 = 0x1426fd090;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; rbx = rcx; rdi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x98u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0xa0u64))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rbx.wrapping_add(0xa0u64))) as u64; r8 = rdi; xmm3 = (xmm2) as u128; rdx = 0; fl_l = 0; fl_r = 0; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x4; bb = 9; continue; }
            5 => { rax = 0x3; bb = 9; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xb5u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm6) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm9) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm10) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm11) as u128); xmm0 = (xmm1) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm12) as u128); xmm12 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x84u64))) as u32) as u128; xmm1 = crate::simd_ops::vshufps(xmm1, xmm1, 0xaau8); crate::simd_ops::wu8(rbx.wrapping_add(0xb5u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; xmm1 = 0u128; xmm1 = crate::simd_ops::vinsertps(xmm1, xmm0, 0x10u8); xmm1 = (xmm1 & 0xffff_ffff_ffff_ffffu128) | ((xmm1 & 0xffff_ffff_ffff_ffffu128) << 64); xmm11 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P1)); xmm3 = (xmm11) & (crate::simd_ops::ru128(P2)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P3)); ymm0_hi = 0; xmm1 = crate::simd_ops::vcvttps2dq(xmm0); xmm2 = crate::simd_ops::vpaddd(xmm1, crate::simd_ops::ru128(P4)); xmm7 = (xmm2) & (crate::simd_ops::ru128(P5)); xmm0 = (xmm7) & (crate::simd_ops::ru128(P6)); ymm0_hi = 0; xmm1 = (xmm7) & (crate::simd_ops::ru128(P7)); xmm10 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = 0u128; ymm0_hi = 0; xmm9 = crate::simd_ops::vpcmpeqd(xmm1, xmm0); xmm5 = crate::simd_ops::vcvtdq2ps(xmm7); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); ymm0_hi = 0; xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P9)); xmm2 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P10)); ymm0_hi = 0; xmm3 = crate::simd_ops::vaddps(xmm2, xmm1); xmm1 = crate::simd_ops::vpsubd(xmm7, crate::simd_ops::ru128(P7)); xmm5 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (!(xmm1)) & (crate::simd_ops::ru128(P6)); ymm0_hi = 0; xmm8 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P11)); ymm0_hi = 0; xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P12)); ymm0_hi = 0; xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P13)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); ymm0_hi = 0; xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P14)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = crate::simd_ops::vsubps(xmm2, xmm1); ymm0_hi = 0; xmm7 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P15)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P16)); ymm0_hi = 0; xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P17)); ymm0_hi = 0; xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P18)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); ymm0_hi = 0; xmm2 = crate::simd_ops::vmulps(xmm0, xmm5); xmm0 = (xmm11) & (crate::simd_ops::ru128(P19)); ymm0_hi = 0; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; xmm6 = crate::simd_ops::vaddps(xmm2, xmm5); xmm2 = (xmm0) ^ (xmm10); xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm3 = (xmm6) & (xmm9); xmm5 = (!(xmm9)) & (xmm7); xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm1 = crate::simd_ops::vaddps(xmm5, xmm3); xmm0 = crate::simd_ops::vsubps(xmm7, xmm5); ymm0_hi = 0; xmm4 = (xmm2) ^ (xmm1); xmm3 = crate::simd_ops::vsubps(xmm6, xmm3); xmm1 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x88u64)))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm2 = (xmm1) ^ (xmm8); xmm4 = crate::simd_ops::vblendps(xmm2, xmm4, 0x7u8); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm4) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((xmm12 as u32)) as u32); xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), ((rax as u32)) as u32); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; bb = 8; continue; }
            8 => { rax = 0x1; bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd0150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1438ce038; const P1: u64 = 0x1438b4e60; const P2: u64 = 0x14254768e; const P3: u64 = 0x20; const P4: u64 = 0x60; const P5: u64 = 0x30; const P6: u64 = 0x70; const P7: u64 = 0xb5;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rbx = rcx;
    r9 = P0;
    rcx = rax;
    r8 = P1;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P5))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P6), (xmm0) as u128); crate::simd_ops::wu8(rbx.wrapping_add(P7), (0x1u8) as u8); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd0360(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14183ad94; const P1: u64 = 0x143428120; const P2: u64 = 0x1426fd060; const P3: u64 = 0x1426fcaa0; const P4: u64 = 0x1426fce00; const P5: u64 = 0x1426fcc90; const P6: u64 = 0x1426fcfc0; const P7: u64 = 0x1426fd020; const P8: u64 = 0x1426fccc0; const P9: u64 = 0x1426fcf60; const P10: u64 = 0x1426fcb90; const P11: u64 = 0x1426fccd0; const P12: u64 = 0x1426fcb00; const P13: u64 = 0x1426fcd70; const P14: u64 = 0x1426fd070; const P15: u64 = 0x1426fcd60; const P16: u64 = 0x1426fcce0; const P17: u64 = 0x1426fcbf0; const P18: u64 = 0x1426fcb70; const P19: u64 = 0x1426fd090; const P20: u64 = 0x142489e40;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 23>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rdi = r8; rbx = rcx; xmm6 = (xmm3) as u128; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); xmm0 = (xmm1) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm12) as u128); xmm12 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x84u64))) as u32) as u128; xmm1 = crate::simd_ops::vshufps(xmm1, xmm1, 0xaau8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc0u64))) as u64; xmm1 = 0u128; xmm1 = crate::simd_ops::vinsertps(xmm1, xmm0, 0x10u8); xmm1 = (xmm1 & 0xffff_ffff_ffff_ffffu128) | ((xmm1 & 0xffff_ffff_ffff_ffffu128) << 64); xmm11 = crate::simd_ops::vmulps(xmm1, crate::simd_ops::ru128(P1)); xmm3 = (xmm11) & (crate::simd_ops::ru128(P2)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P3)); ymm0_hi = 0; xmm1 = crate::simd_ops::vcvttps2dq(xmm0); xmm2 = crate::simd_ops::vpaddd(xmm1, crate::simd_ops::ru128(P4)); xmm7 = (xmm2) & (crate::simd_ops::ru128(P5)); xmm0 = (xmm7) & (crate::simd_ops::ru128(P6)); ymm0_hi = 0; xmm1 = (xmm7) & (crate::simd_ops::ru128(P7)); xmm10 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm0 = 0u128; ymm0_hi = 0; xmm9 = crate::simd_ops::vpcmpeqd(xmm1, xmm0); xmm5 = crate::simd_ops::vcvtdq2ps(xmm7); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P8)); ymm0_hi = 0; xmm1 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P9)); xmm2 = crate::simd_ops::vaddps(xmm0, xmm3); xmm0 = crate::simd_ops::vmulps(xmm5, crate::simd_ops::ru128(P10)); ymm0_hi = 0; xmm3 = crate::simd_ops::vaddps(xmm2, xmm1); xmm1 = crate::simd_ops::vpsubd(xmm7, crate::simd_ops::ru128(P7)); xmm5 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (!(xmm1)) & (crate::simd_ops::ru128(P6)); ymm0_hi = 0; xmm8 = crate::simd_ops::vpslld(xmm0, 0x1du64); xmm3 = crate::simd_ops::vmulps(xmm5, xmm5); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P11)); ymm0_hi = 0; xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P12)); ymm0_hi = 0; xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P13)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); ymm0_hi = 0; xmm1 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P14)); xmm2 = crate::simd_ops::vmulps(xmm0, xmm3); xmm0 = crate::simd_ops::vsubps(xmm2, xmm1); ymm0_hi = 0; xmm7 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P15)); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P16)); ymm0_hi = 0; xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(P17)); ymm0_hi = 0; xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm1 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(P18)); xmm0 = crate::simd_ops::vmulps(xmm1, xmm3); ymm0_hi = 0; xmm2 = crate::simd_ops::vmulps(xmm0, xmm5); xmm0 = (xmm11) & (crate::simd_ops::ru128(P19)); ymm0_hi = 0; xmm11 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; xmm6 = crate::simd_ops::vaddps(xmm2, xmm5); xmm2 = (xmm0) ^ (xmm10); xmm3 = (xmm6) & (xmm9); xmm5 = (!(xmm9)) & (xmm7); xmm1 = crate::simd_ops::vaddps(xmm5, xmm3); xmm0 = crate::simd_ops::vsubps(xmm7, xmm5); ymm0_hi = 0; xmm4 = (xmm2) ^ (xmm1); xmm3 = crate::simd_ops::vsubps(xmm6, xmm3); xmm1 = crate::simd_ops::vaddps(xmm3, xmm0); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x60u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); xmm2 = (xmm1) ^ (xmm8); rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x88u64)))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; xmm4 = crate::simd_ops::vblendps(xmm2, xmm4, 0x7u8); crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm4) as u128); let y_addr: u64 = rsp.wrapping_add(0x20u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((xmm12 as u32)) as u32); xmm12 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu32(rcx.wrapping_add(0x24u64), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x84u64))) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rbx.wrapping_add(0xa8u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), ((xmm0 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xb0u64), (0x0u8) as u8); xmm10 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm9 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x80u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; bb = 24; continue; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 19; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0x10u64))) as u64; rdx = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x30u64))) as u32 as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x28u64))) as u64; r8 = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P20, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((rax as u32)) as u32 as u64; xmm4 = 0u128; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru16(rdi)) as u32) as u32 as u64; rax = ((rax).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32) as u128; ymm0_hi = 0; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xb4u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 10; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 9; continue; }
            9 => { crate::simd_ops::wu8(rbx.wrapping_add(0xb4u64), (0x1u8) as u8); rax = 0x1; bb = 25; continue; }
            10 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0xb0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(0xacu64))) as u32) as u128; ymm0_hi = 0; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 14; continue; }
            12 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = 0u128; xmm0 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); ymm0_hi = 0; fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), ((xmm0 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { crate::simd_ops::wu8(rbx.wrapping_add(0xb0u64), (0x1u8) as u8); bb = 14; continue; }
            14 => { rcx = (rcx & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = 0x3; bb = 25; continue; }
            16 => { fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 17; continue; }
            17 => { crate::simd_ops::wu8(rbx.wrapping_add(0xb4u64), (0x1u8) as u8); bb = 18; continue; }
            18 => { rax = 0x1; bb = 25; continue; }
            19 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 24; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0x88u64)))) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x24u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 23; continue; } bb = 21; continue; }
            21 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x80u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), rax); bb = 23; continue; }
            23 => { crate::simd_ops::wu8(rbx.wrapping_add(0xb4u64), (0x0u8) as u8); bb = 24; continue; }
            24 => { rax = 0; fl_l = 0; fl_r = 0; bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xa0u64))) as u128; rsp = ((rsp).wrapping_add(0xb0u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd09d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x84; const P1: u64 = 0xa8; const P2: u64 = 0xac; const P3: u64 = 0xb0; const P4: u64 = 0xb4;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32) as u128; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm0 as u32)) as u32); crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x0u8) as u8); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 4; continue; }
            4 => { xmm1 = ((crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32) as u128; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P3))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 5; continue; }
            5 => { xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; bb = 8; continue; }
            6 => { xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = 0u128; xmm1 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); xmm4 = 0u128; fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((xmm1 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { crate::simd_ops::wu8(rcx.wrapping_add(P3), (0x1u8) as u8); bb = 8; continue; }
            8 => { rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = 0x1; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            10 => { rax = 0x3; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            11 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu8(rcx.wrapping_add(P4), (0x0u8) as u8); bb = 13; continue; }
            13 => { rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd0b60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405e37f0; const P1: u64 = 0x1405e3070; const P2: u64 = 0x143427f30; const P3: u64 = 0x143427080; const P4: u64 = 0x1434268d0; const P5: u64 = 0x143426514; const P6: u64 = 0x14183adac; const P7: u64 = 0x14065d520;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; rbx = rdx; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xe8u64))) as u64; r8 = rdi.wrapping_add(0x28); rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r8 = rdi.wrapping_add(0xa0); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; rax = rsp.wrapping_add(0x30); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; r9 = rdi.wrapping_add(0x28); crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm6) as u128); rcx = rsp.wrapping_add(0x40); xmm6 = 0u128; rdx = (crate::simd_ops::ru64(rdx)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x60u64), (xmm7) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm8) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0xd8u64))) as u64; r9 = rdi.wrapping_add(0xa0); r8 = (crate::simd_ops::ru64(rbx.wrapping_add(0xe8u64))) as u64; rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm6) as u128); xmm7 = (crate::simd_ops::ru128(rax)) as u128; rdx = (crate::simd_ops::ru64(rdx)) as u64; rax = rsp.wrapping_add(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); xmm8 = crate::simd_ops::vdpps(xmm7, xmm7, 0x7fu8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm5 = (crate::simd_ops::ru128(P2)) as u128; xmm1 = (crate::simd_ops::ru128(rax)) as u128; xmm3 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); xmm0 = crate::simd_ops::vsqrtps(xmm3); xmm2 = crate::simd_ops::vdivps(xmm1, xmm0); xmm1 = crate::simd_ops::vcmpps(xmm5, xmm3, 0x2u8); xmm4 = (xmm2) & (xmm1); xmm0 = crate::simd_ops::vsqrtps(xmm8); xmm3 = crate::simd_ops::vdivps(xmm7, xmm0); xmm0 = 0u128; xmm1 = crate::simd_ops::vcmpps(xmm5, xmm8, 0x2u8); xmm2 = (xmm3) & (xmm1); xmm1 = crate::simd_ops::vdpps(xmm2, xmm4, 0x7fu8); xmm0 = (xmm0 & !0xffff_ffffu128) | (xmm1 & 0xffff_ffffu128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) > f32::from_bits(crate::simd_ops::ru32(P3)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P3)) }).to_bits() as u128); xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x118u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P4)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P4)) }).to_bits() as u128); xmm6 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(P5))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x20u64))) as u32 as u64; xmm2 = (xmm6) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; bb = 4; continue; }
            3 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd17a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14025f9d0; const P1: u64 = 0x1434268d0; const P2: u64 = 0x1401e5730; const P3: u64 = 0x14069ab00; const P4: u64 = 0x1425371a0; const P5: u64 = 0x142533200; const P6: u64 = 0x1401e66f0;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = rdx; rdi = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x0u64) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x7u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x8u64) as u64); xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); r8 = rax; rdx = 0x10; rcx = rsp.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); r8 = rsp.wrapping_add(0x40); rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { r10 = rdi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            15 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            16 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; rsp = ((rsp).wrapping_add(0x80u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd2150(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1401e7c10; const P1: u64 = 0x14069bbc0; const P2: u64 = 0x143c8d074; const P3: u64 = 0x142489ee0; const P4: u64 = 0x143c8ff58; const P5: u64 = 0x143c8d120; const P6: u64 = 0x1421228d0; const P7: u64 = 0x141b9f4d0; const P8: u64 = 0x1426467e8;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xc0u64)) as u64; rbx = (crate::simd_ops::ru64(rcx)) as u64; r8 = rax.wrapping_sub(0x68); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x8u64))) as u128; ymm0_hi = 0; rcx = rax.wrapping_sub(0x58); crate::simd_ops::wu64(rsp.wrapping_add(0xf8u64), rbx); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm0) as u128); r14 = rbx.wrapping_add(0x40); rdx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x90u64))) as u128; ymm1_hi = 0; xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64) as u128; ymm0_hi = 0; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r14); rax = (((xmm1 >> 64) as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x28u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r15); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); fl_l = rax; fl_r = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rax = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(r14.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 4; continue; }
            3 => { rax = ((r15 as u32)) as u32 as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); bb = 5; continue; }
            5 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 29; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x100u64), r12); crate::simd_ops::wu64(rsp.wrapping_add(0x108u64), r13); crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm6) as u128); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdx = 0xffff; rcx = (crate::simd_ops::ru32(rax.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x10u64))) as u32 as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; r11 = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)))) as u64; fl_l = ((r11 as u16)) as u16 as u64; fl_r = ((rdx as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 27; continue; } bb = 8; continue; }
            8 => { r8 = ((crate::simd_ops::ru16(P2)) as u32) as u32 as u64; rcx = (((r11 as u16)) as u32) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(5u32)) as u64; r15 = (crate::simd_ops::ru64(r14.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; r12 = (crate::simd_ops::ru32(rax.wrapping_add(r14).wrapping_add(0x1806cu64))) as u32 as u64; r13 = (crate::simd_ops::ru64(rax.wrapping_add(r14).wrapping_add(0x18058u64))) as u64; rdx = ((r12 as u32)) as u32 as u64; rax = rcx.wrapping_add(0xc03); rcx = r13; rax = ((rax).wrapping_shl(5u32)) as u64; rbx = (crate::simd_ops::ru64(rax.wrapping_add(r14))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xf0u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; r11 = ((r11).wrapping_shr(32u32)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rax = ((rax).wrapping_add(r15)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(r11))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } bb = 9; continue; }
            9 => { r8 = ((crate::simd_ops::ru16(P4)) as u32) as u32 as u64; rdx = ((r12 as u32)) as u32 as u64; rcx = r13; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((((rax as u32)) as i32 as i64) as u64) as u64; rdx = ((r12 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(r8.wrapping_mul(4)))) as u32 as u64; r8 = ((crate::simd_ops::ru16(P5)) as u32) as u32 as u64; rax = ((rax).wrapping_add(r15)) as u64; rbx = (((r11) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xf0u64))) as u64; r11 = ((r11).wrapping_shl(5u32)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(rcx.wrapping_mul(4)))) as u32 as u64; rax = ((rax).wrapping_add(r15)) as u64; r11 = ((r11).wrapping_add(rax)) as u64; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x53u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 19; continue; } bb = 10; continue; }
            10 => { let y_addr: u64 = r11; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x90); let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { r15 = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rdi = ((rdi).wrapping_add(1)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 26; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; bb = 15; continue; }
            15 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 16; continue; }
            16 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 20; continue; } bb = 17; continue; }
            17 => { rax = ((rax).wrapping_add(0x2u64)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 15; continue; } bb = 18; continue; }
            18 => { bb = 25; continue; }
            19 => { let y_addr: u64 = r11; xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rdx = rsp.wrapping_add(0x80); r15 = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; ymm0_hi = 0; xmm6 = (ymm1_hi) as u128; rbx = (crate::simd_ops::ru64(rax.wrapping_add(rdi.wrapping_mul(8)))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm6) as u128); rcx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0xbcu64), ((r15 as u32)) as u32); crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x84u64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x80u64), ((r15 as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8cu64), r15); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xb8u64), ((r15 as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), ((r15 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(0xb8u64), ((rax as u8)) as u8); crate::simd_ops::wu128(rbx.wrapping_add(0xc0u64), (xmm6) as u128); bb = 12; continue; }
            20 => { fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 21; continue; }
            21 => { rcx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x10048u64))) as u32 as u64; bb = 24; continue; }
            23 => { rax = ((r15 as u32)) as u32 as u64; bb = 24; continue; }
            24 => { crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r15); bb = 25; continue; }
            25 => { rcx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; fl_l = rdi; fl_r = rsi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 28; continue; } bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xf8u64))) as u64; bb = 7; continue; }
            27 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P8), rax), rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::lift_ops::trap(0x80000003, 0x141bd245b, rcx, rsp); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            28 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x108u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x100u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xb0u64))) as u128; bb = 29; continue; }
            29 => { rsp = ((rsp).wrapping_add(0xc0u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd2490(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434268d0; const P1: u64 = 0x143426abc; const P2: u64 = 0x14211c310; const P3: u64 = 0x1421228d0; const P4: u64 = 0x141b9f4d0; const P5: u64 = 0x141e22a40; const P6: u64 = 0x141e234c0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 33>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xf0u64)) as u64; r15 = (crate::simd_ops::ru64(rcx.wrapping_add(0x50u64))) as u64; r10 = r9; rsi = r15; r12 = r8; rsi = ((rsi).wrapping_shl(6u32)) as u64; r13 = rdx; rsi = ((rsi).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(0x10u64)))) as u64; r14 = rcx; rax = (crate::simd_ops::ru32(rsi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rsi, ((rdi as u32)) as u32); bb = 12; continue; }
            2 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 12; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rsp.wrapping_add(0x110u64), rbx); rdi = 0; fl_l = 0; fl_r = 0; rbx = (((r15) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rcx))) as u64; fl_l = (crate::simd_ops::ru8(rsi.wrapping_add(0x30u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rsi.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; xmm1 = ((crate::simd_ops::ru32(P0)) as u32) as u128; ymm1_hi = 0; r9 = (crate::simd_ops::ru64(rbx)) as u64; rax = rsp.wrapping_add(0x40); xmm2 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x34u64))) as u32) as u128; rcx = (crate::simd_ops::ru64(r10)) as u64; rdx = rsp.wrapping_add(0x90); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rdi as u32)) as u32); r9 = ((r9).wrapping_add(0x58u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rsi.wrapping_add(0x4u64), ((rdi as u8)) as u8); bb = 11; continue; }
            6 => { xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsi.wrapping_add(0x20u64), (xmm0) as u128); bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x53u64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 9; continue; } bb = 8; continue; }
            8 => { let y_addr: u64 = rsi.wrapping_add(0x10u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsp.wrapping_add(0x40); let y_addr: u64 = rsp.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 10; continue; }
            9 => { let y_addr: u64 = rsi.wrapping_add(0x10u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x8u64))) as u64; rdx = rsp.wrapping_add(0x40); crate::simd_ops::wu128(rsp.wrapping_add(0xe0u64), (xmm6) as u128); xmm0 = 0u128; ymm0_hi = 0; xmm6 = (ymm1_hi) as u128; rbx = (crate::simd_ops::ru64(rax.wrapping_add(r15.wrapping_mul(8)))) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm6) as u128); rcx = rbx; crate::simd_ops::wu32(rbx.wrapping_add(0xbcu64), ((rdi as u32)) as u32); crate::simd_ops::wu128(rbx.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rbx.wrapping_add(0x84u64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x80u64), ((rdi as u32)) as u32); crate::simd_ops::wu64(rbx.wrapping_add(0x8cu64), rdi); crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rdi as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x28u64), ((rdi as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0xb8u64), ((rdi as u8)) as u8); crate::simd_ops::wu32(rbx.wrapping_add(0xacu64), ((rdi as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu128(rbx.wrapping_add(0xc0u64), (xmm6) as u128); xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0xe0u64))) as u128; crate::simd_ops::wu8(rbx.wrapping_add(0xb8u64), ((rax as u8)) as u8); bb = 10; continue; }
            10 => { let y_addr: u64 = rsi.wrapping_add(0x10u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; r8 = rsp.wrapping_add(0x90); xmm1 = (crate::simd_ops::ru128(r12)) as u128; ymm1_hi = 0; rdi = (crate::simd_ops::ru64(r14.wrapping_add(0x18u64))) as u64; rdx = rsp.wrapping_add(0x40); rbx = (crate::simd_ops::ru64(r14.wrapping_add(0x48u64))) as u64; rcx = rsp.wrapping_add(0xb0); let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(r12.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r15); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64) as u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0xb0u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); let y_addr: u64 = rsi.wrapping_add(0x10u64); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x28u64))) as u64; r8 = rsp.wrapping_add(0x90); xmm1 = (crate::simd_ops::ru128(r13)) as u128; ymm1_hi = 0; let y_addr: u64 = rsp.wrapping_add(0x90u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm0 = ((crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rax); rdx = rsp.wrapping_add(0x40); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; rcx = rsp.wrapping_add(0xb0); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rbx); let y_addr: u64 = rsp.wrapping_add(0x60u64); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), r15); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64) as u128; ymm0_hi = 0; let y_addr: u64 = rsp.wrapping_add(0xb0u64); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi); crate::simd_ops::wu64(rsp.wrapping_add(0xd0u64), ((xmm0 as u64)) as u64); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(r14.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu8(r15.wrapping_add(rax), (0x1u8) as u8); crate::simd_ops::wu8(rsi.wrapping_add(0x4u64), (0x1u8) as u8); bb = 11; continue; }
            11 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x110u64))) as u64; crate::simd_ops::wu32(rsi, (0x2u32) as u32); bb = 12; continue; }
            12 => { r11 = rsp.wrapping_add(0xf0); rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd2770(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x4;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu32(rcx, (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd27a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x34; const P2: u64 = 0x4; const P3: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru32(rcx)) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = rcx.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm3 as u32)) as u32);
    crate::simd_ops::wu32(rcx, (0x1u32) as u32);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    crate::simd_ops::wu8(rcx.wrapping_add(P2), (0x0u8) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(P3), ((r8 as u8)) as u8);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd27e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14387cbe8; const P1: u64 = 0x14069c6b0; const P2: u64 = 0x1402116e0; const P3: u64 = 0x1427c93a0; const P4: u64 = 0x1410d6ca0; const P5: u64 = 0x14069bd30; const P6: u64 = 0x143823420; const P7: u64 = 0x14020e980; const P8: u64 = 0x14020afa0; const P9: u64 = 0x1420a6b70; const P10: u64 = 0x14069c050; const P11: u64 = 0x143823500;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbx = rcx; rcx = (crate::simd_ops::ru64(rcx)) as u64; rcx = ((rcx).wrapping_add(0xc0u64)) as u64; rax = (crate::simd_ops::ru32(P0)) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = P1; crate::simd_ops::wu64(r11.wrapping_sub(0x70u64), rax); r8 = r11.wrapping_sub(0x78); rdx = r11.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x8u64)) as u64; xmm0 = (crate::simd_ops::ru128(P3)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x18); r8 = rsp.wrapping_add(0x20); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; r15 = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P6)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; rsi = rbx.wrapping_add(0x8); r12 = 0xffff; r13 = (0x2aaaaaaaaaaaaaabu64) as u64; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 4; continue; } bb = 1; continue; }
            1 => { rbx = (crate::simd_ops::ru64(rsi)) as u64; xmm0 = (crate::simd_ops::ru128(P6)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rdx = rdi.wrapping_add(0x1); rcx = rbx.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rbx.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 4; continue; }
            4 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((r15 as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rsp.wrapping_add(0x30); rcx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); r9 = 0x1; r8 = (((r15 as u16)) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rsi.wrapping_add(0x48); xmm0 = (crate::simd_ops::ru128(P11)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rbx = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; fl_l = ((rbx as u16)) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 8; continue; } bb = 5; continue; }
            5 => { rdi = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(P11)) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rcx.wrapping_add(rcx.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rbx = rdx; rbx = ((((rbx) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rbx; rax = ((rax).wrapping_shr(63u32)) as u64; rbx = ((rbx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rdx = rbx.wrapping_add(0x1); rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 7; continue; }
            7 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; fl_l = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rdx.wrapping_sub(0x30u64), (xmm0) as u128); crate::simd_ops::wu32(rdx.wrapping_sub(0xeu64), (0xffffffffu32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu64(rdx.wrapping_sub(0x20u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu64(rdx.wrapping_sub(0x18u64), rbx); crate::simd_ops::wu16(r14, ((rbx as u16)) as u16); bb = 8; continue; }
            8 => { r9 = 0x1; r8 = (((rbp as u16)) as u32) as u32 as u64; rdx = (((rbx as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = rsp.wrapping_add(0x70); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd2b90(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xbc; const P1: u64 = 0xac;
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(P0), (1) as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x3u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd2bd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143426780; const P1: u64 = 0x143427048;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; r11 = rdx; r9 = r8; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (0xffffffffffffffffu64) as u64); r9 = ((r9).wrapping_shr(32u32)) as u64; r10 = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (0xffffffffffffffffu64) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 6; continue; }
            5 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0xffffffff; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 10; continue; } bb = 7; continue; }
            7 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 8; continue; }
            8 => { rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = r8; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 22; continue; } bb = 10; continue; }
            10 => { xmm0 = ((crate::simd_ops::ru32(P0)) as u32) as u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(r11.wrapping_add(0x20u64))) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(r10.wrapping_add(0x8u64))) as u64; xmm0 = (crate::simd_ops::ru128(rax)) as u128; xmm1 = crate::simd_ops::vdpps(xmm0, crate::simd_ops::ru128(r11.wrapping_add(0x50u64)), 0x7fu8); fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P1)) as u32 as u64; if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 22; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (0xffffffffffffffffu64) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rcx = r8; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 23; continue; } bb = 14; continue; }
            14 => { rdx = (crate::simd_ops::ru64(r10.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (0xffffffffffffffffu64) as u64); fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0xffffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 15; continue; }
            15 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x10u64))) as u64; rcx = r8; rcx = ((rcx).wrapping_shr(32u32)) as u64; rax = ((rax).wrapping_shr(32u32)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 17; continue; } bb = 16; continue; }
            16 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 18; continue; }
            17 => { rcx = (rcx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0xffffffff; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 23; continue; } bb = 19; continue; }
            19 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 21; continue; }
            21 => { rax = ((rax).wrapping_shr(32u32)) as u64; r8 = ((r8).wrapping_shr(32u32)) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            23 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd2d00(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143c8f63c; const P1: u64 = 0x143c8d08c; const P2: u64 = 0x1401fa730; const P3: u64 = 0x143426494; const P4: u64 = 0x143428460; const P5: u64 = 0x143428120; const P6: u64 = 0x143241f30; const P7: u64 = 0x1401fa380; const P8: u64 = 0x141e91e10;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 39>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rax.wrapping_sub(0x38); rsp = ((rsp).wrapping_sub(0x120u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm8) as u128); rsi = r9; r14 = r8; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(0x580e0u64))) as u64; rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64; rdi = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = ((crate::simd_ops::ru16(P1)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(r10.wrapping_add(rax.wrapping_mul(8)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0xb0u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rax); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 3; continue; }
            2 => { r8 = rbp.wrapping_add(0x48); rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), rax); bb = 6; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), (0xffffffffffffffffu64) as u64); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rbp.wrapping_add(0x68u64))) as u64; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), rcx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; bb = 9; continue; }
            8 => { r8 = rbp.wrapping_add(0x48); rdx = rsp.wrapping_add(0x30); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = (0xffffffffffffffffu64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x40u64))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rbx.wrapping_add(0x58u64)))) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), rax); bb = 12; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_add(0x68u64), (0xffffffffffffffffu64) as u64); bb = 12; continue; }
            12 => { xmm6 = ((crate::simd_ops::ru32(rbp.wrapping_add(0x60u64))) as u32) as u128; xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(P3))).to_bits() as u128); xmm1 = 0u128; xmm1 = crate::simd_ops::vinsertps(xmm1, xmm0, 0x10u8); xmm7 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(rsi)); xmm8 = crate::simd_ops::vaddps(xmm1, crate::simd_ops::ru128(r14)); xmm5 = crate::simd_ops::vsubps(xmm7, xmm8); xmm0 = crate::simd_ops::vdpps(xmm5, xmm5, 0x7fu8); xmm2 = (crate::simd_ops::ru128(P4)) as u128; xmm3 = crate::simd_ops::vrsqrtps(xmm0); xmm0 = crate::simd_ops::vmulps(xmm3, xmm0); xmm1 = crate::simd_ops::vmulps(xmm0, xmm3); xmm2 = crate::simd_ops::vsubps(xmm2, xmm1); xmm0 = crate::simd_ops::vmulps(xmm3, crate::simd_ops::ru128(P5)); xmm1 = crate::simd_ops::vmulps(xmm0, xmm2); xmm0 = crate::simd_ops::vmulps(xmm1, xmm5); crate::simd_ops::wu128(rbp.wrapping_sub(0x80u64), (xmm0) as u128); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x60u64), (0x15u64) as u64); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); crate::simd_ops::wu32(rbp.wrapping_sub(0x18u64), ((rax as u32)) as u32); rax = P6; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); rax = rbp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); rax = rbp.wrapping_sub(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rax); rax = rbp.wrapping_add(0x68); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); rax = rsp.wrapping_add(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rax); rdx = rbp.wrapping_sub(0x58); rcx = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rsp.wrapping_add(0x40); fl_l = rcx; fl_r = rdx; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x70u64), (xmm8) as u128); rax = rbp.wrapping_sub(0x60); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x30); r8 = rbp.wrapping_sub(0x70); rdx = ((xmm6 as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rdi)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (((rax as u8)) as u32) as u32 as u64; rbx = (rbx & !0xffu64) | ((((((rbx as u8)) ^ (0x1u8))) as u8 as u64)); rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x20u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { r8 = (crate::simd_ops::ru64(rcx)) as u64; rax = rbp.wrapping_sub(0x58); fl_l = rcx; fl_r = rax; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) != ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 16; continue; }
            16 => { rax = (((rbx as u8)) as u32) as u32 as u64; r11 = rsp.wrapping_add(0x120); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd2f50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xbc; const P1: u64 = 0x30; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x28; const P5: u64 = 0x84; const P6: u64 = 0x80; const P7: u64 = 0x8c; const P8: u64 = 0xb8; const P9: u64 = 0xac; const P10: u64 = 0x141b9f4d0; const P11: u64 = 0xc0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    rbx = rdx;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    rdi = rcx;
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P2))) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P8), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P9), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P2))) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu128(rdi.wrapping_add(P11), (xmm0) as u128);
    crate::simd_ops::wu8(rdi.wrapping_add(0xb8u64), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd2fd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x1434264ec;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P0))) as u128;
    xmm1 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(rdx));
    xmm0 = 0u128;
    xmm3 = (xmm0 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128);
    xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128);
    xmm4 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8);
    fl_l = ((xmm3 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd3000(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141b9f4d0; const P1: u64 = 0xc0; const P2: u64 = 0xb8;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    xmm0 = (crate::simd_ops::ru128(rdx)) as u128;
    rbx = rdx;
    rdi = rcx;
    rdx = rsp.wrapping_add(0x20);
    crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    xmm0 = (crate::simd_ops::ru128(rbx)) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    crate::simd_ops::wu128(rdi.wrapping_add(P1), (xmm0) as u128);
    crate::simd_ops::wu8(rdi.wrapping_add(P2), ((rax as u8)) as u8);
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd3050(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0xb8; const P2: u64 = 0xac; const P3: u64 = 0xa8; const P4: u64 = 0x90; const P5: u64 = 0x88; const P6: u64 = 0x80; const P7: u64 = 0x20; const P8: u64 = 0x10; const P9: u64 = 0xffffffffffffffff; const P10: u64 = 0x60; const P11: u64 = 0x70;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = rcx; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rcx.wrapping_add(P1), (0x0u8) as u8); crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x3u32) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rcx.wrapping_add(P2), (0x2u32) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { xmm0 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P3)))).to_bits() as u128); xmm2 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P4)))).to_bits() as u128); xmm1 = 0u128; xmm4 = (xmm1 & !0xffff_ffffu128) | (xmm2 & 0xffff_ffffu128); xmm4 = (xmm4 & !0xffff_ffffu128) | ((if f32::from_bits(xmm4 as u32) < f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P5))) { f32::from_bits(xmm4 as u32) } else { f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(P5))) }).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(P4), ((xmm4 as u32)) as u32); rcx = (((crate::simd_ops::ru32(rcx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rdx = rax; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 10; continue; } bb = 5; continue; }
            5 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(r8.wrapping_add(P7))) as u64; r9 = ((((rcx as u32)) as i32 as i64) as u64) as u64; xmm1 = (xmm4) as u128; xmm2 = ((crate::simd_ops::ru32(rax.wrapping_add(r9.wrapping_mul(4)))) as u32) as u128; fl_l = ((xmm2 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; rax = ((((rcx as u32)) as i32 as i64) as u64) as u64; crate::simd_ops::wu32(r8.wrapping_add(P6), ((rcx as u32)) as u32); fl_l = rax; fl_r = rdx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } bb = 8; continue; }
            8 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rcx = (crate::simd_ops::ru64(r8.wrapping_add(P8))) as u64; rdx = ((r9.wrapping_add(P9)) as u64) as u64; xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(crate::simd_ops::ru32(rax.wrapping_add(rdx.wrapping_mul(4))))).to_bits() as u128); xmm2 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); r9 = ((r9).wrapping_add(r9)) as u64; rax = ((rax.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; xmm2 = crate::simd_ops::vbroadcastss((xmm2 as u32)); xmm4 = (crate::simd_ops::ru128(rcx.wrapping_add(r9.wrapping_mul(8)))) as u128; xmm1 = crate::simd_ops::vsubps(xmm4, crate::simd_ops::ru128(rcx.wrapping_add(rdx.wrapping_mul(8)))); xmm0 = crate::simd_ops::vmulps(xmm2, xmm1); xmm3 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(rcx.wrapping_add(rdx.wrapping_mul(8)))); crate::simd_ops::wu128(r8.wrapping_add(P10), (xmm3) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(r9.wrapping_mul(8)))) as u128; xmm2 = crate::simd_ops::vsubps(xmm0, crate::simd_ops::ru128(rcx.wrapping_add(rdx.wrapping_mul(8)))); crate::simd_ops::wu128(r8.wrapping_add(P11), (xmm2) as u128); bb = 10; continue; }
            10 => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd3140(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1434264e4; const P1: u64 = 0x1434268d0; const P2: u64 = 0x143428460; const P3: u64 = 0x143428120;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(0x18u64))) as u32 as u64; rsi = r9; rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1e0u64))) as u32 as u64; rdi = r8; r10 = (((crate::simd_ops::ru32(rcx.wrapping_add(0x80u64))) as i32 as i64) as u64) as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0xc0u64))) as u128; rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rbx as u32) } else { (rax as u32) }) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x10u64), (xmm6) as u128); crate::simd_ops::wu128(rsp, (xmm7) as u128); r11 = rcx; r9 = 0x1; crate::simd_ops::wu128(rdx, (xmm0) as u128); fl_l = ((r10 as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if ((((fl_l as u32)) as i32) >= (((fl_r as u32)) as i32)) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = r10; r8 = ((r9.wrapping_add(0xfu64)) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; r10 = ((rax as u32)) as u32 as u64; r9 = rax.wrapping_add(0x1); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; r8 = r8.wrapping_add(0x10); rcx = rcx.wrapping_add(0x10); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(rax).wrapping_sub(0x10u64))) as u128; crate::simd_ops::wu128(r8.wrapping_add(rdx).wrapping_sub(0x10u64), (xmm0) as u128); fl_l = r10; fl_r = (0x1u64) as u64; r10 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { xmm6 = (crate::simd_ops::ru128(rdi)) as u128; xmm2 = crate::simd_ops::vdpps(xmm6, xmm6, 0x7fu8); xmm7 = crate::simd_ops::vsqrtps(xmm2); fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) > 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(P0)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 9; continue; } bb = 5; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru32(rsi.wrapping_add(0x8u64))) as u32) as u128; xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm5 = 0u128; fl_l = ((xmm1 as u32)) as u32 as u64; fl_r = ((xmm5 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 7; continue; } bb = 6; continue; }
            6 => { if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm5 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); bb = 8; continue; }
            8 => { xmm1 = (crate::simd_ops::ru128(P2)) as u128; xmm4 = crate::simd_ops::vrsqrtps(xmm2); xmm0 = crate::simd_ops::vmulps(xmm4, xmm2); xmm2 = crate::simd_ops::vmulps(xmm0, xmm4); xmm0 = crate::simd_ops::vmulps(xmm4, crate::simd_ops::ru128(P3)); xmm3 = crate::simd_ops::vsubps(xmm1, xmm2); xmm1 = crate::simd_ops::vmulps(xmm3, xmm0); rax = r9; xmm4 = crate::simd_ops::vmulps(xmm1, xmm6); rax = ((rax).wrapping_add(rax)) as u64; xmm2 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm0 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm5 as u32)).to_bits() as u128); xmm0 = crate::simd_ops::vbroadcastss((xmm0 as u32)); xmm0 = crate::simd_ops::vmulps(xmm4, xmm0); xmm1 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(r11.wrapping_add(0xc0u64))); crate::simd_ops::wu128(rdx.wrapping_add(rax.wrapping_mul(8)), (xmm1) as u128); fl_l = ((r9).wrapping_add(1)) as u64; r9 = ((fl_l as u64)) as u64; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x1e0u64))) as u64; fl_l = r9; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 13; continue; } bb = 10; continue; }
            10 => { rax = ((rax).wrapping_add(rax)) as u64; rcx = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = r9; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rdx)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { bb = 12; continue; }
            12 => { xmm0 = 0u128; crate::simd_ops::wu128(rcx, (xmm0) as u128); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 13; continue; }
            13 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp)) as u128; crate::simd_ops::wu64(rdx.wrapping_add(0x1e0u64), r9); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd32c0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bd2d00; const P1: u64 = 0x14062ee30; const P2: u64 = 0x1405ebda0;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm7) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm8) as u128); r14 = r9; rbp = r8; rdi = rcx; r15 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rcx.wrapping_add(0x18u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rcx.wrapping_add(0x28u64), ((r15 as u32)) as u32); xmm0 = 0u128; crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm0) as u128); rsi = rcx.wrapping_add(0x88); crate::simd_ops::wu32(rsi, ((r15 as u32)) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x80u64), r15); crate::simd_ops::wu64(rcx.wrapping_add(0x8cu64), r15); crate::simd_ops::wu8(rcx.wrapping_add(0xb8u64), ((r15 as u8)) as u8); xmm0 = (crate::simd_ops::ru128(r9)) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm0) as u128); xmm1 = (crate::simd_ops::ru128(r8)) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm1) as u128); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0xa0u64))) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((xmm0 as u32)) as u32); r9 = rsp.wrapping_add(0x30); r8 = rsp.wrapping_add(0x40); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 12; continue; } bb = 1; continue; }
            1 => { xmm7 = (crate::simd_ops::ru128(rbp)) as u128; xmm6 = 0u128; xmm8 = 0u128; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm1 = crate::simd_ops::vsubps(xmm7, crate::simd_ops::ru128(rdx.wrapping_sub(0x10u64))); xmm0 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); xmm8 = crate::simd_ops::vsqrtps(xmm0); bb = 4; continue; }
            3 => { crate::simd_ops::wu128(rdi.wrapping_add(0x30u64), (xmm7) as u128); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu128(rax, (xmm7) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); xmm0 = (xmm8 & !0xffff_ffffu128) | ((f32::from_bits(xmm8 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi))).to_bits() as u128); crate::simd_ops::wu32(rsi, ((xmm0 as u32)) as u32); rdx = rsi; rcx = rdi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm7 = (crate::simd_ops::ru128(r14)) as u128; rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 7; continue; }
            7 => { xmm1 = crate::simd_ops::vsubps(xmm7, crate::simd_ops::ru128(rdx.wrapping_sub(0x10u64))); xmm0 = crate::simd_ops::vdpps(xmm1, xmm1, 0x7fu8); xmm6 = crate::simd_ops::vsqrtps(xmm0); bb = 9; continue; }
            8 => { crate::simd_ops::wu128(rdi.wrapping_add(0x30u64), (xmm7) as u128); bb = 9; continue; }
            9 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rdx as u32)) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; fl_l = rax; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = rdx.wrapping_add(0x1); rdx = ((r8 as u32)) as u32 as u64; rdx = ((rdx).wrapping_shr(1u32)) as u64; rdx = ((rdx).wrapping_add(r8)) as u64; fl_l = rdx; fl_r = rax; rdx = (if (((fl_l as u64)) < ((fl_r as u64))) { rax } else { rdx }) as u64; rcx = rdi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x10u64)))) as u64; crate::simd_ops::wu128(rax, (xmm7) as u128); crate::simd_ops::lk_add_u32(rdi.wrapping_add(0x18u64), (1) as u32); xmm0 = (xmm6 & !0xffff_ffffu128) | ((f32::from_bits(xmm6 as u32) + f32::from_bits(crate::simd_ops::ru32(rsi))).to_bits() as u128); crate::simd_ops::wu32(rsi, ((xmm0 as u32)) as u32); rdx = rsi; rcx = rdi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0xbcu64), ((r15 as u32)) as u32); rax = 0x1; bb = 13; continue; }
            12 => { fl_l = (crate::simd_ops::lk_add_u32(rdi.wrapping_add(0xbcu64), (1) as u32)) as u32 as u64; rax = 0x3; bb = 13; continue; }
            13 => { crate::simd_ops::wu32(rdi.wrapping_add(0xacu64), ((rax as u32)) as u32); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = (rax & !0xffu64) | (((if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); r11 = rsp.wrapping_add(0x80); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd34e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x18; const P1: u64 = 0x28; const P2: u64 = 0x84; const P3: u64 = 0x80; const P4: u64 = 0x8c; const P5: u64 = 0x30; const P6: u64 = 0xb8; const P7: u64 = 0xac;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    xmm0 = 0u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P5), (xmm0) as u128);
    crate::simd_ops::wu8(rcx.wrapping_add(P6), ((rax as u8)) as u8);
    crate::simd_ops::wu32(rcx.wrapping_add(P7), ((rax as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd35e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bab7d0; const P1: u64 = 0x141bd32c0; const P2: u64 = 0x143427f30;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rbx = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; r9 = ((rdx as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(r8.wrapping_add(0xb0u64))) as u64; rdi = r8; rdx = (((rbx) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rsi = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; rbp = rcx; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0x48u64)))) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0x20u64)))) as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { r9 = rbx; r8 = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsi)) as u64; r9 = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x78u64))) as u32 as u64; r8 = rsp.wrapping_add(0x40); crate::simd_ops::wu32(rcx.wrapping_add(0xa8u64), ((rax as u32)) as u32); rax = rbp.wrapping_add(0x70); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_add(0x60u64))) as u128; xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; rcx = (crate::simd_ops::ru64(rsi)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm1) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 11; continue; }
            3 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 9; continue; } bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0xacu64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = ((r9.wrapping_add(0x3u64)) as u32) as u32 as u64; bb = 12; continue; }
            6 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 8; continue; } bb = 7; continue; }
            7 => { rax = 0x4; bb = 12; continue; }
            8 => { xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(0x60u64))) as u128; xmm2 = crate::simd_ops::vsubps(xmm1, crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))); xmm1 = (crate::simd_ops::ru128(P2)) as u128; rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x84u64))) as u32) as u32 as u64; xmm4 = crate::simd_ops::vdpps(xmm2, xmm2, 0x7fu8); xmm0 = crate::simd_ops::vsqrtps(xmm4); xmm3 = crate::simd_ops::vdivps(xmm2, xmm0); xmm0 = crate::simd_ops::vbroadcastss(crate::simd_ops::ru32(rcx.wrapping_add(0xa8u64))); xmm2 = crate::simd_ops::vcmpps(xmm1, xmm4, 0x2u8); xmm4 = (xmm3) & (xmm2); xmm1 = crate::simd_ops::vmulps(xmm4, xmm0); crate::simd_ops::wu8(rdx.wrapping_add(0x50u64), ((rax as u8)) as u8); rax = 0x1; crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm1) as u128); bb = 12; continue; }
            9 => { fl_l = ((r9 as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 11; continue; } bb = 10; continue; }
            10 => { xmm1 = (crate::simd_ops::ru128(rcx.wrapping_add(0x60u64))) as u128; xmm2 = (crate::simd_ops::ru128(rbx.wrapping_add(0x10u64))) as u128; rax = (crate::simd_ops::ru64(rsi)) as u64; rcx = 0; fl_l = 0; fl_r = 0; xmm0 = 0u128; xmm1 = crate::simd_ops::vsubps(xmm1, xmm2); xmm2 = crate::simd_ops::vmulps(xmm1, xmm0); crate::simd_ops::wu32(rax.wrapping_add(0x18u64), ((rcx as u32)) as u32); crate::simd_ops::wu32(rax.wrapping_add(0x28u64), ((rcx as u32)) as u32); crate::simd_ops::wu128(rax.wrapping_add(0x30u64), (xmm0) as u128); crate::simd_ops::wu64(rax.wrapping_add(0x84u64), rcx); crate::simd_ops::wu32(rax.wrapping_add(0x80u64), ((rcx as u32)) as u32); crate::simd_ops::wu64(rax.wrapping_add(0x8cu64), rcx); crate::simd_ops::wu8(rax.wrapping_add(0xb8u64), ((rcx as u8)) as u8); crate::simd_ops::wu32(rax.wrapping_add(0xacu64), ((rcx as u32)) as u32); rax = ((crate::simd_ops::ru8(rdx.wrapping_add(0x84u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdx.wrapping_add(0x50u64), ((rax as u8)) as u8); crate::simd_ops::wu128(rdx.wrapping_add(0x20u64), (xmm2) as u128); bb = 11; continue; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            12 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd38e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba4aa0; const P1: u64 = 0x143241f68; const P2: u64 = 0x1430f6b98; const P3: u64 = 0x40; const P4: u64 = 0x8; const P5: u64 = 0x48; const P6: u64 = 0xc; const P7: u64 = 0x4c; const P8: u64 = 0x10; const P9: u64 = 0x50; const P10: u64 = 0x14; const P11: u64 = 0x54; const P12: u64 = 0x14323c9d8; const P13: u64 = 0x1431f8560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rdi, rax); rax = P2; crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P11), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x58u64), ((rax as u8)) as u8); rax = P12; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rax); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x60u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x38u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x78u64), ((rax as u32)) as u32); rax = P13; crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x88u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rax); rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd3980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1405e37f0; const P1: u64 = 0x1405e3070; const P2: u64 = 0x141babe30; const P3: u64 = 0x143239df8; const P4: u64 = 0x141bac380; const P5: u64 = 0x141bac240;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bd3980, funcinfo_rva: 0x373f448, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = (rdx) as u64; rbx = (rcx) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(0xd8u64))) as u64; r8 = ((rsi.wrapping_add(0x10u64)) as u64) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0xe8u64))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd39b1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x40u64), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0xd8u64))) as u64; rax = ((rsp.wrapping_add(0x40u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = ((rsi.wrapping_add(0x10u64)) as u64) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(0xe8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd39ec); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P1) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0xc8u64))) as u64; r11 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r11 = ((r11).wrapping_shl(5u32)) as u64; r11 = ((r11).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(0x70u64)))) as u64; rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rcx = (crate::simd_ops::ru32(r11.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 4; continue; }
            4 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(r11)) as u64; rcx = (crate::simd_ops::ru32(r11.wrapping_add(0x8u64))) as u32 as u64; r8 = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 6; continue; }
            6 => { rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64 as u64; fl_r = (rdx) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 7; continue; }
            7 => { rax = ((rax).wrapping_add(0x8u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (r8) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 8; continue; }
            8 => { rcx = (r11) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd3a5b); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rax as u8)) as u8); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), (0x1u32) as u32); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (r11) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd3aa2); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbx.wrapping_add(0x20u64), (rax) as u64); bb = 11; continue; }
            11 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rax as u8)) as u8); rax = ((P3) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), (0x1u32) as u32); r8 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (r11) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd3af7); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 11; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd3b10(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0xc8; const P2: u64 = 0x70; const P3: u64 = 0x20; const P4: u64 = 0x141bac780;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    r8 = rcx;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64;
    rcx = ((rcx).wrapping_shl(5u32)) as u64;
    fl_l = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P2)))) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rdx = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64;
    return crate::simd_ops::call_fwd(P4, rax, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd3b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1c; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x10; const P4: u64 = 0xd8; const P5: u64 = 0xe8; const P6: u64 = 0x1405e37f0; const P7: u64 = 0xc8; const P8: u64 = 0x70; const P9: u64 = 0x1405e3070; const P10: u64 = 0x143239df8; const P11: u64 = 0x141bac380;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bd3b40, funcinfo_rva: 0x373f480, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rbp) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rdi = (rdx) as u64; rsi = (rcx) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = (crate::simd_ops::ru32(r8.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32); rbp = ((r8.wrapping_add(P3)) as u64) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; r8 = (rbp) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd3b84); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; rbx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rbx = ((rbx).wrapping_shl(5u32)) as u64; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(rdi.wrapping_add(P8)))) as u64; xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = (crate::simd_ops::ru64(rdi.wrapping_add(P4))) as u64; rax = ((rsp.wrapping_add(0x30u64)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rax) as u64); r9 = (rbp) as u64; r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = ((rsp.wrapping_add(0x40u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd3bd1); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rax = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), (rax) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); crate::simd_ops::wu8(rsp.wrapping_add(0x68u64), ((rax as u8)) as u8); rax = ((P10) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x70u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x64u64), (0x1u32) as u32); rdx = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd3c0d); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rsi.wrapping_add(0x20u64), (rax) as u64); bb = 2; continue; }
            2 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd3c40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432421c0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rdx);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rdx as u32)) as u32);
    crate::simd_ops::wu8(rcx.wrapping_add(P4), ((rdx as u8)) as u8);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rdx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd41d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x1c; const P4: u64 = 0x20;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P1), rax);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P3))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P3), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P4), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4230(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1432421c0; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x1c; const P5: u64 = 0x20;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rcx);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rcx);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), ((rcx as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rcx as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd43a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1d; const P1: u64 = 0x80; const P2: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P1))) as u128; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); xmm0 = crate::simd_ops::vaddps(xmm0, crate::simd_ops::ru128(rcx.wrapping_add(P2))); crate::simd_ops::wu128(rcx.wrapping_add(P2), (xmm0) as u128); }
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0xc0; const P2: u64 = 0x48; const P3: u64 = 0x84;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x0u32) as u32);
    rcx = (((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(P3), (0x1u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4460(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x24; const P1: u64 = 0xc0; const P2: u64 = 0x48; const P3: u64 = 0x84;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), (0x1u32) as u32);
    rcx = (((crate::simd_ops::ru64(rdx.wrapping_add(P1))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(P3), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4590(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1d; const P1: u64 = 0x1438c7808; const P2: u64 = 0x1438b87a0; const P3: u64 = 0x14254768e; const P4: u64 = 0x10; const P5: u64 = 0x80;
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rax = rdx;
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8);
    rbx = rcx;
    crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), (0x0u32) as u32);
    rcx = rax;
    r9 = P1;
    r8 = P2;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(P4))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P5), (xmm0) as u128); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd45f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x60;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P0))) as u128;
    crate::simd_ops::wu128(rcx.wrapping_add(P1), (xmm0) as u128);
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4610(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x40; const P1: u64 = 0x18; const P2: u64 = 0x20; const P3: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rsi = rcx;
    rcx = ((rcx).wrapping_add(0x40u64)) as u64;
    rdi = rdx;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    r8 = (crate::simd_ops::ru64(rdi)) as u64;
    rcx = rdi;
    rbx = ((rax as u32)) as u32 as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rbx as u32)) as u32 as u64;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P2))) as u128;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64));
    crate::simd_ops::wu128(rsi.wrapping_add(P3), (xmm0) as u128);
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x98; const P1: u64 = 0xa0; const P2: u64 = 0x141bd4720; const P3: u64 = 0xc0; const P4: u64 = 0x48; const P5: u64 = 0x84;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; rsi = rdx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm3 = 0u128; r8 = rsi; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdi.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = P2; crate::simd_ops::wu64(rdi.wrapping_add(P0), rdi); xmm3 = 0u128; r8 = rsi; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); rdx = 0x1; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (((crate::simd_ops::ru64(rsi.wrapping_add(P3))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax).wrapping_add(P5), (0x1u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0xc0; const P1: u64 = 0x48; const P2: u64 = 0x84; const P3: u64 = 0x60; const P4: u64 = 0x70; const P5: u64 = 0x88;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (((crate::simd_ops::ru64(r8.wrapping_add(P0))) as i64).wrapping_mul((0xb0u64) as i64) as u64) as u64; rdi = rcx; rbx = ((rbx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(P1)))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 1; continue; }
            1 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), ((rdx as u8)) as u8); rax = 0; fl_l = 0; fl_r = 0; xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm0) as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x1u8) as u8); rax = 0x1; xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(P3))) as u128; crate::simd_ops::wu128(rbx.wrapping_add(P4), (xmm0) as u128); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            4 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x2; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu8(rbx.wrapping_add(P2), (0x0u8) as u8); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd47b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x1431f8560; const P1: u64 = 0x90; const P2: u64 = 0x1431283f0; const P3: u64 = 0x70; const P4: u64 = 0x1430f6b98; const P5: u64 = 0x40; const P6: u64 = 0x1432316d0;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = P2;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = P4;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = P6;
    crate::simd_ops::wu64(rcx, rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd47f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141ba4aa0; const P1: u64 = 0x1432422c0; const P2: u64 = 0x1430f6b98; const P3: u64 = 0x40; const P4: u64 = 0x8; const P5: u64 = 0x48; const P6: u64 = 0xc; const P7: u64 = 0x4c; const P8: u64 = 0x10; const P9: u64 = 0x50; const P10: u64 = 0x14; const P11: u64 = 0x54; const P12: u64 = 0x143239df8; const P13: u64 = 0x1432115d0; const P14: u64 = 0x1431f8560;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = P1; crate::simd_ops::wu64(rdi, rax); rax = P2; crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P9), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbx.wrapping_add(P10))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(P11), ((rax as u32)) as u32); rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rdi.wrapping_add(0x58u64), ((rax as u8)) as u8); rax = P12; crate::simd_ops::wu64(rdi.wrapping_add(0x40u64), rax); rax = P13; xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(0x20u64))) as u128; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu128(rdi.wrapping_add(0x60u64), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x70u64), rax); rax = P14; xmm0 = 0u128; crate::simd_ops::wu128(rdi.wrapping_add(0x80u64), (xmm0) as u128); crate::simd_ops::wu64(rdi.wrapping_add(0x90u64), rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rdi.wrapping_add(0x98u64), rax); crate::simd_ops::wu64(rdi.wrapping_add(0xa0u64), rax); rax = rdi; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd48a0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1433e1380; const P1: u64 = 0x143932190; const P2: u64 = 0x14169edf0; const P3: u64 = 0x1433e22c8; const P4: u64 = 0x14171df00; const P5: u64 = 0x14171de30; const P6: u64 = 0x14169edd0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm6) as u128); r12 = rdx; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(0x1c8u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } bb = 1; continue; }
            1 => { r13 = 0; fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; fl_r = r13; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((r13 as u32)) as u32); bb = 4; continue; }
            4 => { rdi = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0xa8u64), rdi); rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 17; continue; } bb = 5; continue; }
            5 => { xmm6 = 0u128; bb = 6; continue; }
            6 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x18u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 7; continue; }
            7 => { r14 = r13; fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 16; continue; } bb = 8; continue; }
            8 => { rbp = r13; rdi = P0; bb = 9; continue; }
            9 => { r15 = (crate::simd_ops::ru64(rbx.wrapping_add(0x20u64))) as u64; rcx = (crate::simd_ops::ru64(r12)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r13); crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), (0x11u32) as u32); xmm0 = (crate::simd_ops::ru128(r15.wrapping_add(rbp).wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vshufps(xmm0, xmm0, 0xd8u8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), ((xmm1 as u64)) as u64); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), (((xmm1 >> 64) as u32) as u32) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); crate::simd_ops::wu32(rsp.wrapping_add(0x44u64), ((xmm6 as u32)) as u32); rcx = (crate::simd_ops::ru32(r15.wrapping_add(rbp).wrapping_add(0x10u64))) as u32 as u64; rcx = ((rcx).wrapping_shl(32u32)) as u64; rax = (crate::simd_ops::ru32(r15.wrapping_add(rbp).wrapping_add(0x14u64))) as u32 as u64; rcx = (((rcx) | (rax))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rcx); rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; r8 = 0; fl_l = 0; fl_r = 0; rdx = ((r8.wrapping_add(0x60u64)) as u32) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(0x48u64))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsi = rax; crate::simd_ops::wu64(rsp.wrapping_add(0xb0u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu32(rax.wrapping_add(0x8u64), (0x1u32) as u32); crate::simd_ops::wu64(rax, rdi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), (0xffffffffffffffffu64) as u64); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rsi.wrapping_add(0x10u64), r13); rax = P3; crate::simd_ops::wu64(rsi, rax); crate::simd_ops::wu64(rsi.wrapping_add(0x38u64), r13); crate::simd_ops::wu32(rsi.wrapping_add(0x40u64), ((r13 as u32)) as u32); crate::simd_ops::wu64(rsi.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsi.wrapping_add(0x50u64), r13); crate::simd_ops::wu32(rsi.wrapping_add(0x58u64), ((r13 as u32)) as u32); bb = 12; continue; }
            11 => { rsi = r13; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(r15.wrapping_add(rbp).wrapping_add(0x50u64), rsi); rdx = rsp.wrapping_add(0x20); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r15.wrapping_add(rbp).wrapping_add(0x50u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 14; continue; }
            14 => { crate::simd_ops::wu8(rbx.wrapping_add(0x18u64), (0x1u8) as u8); r14 = ((r14).wrapping_add(1)) as u64; rbp = ((rbp).wrapping_add(0x60u64)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x28u64))) as u32 as u64; fl_l = r14; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 9; continue; } bb = 15; continue; }
            15 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0xa8u64))) as u64; bb = 16; continue; }
            16 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 17; continue; }
            17 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xa0u64))) as u64; xmm6 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4a60(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x30; const P3: u64 = 0x10; const P4: u64 = 0x28; const P5: u64 = 0x20; const P6: u64 = 0x40;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r10 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rcx = (0x100000001b3u64) as u64; r9 = (((rdx as u8)) as u32) as u32 as u64; rax = (0xcbf29ce484222325u64) as u64; r11 = r8; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x11u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x12u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x13u64))) as u32) as u32 as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; r9 = (((r9) ^ (rax))) as u64; rax = (crate::simd_ops::ru64(r10.wrapping_add(P1))) as u64; r9 = (((r9) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(r10.wrapping_add(P2))) as u64; rcx = (((rcx) & (r9))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rcx; fl_r = rcx; rcx = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r8 } else { rcx }) as u64; fl_l = rcx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x14u64))) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P4))) as u32 as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; fl_l = ((rax).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P5)))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; let y_addr: u64 = rax.wrapping_add(P5); xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = r11; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rax.wrapping_add(P6))) as u128; crate::simd_ops::wu128(r11.wrapping_add(0x20u64), (xmm1) as u128); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            9 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4b40(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14069e670; const P1: u64 = 0x143242358; const P2: u64 = 0x14069df10; const P3: u64 = 0x1425371a0; const P4: u64 = 0x142533200;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 21>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp; rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rsi = rdx; r14 = rcx; crate::simd_ops::wu8(rbp.wrapping_sub(0x40u64), (0x0u8) as u8); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x38u64), (xmm0) as u128); r12 = 0; fl_l = 0; fl_r = 0; r9 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r12); r10 = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r12); r11 = ((r12 as u32)) as u32 as u64; rdi = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = rdi; rcx = rbp.wrapping_sub(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x2cu64))) as u32 as u64; r10 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; bb = 2; continue; }
            2 => { rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rbp.wrapping_sub(0x40u64), ((rdx as u8)) as u8); r15 = (crate::simd_ops::ru32(r14.wrapping_add(0x40u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; rcx = ((rdi.wrapping_add(rdi.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rsi = rbx; rsi = ((rsi).wrapping_add(rcx)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { crate::simd_ops::wu32(rbp.wrapping_add(0x48u64), ((r15 as u32)) as u32); r13 = (0x2aaaaaaaaaaaaaabu64) as u64; r14 = P1; bb = 4; continue; }
            4 => { rax = ((r10 as u32)) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(r9)) as u64; rcx = r8; rcx = ((rcx).wrapping_sub(r9)) as u64; rax = r13; let prod = (rax as i64 as i128).wrapping_mul((rcx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(4u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = ((r11 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rdx = rdi.wrapping_add(0x1); rcx = rbp.wrapping_sub(0x38); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r11 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x2cu64))) as u32 as u64; r10 = (crate::simd_ops::ru32(rbp.wrapping_sub(0x30u64))) as u32 as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as u64; bb = 6; continue; }
            6 => { rax = ((r10 as u32)) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(5u32)) as u64; rcx = ((rcx).wrapping_add(r9)) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x20u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); let y_addr: u64 = rcx.wrapping_add(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu64(rcx, r14); crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), (0xffffffffffffffffu64) as u64); crate::simd_ops::wu32(rcx.wrapping_add(0x44u64), (0x0u32) as u32); crate::simd_ops::wu64(rcx.wrapping_add(0x50u64), r12); r10 = (((r10 as u32)).wrapping_add(1)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x30u64), ((r10 as u32)) as u32); rax = ((r10.wrapping_add(r10.wrapping_mul(2))) as u64) as u64; rax = ((rax).wrapping_shl(5u32)) as u64; let y_addr: u64 = rbx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16)); let y_addr: u64 = rax.wrapping_add(r9).wrapping_sub(0x40u64); crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); xmm1 = (crate::simd_ops::ru128(rbx.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rax.wrapping_add(r9).wrapping_sub(0x20u64), (xmm1) as u128); crate::simd_ops::wu32(rbp.wrapping_add(0x4cu64), ((rdi as u32)) as u32); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; crate::simd_ops::wu64(rax.wrapping_add(r9).wrapping_sub(0x50u64), rcx); rbx = ((rbx).wrapping_add(0x30u64)) as u64; fl_l = rbx; fl_r = rsi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 7; continue; }
            7 => { rdx = ((crate::simd_ops::ru8(rbp.wrapping_sub(0x40u64))) as u32) as u32 as u64; r14 = (crate::simd_ops::ru64(rbp.wrapping_add(0x40u64))) as u64; bb = 8; continue; }
            8 => { crate::simd_ops::wu32(rbp.wrapping_sub(0x28u64), ((r15 as u32)) as u32); crate::simd_ops::wu8(rbp.wrapping_sub(0x20u64), ((rdx as u8)) as u8); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rbp.wrapping_sub(0x18u64), (xmm0) as u128); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), r9); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), ((r10 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0xcu64), ((r11 as u32)) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r12); crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), r12); r8 = rbp.wrapping_sub(0x28); rdx = rbp.wrapping_sub(0x50); rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x10u64))) as u32 as u64; rdi = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rdi = ((rdi).wrapping_shl(5u32)) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rdi = ((rdi).wrapping_add(rsi)) as u64; fl_l = rsi; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 11; continue; } bb = 9; continue; }
            9 => { rbx = rsi; bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rax)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = ((rbx).wrapping_add(0x60u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } bb = 11; continue; }
            11 => { fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 12; continue; }
            12 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 16; continue; } bb = 14; continue; }
            14 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 19; continue; } bb = 15; continue; }
            15 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            16 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 18; continue; }
            17 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 18; continue; }
            18 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 19; continue; }
            19 => { crate::simd_ops::lk_add_u32(r14.wrapping_add(0x40u64), (1) as u32); rax = ((r15 as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0xc0u64))) as u64; rsp = ((rsp).wrapping_add(0x70u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14069e740; const P1: u64 = 0x1401e66f0;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu32(rsp.wrapping_add(0x18u64), ((r8 as u32)) as u32); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r9 = rdx; rcx = (((r8 as u8)) as u32) as u32 as u64; rsi = (0xcbf29ce484222325u64) as u64; rcx = (((rcx) ^ (rsi))) as u64; rdi = (0x100000001b3u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x41u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x42u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x43u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; rax = r10; rax = (((rax) & (rcx))) as u64; r11 = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; rax = ((rax).wrapping_add(rax)) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rax = (crate::simd_ops::ru64(r11.wrapping_add(rax.wrapping_mul(8)))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } bb = 4; continue; }
            4 => { bb = 6; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 15; continue; } bb = 7; continue; }
            7 => { fl_l = rbx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 15; continue; } bb = 8; continue; }
            8 => { rcx = ((crate::simd_ops::ru8(rbx.wrapping_add(0x10u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rsi))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x11u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x12u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; rax = ((crate::simd_ops::ru8(rbx.wrapping_add(0x13u64))) as u32) as u32 as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((rdi) as i64) as u64) as u64; r10 = (((r10) & (rcx))) as u64; r10 = ((r10).wrapping_add(r10)) as u64; rax = (crate::simd_ops::ru64(r11.wrapping_add(r10.wrapping_mul(8)))) as u64; fl_l = (crate::simd_ops::ru64(r11.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 12; continue; } bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { crate::simd_ops::wu64(r11.wrapping_add(r10.wrapping_mul(8)), rdx); crate::simd_ops::wu64(r11.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x8u64), rdx); bb = 14; continue; }
            11 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(r11.wrapping_add(r10.wrapping_mul(8)).wrapping_add(0x8u64), rdx); bb = 14; continue; }
            12 => { fl_l = rax; fl_r = rbx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 14; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(r11.wrapping_add(r10.wrapping_mul(8)), rax); bb = 14; continue; }
            14 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::lk_sub_u64(r9.wrapping_add(0x10u64), (1) as u64); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rcx); rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), rax); rcx = rbx.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 15; continue; }
            15 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd4f20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14024d8b0; const P1: u64 = 0x1434268d0; const P2: u64 = 0x1401e5730; const P3: u64 = 0x143ca77b8; const P4: u64 = 0x143cc2e40; const P5: u64 = 0x14069eb80; const P6: u64 = 0x14069ed10; const P7: u64 = 0x1425371a0; const P8: u64 = 0x142533200; const P9: u64 = 0x14069e740; const P10: u64 = 0x1401e66f0; const P11: u64 = 0x14069dc70; const P12: u64 = 0x1438234c0; const P13: u64 = 0x14020e980; const P14: u64 = 0x14020afa0; const P15: u64 = 0x1420a6b70;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut ymm0_hi: u128 = 0; let mut xmm6: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0xb0u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm6) as u128); rbp = rcx; rdi = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rax.wrapping_sub(0x68u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rax, rax); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rax); xmm0 = 0u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x68u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), (0x7u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), (0x8u64) as u64); xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; ymm0_hi = 0; crate::simd_ops::wu32(rsp.wrapping_add(0x50u64), ((xmm0 as u32)) as u32); r8 = rax; rdx = ((rdi.wrapping_add(0x10u64)) as u32) as u32 as u64; rcx = rsp.wrapping_add(0x68); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x90u64), ((rdi as u32)) as u32); rbx = ((crate::simd_ops::ru16(P3)) as u32) as u32 as u64; rcx = ((rbx as u32)) as u32 as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_add(0x90u64)))) as u64; rax = (crate::simd_ops::ru32(P4)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x3cu64), ((rax as u32)) as u32); rax = P5; crate::simd_ops::wu64(rcx.wrapping_add(0x30u64), rax); crate::simd_ops::wu8(rcx.wrapping_add(0x38u64), ((rdi as u8)) as u8); rdx = rsp.wrapping_add(0x50); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0xa0u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rbx.wrapping_mul(8)), rax); r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = ((rdi as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, rdi); rdi = (crate::simd_ops::ru64(rsi)) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = rbx; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 10; continue; }
            10 => { r10 = rsi.wrapping_sub(0x1); r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = r9; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = r10.wrapping_add(0x108); rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, rax); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), r9); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 17; continue; } bb = 13; continue; }
            13 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            14 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 16; continue; }
            15 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            16 => { rdx = rcx; rcx = r10; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 17; continue; }
            17 => { rdx = rbp; rcx = rsp.wrapping_add(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rax)) as u64; rbp = ((crate::simd_ops::ru16(rax.wrapping_add(0xau64))) as u32) as u32 as u64; xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(0x10u64))) as u128; ymm0_hi = 0; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rcx = rbx.wrapping_add(0x48); xmm1 = (crate::simd_ops::ru128(P12)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm1) as u128); rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = rax; rdi = ((crate::simd_ops::ru16(rax)) as u32) as u32 as u64; r15 = 0xffff; fl_l = ((rdi as u16)) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 21; continue; } bb = 18; continue; }
            18 => { rsi = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; xmm6 = (crate::simd_ops::ru128(P12)) as u128; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((r8) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdi = rdx; rdi = ((((rdi) as i64).wrapping_shr(3u32) as u64)) as u64; rax = rdi; rax = ((rax).wrapping_shr(63u32)) as u64; rdi = ((rdi).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 19; continue; }
            19 => { rdx = rdi.wrapping_add(0x1); rcx = rsi.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P14, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 20; continue; }
            20 => { rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rcx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rsi.wrapping_add(0x10u64)))) as u64; xmm0 = 0u128; ymm0_hi = 0; let y_addr: u64 = rcx; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::lk_add_u32(rsi.wrapping_add(0x18u64), (1) as u32); rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x18u64))) as u32 as u64; rdx = ((rax.wrapping_add(rax.wrapping_mul(2))) as u64) as u64; fl_l = ((rdx).wrapping_add(rdx)) as u64; fl_r = fl_l; rdx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu128(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x30u64), (xmm6) as u128); crate::simd_ops::wu32(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0xeu64), (0xffffffffu32) as u32); rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x20u64), rax); crate::simd_ops::wu64(rcx.wrapping_add(rdx.wrapping_mul(8)).wrapping_sub(0x18u64), rdi); crate::simd_ops::wu16(r14, ((rdi as u16)) as u16); bb = 21; continue; }
            21 => { r9 = 0x2; r8 = (((rdi as u16)) as u32) as u32 as u64; rdx = (((rbp as u16)) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; r11 = rsp.wrapping_add(0xb0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd52e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0xb0; const P1: u64 = 0xc8; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x30; const P5: u64 = 0x8; const P6: u64 = 0x10; const P7: u64 = 0x28; const P8: u64 = 0x1a;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; r8 = rcx; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r9 = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; r10 = (crate::simd_ops::ru64(rdx.wrapping_add(P5))) as u64; rcx = (((rcx) & (r9))) as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rdx.wrapping_add(P2)))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P5))) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rcx)) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; fl_l = r9; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P6))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { rax = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rax; fl_r = rax; rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r10 } else { rax }) as u64; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 9; continue; }
            9 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 10; continue; }
            10 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P7))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P8))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } bb = 12; continue; }
            12 => { rax = 0x2; return rax; }
            13 => { rax = 0x18; rax = (crate::simd_ops::ru64(rax.wrapping_add(r8))) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P5))) as u32 as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd5380(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bc8970; const P1: u64 = 0xb0; const P2: u64 = 0xc8; const P3: u64 = 0x18; const P4: u64 = 0x140632ab0; const P5: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64; rdx = rsp.wrapping_add(0x20); r8 = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(0x20u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P5), (1) as u64)) as u64; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd53e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0xb0; const P4: u64 = 0xc8; const P5: u64 = 0x140632ab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = rsp.wrapping_add(0x20); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(0x20u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::lk_sub_u64(rcx.wrapping_add(P1), (1) as u64)) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd5480(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x28; const P2: u64 = 0x20; const P3: u64 = 0xb0; const P4: u64 = 0xc8; const P5: u64 = 0x140632ab0;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); rsi = rdx; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbp = rcx; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rbx = rax; rdi = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = rsi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk(({ let t = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = ((rbx).wrapping_add(0x8u64)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(P4))) as u64; rdx = rsp.wrapping_add(0x20); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; rcx = ((rcx).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(0x20u64)) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rax)) as u64; fl_l = (crate::simd_ops::lk_add_u64(rcx.wrapping_add(P1), (1) as u64)) as u64; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd5dd0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3a; const P1: u64 = 0xa0; const P2: u64 = 0xa8;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x1u8) as u8); r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_r = r9; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; r8 = rdx; rdx = 0; fl_l = 0; fl_r = 0; rcx = r9; xmm3 = (xmm2) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(rax, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } bb = 3; continue; }
            3 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } bb = 4; continue; }
            4 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } bb = 5; continue; }
            5 => { rax = 0x4; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            6 => { rax = 0x3; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            7 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            8 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd5eb0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x3a; const P1: u64 = 0xa0; const P2: u64 = 0xa8; const P3: u64 = 0x141bd5c60; const P4: u64 = 0x1e;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rcx; crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8); rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rsi = rdx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 2; continue; }
            2 => { xmm3 = 0u128; r8 = rsi; rdx = 0x2; (rax, xmm0) = crate::simd_ops::call_stk_xmm(({ let t = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; rdx = rsi; rcx = rdi; crate::simd_ops::wu64(rdi.wrapping_add(P1), rax); crate::simd_ops::wu64(rdi.wrapping_add(P2), rax); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu8(rdi.wrapping_add(P4), (0x0u8) as u8); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd6090(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bd5d00; const P1: u64 = 0x141bd68d0; const P2: u64 = 0x141bd6940;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r10 = r8; rbx = rcx; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 13; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; rdx = ((rdx).wrapping_shl(6u32)) as u64; rdx = ((rdx).wrapping_add(crate::simd_ops::ru64(r8.wrapping_add(0xa8u64)))) as u64; r9 = (crate::simd_ops::ru64(rdx.wrapping_add(0x30u64))) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(0x18u64))) as u64; r9 = (((r9) & (rax))) as u64; r9 = ((r9).wrapping_add(r9)) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)).wrapping_add(0x8u64))) as u64; fl_l = rcx; fl_r = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(r8.wrapping_add(r9.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 8; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 5; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(0x8u64))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(0x10u64))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 6; continue; }
            6 => { bb = 8; continue; }
            7 => { rcx = 0; fl_l = 0; fl_r = 0; bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 40; continue; } bb = 9; continue; }
            9 => { xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(0x88u64))) as u32) as u128; crate::simd_ops::wu8(rbx.wrapping_add(0x3bu64), (0x0u8) as u8); crate::simd_ops::wu8(rbx.wrapping_add(0x39u64), (0x0u8) as u8); xmm0 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm1 as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((xmm1 as u32)) as u32); crate::simd_ops::wu8(rbx.wrapping_add(0x50u64), (0x1u8) as u8); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 12; continue; } bb = 10; continue; }
            10 => { rdx = r10; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            12 => { crate::simd_ops::wu8(rbx.wrapping_add(0x50u64), (0x0u8) as u8); rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 11; continue; } bb = 14; continue; }
            14 => { xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x4cu64))) as u32) as u128; xmm4 = 0u128; fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 19; continue; } bb = 15; continue; }
            15 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x50u64))) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 18; continue; } bb = 16; continue; }
            16 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = 0u128; xmm0 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x4cu64), ((xmm0 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 18; continue; } bb = 17; continue; }
            17 => { rdx = r10; crate::simd_ops::wu8(rcx.wrapping_add(0x50u64), (0x1u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rax = 0x1; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            19 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x39u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 25; continue; } bb = 20; continue; }
            20 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0x44u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; xmm0 = ((crate::simd_ops::ru32(rcx.wrapping_add(0x40u64))) as u32) as u128; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 22; continue; } bb = 21; continue; }
            21 => { fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; bb = 24; continue; }
            22 => { xmm1 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) - f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm2 = 0u128; xmm0 = (xmm2 & !0xffff_ffffu128) | ((if f32::from_bits(xmm2 as u32) > f32::from_bits(xmm1 as u32) { f32::from_bits(xmm2 as u32) } else { f32::from_bits(xmm1 as u32) }).to_bits() as u128); fl_l = ((xmm4 as u32)) as u32 as u64; fl_r = ((xmm0 as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 24; continue; } bb = 23; continue; }
            23 => { crate::simd_ops::wu8(rcx.wrapping_add(0x44u64), (0x1u8) as u8); bb = 24; continue; }
            24 => { rax = (rax & !0xffu64) | (((if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 40; continue; } bb = 25; continue; }
            25 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(0xc0u64))) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x78u64))) as u64; r8 = ((r8).wrapping_shl(6u32)) as u64; r8 = ((r8).wrapping_add(crate::simd_ops::ru64(r10.wrapping_add(0xa8u64)))) as u64; rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 40; continue; } bb = 26; continue; }
            26 => { rcx = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x38u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 29; continue; } bb = 27; continue; }
            27 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = 0x3; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 18; continue; } bb = 28; continue; }
            28 => { crate::simd_ops::wu8(rbx.wrapping_add(0x38u64), (0x0u8) as u8); bb = 29; continue; }
            29 => { rcx = (rcx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; rdx = (rdx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x84u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 31; continue; } bb = 30; continue; }
            30 => { rcx = ((crate::simd_ops::ru8(rax.wrapping_add(0x5u64))) as u32) as u32 as u64; bb = 32; continue; }
            31 => { rdx = ((crate::simd_ops::ru8(rax.wrapping_add(0x4u64))) as u32) as u32 as u64; bb = 32; continue; }
            32 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(0x39u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 36; continue; } bb = 33; continue; }
            33 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 35; continue; } bb = 34; continue; }
            34 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } bb = 35; continue; }
            35 => { crate::simd_ops::wu8(rbx.wrapping_add(0x39u64), (0x1u8) as u8); bb = 36; continue; }
            36 => { fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 37; continue; }
            37 => { fl_l = ((rcx as u8)) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 18; continue; } bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(0x6u64))) as u8 as u64; fl_r = ((rcx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 40; continue; } bb = 39; continue; }
            39 => { crate::simd_ops::wu8(rbx.wrapping_add(0x3bu64), (0x1u8) as u8); rax = 0x3; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            40 => { rax = 0x4; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd6430(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1404bde90; const P1: u64 = 0x1434268d0; const P2: u64 = 0x1401e5730; const P3: u64 = 0x1406a0030; const P4: u64 = 0x1425371a0; const P5: u64 = 0x142533200; const P6: u64 = 0x1401e66f0; const P7: u64 = 0x141bd6680;
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 17>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    static EH_M: crate::dll_native::lift::eh::EhMeta = crate::dll_native::lift::eh::EhMeta { va: 0x141bd6430, funcinfo_rva: 0x373f7a4, handler: 1, shape: 1, frame_reg: 0, establisher_off: 0x88, conts: &[] };
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut eh = crate::dll_native::lift::eh::EhFrame::new(&EH_M, rsp); let _eh = eh.push();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rsp = ((rsp).wrapping_sub(0x80u64)) as u64; rbx = (r8) as u64; r14 = (rdx) as u64; rdi = (rcx) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x0u64) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd645a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P0) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rax, (rax) as u64); crate::simd_ops::wu64(rax.wrapping_add(0x8u64), (rax) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), (rax) as u64); xmm0 = 0u128; crate::simd_ops::wu128(rsp.wrapping_add(0x58u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), (0x0u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x70u64), (0x7u64) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x78u64), (0x8u64) as u64); xmm0 = ((crate::simd_ops::ru32(P1)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm0 as u32)) as u32); r8 = (rax) as u64; rdx = (0x10u32) as u32 as u64; rcx = ((rsp.wrapping_add(0x58u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd64ab); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P2) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x10u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(rcx.wrapping_mul(8)).wrapping_add(0x10u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (rcx) as u64); crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64) as u128; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((rax as u32)) as u32); r8 = ((rsp.wrapping_add(0x40u64)) as u64) as u64; rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd64e9); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P3) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rsi = (rax) as u64; r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; fl_l = (r10) as u64 as u64; fl_r = (r10) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } bb = 1; continue; }
            1 => { r10 = ((r10).wrapping_sub(1)) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rcx = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rcx) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 8; continue; } bb = 4; continue; }
            4 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd655a); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            5 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 7; continue; }
            6 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rdx = (rcx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd656f); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 8; continue; }
            8 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; crate::simd_ops::wu64(rax, (0x0u64) as u64); rdx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = (rdx) as u64 as u64; fl_r = (rdx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 9; continue; }
            9 => { bb = 10; continue; }
            10 => { rbx = (crate::simd_ops::ru64(rdx)) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd6593); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (rbx) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { r10 = ((rdi.wrapping_sub(0x1u64)) as u64) as u64; r10 = (((r10) & (0xfffffffffe000000u64))) as u64; rax = ((crate::exe::teb_self() as u64)) as u64; fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(r10.wrapping_add(0x100u64))) as u64 as u64; rdx = (rdx & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64)); r9 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = (r9) as u64; rax = ((rax).wrapping_sub(r10)) as u64; rax = ((rax).wrapping_shr(16u32)) as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rcx = ((r10.wrapping_add(0x108u64)) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as u32 as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; fl_l = ((rdx as u8)) as u8 as u64; fl_r = ((rdx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 16; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(0xeu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 15; continue; } bb = 13; continue; }
            13 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; crate::simd_ops::wu64(r9, (rax) as u64); crate::simd_ops::wu64(rcx.wrapping_add(0x20u64), (r9) as u64); fl_l = (crate::simd_ops::lk_add_u32(rcx.wrapping_add(0x18u64), (0xffffffffu32) as u32)) as u32 as u64; fl_r = fl_l; if (((fl_l as u32)) != 0) { bb = 18; continue; } bb = 14; continue; }
            14 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd6600); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P4) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            15 => { r8 = (r8 & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 17; continue; }
            16 => { r8 = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { rdx = (rcx) as u64; rcx = (r10) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd6615); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P5) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 18; continue; }
            18 => { rbx = (crate::simd_ops::ru64(r14)) as u64; rdi = (crate::simd_ops::ru32(r14.wrapping_add(0x8u64))) as u32 as u64; rdi = ((rdi).wrapping_shl(4u32)) as u64; rdi = ((rdi).wrapping_add(rbx)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 21; continue; } bb = 19; continue; }
            19 => { bb = 20; continue; }
            20 => { xmm0 = (crate::simd_ops::ru128(rbx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm0) as u128); rdx = ((rsp.wrapping_add(0x30u64)) as u64) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); eh.site(0x141bd6642); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rbx = ((rbx).wrapping_add(0x10u64)) as u64; fl_l = (rbx) as u64 as u64; fl_r = (rdi) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 20; continue; } bb = 21; continue; }
            21 => { r11 = ((rsp.wrapping_add(0x80u64)) as u64) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd6680(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14069fd60; const P1: u64 = 0x143242eb0; const P2: u64 = 0x1401fa880; const P3: u64 = 0x143242ec8; const P4: u64 = 0x143242e90;
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rax = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; rbx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x3u64) as u64); rsi = rdx; crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rbx); r10 = rcx; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rbx); r15 = (0xcbf29ce484222325u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rbx); r14 = (0x100000001b3u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbx); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = ((rbx as u32)) as u32 as u64; bb = 5; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rdx)) as u64; rcx = r15; r8 = ((rdx.wrapping_add(rax)) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { bb = 4; continue; }
            4 => { rax = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rax = (((rax) | (0x20u64))) as u64; rcx = (((rcx) ^ (rax))) as u64; rcx = (((rcx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0x80u64), rcx); r9 = rsp.wrapping_add(0x30); rcx = r10; r8 = rsp.wrapping_add(0x80); rdx = rsp.wrapping_add(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 26; continue; } bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x11u64) as u64); rax = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbp); rcx = rsp.wrapping_add(0x20); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rdx = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); crate::simd_ops::wu64(rsp.wrapping_add(0x90u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (0xffffffffffffffffu64) as u64; rdx = rax; rcx = rdi; bb = 7; continue; }
            7 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 8; continue; }
            8 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 9; continue; }
            9 => { rax = rbx; bb = 12; continue; }
            10 => { r8 = ((rcx.wrapping_add(rdx)) as u64) as u64; rax = r15; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rcx = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (((rcx) | (0x20u64))) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x15u64) as u64); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rdx = rsi; rax = P3; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rax; rcx = rdi; bb = 13; continue; }
            13 => { rcx = ((rcx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rcx))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 13; continue; } bb = 14; continue; }
            14 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 16; continue; } bb = 15; continue; }
            15 => { rax = rbx; bb = 19; continue; }
            16 => { r8 = ((rcx.wrapping_add(rdx)) as u64) as u64; rax = r15; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 19; continue; } bb = 17; continue; }
            17 => { bb = 18; continue; }
            18 => { rcx = (((crate::simd_ops::ru8(rdx)) as i8 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = (((rcx) | (0x20u64))) as u64; rax = (((rax) ^ (rcx))) as u64; rax = (((rax) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = rdx; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 18; continue; } bb = 19; continue; }
            19 => { crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x1eu64) as u64); rcx = rsp.wrapping_add(0x20); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), rax); rdx = rsi; rax = P4; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; bb = 20; continue; }
            20 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(rdi))) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 20; continue; } bb = 21; continue; }
            21 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } bb = 22; continue; }
            22 => { rdx = ((rdi.wrapping_add(rax)) as u64) as u64; rbx = r15; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 25; continue; } bb = 23; continue; }
            23 => { bb = 24; continue; }
            24 => { rax = (((crate::simd_ops::ru8(rcx)) as i8 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(1)) as u64; rax = (((rax) | (0x20u64))) as u64; rbx = (((rbx) ^ (rax))) as u64; rbx = (((rbx) as i64).wrapping_mul((r14) as i64) as u64) as u64; fl_l = rcx; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 24; continue; } bb = 25; continue; }
            25 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x38u64), rbx); rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; bb = 26; continue; }
            26 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd68d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x18; const P2: u64 = 0x8; const P3: u64 = 0x10;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = (((r9) & (rdx))) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P1)))) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r9)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 2; continue; }
            2 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 4; continue; }
            4 => { fl_l = rax; fl_r = rax; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            5 => { rax = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rax; fl_r = rax; rax = (rax & !0xffu64) | (((if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd6920(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x141bd6940;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru32(rax)) as u32 as u64;
    crate::simd_ops::wu32(rax, (0x3u32) as u32);
    rax = ((rcx as u32)) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd6940(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x30; const P1: u64 = 0x8; const P2: u64 = 0x18; const P3: u64 = 0x10; const P4: u64 = 0x143873bf0; const P5: u64 = 0x1431f06a0; const P6: u64 = 0x142645890;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; r9 = (((r9) & (rdx))) as u64; r9 = ((r9).wrapping_shl(4u32)) as u64; r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P2)))) as u64; rax = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; r8 = rax; fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(r9)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 7; continue; } bb = 2; continue; }
            2 => { bb = 3; continue; }
            3 => { fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } bb = 4; continue; }
            4 => { r8 = (crate::simd_ops::ru64(r8.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 5; continue; }
            5 => { bb = 7; continue; }
            6 => { r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 9; continue; } bb = 8; continue; }
            8 => { rax = P4; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            9 => { fl_l = rax; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 10; continue; }
            10 => { rcx = (crate::simd_ops::ru64(r9)) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 13; continue; } bb = 11; continue; }
            11 => { fl_l = rax; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 13; continue; }
            13 => { rax = ((rax).wrapping_add(0x18u64)) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            14 => { rcx = P5; rax = crate::simd_ops::call_stk(crate::simd_ops::ind_tgt(crate::simd_ops::ru64(P6), rax), rax, rcx, rdx, r8, r9, rsp); crate::lift_ops::trap(0x80000003, 0x141bd69c8, rcx, rsp); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd69d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x1c; const P2: u64 = 0x28; const P3: u64 = 0x140241da0; const P4: u64 = 0x1d; const P5: u64 = 0x30; const P6: u64 = 0x1e; const P7: u64 = 0x38;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = r8; rbp = rdx; rbx = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { bb = 2; continue; }
            2 => { rcx = (crate::simd_ops::ru64(rsi)) as u64; r9 = ((rbx.wrapping_add(P1)) as u64) as u64; r8 = ((rbx.wrapping_add(P2)) as u64) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; r9 = ((rbx.wrapping_add(P4)) as u64) as u64; r8 = ((rbx.wrapping_add(P5)) as u64) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsi)) as u64; r9 = ((rbx.wrapping_add(P6)) as u64) as u64; r8 = ((rbx.wrapping_add(P7)) as u64) as u64; rdx = rbp; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd8670(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x20; const P1: u64 = 0x40; const P2: u64 = 0x50;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut ymm0_hi: u128 = 0; let mut ymm1_hi: u128 = 0;
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let y_addr: u64 = rdx; xmm0 = crate::simd_ops::ru128(y_addr); ymm0_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8; crate::simd_ops::wu128(y_addr, xmm0); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm0_hi);
    let y_addr: u64 = rdx.wrapping_add(P0); xmm1 = crate::simd_ops::ru128(y_addr); ymm1_hi = crate::simd_ops::ru128(y_addr.wrapping_add(16));
    let y_addr: u64 = r8.wrapping_add(P0); crate::simd_ops::wu128(y_addr, xmm1); crate::simd_ops::wu128(y_addr.wrapping_add(16), ymm1_hi);
    xmm0 = (crate::simd_ops::ru128(rdx.wrapping_add(P1))) as u128; ymm0_hi = 0;
    crate::simd_ops::wu128(r8.wrapping_add(P1), (xmm0) as u128);
    xmm1 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128; ymm1_hi = 0;
    crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm1 as u64)) as u64);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd86d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0xc; const P2: u64 = 0x10; const P3: u64 = 0x14; const P4: u64 = 0x18; const P5: u64 = 0x1c; const P6: u64 = 0x20; const P7: u64 = 0x24; const P8: u64 = 0x28; const P9: u64 = 0x2c; const P10: u64 = 0x30; const P11: u64 = 0x34;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rdx, (0x0u64) as u64);
    rax = rdx;
    crate::simd_ops::wu16(rdx.wrapping_add(P0), (0x101u16) as u16);
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0x3e800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x3f000000u32) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P3), (0x1u8) as u8);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P5), (0x3f000000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), (0x40400000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P7), (0x3dcccccdu32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), (0x42b40000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P9), (0x14u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P10), (0x3f4ccccdu32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P11), (0x3e19999au32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x38u64), (0x3d4ccccdu32) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(0x3cu64), (0x1u16) as u16);
    crate::simd_ops::wu32(rdx.wrapping_add(0x40u64), (0x43b40000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x44u64), (0x41000000u32) as u32);
    crate::simd_ops::wu16(rdx.wrapping_add(0x48u64), (0x0u16) as u16);
    crate::simd_ops::wu32(rdx.wrapping_add(0x4cu64), (0x3f800000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(0x50u64), (0x3f666666u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd8860(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143245c00; const P1: u64 = 0x8; const P2: u64 = 0x14384e158; const P3: u64 = 0x1432448d8; const P4: u64 = 0x143245bf0; const P5: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rax = rdx;
    r9 = P0;
    rdx = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    rcx = P3;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rcx = P4;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd8a20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x14384e158; const P1: u64 = 0x143245c00; const P2: u64 = 0x1432448d8; const P3: u64 = 0x143245bf0; const P4: u64 = 0x1401de410;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x48u64)) as u64;
    rcx = P0;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rcx);
    r9 = P1;
    rcx = P2;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), (0x0u64) as u64);
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rcx);
    rdx = ((rdx).wrapping_add(0x8u64)) as u64;
    rcx = P3;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    rcx = rax;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x48u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd9530(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426dd4; const P1: u64 = 0x14321a930; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x10; const P5: u64 = 0x14; const P6: u64 = 0x1c;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    rax = P1;
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0xc1200000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P3), (0xc1200000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), (0xc1200000u32) as u32);
    xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((xmm3 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd9c20(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x18; const P2: u64 = 0x10; const P3: u64 = 0x1406d6540; const P4: u64 = 0x28; const P5: u64 = 0x1406d6620;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu64(r8.wrapping_add(P0), rax);
    rbx = rdx;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P2))) as u32) as u32 as u64;
    rdi = r8;
    rdx = ((rdx).wrapping_add(0x18u64)) as u64;
    crate::simd_ops::wu8(r8.wrapping_add(P2), ((rax as u8)) as u8);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P4)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P5, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bd9ca0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x10; const P1: u64 = 0x143248ed0; const P2: u64 = 0x8; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x30;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    crate::simd_ops::wu8(rdx.wrapping_add(P0), (0x1u8) as u8);
    rax = P1;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bda320(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143426dd4; const P1: u64 = 0x4; const P2: u64 = 0x8; const P3: u64 = 0xc; const P4: u64 = 0x14; const P5: u64 = 0x18; const P6: u64 = 0x20; const P7: u64 = 0x28;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm3 = ((crate::simd_ops::ru32(P0)) as u32) as u128;
    crate::simd_ops::wu32(rdx, (0xc1200000u32) as u32);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rdx.wrapping_add(P1), (0xc1200000u32) as u32);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0xc1200000u32) as u32);
    xmm0 = crate::simd_ops::vunpcklps(xmm3, xmm3);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), ((xmm0 as u64)) as u64);
    crate::simd_ops::wu32(rdx.wrapping_add(P4), ((xmm3 as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = rdx;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bda450(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x1432440a8; const P1: u64 = 0x143244128; const P2: u64 = 0x1406db220;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = P0;
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P1;
    rcx = rax;
    r8 = 0x5;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bda4b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x143244390; const P1: u64 = 0x1432443b0; const P2: u64 = 0x14020f270;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    rax = r8;
    crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), (0x0u64) as u64);
    rcx = P0;
    rdx = ((rdx).wrapping_add(0x40u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx);
    r9 = P1;
    rcx = rax;
    r8 = 0x3;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bda8f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1404b9990; const P3: u64 = 0x30; const P4: u64 = 0x1403d79a0; const P5: u64 = 0x40; const P6: u64 = 0x44; const P7: u64 = 0x48; const P8: u64 = 0x4c;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    rbx = rdx;
    crate::simd_ops::wu8(r8.wrapping_add(P0), ((rax as u8)) as u8);
    rdx = ((rdx).wrapping_add(0x10u64)) as u64;
    rdi = r8;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P5))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P5), ((rax as u32)) as u32);
    rax = ((crate::simd_ops::ru8(rbx.wrapping_add(P6))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rdi.wrapping_add(P6), ((rax as u8)) as u8);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P7))) as u32 as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rax as u32)) as u32);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P8))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P8), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bda980(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14315bfe8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x30; const P6: u64 = 0x38; const P7: u64 = 0x48;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P2), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(P4), ((rax as u8)) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bdaf50(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x14; const P3: u64 = 0x1c; const P4: u64 = 0x20; const P5: u64 = 0x1403d79a0; const P6: u64 = 0x30; const P7: u64 = 0x38; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x1406f27c0; const P11: u64 = 0x60; const P12: u64 = 0x1404b9990;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r8; rdi = rdx; xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P0), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P1), ((rax as u32)) as u32); xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64) as u128; crate::simd_ops::wu64(r8.wrapping_add(P2), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; crate::simd_ops::wu32(r8.wrapping_add(P3), ((rax as u32)) as u32); rdx = ((rdx).wrapping_add(0x20u64)) as u64; rcx = ((r8.wrapping_add(P4)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P6))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P6), ((rax as u32)) as u32); rdx = ((rdi.wrapping_add(P7)) as u64) as u64; rcx = ((rbx.wrapping_add(P7)) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru32(rdi.wrapping_add(P8))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(P8), ((rax as u32)) as u32); rax = ((rdi.wrapping_add(P9)) as u64) as u64; rcx = ((rbx.wrapping_add(P9)) as u64) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 2; continue; }
            2 => { rax = ((rdi.wrapping_add(P11)) as u64) as u64; rcx = rbx.wrapping_add(0x60); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rax)) as u64; r8 = (crate::simd_ops::ru32(rax.wrapping_add(0x8u64))) as u32 as u64; r8 = ((r8).wrapping_shl(5u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 4; continue; }
            4 => { rax = ((crate::simd_ops::ru8(rdi.wrapping_add(0x70u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x70u64), ((rax as u8)) as u8); rdx = rdi.wrapping_add(0x78); rcx = rbx.wrapping_add(0x78); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bdb040(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14320ee50; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x38; const P7: u64 = 0x40; const P8: u64 = 0x48; const P9: u64 = 0x50; const P10: u64 = 0x58; const P11: u64 = 0x60;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P5), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P6), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P7), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(P8), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x68u64), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(0x78u64), rax);
    crate::simd_ops::wu32(rdx.wrapping_add(0x80u64), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rdx.wrapping_add(0x88u64), rax);
    crate::simd_ops::wu8(rdx.wrapping_add(0x90u64), ((rax as u8)) as u8);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bdb290(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0xc;
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    r9 = ((rcx.wrapping_add(P1)) as u64) as u64;
    r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r10 = rdx;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r8.wrapping_add(rcx).wrapping_add(P0), ((rax as u8)) as u8);
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(rdx))) as u64;
    r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P1))) as u32) as u32 as u64;
    crate::simd_ops::wu8(r9.wrapping_add(r8), ((rax as u8)) as u8);
    rax = ((rcx.wrapping_add(P2)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r10.wrapping_add(P0))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r10)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rax)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bdb5b0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x9; const P2: u64 = 0xc;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64;
    r9 = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu8(rcx.wrapping_add(rdx).wrapping_add(P0), ((rax as u8)) as u8);
    rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u32) as u32 as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    r9 = ((r9).wrapping_sub(crate::simd_ops::ru64(r8))) as u64;
    crate::simd_ops::wu8(r9.wrapping_add(rcx), ((rax as u8)) as u8);
    rax = ((rdx.wrapping_add(P2)) as u64) as u64;
    rcx = (crate::simd_ops::ru64(r8.wrapping_add(P0))) as u64;
    rdx = rax;
    fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64;
    rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    rax = (crate::simd_ops::ru32(rax)) as u32 as u64;
    crate::simd_ops::wu32(rdx.wrapping_add(rcx), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bdb7f0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x143248d78; const P1: u64 = 0x8; const P2: u64 = 0xc;
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu16(rdx.wrapping_add(P1), (0x101u16) as u16);
    crate::simd_ops::wu64(rdx, rax);
    rax = rdx;
    crate::simd_ops::wu32(rdx.wrapping_add(P2), (0x1u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bdd2d0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x1406f2520; const P3: u64 = 0x20; const P4: u64 = 0x14240aa00; const P5: u64 = 0x68; const P6: u64 = 0x140636a20;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = r8;
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    fl_l = rcx; fl_r = rax;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rdx = (crate::simd_ops::ru64(rax)) as u64; rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; r8 = ((rdx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rbx.wrapping_add(P3)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P3)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P5)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P6, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bdd970(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x10; const P2: u64 = 0x18; const P3: u64 = 0x20; const P4: u64 = 0x28; const P5: u64 = 0x14240aa00; const P6: u64 = 0xb0; const P7: u64 = 0xb8; const P8: u64 = 0x1406f2210;
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; rbx = rdx; rax = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P0), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P1), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P2), rax); rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P3), rax); rdx = ((rdx).wrapping_add(0x28u64)) as u64; rcx = ((r8.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; crate::simd_ops::wu64(rdi.wrapping_add(P6), rax); rbx = ((rbx).wrapping_add(0xb8u64)) as u64; rcx = ((rdi.wrapping_add(P7)) as u64) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru32(rbx.wrapping_add(P0))) as u32 as u64; r8 = ((rax.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rdx)) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bdda30(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    const P0: u64 = 0x14321aa88; const P1: u64 = 0x8; const P2: u64 = 0x10; const P3: u64 = 0x18; const P4: u64 = 0x20; const P5: u64 = 0x28; const P6: u64 = 0x2c; const P7: u64 = 0x30; const P8: u64 = 0x2b; const P9: u64 = 0xb0; const P10: u64 = 0xb8; const P11: u64 = 0xc0;
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rdx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P4), rax);
    crate::simd_ops::lk_and_u32(rdx.wrapping_add(P5), (0xff000010u32) as u32);
    fl_l = (crate::simd_ops::lk_or_u32(rdx.wrapping_add(P5), (0x10u32) as u32)) as u32 as u64; fl_r = fl_l;
    crate::simd_ops::wu32(rdx.wrapping_add(P6), ((rax as u32)) as u32);
    crate::simd_ops::wu8(rdx.wrapping_add(P7), ((rax as u8)) as u8);
    crate::simd_ops::wu8(rdx.wrapping_add(P8), (0x10u8) as u8);
    crate::simd_ops::wu64(rdx.wrapping_add(P9), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P10), rax);
    crate::simd_ops::wu64(rdx.wrapping_add(P11), rax);
    rax = rdx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_141bde6e0(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x8; const P1: u64 = 0x20; const P2: u64 = 0xc; const P3: u64 = 0x10; const P4: u64 = 0x18; const P5: u64 = 0x1403d79a0; const P6: u64 = 0x30;
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64;
    rcx = ((r8.wrapping_add(P1)) as u64) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P0), ((rax as u32)) as u32);
    rbx = rdx;
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P2))) as u32 as u64;
    rdi = r8;
    crate::simd_ops::wu32(r8.wrapping_add(P2), ((rax as u32)) as u32);
    xmm0 = ((crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64) as u128;
    crate::simd_ops::wu64(r8.wrapping_add(P3), ((xmm0 as u64)) as u64);
    rax = (crate::simd_ops::ru32(rdx.wrapping_add(P4))) as u32 as u64;
    rdx = ((rdx).wrapping_add(0x20u64)) as u64;
    crate::simd_ops::wu32(r8.wrapping_add(P4), ((rax as u32)) as u32);
    (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    crate::simd_ops::wu32(rdi.wrapping_add(P6), ((rax as u32)) as u32);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}
