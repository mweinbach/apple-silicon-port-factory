#![allow(unused_imports, unused_variables, unused_mut, unused_assignments, non_snake_case, dead_code, clippy::all, unreachable_code, unused_parens)]

#[inline(never)]
pub unsafe extern "win64" fn tpl_27515<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 15; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } rbx = rdx.wrapping_add(0x2); bb = 3; continue; }
            2 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 7; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 9; continue; } fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x28); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P4; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 18; continue; } fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; bb = 13; continue; }
            5 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 6; continue; }
            6 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = r14.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = rbx; rdx = rax; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 18; continue; } fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 6; continue; }
            7 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 9; continue; } fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(r14.wrapping_add(0x30u64), rax); bb = 8; continue; }
            8 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r8 = rbx; rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            9 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 17; continue; } fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 17; continue; } rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 11; continue; }
            10 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P11, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 11; continue; }
            11 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P12)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 12; continue; }
            12 => { fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); bb = 13; continue; }
            13 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 18; continue; } bb = 14; continue; }
            14 => { rcx = rsi; bb = 15; continue; }
            15 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P13)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 16; continue; }
            16 => { r11 = rsp.wrapping_add(0x50); rax = rbx; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            17 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 16; continue; }
            18 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 16; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27516<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rsi = r8; rbx = rdx; rdi = rcx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x30u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rcx = r8; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rsi; r8 = rbx; rcx = 0x1; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 2; continue; }
            2 => { r14 = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; rbp = 0; fl_l = 0; fl_r = 0; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rdx = ((rbp as u32)) as u32 as u64; rcx = rdi.wrapping_add(0x10); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x2; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbp = (((rbp as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((r14 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } bb = 5; continue; }
            5 => { rbp = rdi.wrapping_add(0x28); rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 7; continue; } rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rax)) as u64; bb = 6; continue; }
            6 => { r9 = P7; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x3; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 8; continue; } rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P10)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 8; continue; }
            8 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27517<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x38u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27518<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0x1;
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
pub unsafe extern "win64" fn tpl_27519<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rcx = ((rbx.wrapping_add(P1)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdx = 0x40; rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P4), (0x0u64) as u64); rcx = ((rbx.wrapping_add(P7)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27520<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0x2;
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
pub unsafe extern "win64" fn tpl_27525<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x3cu64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27526<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0x3;
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
pub unsafe extern "win64" fn tpl_27531<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x44u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27532<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0x4;
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
pub unsafe extern "win64" fn tpl_27533<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x48; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27538<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0xa8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27539<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rdx;
    rcx = rsp.wrapping_add(0x20);
    rdx = 0x5;
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
pub unsafe extern "win64" fn tpl_27540<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp);
    crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rdi = rcx;
    if ((((fl_l as u32)) as i32) <= (((fl_r as u32)) as i32)) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    rbx = 0;
    fl_l = 0; fl_r = 0;
    rbp = (((crate::simd_ops::ru32(rcx.wrapping_add(P0))) as i32 as i64) as u64) as u64;
    loop { rcx = (crate::simd_ops::ru64(rsi.wrapping_add(rbx.wrapping_mul(8)).wrapping_add(P0))) as u64; rbx = ((rbx).wrapping_add(1)) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbp; if !(((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64))) { break; } }
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(P0), (0x0u32) as u32);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64;
    rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27541<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } rcx = ((rbx.wrapping_add(P0)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk_caught(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu64(rbx, rax); bb = 2; continue; }
            1 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { bb = 4; continue; }
            3 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r9 = 0; fl_l = 0; fl_r = 0; rdx = ((r9.wrapping_add(P0)) as u32) as u32 as u64; r8 = ((r9.wrapping_add(P4)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk_caught(P2, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27542<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } rcx = 0xb0; rax = crate::simd_ops::call_stk_caught(P0, rax, rcx, rdx, r8, r9, rsp); rbx = rax; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rdx = 0; fl_l = 0; fl_r = 0; rcx = rax; rax = crate::simd_ops::call_stk_caught(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; crate::simd_ops::wu64(rbx, rax); bb = 2; continue; }
            1 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { bb = 4; continue; }
            3 => { crate::simd_ops::lk_and_u64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); r9 = 0; fl_l = 0; fl_r = 0; rdx = 0xb0; r8 = ((r9.wrapping_add(P3)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk_caught((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rax; rax = crate::simd_ops::call_stk_caught(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rax = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27544<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r14 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 3; continue; } rdi = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x28; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27545<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r14 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 3; continue; } rdi = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x40; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27546<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r14 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 3; continue; } rdi = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0xb0; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27549<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    rbx = rdx;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; } else { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    rdx = rbx;
    rcx = rax;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27558<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rsi = rdx;
    rdi = r9;
    rdx = 0x2;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rsi.wrapping_add(P1)) as u64) as u64;
    rbx = rax;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rax as u32)) as u32 as u64;
    rdx = rbx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp);
    r8 = rdi;
    rcx = rsi;
    rdx = rax;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P4, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27559<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) == ((fl_r as u32)))) { r8 = P1; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27560<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (((crate::simd_ops::ru32(rsp.wrapping_add(0x70u64))) as i32 as i64) as u64) as u64; rdi = 0; fl_l = 0; fl_r = 0; rsi = ((((r9 as u32)) as i32 as i64) as u64) as u64; r15 = r8; rbp = rdx; r13 = rcx; fl_l = rbx; fl_r = rbx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 2; continue; } r12 = rdx; r14 = r8; fl_l = r12; fl_r = r8; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 1; continue; }
            1 => { fl_l = rdi; fl_r = rsi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 2; continue; } rdx = (crate::simd_ops::ru64(r12.wrapping_add(r14))) as u64; rcx = (crate::simd_ops::ru64(r14)) as u64; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdi = ((rdi).wrapping_add(1)) as u64; r14 = ((r14).wrapping_add(0x8u64)) as u64; fl_l = rdi; fl_r = rbx; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 1; continue; } bb = 2; continue; }
            2 => { r12 = (crate::simd_ops::ru64(r13)) as u64; fl_l = rbx; fl_r = rsi; if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { bb = 4; continue; } r14 = ((rbx.wrapping_mul(8)) as u64) as u64; r15 = ((r15).wrapping_sub(rbp)) as u64; r14 = ((r14).wrapping_add(rbp)) as u64; fl_l = rsi; fl_r = rbx; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 3; continue; }
            3 => { rdi = (crate::simd_ops::ru64(r15.wrapping_add(r14))) as u64; rcx = r12; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = rdi; rbx = rax; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14, rbx); r14 = ((r14.wrapping_add(P2)) as u64) as u64; fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27561<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = r8; rsi = rdx; r8 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rdi = rcx; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 1; continue; } fl_l = ((rbx).wrapping_add(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            1 => { rdx = rbx; rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P2))) as u32 as u64; fl_r = 0x1; crate::simd_ops::wu32(rdi.wrapping_add(P2), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); rbp = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)).wrapping_sub((fl_r as u32))) as i32) < 0) { bb = 3; continue; } r8 = rdi; rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } crate::simd_ops::lk_add_u32(rdi.wrapping_add(P2), (1) as u32); rdx = ((rbp as u32)) as u32 as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = 0; fl_r = ((rax as u8)) as u8 as u64; rax = (rax & !0xffu64) | (((0u8.wrapping_sub(fl_r as u8)) as u8 as u64)); rax = (if (((fl_l as u8)) < ((fl_r as u8))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rax) & (rbx))) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 4; continue; }
            3 => { rax = 0; fl_l = 0; fl_r = 0; bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27567<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rdx = rcx;
    rsi = 0x8;
    r8 = ((rsi as u32)) as u32 as u64;
    rcx = rsp.wrapping_add(0x30);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = ((rsi as u32)) as u32 as u64;
    rdx = rbx;
    rcx = rdi;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = ((rsi as u32)) as u32 as u64;
    rdx = rsp.wrapping_add(0x30);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P1)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27569<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P2), rbx);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(P3), (0x0u32) as u32);
    crate::simd_ops::lk_and_u32(rdi.wrapping_add(P4), (0x0u32) as u32);
    crate::simd_ops::lk_and_u64(rdi.wrapping_add(P5), (0x0u64) as u64);
    rcx = ((rdi.wrapping_add(P6)) as u64) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rdi.wrapping_add(P9)) as u64) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P10)) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u64(rdi.wrapping_add(P11), (0x0u64) as u64);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27576<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rax);
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27577<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27578<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = P4; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = 0x30; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdx = ((rbx.wrapping_add(P8)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 3; continue; }
            2 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P12)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27579<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = P4; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rdx = 0x40; rcx = rdi; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdx = ((rbx.wrapping_add(P8)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 3; continue; }
            2 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdx; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = ((rbx.wrapping_add(P10)) as u64) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P12)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27583<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = rax; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 2; continue; }
            1 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27584<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rdx = ((rcx.wrapping_add(P1)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 1; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 2; continue; }
            1 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 3; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 2; continue; }
            2 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 3; continue; }
            3 => { rcx = rdx; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rcx = ((rbx.wrapping_add(P3)) as u64) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27585<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x28; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27587<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rsi = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdx = ((rdi.wrapping_add(P2)) as u64) as u64; rcx = ((rsi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 2; continue; } rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsi; rbx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = ((rsi.wrapping_add(P4)) as u64) as u64; r8 = rbx; r9 = rax; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rax = P8; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 4; continue; } fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(P9))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P9), rax); bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P9))) as u64; fl_r = (0x0u64) as u64; rdx = P11; rcx = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rdi.wrapping_add(0x30u64)) } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P12, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27592<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rcx;
    rbx = rdx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27595<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((((rdx as u32)) as i32 as i64) as u64) as u64;
    rdi = rcx;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = P0;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    rax = (crate::simd_ops::ru64(P2)) as u64;
    rdx = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rdx = ((rdx).wrapping_add(rdx)) as u64;
    xmm0 = (crate::simd_ops::ru128(rax.wrapping_add(rdx.wrapping_mul(8)))) as u128;
    rax = rdi;
    crate::simd_ops::wu128(rdi, (xmm0) as u128);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27596<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rdi = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rdx = 0x28; rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rcx = ((rbx.wrapping_add(P4)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27597<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = 0; fl_l = 0; fl_r = 0; rax = P0; rbx = rcx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = rdi; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax.wrapping_add(P4)) as u64) as u64; bb = 1; continue; }
            1 => { rcx = ((rbx.wrapping_add(P5)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 2; continue; } r8 = ((rbx.wrapping_add(P6)) as u64) as u64; rdx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            2 => { rdx = ((rdi as u32)) as u32 as u64; rcx = rbx; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rax = rdi; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27598<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 9; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } rbx = rdx.wrapping_add(0x2); bb = 3; continue; }
            2 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x10); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rcx = r14; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(r14.wrapping_add(0x10u64), rax); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x10u64))) as u64; r8 = rbx; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            5 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 11; continue; } fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 11; continue; } rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 7; continue; }
            6 => { rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 7; continue; }
            7 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 12; continue; } rcx = rsi; bb = 9; continue; }
            9 => { rdx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 10; continue; }
            10 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            11 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 10; continue; }
            12 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 10; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27599<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = P0; rsi = r8; rbx = rdx; rdi = rcx; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rcx = r8; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 1; continue; }
            1 => { rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = rax; r9 = rsi; r8 = rbx; rcx = 0x1; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P5))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P5))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 3; continue; }
            3 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27600<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27603<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 11; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } rbx = rdx.wrapping_add(0x2); bb = 3; continue; }
            2 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x30); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } rcx = r14; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = r14.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; r8 = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = rsi; rcx = rax; rbx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); rax = rbx; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); rdx = P4; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = rbp.wrapping_sub(0x10); crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), rax); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 14; continue; } fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_r = (0x0u64) as u64; bb = 10; continue; }
            5 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } rdx = rbp.wrapping_add(0x38); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), rcx); bb = 9; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 13; continue; } fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 8; continue; }
            7 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 8; continue; }
            8 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 9; continue; }
            9 => { fl_l = rax; fl_r = rax; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); bb = 10; continue; }
            10 => { if (((fl_l as u64)) == ((fl_r as u64))) { bb = 14; continue; } rcx = rsi; bb = 11; continue; }
            11 => { rdx = rbp.wrapping_add(0x28); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(P9)) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 12; continue; }
            12 => { r11 = rsp.wrapping_add(0x50); rax = rbx; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = r11; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            13 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 12; continue; }
            14 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 12; continue; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27604<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rax.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(0x18u64))) as u64; fl_r = (0x0u64) as u64; rsi = r8; rbx = rdx; rdi = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } fl_l = rdx; fl_r = (crate::simd_ops::ru64(r8)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rcx = r8; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rdi.wrapping_add(0x18u64))) as u64; r8 = rbx; rcx = 0x1; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 2; continue; }
            2 => { rbp = rdi.wrapping_add(0x10); rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x10u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 4; continue; } rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; r14 = (crate::simd_ops::ru32(rax.wrapping_add(0x10u64))) as u32 as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rax)) as u64; bb = 3; continue; }
            3 => { r9 = P3; r8 = 0x1; rdx = ((r14 as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = rbx; rdx = 0x2; r8 = rax; rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x8u64))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 5; continue; }
            5 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27605<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x20u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27606<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rdx = rax;
    rcx = ((rdi.wrapping_add(P5)) as u64) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u64(rdi.wrapping_add(P7), (0x0u64) as u64);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27607<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    rcx = ((rbx.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rbx, rax);
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::lk_and_u64(rbx.wrapping_add(P4), (0x0u64) as u64);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27608<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rbx = rcx; rax = P0; crate::simd_ops::wu64(rcx, rax); rax = P1; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 1; continue; } rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rdi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rdx = 0x28; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdx = ((rbx.wrapping_add(P5)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rdx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rdx)) as u64; fl_l = ((rcx as u8)) as u8 as u64; fl_r = (0x3u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; bb = 3; continue; }
            2 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(1u32)) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 4; continue; } fl_l = (((rcx) & (0xfffffffffffffffcu64))) as u64; fl_r = fl_l; rcx = ((fl_l as u64)) as u64; rcx = (crate::simd_ops::ru64(rcx)) as u64; bb = 3; continue; }
            3 => { fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { rcx = rdx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rcx = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27610<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; rdi = rcx; rbx = rdx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rax = P2; fl_l = rbx; fl_r = rax; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rdi.wrapping_add(P3), rax); bb = 2; continue; }
            2 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_r = (0x0u64) as u64; rdx = P6; rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; rdx = (if (((fl_l as u64)) != ((fl_r as u64))) { crate::simd_ops::ru64(rbx.wrapping_add(P3)) } else { rdx }) as u64; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27611<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = rdx;
    rdx = rcx;
    rcx = rax;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27636<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x10u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
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
pub unsafe extern "win64" fn tpl_27637<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbp = rcx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rdx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = 0; fl_r = rax; rax = (0u64.wrapping_sub(fl_r as u64)) as u64; rax = ((((rsi as u32)) as i32 as i64) as u64) as u64; rbx = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffffffffffu64 } else { 0u64 }) as u64; fl_l = (((rbx) & (rdx))) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; rdi = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = ((rsi).wrapping_add(rax)) as u64; fl_l = ((rbx).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rbx = ((fl_l as u64)) as u64; bb = 2; continue; }
            2 => { fl_l = rbx; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = ((rax).wrapping_add(1)) as u64; fl_l = ((rsi).wrapping_add(rax)) as u64; fl_r = fl_l; rsi = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rcx = rbp.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } r8 = rbp.wrapping_add(0x2c); rdx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            4 => { rdx = ((rsi as u32)) as u32 as u64; rcx = rbp; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); rax = rsi; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27639<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbp = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r14 = 0; fl_l = 0; fl_r = 0; rsi = r8; rbx = rdx; rdi = rcx; fl_l = ((rbp as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 2; continue; } rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 2; continue; }
            2 => { rdx = ((r14 as u32)) as u32 as u64; rcx = ((rdi.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rbx; rdx = rax; rcx = 0x1; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); r14 = (((r14 as u32)).wrapping_add(1)) as u32 as u64; rbx = rax; fl_l = ((r14 as u32)) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; fl_r = 0x0; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } fl_l = rbx; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } rdx = rbx; rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P5))) as u32 as u64; r8 = rbx; rcx = 0x2; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 5; continue; }
            5 => { fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P7))) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 6; continue; } rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P7))) as u64; r8 = rsi; rcx = (((rcx) & (0xfffffffffffffffcu64))) as u64; rdx = rbx; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 6; continue; }
            6 => { rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27640<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x2cu64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27641<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rdi, rax);
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    crate::simd_ops::wu64(rdi.wrapping_add(P3), rbx);
    rbx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rdi.wrapping_add(P4), rbx);
    crate::simd_ops::wu64(rdi.wrapping_add(P5), rbx);
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp);
    crate::simd_ops::wu32(rdi.wrapping_add(P7), ((rbx as u32)) as u32);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27643<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rdi = rcx;
    rbx = rdx;
    rcx = ((rcx).wrapping_add(0x8u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; rdx = (((rdx) & (0xfffffffffffffffcu64))) as u64; rdx = ((rdx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rdx = ((rbx.wrapping_add(P2)) as u64) as u64;
    rcx = ((rdi.wrapping_add(P2)) as u64) as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { crate::simd_ops::wu32(rdi.wrapping_add(P4), ((rax as u32)) as u32); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27648<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x64u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27649<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } fl_l = (crate::simd_ops::ru64(rcx)) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } r14 = (((crate::simd_ops::ru32(rsi)) as i32 as i64) as u64) as u64; fl_l = r14; fl_r = r14; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 3; continue; } rdi = 0; fl_l = 0; fl_r = 0; bb = 1; continue; }
            1 => { rbp = (crate::simd_ops::ru64(rsi.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(P0))) as u64; fl_l = rbp; fl_r = rbp; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = rbp; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = 0x68; rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdi = ((rdi).wrapping_add(1)) as u64; fl_l = rdi; fl_r = r14; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 1; continue; } bb = 3; continue; }
            3 => { rdx = (((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as i32 as i64) as u64) as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = ((rdx.wrapping_mul(8).wrapping_add(P0)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::lk_and_u64(rbx.wrapping_add(P1), (0x0u64) as u64); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27661<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x30u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27662<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rbp = rsp; rsp = ((rsp).wrapping_sub(0x30u64)) as u64; r14 = rcx; rsi = r8; rcx = r8; bb = 10; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = rdx.wrapping_add(0x1); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 3; continue; } rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x80; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } rbx = rdx.wrapping_add(0x2); bb = 3; continue; }
            2 => { r8 = ((rdi as u32)) as u32 as u64; rcx = rbp.wrapping_sub(0x10); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x8u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x10u64))) as u64; bb = 3; continue; }
            3 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 4; continue; } fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x10u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } rdx = rbp.wrapping_add(0x38); rcx = rbx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; crate::simd_ops::wu32(r14.wrapping_add(0x28u64), ((rax as u32)) as u32); bb = 9; continue; }
            4 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 6; continue; } fl_l = ((rbx).wrapping_sub(1)) as u64; rbx = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = r14.wrapping_add(0x10); rdx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rbx); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); r8 = rbx; rdx = rax; rcx = rsi; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi)) as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } rcx = rax; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 9; continue; } rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 5; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 12; continue; } fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } rcx = r14.wrapping_add(0x8); fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 7; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 8; continue; }
            7 => { rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rcx = ((rdi as u32)) as u32 as u64; r9 = rsi; r8 = rbx; rdx = rax; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } bb = 9; continue; }
            9 => { rcx = rsi; bb = 10; continue; }
            10 => { rdx = rbp.wrapping_add(0x28); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; bb = 11; continue; }
            11 => { rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            12 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 11; continue; }
            13 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 11; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27669<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x60u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27675<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x14u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27686<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x98; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27701<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x48u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(P0)) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27714<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(P0), rbx); crate::simd_ops::wu64(rax.wrapping_add(P1), rbp); crate::simd_ops::wu64(rax.wrapping_add(P2), rsi); crate::simd_ops::wu64(rax.wrapping_add(P3), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = r9; rbp = r8; r14 = ((rdx as u32)) as u32 as u64; rsi = rcx; fl_l = r9; fl_r = (crate::simd_ops::ru64(rcx)) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 1; continue; } rdx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = rax; bb = 1; continue; }
            1 => { rcx = rbp; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rdi = ((rax as u32)) as u32 as u64; r9 = rbx; rdi = (((rdi as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = ((r14 as u32)) as u32 as u64; r8 = ((rdi as u32)) as u32 as u64; rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P6)) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rbp; rbx = rax; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); r9 = rbx; r8 = ((rdi as u32)) as u32 as u64; rdx = rax; rcx = rsi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27742<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u32(rcx.wrapping_add(P0), (0x0u32) as u32)) as u32 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27762<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x80; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27770<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27784<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rbx = rdx; rsi = (rsi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 2; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rdi)) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; r8 = (crate::simd_ops::ru64(rdx.wrapping_add(P1))) as u64; rdx = rbx; rax = crate::simd_ops::call_stk(r8, rax, rcx, rdx, r8, r9, rsp); rdi = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; fl_l = ((((rsi as u8)) | ((rax as u8)))) as u8 as u64; fl_r = fl_l; rsi = (rsi & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 2; continue; }
            2 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (rax & !0xffu64) | ((((rsi as u8)) as u8 as u64)); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27791<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; rbx = (crate::simd_ops::ru64(rbx)) as u64; bb = 7; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rdx = rdi; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; bb = 3; continue; }
            2 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } rbx = rax; rax = (crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64; bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 2; continue; } bb = 4; continue; }
            4 => { rbx = rax; bb = 7; continue; }
            5 => { rcx = (crate::simd_ops::ru64(rax)) as u64; rbx = rax; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 7; continue; } bb = 6; continue; }
            6 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rbx = rcx; rcx = rax; fl_l = (crate::simd_ops::ru8(rax.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 6; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P4))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27806<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = rcx;
    rcx = 0x8;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P1; crate::simd_ops::wu64(rax, rcx); }
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x30u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27866<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rax = 0xffffffff;
    let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P0), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64;
    rax = (xa_old) as u32 as u64;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1;
    if (((fl_l as u32)) != ((fl_r as u32))) { rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = r8; fl_r = r8;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(r8)) as u64; rdx = rcx; rcx = r8; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); } else { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = 0x1; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27940<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) != 0)) { rax = 0; fl_l = 0; fl_r = 0; return rax; }
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27946<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x27;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rcx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = P3; crate::simd_ops::wu64(rcx, rax); rax = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rcx.wrapping_add(P2), rax); crate::simd_ops::wu64(rcx.wrapping_add(P4), rax); crate::simd_ops::wu64(rcx.wrapping_add(P5), rax); crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32); rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27948<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm7: u128 = 0; let mut xmm8: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rdi = r9; crate::simd_ops::wu64(rcx.wrapping_add(0x8u64), r9); r14 = r8; crate::simd_ops::wu64(rcx.wrapping_add(0x10u64), r8); rsi = rdx; rax = ((crate::simd_ops::ru8(r9.wrapping_add(0x1cu64))) as u32) as u32 as u64; rbx = rcx; crate::simd_ops::wu8(rcx.wrapping_add(0x50u64), ((rax as u8)) as u8); r15 = ((crate::simd_ops::ru8(r9.wrapping_add(0x1du64))) as u32) as u32 as u64; crate::simd_ops::wu8(rcx.wrapping_add(0x51u64), ((r15 as u8)) as u8); rax = ((crate::simd_ops::ru8(rbp.wrapping_add(0x4u64))) as u32) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rbp)) as u32 as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x30u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(P0)) as u32) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x20u64), (xmm8) as u128); xmm0 = (xmm7) as u128; xmm8 = 0u128; xmm8 = (xmm8 & !0xffff_ffffu128) | ((((rax) as i64) as f32).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x20u64), ((xmm8 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(r9.wrapping_add(0x10u64))) as u32) as u128; crate::simd_ops::wu32(rcx.wrapping_add(0x40u64), ((xmm1 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x44u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x14u64))) as u32) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x48u64), ((xmm1 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((if f32::from_bits(xmm1 as u32) > f32::from_bits(crate::simd_ops::ru32(P1)) { f32::from_bits(xmm1 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P1)) }).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm7 = (xmm7 & !0xffff_ffffu128) | ((f32::from_bits(xmm7 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm7) as u128; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x4cu64), ((xmm0 as u32)) as u32); r12 = 0x1; rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rcx = (((rcx as u8)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rcx as u32)) as u32); rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x300; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((r12 as u32)) as u32); rax = ((r12 as u32)) as u32 as u64; crate::simd_ops::wu8(rbx.wrapping_add(0x28u64), ((r12 as u8)) as u8); bb = 2; continue; }
            1 => { crate::simd_ops::wu8(rbx.wrapping_add(0x28u64), (0x0u8) as u8); rax = ((rcx as u32)) as u32 as u64; fl_l = (crate::simd_ops::ru32(rbp.wrapping_add(0x4u64))) as u32 as u64; fl_r = 0x8000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(0x1cu64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 2; continue; } rax = ((rcx.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x24u64), ((rax as u32)) as u32); bb = 2; continue; }
            2 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; r8 = P3; r9 = 0x76; rcx = rsi; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (r12 as u32) } else { (rax as u32) }) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x2cu64), ((rdx as u32)) as u32); rax = (crate::simd_ops::ru64(rsi)) as u64; rdx = ((rdx).wrapping_shl(3u32)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu64(rbx.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } rax = 0x34; bb = 5; continue; }
            3 => { xmm0 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(crate::simd_ops::ru32(rbx.wrapping_add(0x20u64)))).to_bits() as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); rcx = r14; rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x18u64))) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x18u64), ((rax as u32)) as u32); rdx = (crate::simd_ops::ru64(r14)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rdx.wrapping_add(0x58u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = 0x80; rcx = rax; r8 = (crate::simd_ops::ru64(rax)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(r8.wrapping_add(0x8u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } crate::simd_ops::wu32(rbx.wrapping_add(0x54u64), (0x80u32) as u32); bb = 4; continue; }
            4 => { rax = ((r12 as u32)) as u32 as u64; bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; xmm8 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rsp = ((rsp).wrapping_add(0x40u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27949<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rdx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; rbp = rsp.wrapping_add(0x50); r9 = ((crate::simd_ops::ru16(rdx.wrapping_add(0x12u64))) as u32) as u32 as u64; r14 = rcx; crate::simd_ops::wu64(rbp.wrapping_add(0x90u64), rbx); r8 = rdx; crate::simd_ops::wu128(rbp.wrapping_add(0x30u64), (xmm6) as u128); xmm6 = (xmm3) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x20u64), (xmm7) as u128); xmm7 = (xmm2) as u128; rcx = ((r9.wrapping_mul(4)) as u64) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x80u64), ((r9 as u32)) as u32); rax = rcx.wrapping_add(0xf); crate::simd_ops::wu64(rbp, r9); fl_l = rax; fl_r = rcx; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 1; continue; } rax = (0xffffffffffffff0u64) as u64; bb = 1; continue; }
            1 => { rax = (((rax) & (0xfffffffffffffff0u64))) as u64; rdx = (crate::simd_ops::ru32(rbp.wrapping_add(0xa0u64))) as u32 as u64; rsp = ((rsp).wrapping_sub(rax)) as u64; rsi = rsp.wrapping_add(0x50); fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 19; continue; } r13 = 0; fl_l = 0; fl_r = 0; r9 = ((r9.wrapping_mul(4)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru32(r14.wrapping_add(0x54u64))) as u32 as u64; fl_r = 0x80; rbx = ((rcx.wrapping_add(rsi)) as u64) as u64; r12 = ((rdx as u32)) as u32 as u64; r15 = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 10; continue; } crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r9); bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru16(r8.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r8)) as u64; r8 = r9; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = rsi; crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp)) as u64; fl_l = rsi; fl_r = rbx; rcx = rsi; r10 = r13; r8 = (if (((fl_l as u64)) > ((fl_r as u64))) { r13 } else { r8 }) as u64; rdx = rdi; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } fl_l = r8; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 5; continue; } rax = r8.wrapping_sub(0x1); rax = ((rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = r8.wrapping_sub(0x1); r9 = ((rsi.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 3; continue; } fl_l = r9; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 5; continue; } bb = 3; continue; }
            3 => { fl_l = (((r8) & (0xfffffffffffffff0u64))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; rdx = rdi; bb = 4; continue; }
            4 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; r10 = ((r10).wrapping_add(0x10u64)) as u64; xmm1 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx, (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x20u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x30u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; rdx = ((rdx).wrapping_add(0x40u64)) as u64; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); rcx = ((rcx).wrapping_add(0x40u64)) as u64; fl_l = r10; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 4; continue; } bb = 5; continue; }
            5 => { fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } rax = rbx; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 7; continue; } rax = rbx.wrapping_sub(0xc); bb = 6; continue; }
            6 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32) as u128; rdx = ((rdx).wrapping_add(0x10u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((xmm1 as u32)) as u32); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 6; continue; } fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 8; continue; }
            8 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 8; continue; } bb = 9; continue; }
            9 => { xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32) as u128; r8 = 0x1; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rdx = rsi; xmm1 = ((crate::simd_ops::ru32(r14.wrapping_add(0x4cu64))) as u32) as u128; r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0x44u64))) as u32) as u128; rcx = ((rax.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rbp.wrapping_add(0x8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; r15 = ((r15).wrapping_add(1)) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x10u64))) as u64; fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 19; continue; }
            10 => { crate::simd_ops::wu64(rbp.wrapping_add(0x8u64), r9); bb = 11; continue; }
            11 => { rcx = ((crate::simd_ops::ru16(r8.wrapping_add(0x10u64))) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = (crate::simd_ops::ru64(r8)) as u64; r8 = r9; rcx = (((rcx) as i64).wrapping_mul((r15) as i64) as u64) as u64; rdi = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; rcx = rsi; crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), rdi); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp)) as u64; fl_l = rsi; fl_r = rbx; rcx = rsi; r10 = r13; r8 = (if (((fl_l as u64)) > ((fl_r as u64))) { r13 } else { r8 }) as u64; rdx = rdi; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 14; continue; } fl_l = r8; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 14; continue; } rax = r8.wrapping_sub(0x1); rax = ((rdi.wrapping_add(rax.wrapping_mul(4))) as u64) as u64; r9 = r8.wrapping_sub(0x1); r9 = ((rsi.wrapping_add(r9.wrapping_mul(4))) as u64) as u64; fl_l = rsi; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 12; continue; } fl_l = r9; fl_r = rdi; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 14; continue; } bb = 12; continue; }
            12 => { fl_l = (((r8) & (0xfffffffffffffff0u64))) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; rdx = rdi; bb = 13; continue; }
            13 => { xmm0 = (crate::simd_ops::ru128(rcx)) as u128; r10 = ((r10).wrapping_add(0x10u64)) as u64; xmm1 = (crate::simd_ops::ru128(rdx)) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx, (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x10u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x10u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x10u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x20u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x20u64))) as u128; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x20u64), (xmm1) as u128); xmm0 = (crate::simd_ops::ru128(rcx.wrapping_add(0x30u64))) as u128; xmm1 = (crate::simd_ops::ru128(rdx.wrapping_add(0x30u64))) as u128; rdx = ((rdx).wrapping_add(0x40u64)) as u64; xmm1 = crate::simd_ops::vmulps(xmm1, xmm1); xmm1 = crate::simd_ops::vaddps(xmm1, xmm0); crate::simd_ops::wu128(rcx.wrapping_add(0x30u64), (xmm1) as u128); rcx = ((rcx).wrapping_add(0x40u64)) as u64; fl_l = r10; fl_r = r8; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 13; continue; } bb = 14; continue; }
            14 => { fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } rax = rbx; rax = ((rax).wrapping_sub(rcx)) as u64; rax = ((rax).wrapping_add(0x3u64)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = rax; fl_r = (0x10u64) as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 16; continue; } rax = rbx.wrapping_sub(0xc); bb = 15; continue; }
            15 => { xmm0 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x4u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0x8u64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0x8u64), ((xmm0 as u32)) as u32); xmm1 = ((crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32) as u128; rdx = ((rdx).wrapping_add(0x10u64)) as u64; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx.wrapping_add(0xcu64)))).to_bits() as u128); crate::simd_ops::wu32(rcx.wrapping_add(0xcu64), ((xmm1 as u32)) as u32); rcx = ((rcx).wrapping_add(0x10u64)) as u64; fl_l = rcx; fl_r = rax; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 15; continue; } fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 18; continue; } bb = 16; continue; }
            16 => { fl_l = rdx; fl_r = rcx; rdx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; bb = 17; continue; }
            17 => { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(rcx))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rcx))).to_bits() as u128); crate::simd_ops::wu32(rcx, ((xmm0 as u32)) as u32); rcx = ((rcx).wrapping_add(0x4u64)) as u64; fl_l = rcx; fl_r = rbx; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 17; continue; } bb = 18; continue; }
            18 => { xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0x30u64))) as u32) as u128; r8 = 0x1; rax = (crate::simd_ops::ru64(r14.wrapping_add(0x38u64))) as u64; rdx = rsi; xmm1 = ((crate::simd_ops::ru32(r14.wrapping_add(0x4cu64))) as u32) as u128; r9 = (crate::simd_ops::ru32(rbp.wrapping_add(0x80u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm0 as u32)) as u32); xmm0 = ((crate::simd_ops::ru32(r14.wrapping_add(0x44u64))) as u32) as u128; rcx = ((rax.wrapping_add(r15.wrapping_mul(8))) as u64) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rcx); rcx = rbp.wrapping_add(0x10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x88u64))) as u64; r15 = ((r15).wrapping_add(1)) as u64; r9 = (crate::simd_ops::ru64(rbp.wrapping_add(0x8u64))) as u64; fl_l = r12; fl_r = (0x1u64) as u64; r12 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x90u64))) as u64; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x30u64))) as u128; xmm7 = (crate::simd_ops::ru128(rbp.wrapping_add(0x20u64))) as u128; rsp = rbp.wrapping_add(0x40); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27950<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedStack::<1>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x70u64)) as u64; rbp = rsp.wrapping_add(0x50); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; r15 = rcx; r14 = ((crate::simd_ops::ru16(rdx.wrapping_add(0x12u64))) as u32) as u32 as u64; rax = ((((rax as u32)) & (0xf00u32))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x50u64), rbx); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x200; crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), rsi); r9 = rdx; rsi = (crate::simd_ops::ru32(rbp.wrapping_add(0x70u64))) as u32 as u64; rcx = 0x1; crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), rdi); r12 = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x10u64), (xmm6) as u128); r12 = (if (((fl_l as u32)) == ((fl_r as u32))) { (rcx as u32) } else { (r12 as u32) }) as u32 as u64; crate::simd_ops::wu128(rbp, (xmm7) as u128); xmm6 = (xmm3) as u128; rax = ((rsi.wrapping_mul(8)) as u64) as u64; xmm7 = (xmm2) as u128; rdx = rax.wrapping_add(0xf); rbx = ((rsi as u32)) as u32 as u64; r13 = (0xffffffffffffff0u64) as u64; fl_l = rdx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 1; continue; } rdx = r13; bb = 1; continue; }
            1 => { rdx = (((rdx) & (0xfffffffffffffff0u64))) as u64; rax = rdx; rsp = ((rsp).wrapping_sub(rdx)) as u64; rdi = rsp.wrapping_add(0x50); fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 3; continue; } rdx = 0; fl_l = 0; fl_r = 0; r8 = rdi; bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru16(r9.wrapping_add(0x10u64))) as u32) as u32 as u64; r8 = r8.wrapping_add(0x8); rax = (crate::simd_ops::ru64(r9)) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rdx = ((rdx).wrapping_add(1)) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(r8.wrapping_sub(0x8u64), rcx); fl_l = rbx; fl_r = (0x1u64) as u64; rbx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } bb = 3; continue; }
            3 => { rax = r14; rax = ((rax).wrapping_shl(2u32)) as u64; rcx = rax.wrapping_add(0xf); fl_l = rcx; fl_r = rax; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 4; continue; } rcx = r13; bb = 4; continue; }
            4 => { rcx = (((rcx) & (0xfffffffffffffff0u64))) as u64; rax = rcx; rsp = ((rsp).wrapping_sub(rcx)) as u64; r8 = ((r14.wrapping_mul(4)) as u32) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; rbx = rsp.wrapping_add(0x50); rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rbx; r8 = rdi; rdx = ((r14 as u32)) as u32 as u64; rcx = ((r12 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru32(r15.wrapping_add(0x54u64))) as u32 as u64; fl_r = 0x80; r9 = ((r14 as u32)) as u32 as u64; xmm2 = ((crate::simd_ops::ru32(r15.wrapping_add(0x30u64))) as u32) as u128; r8 = ((rsi as u32)) as u32 as u64; xmm3 = ((crate::simd_ops::ru32(r15.wrapping_add(0x4cu64))) as u32) as u128; rdx = rbx; xmm4 = ((crate::simd_ops::ru32(r15.wrapping_add(0x44u64))) as u32) as u128; rcx = rdi; rax = (crate::simd_ops::ru64(r15.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x48u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x40u64), ((xmm3 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm4 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((xmm7 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            5 => { (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rbp.wrapping_add(0x58u64))) as u64; rdi = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; xmm6 = (crate::simd_ops::ru128(rbp.wrapping_add(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(rbp)) as u128; rsp = rbp.wrapping_add(0x20); r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27953<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rdx;
    rdi = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rax = (crate::simd_ops::ru64(rdi)) as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rcx = rdi;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rax = (crate::simd_ops::ru64(rbx)) as u64;
    rdx = rdi;
    rcx = rbx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27954<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = (crate::simd_ops::ru64(rdx)) as u64;
    r8 = P0;
    r10 = rdx;
    r9 = 0x2a;
    rbx = rcx;
    rcx = r10;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P3; crate::simd_ops::wu64(rax, rcx); xmm0 = (crate::simd_ops::ru128(rbx.wrapping_add(P2))) as u128; crate::simd_ops::wu128(rax.wrapping_add(P2), (xmm0) as u128); xmm1 = ((crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64) as u128; crate::simd_ops::wu64(rax.wrapping_add(P4), ((xmm1 as u64)) as u64); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    crate::simd_ops::set_xmm_ret(xmm0); let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27955<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x14;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P3; crate::simd_ops::wu64(rax, rcx); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27963<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27979<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r10 = rdx;
    r10 = ((r10).wrapping_sub(r8)) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = ((rax).wrapping_add(r10)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu64(rcx.wrapping_add(P0), rax); }
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_r = (0x0u64) as u64;
    if (((fl_l as u64)) == ((fl_r as u64))) { rax = 0x1; return rax; }
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { rax = 0x1; return rax; }
    loop { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; crate::simd_ops::lk_add_u64(rax.wrapping_add(rdx.wrapping_mul(8)), (r10) as u64); rdx = (((rdx as u32)).wrapping_add(1)) as u32 as u64; rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if !((((fl_l as u32)) < ((fl_r as u32)))) { break; } }
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_27980<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64; rsi = 0; fl_l = 0; fl_r = 0; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P1), rsi); bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P3), rsi); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P4), rsi); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P5), rsi); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P6), rsi); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28006<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x23;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28029<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x12;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P3; crate::simd_ops::wu64(rax, rcx); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28031<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdx as u8)) as u8);
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rdx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28041<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = P0;
    r9 = 0x13;
    rdx = ((r9.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P3; crate::simd_ops::wu64(rax, rcx); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28060<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P2), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28080<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rbx; fl_r = rbx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; rdx = rbx; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28103<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x18u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = rdx; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rsi = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); xmm8 = ((crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(8)))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm12) as u128); xmm12 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; rdi = ((r8 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm9 = ((crate::simd_ops::ru32(P1)) as u32) as u128; xmm0 = (xmm12) as u128; xmm2 = ((crate::simd_ops::ru32(P2)) as u32) as u128; xmm1 = (xmm7) as u128; xmm5 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm4 = 0u128; fl_l = ((xmm12 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm10 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((xmm12 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); crate::simd_ops::wu32(rbx, ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 3; continue; } xmm3 = (xmm7) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm0 = (xmm3) as u128; xmm0 = (xmm0) & (crate::simd_ops::ru128(P6)); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm10 as u32)) as u32 as u64; xmm0 = (xmm3) as u128; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 2; continue; } fl_l = ((xmm12 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 1; continue; } xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); bb = 7; continue; }
            1 => { xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); bb = 7; continue; }
            2 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm1 = (xmm3) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm3 = (xmm9) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 7; continue; } xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); bb = 7; continue; }
            3 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 4; continue; } xmm3 = (xmm9) as u128; bb = 7; continue; }
            4 => { fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 6; continue; } fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; fl_r = 0x80000000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 5; continue; } xmm3 = (xmm5) as u128; bb = 7; continue; }
            5 => { xmm3 = 0u128; bb = 7; continue; }
            6 => { xmm3 = (xmm2) as u128; bb = 7; continue; }
            7 => { fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm0 = (xmm8) as u128; crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((xmm3 as u32)) as u32); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm1 = (xmm6) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0xa0u64), ((xmm8 as u32)) as u32); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = 0u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32)).sqrt().to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(8)), ((xmm0 as u32)) as u32); if (f32::from_bits((fl_l as u32)) == f32::from_bits((fl_r as u32)) || f32::from_bits((fl_l as u32)).is_nan() || f32::from_bits((fl_r as u32)).is_nan()) { bb = 10; continue; } xmm2 = (xmm6) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm0 = (xmm2) as u128; xmm0 = (xmm0) & (crate::simd_ops::ru128(P6)); fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm10 as u32)) as u32 as u64; xmm0 = (xmm2) as u128; if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 8; continue; } fl_l = ((xmm8 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) / f32::from_bits(xmm0 as u32)).to_bits() as u128); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 13; continue; } fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 9; continue; } xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(xmm5 as u32)).to_bits() as u128); bb = 13; continue; }
            8 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm1 = (xmm2) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm9) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) / f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); if (f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32))) { bb = 13; continue; } bb = 9; continue; }
            9 => { xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm5 as u32)).to_bits() as u128); bb = 13; continue; }
            10 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 11; continue; } xmm2 = (xmm9) as u128; bb = 13; continue; }
            11 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; if (f32::from_bits((fl_l as u32)) != f32::from_bits((fl_r as u32)) && !f32::from_bits((fl_l as u32)).is_nan() && !f32::from_bits((fl_r as u32)).is_nan()) { bb = 13; continue; } fl_l = (crate::simd_ops::ru32(rsp.wrapping_add(0xa0u64))) as u32 as u64; fl_r = 0x80000000; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } xmm2 = (xmm5) as u128; bb = 13; continue; }
            12 => { xmm2 = 0u128; bb = 13; continue; }
            13 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; r11 = rsp.wrapping_add(0x90); xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x70u64))) as u128; crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x4u64), ((xmm2 as u32)) as u32); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xdu64), (0x1u8) as u8); rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28104<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28109<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm4: u128 = 0; let mut xmm5: u128 = 0; let mut xmm6: u128 = 0; let mut xmm7: u128 = 0; let mut xmm8: u128 = 0; let mut xmm9: u128 = 0; let mut xmm10: u128 = 0; let mut xmm11: u128 = 0; let mut xmm12: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 19>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x90u64)) as u64; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rbx = rdx; crate::simd_ops::wu128(rax.wrapping_sub(0x18u64), (xmm6) as u128); rsi = rcx; crate::simd_ops::wu128(rax.wrapping_sub(0x28u64), (xmm7) as u128); xmm7 = ((crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x38u64), (xmm8) as u128); r8 = (((r8 as u32)).wrapping_shr(1u32)) as u32 as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm9) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm10) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x68u64), (xmm11) as u128); xmm8 = ((crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(8)))) as u32) as u128; xmm6 = ((crate::simd_ops::ru32(rdx.wrapping_add(r8.wrapping_mul(8)).wrapping_add(0x4u64))) as u32) as u128; crate::simd_ops::wu128(rax.wrapping_sub(0x78u64), (xmm12) as u128); xmm12 = ((crate::simd_ops::ru32(rdx)) as u32) as u128; rdi = ((r8 as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); xmm4 = ((crate::simd_ops::ru32(P1)) as u32) as u128; fl_l = ((xmm7 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm3 = ((crate::simd_ops::ru32(P2)) as u32) as u128; if !(f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 1; continue; } xmm1 = (xmm3) as u128; bb = 2; continue; }
            1 => { xmm1 = (xmm4) as u128; bb = 2; continue; }
            2 => { fl_l = ((xmm6 as u32)) as u32 as u64; fl_r = ((xmm4 as u32)) as u32 as u64; xmm5 = ((crate::simd_ops::ru32(P3)) as u32) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm9 = ((crate::simd_ops::ru32(P4)) as u32) as u128; xmm10 = ((crate::simd_ops::ru32(P5)) as u32) as u128; xmm11 = ((crate::simd_ops::ru32(P6)) as u32) as u128; xmm0 = (xmm1) as u128; xmm2 = (xmm1) as u128; xmm0 = (xmm0) & (xmm5); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm1 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm7) as u128; xmm0 = (xmm0) & (xmm5); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm7 as u32)).to_bits() as u128); xmm1 = (xmm2) as u128; xmm1 = (xmm1) & (xmm5); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm7) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx, ((xmm1 as u32)) as u32); xmm1 = (xmm2) as u128; xmm1 = (xmm1) & (xmm5); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm12 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(0x4u64), ((xmm1 as u32)) as u32); if (f32::from_bits((fl_l as u32)) > f32::from_bits((fl_r as u32))) { bb = 3; continue; } xmm3 = (xmm4) as u128; bb = 3; continue; }
            3 => { xmm7 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; r11 = rsp.wrapping_add(0x90); xmm12 = (crate::simd_ops::ru128(r11.wrapping_sub(0x70u64))) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((f32::from_bits(xmm3 as u32) + f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm2 = (xmm3) as u128; xmm0 = (xmm3) as u128; xmm0 = (xmm0) & (xmm5); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm3 as u32)).to_bits() as u128); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); xmm0 = (xmm6) as u128; xmm0 = (xmm0) & (xmm5); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm6 as u32)).to_bits() as u128); xmm1 = (xmm2) as u128; xmm1 = (xmm1) & (xmm5); xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) * f32::from_bits(xmm10 as u32)).to_bits() as u128); xmm10 = (crate::simd_ops::ru128(r11.wrapping_sub(0x50u64))) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm2 = (xmm6) as u128; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) * f32::from_bits(xmm9 as u32)).to_bits() as u128); xmm9 = (crate::simd_ops::ru128(r11.wrapping_sub(0x40u64))) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) - f32::from_bits(xmm0 as u32)).to_bits() as u128); crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(8)), ((xmm1 as u32)) as u32); xmm1 = (xmm2) as u128; xmm1 = (xmm1) & (xmm5); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) - f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm11 as u32)).to_bits() as u128); xmm11 = (crate::simd_ops::ru128(r11.wrapping_sub(0x60u64))) as u128; xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128); xmm1 = (xmm1 & !0xffff_ffffu128) | ((f32::from_bits(xmm1 as u32) * f32::from_bits(xmm8 as u32)).to_bits() as u128); xmm8 = (crate::simd_ops::ru128(r11.wrapping_sub(0x30u64))) as u128; crate::simd_ops::wu32(rbx.wrapping_add(rdi.wrapping_mul(8)).wrapping_add(0x4u64), ((xmm1 as u32)) as u32); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; crate::simd_ops::wu8(rsi.wrapping_add(0xdu64), (0x0u8) as u8); rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28121<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = 0x1;
    crate::simd_ops::wu16(rcx.wrapping_add(P0), ((rax as u16)) as u16);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28123<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P2))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28133<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    xmm0 = (xmm1) as u128;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm1 as u32)) as u32);
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(xmm2 as u32)).to_bits() as u128);
    xmm3 = 0u128;
    fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64;
    if !((f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm3 as u32)) as u32); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    xmm0 = (xmm0 & !0xffff_ffffu128) | ((if f32::from_bits(xmm0 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm0 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((xmm0 as u32)) as u32);
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28185<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru8(P0)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28187<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((((rcx as u32)) >> (29u32)) & 1) as u64;
    if !(((fl_l as u32) == 0)) { r8 = rdx; rcx = 0; fl_l = 0; fl_r = 0; rdx = (crate::simd_ops::ru64(P0)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28191<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28225<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = r8;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28265<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), (0x0u64) as u64);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28296<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28319<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0x18u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28335<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    fl_l = (crate::simd_ops::atomic_add_u32(rcx.wrapping_add(P0), 1)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28339<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0x80u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28345<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28365<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28384<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; r15 = rcx; rbx = r9; rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; r12 = 0x1; crate::simd_ops::wu32(rsp.wrapping_add(0x24u64), ((rcx as u32)) as u32); rdx = ((rcx as u32)) as u32 as u64; rcx = r15.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = ((r12.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbp); rbp = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rdi); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r13); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r14); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rsi = rbx.wrapping_add(0x8); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x48u64))) as u32 as u64; r13 = rbx.wrapping_add(0x40); r14 = 0; fl_l = 0; fl_r = 0; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r13)) as u64; rdi = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } rdi = (crate::simd_ops::ru64(rdi.wrapping_add(0x8u64))) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rdi = r14; bb = 6; continue; }
            6 => { fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } fl_l = (crate::simd_ops::atomic_add_u32(rdi.wrapping_add(0x14u64), 1)) as u32 as u64; bb = 9; continue; }
            7 => { rsi = rbx.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 3; continue; } rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            8 => { rdi = r14; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rbx, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } rax = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } bb = 13; continue; }
            11 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } rax = (crate::simd_ops::ru32(rbx.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 14; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            12 => { rsi = rbx.wrapping_add(0x8); bb = 13; continue; }
            13 => { rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { fl_l = rdi; fl_r = rdi; rsi = rdi.wrapping_sub(0x10); rsi = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { rsi }) as u64; fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 27; continue; } rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rdx = rbx; rcx = rsp.wrapping_add(0x30); rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); rbx = (rbx & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r15 = (r15 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), ((rbx as u8)) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rsi = rax; rdi = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu32(rax.wrapping_add(0x20u64), ((rbp as u32)) as u32); rdx = rax.wrapping_add(0x10); rcx = r13; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } r15 = (((r12 as u8)) as u32) as u32 as u64; bb = 22; continue; }
            15 => { crate::simd_ops::wu32(rsi.wrapping_add(0x20u64), ((r14 as u32)) as u32); rax = ((rdi as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rsi.wrapping_add(0x24u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } rax = (crate::simd_ops::ru64(rsi.wrapping_add(0x10u64))) as u64; rcx = rsi.wrapping_add(0x10); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { rbx = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rsi = r14; bb = 17; continue; }
            17 => { fl_l = ((rbx as u8)) as u8 as u64; fl_r = ((rbx as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } rcx = (crate::simd_ops::ru32(r13.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } rdx = 0; fl_l = 0; fl_r = 0; rax = ((rbp as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(r13)) as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((rbp as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } rdx = (crate::simd_ops::ru64(rdx.wrapping_add(0x8u64))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 19; continue; }
            19 => { rdx = r14; bb = 20; continue; }
            20 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } fl_l = (crate::simd_ops::atomic_add_u32(rdx.wrapping_add(0x14u64), 1)) as u32 as u64; rsi = rdx.wrapping_sub(0x10); bb = 22; continue; }
            21 => { rsi = r14; bb = 22; continue; }
            22 => { rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; fl_l = (crate::simd_ops::atomic_add_u32(rax, 1)) as u32 as u64; rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; if (((fl_l as u32)) != 0) { bb = 23; continue; } crate::simd_ops::wu32(rcx.wrapping_add(0x4u64), ((r14 as u32)) as u32); bb = 23; continue; }
            23 => { rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rsi; fl_r = rsi; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } r12 = ((rsi.wrapping_add(0x34u64)) as u32) as u32 as u64; bb = 25; continue; }
            24 => { fl_l = ((r15 as u8)) as u8 as u64; fl_r = ((r15 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x24u64))) as u32 as u64; rcx = rsi; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); r12 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } rcx = rsi.wrapping_add(0x10); let xa_src: u32 = (rdi as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rdi = (xa_old) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 25; continue; }
            25 => { rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 26; continue; } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = ((rcx).wrapping_add(0x8u64)) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r12 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 28; continue; } rax = ((r12.wrapping_sub(0x5au64)) as u32) as u32 as u64; bb = 29; continue; }
            26 => { r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; bb = 27; continue; }
            27 => { rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r8 = rsi.wrapping_add(0x10); fl_l = rsi; fl_r = rsi; rcx = r15; r8 = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { r14 } else { r8 }) as u64; rax = crate::simd_ops::call_stk(P11, rax, rcx, rdx, r8, r9, rsp); rcx = r15.wrapping_add(0x8); rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            28 => { rax = ((r12 as u32)) as u32 as u64; bb = 29; continue; }
            29 => { r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28389<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64));
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28390<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
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
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rbx.wrapping_add(P8), rax);
    crate::simd_ops::wu64(rbx.wrapping_add(P9), rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28392<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0x10u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28393<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0x28u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28395<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdi = rdx; rbx = rcx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rax, rcx, rdx, r8, r9, rsp); rsi = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = ((rsi.wrapping_add(P4)) as u32) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { crate::simd_ops::wu64(rax, rdi); rdx = rbx; rax = (crate::simd_ops::ru64(rdi)) as u64; rcx = rdi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::atomic_add_u32(rbx.wrapping_add(P5), 1)) as u32 as u64; rax = ((rsi as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28396<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P0))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28401<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rdx; fl_r = rdx;
    rax = ((rdx.wrapping_add(P1)) as u64) as u64;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28412<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rsi = ((r9 as u32)) as u32 as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rbp = r8; r14 = rdx; rdi = rcx; r10 = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = r10; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; r9 = ((rsi as u32)) as u32 as u64; r8 = rbp; rdx = r14; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28416<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rbx = rcx; fl_l = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rdx)) as u64; rcx = rdi; rdx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rcx = ((rbx.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P4))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } r8 = ((r8).wrapping_sub(rax)) as u64; rdx = ((rax.wrapping_add(P6)) as u64) as u64; r8 = ((((r8 as u32)) & (0xfffffff8u32))) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P4), (1) as u32)) as u32 as u64; bb = 2; continue; }
            2 => { rcx = ((rbx.wrapping_add(P8)) as u64) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P9), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P10))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28431<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28467<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28469<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28480<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rdi = rcx;
    crate::simd_ops::wu64(rcx, rax);
    rbx = ((rcx.wrapping_add(P1)) as u64) as u64;
    rax = P2;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    rax = P4;
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    rax = P6;
    crate::simd_ops::wu64(rcx.wrapping_add(P7), rax);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx = rdi; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu32(rbx.wrapping_add(P5), (0x0u32) as u32);
    fl_l = rbx; fl_r = rdx;
    if !((((fl_l as u64)) == ((fl_r as u64)))) { rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64);
    crate::simd_ops::wu32(rbx.wrapping_add(P9), (0x0u32) as u32);
    rcx = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P10, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28501<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rdi = rcx;
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64;
    rdx = ((rbx.wrapping_add(rax.wrapping_mul(8))) as u64) as u64;
    fl_l = rbx; fl_r = rdx;
    if (((fl_l as u64)) == ((fl_r as u64))) { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    loop { rcx = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; rbx = ((rbx).wrapping_add(0x8u64)) as u64; rax = (crate::simd_ops::ru64(rdi.wrapping_add(P0))) as u64; rcx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rbx; fl_r = rcx; if !((((fl_l as u64)) != ((fl_r as u64)))) { break; } }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28507<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64;
    crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8);
    let _ = &stack; return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28510<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rsp = ((rsp).wrapping_sub(0x38u64)) as u64;
    fl_l = (crate::simd_ops::ru32(rdx)) as u32 as u64; fl_r = 0x4;
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rax = ((crate::simd_ops::ru8(rdx.wrapping_add(P0))) as u32) as u32 as u64; fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { xmm0 = ((crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32) as u128; xmm3 = 0u128; fl_l = ((xmm0 as u32)) as u32 as u64; fl_r = ((xmm3 as u32)) as u32 as u64; if !(!(f32::from_bits((fl_l as u32)) >= f32::from_bits((fl_r as u32)))) { xmm3 = (xmm0) as u128; xmm3 = (xmm3 & !0xffff_ffffu128) | ((if f32::from_bits(xmm3 as u32) < f32::from_bits(crate::simd_ops::ru32(P2)) { f32::from_bits(xmm3 as u32) } else { f32::from_bits(crate::simd_ops::ru32(P2)) }).to_bits() as u128); } r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; } r9 = (crate::simd_ops::ru32(rdx.wrapping_add(P1))) as u32 as u64; r8 = (crate::simd_ops::ru32(rdx.wrapping_add(P3))) as u32 as u64; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P4))) as u64; rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64)); rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x1u8))) as u8 as u64)); crate::simd_ops::wu8(rsp.wrapping_add(0x20u64), ((rax as u8)) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rsp = ((rsp).wrapping_add(0x38u64)) as u64; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
    rsp = ((rsp).wrapping_add(0x38u64)) as u64;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28527<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rcx = ((rcx).wrapping_shl(4u32)) as u64; rcx = ((rcx).wrapping_add(r8)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } r8 = ((r8).wrapping_add(0x10u64)) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rdx = 0; fl_l = 0; fl_r = 0; rax = ((r8.wrapping_add(P2)) as u64) as u64; fl_l = r8; fl_r = rcx; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rax }) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rax = (crate::simd_ops::ru64(rax)) as u64; return rax; }
            3 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28545<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax)) as u32 as u64; return rax; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28560<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = P2;
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28638<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = 0x8;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = P2; crate::simd_ops::wu64(rax, rcx); rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28639<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdx = 0xe8;
    rcx = 0x5;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { r8 = P1; rdx = rbx; rcx = rax; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28645<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(crate::simd_ops::ru32(rcx.wrapping_add(P1)))) as u32 as u64; fl_r = fl_l;
    rax = ((fl_l as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28648<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28649<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    xmm0 = ((crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32) as u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28732<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    r9 = 0xf;
    r8 = P0;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28737<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = 0;
    fl_l = 0; fl_r = 0;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28830<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rbx = rcx;
    crate::simd_ops::wu64(rcx, rax);
    fl_l = ((rdx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xc8; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28917<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu8(rcx.wrapping_add(P1), ((rax as u8)) as u8);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28926<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(P0, rcx);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28934<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rdx = rcx; rcx = (crate::simd_ops::ru64(P1)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28954<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r8 = rbx;
    rdx = rdi;
    rcx = ((rax as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28955<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28964<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = ((crate::simd_ops::ru16(P0)) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28965<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru32(P0)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_28971<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = r8; rsi = rdx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = 0x66; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } r8 = rdi; rdx = rsi; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = 0xffffffff; let xa_src: u32 = (rcx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x5e; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            4 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rbx = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } r8 = rdi; rdx = rsi; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = 0xffffffff; let xa_src: u32 = (rcx as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rcx = (xa_old) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            6 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x5e; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29010<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = r9; rdi = r8; rsi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { r8 = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), (0x0u64) as u64); rcx = rsp.wrapping_add(0x40); crate::simd_ops::wu8(rsp.wrapping_add(0x48u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } rax = 0x34; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            2 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru32(rsp.wrapping_add(0x90u64))) as u32 as u64; r9 = rsp.wrapping_add(0x50); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x40u64))) as u128; rdx = ((rax as u32)) as u32 as u64; r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x98u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((rcx as u32)) as u32); rcx = ((rsi as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29011<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 29>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rbp = rsp.wrapping_sub(0x57); rsp = ((rsp).wrapping_sub(0xe0u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = ((r9 as u32)) as u32 as u64; rsi = ((r8 as u32)) as u32 as u64; rdi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = 0x66; bb = 5; continue; }
            1 => { r8 = P1; crate::simd_ops::wu64(rbp.wrapping_sub(0x69u64), (0x0u64) as u64); rcx = rbp.wrapping_sub(0x69); crate::simd_ops::wu8(rbp.wrapping_sub(0x61u64), (0x0u8) as u8); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } rax = 0x34; bb = 5; continue; }
            2 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rbp.wrapping_sub(0x59); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x69u64))) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru64(P5)) as u64; rcx = ((rbx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 3; continue; } fl_l = ((rbx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 3; continue; } xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x69u64))) as u128; fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0xfffffffd; crate::simd_ops::wu32(rbp.wrapping_sub(0x41u64), ((rax as u32)) as u32); rcx = 0x0; crate::simd_ops::wu64(rbp.wrapping_sub(0x29u64), rdx); rcx = (rcx & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64)); crate::simd_ops::wu128(rbp.wrapping_add(0x17u64), (xmm1) as u128); rcx = (((rcx as u32)).wrapping_add(0x6u32)) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x45u64), ((rbx as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x49u64), ((rcx as u32)) as u32); rdx = rbp.wrapping_add(0x7); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x49u64))) as u128; rcx = rbp.wrapping_sub(0x59); crate::simd_ops::wu64(rbp.wrapping_sub(0x21u64), rcx); fl_l = ((rdi as u32)) as u32 as u64; fl_r = 0x2; rcx = (crate::simd_ops::ru64(P3)) as u64; crate::simd_ops::wu128(rbp.wrapping_add(0x7u64), (xmm0) as u128); crate::simd_ops::wu8(rbp.wrapping_sub(0xdu64), (if (((fl_l as u32)) == ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x29u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x27u64), (xmm0) as u128); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x9u64))) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu32(rbp.wrapping_sub(0x19u64), (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_sub(0x11u64), ((rsi as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x19u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x37u64), (xmm1) as u128); crate::simd_ops::wu128(rbp.wrapping_add(0x47u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = ((rax as u32)) as u32 as u64; bb = 4; continue; }
            3 => { r8 = 0x64; bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(P3)) as u64; rdx = rbp.wrapping_sub(0x59); rax = (crate::simd_ops::ru64(rcx)) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x28u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 5; continue; }
            5 => { r11 = rsp.wrapping_add(0xe0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x10u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsp = r11; rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29015<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x40u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rbx = ((r8 as u32)) as u32 as u64; rdi = rdx; rsi = ((rcx as u32)) as u32 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rsp.wrapping_add(0x30); rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } r9 = (crate::simd_ops::ru64(P3)) as u64; rax = rsp.wrapping_add(0x30); r8 = ((rbx as u32)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); rdx = rdi; rcx = ((rsi as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(P1)) as u64; rdx = rsp.wrapping_add(0x30); r8 = ((rax as u32)) as u32 as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(r9.wrapping_add(P5))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rsp = ((rsp).wrapping_add(0x40u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29057<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rdx as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) == 0)) { rdx = ((rbx as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rax = 0xe;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29072<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    if !((((fl_l as u8)) != ((fl_r as u8)))) { rax = 0x66; return rax; }
    r8 = ((rdx as u32)) as u32 as u64;
    rdx = rcx;
    rcx = (crate::simd_ops::ru64(P1)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29076<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) > ((fl_r as u8))) { bb = 2; continue; } fl_l = ((r8 as u16)) as u16 as u64; fl_r = ((r8 as u16)) as u16 as u64; if ((((fl_l as u16)) & ((fl_r as u16))) == 0) { bb = 1; continue; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } bb = 1; continue; }
            1 => { rax = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), ((rax as u8)) as u8); return crate::simd_ops::tail_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
            2 => { rax = 0x1f; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29116<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x30u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rdi = ((r8 as u32)) as u32 as u64; rsi = rdx; rbx = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = 0x66; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { r8 = P1; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rdx = rbx; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x20); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rbx; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); r9 = rsi; r8 = rsp.wrapping_add(0x20); rdx = ((rdi as u32)) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x30u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29117<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 27>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0xd0u64)) as u64; fl_l = (crate::simd_ops::ru8(P0)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; rsi = r9; rbp = r8; rdi = rdx; rbx = rcx; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = 0x66; bb = 4; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rdx = rbx; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x20); r14 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; rcx = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdx as u32)) as u32); rdx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); fl_l = 0; fl_r = rdi; rdi = (0u64.wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rax = ((((rax as u32)) & (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r14 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x0u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            2 => { rax = 0x64; bb = 3; continue; }
            3 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 4; continue; }
            4 => { r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            5 => { crate::simd_ops::wu64(rsp.wrapping_add(0xe0u64), r14); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r8 = P2; crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), (0x0u64) as u64); rdx = rbx; crate::simd_ops::wu8(rsp.wrapping_add(0x28u64), (0x0u8) as u8); rcx = rsp.wrapping_add(0x20); r14 = ((rax as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = (crate::simd_ops::ru32(rsp.wrapping_add(0x100u64))) as u32 as u64; rcx = ((rdx.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1c; if (((fl_l as u32)) <= ((fl_r as u32))) { bb = 2; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x20; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x20u64))) as u128; rcx = (crate::simd_ops::ru64(P4)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((rdx as u32)) as u32); rdx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x68u64), rdi); fl_l = 0; fl_r = rdi; rdi = (0u64.wrapping_sub(fl_r as u64)) as u64; crate::simd_ops::wu128(rsp.wrapping_add(0x90u64), (xmm1) as u128); rax = (if (((fl_l as u64)) < ((fl_r as u64))) { 0xffffffffu32 } else { 0u32 }) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), (0x1u32) as u32); rax = ((((rax as u32)) & (0x2u32))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r14 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x30u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x80u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x60u64), ((rax as u32)) as u32); xmm1 = (crate::simd_ops::ru128(rsp.wrapping_add(0x60u64))) as u128; rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x50u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xa0u64), (xmm0) as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), (0x0u32) as u32); xmm0 = (crate::simd_ops::ru128(rsp.wrapping_add(0x70u64))) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0xb0u64), (xmm1) as u128); crate::simd_ops::wu128(rsp.wrapping_add(0xc0u64), (xmm0) as u128); (rax, xmm0) = crate::simd_ops::call_stk_xmm((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            6 => { rax = 0x64; bb = 7; continue; }
            7 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0xe0u64))) as u64; bb = 8; continue; }
            8 => { r11 = rsp.wrapping_add(0xd0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x18u64))) as u64; rbp = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x28u64))) as u64; rsp = r11; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29129<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rax = P1; crate::simd_ops::wu64(rcx, rax); r9 = ((rdx as u32)) as u32 as u64; rbx = rcx; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; crate::simd_ops::wu64(r8.wrapping_add(P2), rax); bb = 1; continue; }
            1 => { fl_l = rbx; fl_r = (crate::simd_ops::ru64(P3)) as u64; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P2))) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } crate::simd_ops::wu64(P3, rax); bb = 3; continue; }
            2 => { crate::simd_ops::wu64(rdx.wrapping_add(P0), rax); bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(P4)) as u64; fl_l = rbx; fl_r = rax; rax = (if (((fl_l as u64)) == ((fl_r as u64))) { rdx } else { rax }) as u64; crate::simd_ops::lk_sub_u32(P5, (1) as u32); crate::simd_ops::wu64(P4, rax); rax = P6; crate::simd_ops::wu64(rcx, rax); fl_l = ((r9 as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 4; continue; } rdx = 0x20; rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29132<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 1; continue; } rax = ((rcx.wrapping_add(P1)) as u64) as u64; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            1 => { rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rcx = rax; rsp = ((rsp).wrapping_add(0x28u64)) as u64; bb = 0; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29144<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 7>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x30u64)) as u64;
    rbx = r9;
    rdi = ((r8 as u32)) as u32 as u64;
    rsi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    r9 = rbx;
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
pub unsafe extern "win64" fn tpl_29146<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = rsp.wrapping_add(0x40);
    rbx = ((rax as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rcx = (crate::simd_ops::ru64(P2)) as u64;
    r8 = rdi;
    rdx = ((rbx as u32)) as u32 as u64;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64;
    rcx = 0x1;
    rbx = 0xf;
    rbx = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { (rcx as u32) } else { (rbx as u32) }) as u32 as u64;
    rcx = rsp.wrapping_add(0x40);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    rax = ((rbx as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29148<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = r8;
    rdi = rdx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64;
    if !(((((fl_l as u32)) & ((fl_r as u32))) != 0)) { rax = 0xf; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    r8 = rbx;
    rdx = rdi;
    rcx = ((rax as u32)) as u32 as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P1, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29165<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = ((rsp).wrapping_sub(0x28u64)) as u64; rax = (crate::simd_ops::ru64(P0)) as u64; rdx = (crate::simd_ops::ru32(P1)) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(4))) as u64) as u64; rdx = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; r9 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } bb = 1; continue; }
            1 => { fl_l = (crate::simd_ops::ru32(rax)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 2; continue; } rax = ((rax).wrapping_add(0x28u64)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rcx = 0; fl_l = 0; fl_r = 0; rdx = ((rax.wrapping_add(P2)) as u64) as u64; fl_l = rax; fl_r = r9; rdx = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rdx }) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } rax = 0; fl_l = 0; fl_r = 0; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            3 => { rax = (crate::simd_ops::ru64(rdx.wrapping_add(P3))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rax = crate::simd_ops::call_stk(rax, rax, rcx, rdx, r8, r9, rsp); rcx = rax; bb = 4; continue; }
            4 => { rax = rcx; rsp = ((rsp).wrapping_add(0x28u64)) as u64; return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29183<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0x68u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29185<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::lk_and_u8(rcx.wrapping_add(P0), (0xefu8) as u8)) as u8 as u64; fl_r = fl_l;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29278<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    fl_l = (crate::simd_ops::lk_sub_u32(P0, (1) as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29282<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    fl_l = (crate::simd_ops::lk_add_u32(P0, (1) as u32)) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29288<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 9>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x38u64)) as u64; rax = 0; fl_l = 0; fl_r = 0; r8 = P0; rbx = rcx; rcx = ((rax as u32)) as u32 as u64; bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru32(r8)) as u32 as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) > ((fl_r as u32))) { bb = 3; continue; } rcx = (((rcx as u32)).wrapping_add(1)) as u32 as u64; r8 = ((r8).wrapping_add(0x4u64)) as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1b; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = 0x2; rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 2; continue; } crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), rbp); rdx = ((rdi.wrapping_mul(8)) as u64) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r12); rsi = rdi; r12 = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), r13); r13 = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), r14); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), r15); r15 = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; crate::simd_ops::wu64(rbx.wrapping_add(0x8u64), rax); crate::simd_ops::wu64(rbx, rax); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rdx = (crate::simd_ops::ru64(rbx)) as u64; rbp = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 5; continue; } fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } r8 = 0; fl_l = 0; fl_r = 0; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r8 as u32)) as u32); rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { crate::simd_ops::wu64(rbx, r13); rax = 0x2; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((r15 as u32)) as u32); crate::simd_ops::wu32(rbx.wrapping_add(0xcu64), ((r12 as u32)) as u32); bb = 14; continue; }
            5 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } fl_l = rdx; fl_r = rbp; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 6; continue; } rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { rcx = rbx.wrapping_add(0xc); crate::simd_ops::wu64(rbx, rbp); rax = ((rdi as u32)) as u32 as u64; rdx = rbx; crate::simd_ops::wu32(rcx, ((rax as u32)) as u32); r8 = 0; fl_l = 0; fl_r = 0; bb = 7; continue; }
            7 => { rcx = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 8; continue; } rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rbx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru64(rbx)) as u64; rdx = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 8; continue; } crate::simd_ops::wu64(rdx, r8); rdi = (crate::simd_ops::ru32(rbx.wrapping_add(0xcu64))) as u32 as u64; bb = 8; continue; }
            8 => { fl_l = rsi; fl_r = (0x1u64) as u64; rsi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 7; continue; } fl_l = ((r15 as u32)) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 12; continue; } r10 = r13; r11 = r15; bb = 9; continue; }
            9 => { rcx = (crate::simd_ops::ru64(r10)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru32(rcx.wrapping_add(0x8u64))) as u32 as u64; rdx = 0; fl_l = 0; fl_r = 0; let d = (crate::simd_ops::ru32(rbx.wrapping_add(0x8u64))) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rax = (crate::simd_ops::ru64(rbx)) as u64; r9 = (crate::simd_ops::ru64(rcx)) as u64; r8 = ((rdx as u32)) as u32 as u64; rdx = (crate::simd_ops::ru64(rax.wrapping_add(rdx.wrapping_mul(8)))) as u64; crate::simd_ops::wu64(rcx, rdx); rax = (crate::simd_ops::ru64(rbx)) as u64; crate::simd_ops::wu64(rax.wrapping_add(r8.wrapping_mul(8)), rcx); rcx = r9; fl_l = r9; fl_r = r9; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 10; continue; } bb = 11; continue; }
            11 => { r10 = ((r10).wrapping_add(0x8u64)) as u64; fl_l = r11; fl_r = (0x1u64) as u64; r11 = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 9; continue; } bb = 12; continue; }
            12 => { fl_l = r13; fl_r = r13; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 13; continue; } rdx = r13; rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 13; continue; }
            13 => { rax = 0x1; bb = 14; continue; }
            14 => { r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; r13 = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x20u64))) as u64; rsp = ((rsp).wrapping_add(0x38u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29294<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdi = rdx; rcx = ((rcx).wrapping_add(0x18u64)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp); r9 = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 7; continue; } rcx = (crate::simd_ops::ru64(rbx)) as u64; r10 = 0; fl_l = 0; fl_r = 0; r8 = ((r10 as u32)) as u32 as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 2; continue; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } bb = 3; continue; }
            2 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 7; continue; } bb = 3; continue; }
            3 => { fl_l = (crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } fl_l = (crate::simd_ops::lk_and_u32(rax.wrapping_add(P3), (0xfff00000u32) as u32)) as u32 as u64; fl_r = fl_l; crate::simd_ops::wu64(rax.wrapping_add(P4), r10); bb = 4; continue; }
            4 => { rcx = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; rax = rcx; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; bb = 5; continue; }
            5 => { r8 = (((r8 as u32)).wrapping_add(1)) as u32 as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 6; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rax = (crate::simd_ops::ru64(rax.wrapping_add(r8.wrapping_mul(8)))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 3; continue; }
            6 => { fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } bb = 7; continue; }
            7 => { rcx = ((rbx.wrapping_add(P6)) as u64) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P7)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P8)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru32(P9)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 8; continue; } rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P10))) as u64; rdx = 0xffffffff; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P11)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29344<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; r9 = ((rdx as u32)) as u32 as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } r10 = ((crate::simd_ops::ru8(r8)) as u32) as u32 as u64; rcx = 0; fl_l = 0; fl_r = 0; fl_l = (((r9 as u8)).wrapping_add(0x4au8)) as u8 as u64; fl_r = fl_l; r9 = (r9 & !0xffu64) | ((((fl_l as u8)) as u8 as u64)); bb = 1; continue; }
            1 => { rdx = ((rcx.wrapping_add(P1)) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(r8))) as u8 as u64; fl_r = ((r9 as u8)) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } rcx = ((rdx as u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = ((r10 as u32)) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 1; continue; } bb = 2; continue; }
            2 => { rax = 0; fl_l = 0; fl_r = 0; return rax; }
            3 => { rax = ((r10.wrapping_add(P2)) as u32) as u32 as u64; rdx = 0xfffffffc; rax = (((rax) & (rdx))) as u64; fl_l = ((rax).wrapping_add(r8)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(4))) as u64) as u64; return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29354<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    r8 = rcx;
    fl_l = ((rcx).wrapping_add(0xb8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29360<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u32) as u32 as u64;
    rax = (rax & !0xffu64) | (((((rax as u8)).wrapping_shr(1u32)) as u8 as u64));
    fl_l = ((((rax as u8)) & (0x1u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29408<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rbx = rcx;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32); rcx = 0; fl_l = 0; fl_r = 0; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x0u32) as u32); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29421<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rdx;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rcx = ((rbx.wrapping_add(P1)) as u64) as u64;
    rdx = rdi;
    rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp);
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29425<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64; return rax; }
    rax = (0xffffffffffffffffu64) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29465<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = P1;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    rax = P3;
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    rax = P5;
    crate::simd_ops::wu64(rcx.wrapping_add(P6), rax);
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P7, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29466<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xf0; rcx = rdi; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29514<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = rcx;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29552<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx, (0x0u32) as u32);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), (0xffffffffffffffffu64) as u64);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29573<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 15>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), r8); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = ((rsp).wrapping_sub(0x60u64)) as u64; rdx = (crate::simd_ops::ru32(rdx.wrapping_add(0x1u64))) as u32 as u64; rax = rcx.wrapping_add(0x8); rcx = rax; crate::simd_ops::wu32(rsp.wrapping_add(0x20u64), ((rdx as u32)) as u32); rdi = r9; crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rbp = r8; r13 = 0x1; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } rax = ((r13.wrapping_add(0x37u64)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x98u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x40u64), r15); r15 = (crate::simd_ops::ru32(rax)) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x58u64), rsi); bb = 2; continue; }
            2 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 7; continue; } rcx = ((rax.wrapping_add(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) != ((fl_r as u32))) { bb = 2; continue; } rsi = rdi.wrapping_add(0x8); bb = 3; continue; }
            3 => { rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 8; continue; } rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 6; continue; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 5; continue; }
            5 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 6; continue; }
            6 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } fl_l = (crate::simd_ops::atomic_add_u32(rbx.wrapping_add(0x14u64), 1)) as u32 as u64; bb = 9; continue; }
            7 => { rsi = rdi.wrapping_add(0x8); rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 3; continue; } rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            8 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 9; continue; }
            9 => { rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) < 0) { bb = 12; continue; } bb = 10; continue; }
            10 => { rcx = ((rax.wrapping_sub(0x1u64)) as u32) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; let cx_src: u32 = (rcx as u32); let (cx_ok, cx_old) = crate::simd_ops::atomic_cmpxchg_u32(rdi, fl_l as u32, cx_src); fl_r = cx_old as u64; if !cx_ok { rax = ((fl_r as u32)) as u32 as u64; } if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } rax = (crate::simd_ops::ru32(rdi)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) >= 0) { bb = 10; continue; } rcx = rsi; bb = 13; continue; }
            11 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 14; continue; } rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x30u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((rax as u32)) as u32 as u64; if (((((fl_l as u32)) & ((fl_r as u32))) as i32) <= 0) { bb = 14; continue; } rcx = (crate::simd_ops::ru64(rdi.wrapping_add(0x38u64))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            12 => { rcx = rdi.wrapping_add(0x8); bb = 13; continue; }
            13 => { rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 14; continue; }
            14 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } rcx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), ((r14 as u8)) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rbp = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r15 as u32)) as u32); rdx = rax; rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } r12 = (((r13 as u8)) as u32) as u32 as u64; bb = 22; continue; }
            15 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x0u32) as u32); rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 16; continue; }
            16 => { r14 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 17; continue; }
            17 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 19; continue; }
            19 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 20; continue; }
            20 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } fl_l = (crate::simd_ops::atomic_add_u32(rbx.wrapping_add(0x14u64), 1)) as u32 as u64; bb = 22; continue; }
            21 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 22; continue; }
            22 => { fl_l = (crate::simd_ops::atomic_add_u32(rdi, 1)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if (((fl_l as u32)) != 0) { bb = 23; continue; } crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x0u32) as u32); bb = 23; continue; }
            23 => { rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } rcx = rsp.wrapping_add(0x80); r13 = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            24 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            25 => { rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 26; continue; }
            26 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 27; continue; } rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((rax as u32)) as u32); r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } crate::simd_ops::wu64(r8, rbx); bb = 27; continue; }
            27 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 28; continue; }
            28 => { rax = ((r13 as u32)) as u32 as u64; bb = 29; continue; }
            29 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            30 => { crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r12); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 26; continue; } rcx = rsp.wrapping_add(0x80); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), r14); rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); rcx = rdi; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); r14 = (r14 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; r12 = (r12 & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; crate::simd_ops::wu8(rsp.wrapping_add(0x88u64), ((r14 as u8)) as u8); rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); rbx = rax; rbp = 0xffffffff; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 17; continue; } r8 = rsp.wrapping_add(0x88); crate::simd_ops::wu32(rax.wrapping_add(0x10u64), ((r15 as u32)) as u32); rdx = rax; rcx = rdi.wrapping_add(0x40); rax = crate::simd_ops::call_stk(P7, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 15; continue; } r12 = (((r13 as u8)) as u32) as u32 as u64; bb = 22; continue; }
            31 => { crate::simd_ops::wu32(rbx.wrapping_add(0x10u64), (0x0u32) as u32); rax = ((rbp as u32)) as u32 as u64; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 16; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 32; continue; }
            32 => { r14 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x88u64))) as u32) as u32 as u64; rbx = 0; fl_l = 0; fl_r = 0; bb = 33; continue; }
            33 => { fl_l = ((r14 as u8)) as u8 as u64; fl_r = ((r14 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 22; continue; } rcx = (crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 21; continue; } rbx = (crate::simd_ops::ru64(rdi.wrapping_add(0x40u64))) as u64; rdx = 0; fl_l = 0; fl_r = 0; rax = ((r15 as u32)) as u32 as u64; let d = ((rcx as u32)) as u32 as u64; let n = ((rdx as u32 as u64) << 32) | (rax as u32 as u64); if d != 0 { rax = (n / d) as u32 as u64; rdx = (n % d) as u32 as u64; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(rdx.wrapping_mul(8)))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } bb = 34; continue; }
            34 => { fl_l = (crate::simd_ops::ru32(rbx.wrapping_add(0x10u64))) as u32 as u64; fl_r = ((r15 as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(0x8u64))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 18; continue; } bb = 35; continue; }
            35 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 36; continue; }
            36 => { fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } fl_l = (crate::simd_ops::atomic_add_u32(rbx.wrapping_add(0x14u64), 1)) as u32 as u64; bb = 22; continue; }
            37 => { rbx = 0; fl_l = 0; fl_r = 0; bb = 38; continue; }
            38 => { fl_l = (crate::simd_ops::atomic_add_u32(rdi, 1)) as u32 as u64; r14 = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; if (((fl_l as u32)) != 0) { bb = 23; continue; } crate::simd_ops::wu32(rdi.wrapping_add(0x4u64), (0x0u32) as u32); bb = 39; continue; }
            39 => { rcx = rsi; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P2)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 24; continue; } rcx = rsp.wrapping_add(0x80); r13 = ((rbx.wrapping_add(0x34u64)) as u32) as u32 as u64; rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); bb = 28; continue; }
            40 => { fl_l = ((r12 as u8)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 25; continue; } r8 = (crate::simd_ops::ru32(rsp.wrapping_add(0x20u64))) as u32 as u64; rcx = rbx; rdx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = crate::simd_ops::call_stk(P10, rax, rcx, rdx, r8, r9, rsp); r13 = ((rax as u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 25; continue; } let xa_src: u32 = (rbp as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rbx.wrapping_add(0x14u64), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rbp = (xa_old) as u32 as u64; fl_l = ((rbp as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 25; continue; } rax = (crate::simd_ops::ru64(rbx)) as u64; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(0x18u64))) as u64, rax, rcx, rdx, r8, r9, rsp); rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); bb = 27; continue; }
            41 => { rcx = rsp.wrapping_add(0x80); rax = crate::simd_ops::call_stk(P8, rax, rcx, rdx, r8, r9, rsp); fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 27; continue; } rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; bb = 42; continue; }
            42 => { rcx = (crate::simd_ops::ru32(rbp.wrapping_add(0x58u64))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rbp.wrapping_add(0x5cu64))) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 27; continue; } rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((rcx as u32)) as u32 as u64; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x50u64))) as u64; crate::simd_ops::wu32(rbp.wrapping_add(0x58u64), ((rax as u32)) as u32); r8 = ((rcx.wrapping_add(rdx.wrapping_mul(8))) as u64) as u64; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 27; continue; } crate::simd_ops::wu64(r8, rbx); bb = 43; continue; }
            43 => { rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x28u64))) as u64; rax = crate::simd_ops::call_stk(P9, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; fl_l = ((r13 as u32)) as u32 as u64; fl_r = 0x5b; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 29; continue; } bb = 44; continue; }
            44 => { rax = ((r13 as u32)) as u32 as u64; bb = 45; continue; }
            45 => { r12 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x58u64))) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x98u64))) as u64; r15 = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rsp = ((rsp).wrapping_add(0x60u64)) as u64; r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29629<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rax = P1;
    crate::simd_ops::wu64(rbx.wrapping_add(P2), (0x0u64) as u64);
    crate::simd_ops::wu64(rbx, rax);
    rax = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29642<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xf0; rcx = rdi; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29742<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax.wrapping_add(P1))) as u32 as u64; return rax; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29744<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = rdx;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29748<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0xffffffffffffffa8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29778<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_r = (0x0u64) as u64;
    rax = (rax & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29781<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29791<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rdx = 0x1;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29814<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    r9 = (((rdx as u16)) as u32) as u32 as u64;
    r9 = ((r9).wrapping_shr(3u32)) as u64;
    r9 = ((r9).wrapping_add(crate::simd_ops::ru64(rcx.wrapping_add(P0)))) as u64;
    rax = (((rdx as u16)) as u32) as u32 as u64;
    rax = ((((rax as u32)) & (0x7u32))) as u32 as u64;
    rcx = (((crate::simd_ops::ru8(r9)) as i8 as i32) as u32) as u32 as u64;
    fl_l = ((((rcx as u32)) >> ((((rax as u32)) as u32 & 31))) & 1) as u64;
    rcx = (((rcx as u32)) | ((1u32) << ((((rax as u32)) as u32 & 31)))) as u32 as u64;
    crate::simd_ops::wu8(r9, ((rcx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29819<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    r8 = (((rdx as u16)) as u32) as u32 as u64;
    r8 = ((r8).wrapping_shr(3u32)) as u64;
    rcx = (((crate::simd_ops::ru8(r8.wrapping_add(rax))) as i8 as i32) as u32) as u32 as u64;
    rax = (((rdx as u16)) as u32) as u32 as u64;
    rax = ((((rax as u32)) & (0x7u32))) as u32 as u64;
    fl_l = ((((rcx as u32)) >> ((((rax as u32)) as u32 & 31))) & 1) as u64;
    rax = (rax & !0xffu64) | (((if ((fl_l as u32) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29822<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x0u32) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29823<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = P0;
    crate::simd_ops::wu64(rcx, rax);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((rax as u32)) as u32);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P3), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29831<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    crate::simd_ops::wu32(rcx.wrapping_add(P1), ((r8 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29835<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = P0;
    crate::simd_ops::wu32(rcx.wrapping_add(P1), (0x0u32) as u32);
    crate::simd_ops::wu64(rcx, rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29846<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::set_xmm_ret(c_x0); return c_rax; }
    xmm0 = 0u128;
    crate::simd_ops::set_xmm_ret(xmm0); return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29853<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x40u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_29855<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    rax = 0x2;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30049<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = ((crate::simd_ops::ru8(rax.wrapping_add(P1))) as u32) as u32 as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30063<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0xc8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30071<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l) = (crate::simd_ops::get_rax_in(), 0u64);
    rax = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64;
    fl_l = (((rax as u32)).wrapping_add(1)) as u32 as u64;
    rax = ((fl_l as u32)) as u32 as u64;
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rax as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30142<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P2), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P4), rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P5), rax);
    crate::simd_ops::wu32(rcx.wrapping_add(P6), ((rax as u32)) as u32);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30209<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0xf0u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30254<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 0>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    rax = r8;
    xmm0 = (r9) as u128;
    r9 = ((crate::simd_ops::ru8(rsp.wrapping_add(0x28u64))) as u32) as u32 as u64;
    rcx = rdx;
    r8 = ((xmm0 as u64)) as u64;
    rdx = rax;
    let _ = &stack; return crate::simd_ops::tail_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30308<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = 0;
    fl_l = 0; fl_r = 0;
    rax = ((rcx.wrapping_add(P0)) as u64) as u64;
    fl_l = rcx; fl_r = rcx;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rdx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30345<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64;
    rax = (rax & !0xffu64) | (((if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30363<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), ((rdx as u8)) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30380<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0xa8; rcx = rdi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30385<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0xa8;
    rcx = 0x2;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = P2; crate::simd_ops::wu64(rbx, rax); rax = P3; crate::simd_ops::wu64(rbx.wrapping_add(P4), rax); rax = rbx; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30403<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = P1; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rcx = rbx;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P3, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30417<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rdi = rdx;
    rcx = (crate::simd_ops::ru64(rdx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = rbx; crate::simd_ops::wu64(rbx.wrapping_add(P0), rdi); rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = 0x1; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = 0x2;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30419<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30421<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = rbx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rbx.wrapping_add(P0), (0x0u64) as u64); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30459<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    rax = (rax & !0xffu64) | (((if (((fl_l as u32)) != ((fl_r as u32))) { 1u8 } else { 0u8 }) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30461<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x50; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30467<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = ((rcx).wrapping_add(0x118u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30476<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; crate::simd_ops::wu64(rcx.wrapping_add(P0), r8); rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; rbx = rcx; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rdx = rdi; rcx = (crate::simd_ops::ru64(P2)) as u64; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { crate::simd_ops::wu64(rbx.wrapping_add(P1), rdi); rax = 0x1; bb = 3; continue; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30499<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; r8 = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; rsi = rcx; rdx = (crate::simd_ops::ru64(rdx.wrapping_add(P2))) as u64; r10 = (crate::simd_ops::ru64(rdi.wrapping_add(P3))) as u64; r11 = (crate::simd_ops::ru32(rdi.wrapping_add(P4))) as u32 as u64; r8 = ((r8.wrapping_add(r8.wrapping_mul(2))) as u64) as u64; r8 = ((r8).wrapping_shl(4u32)) as u64; r8 = ((r8).wrapping_add(rax)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { fl_l = rdx; fl_r = (crate::simd_ops::ru64(rax)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } fl_l = r10; fl_r = (crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 2; continue; } fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rax.wrapping_add(P3))) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 3; continue; } bb = 2; continue; }
            2 => { rax = ((rax).wrapping_add(0x30u64)) as u64; fl_l = rax; fl_r = r8; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rcx = 0; fl_l = 0; fl_r = 0; rbx = ((rax.wrapping_add(P0)) as u64) as u64; fl_l = rax; fl_r = r8; rbx = (if (((fl_l as u64)) == ((fl_r as u64))) { rcx } else { rbx }) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } rbx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 6; continue; } bb = 4; continue; }
            4 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 5; continue; } rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 4; continue; } bb = 6; continue; }
            6 => { rbx = (crate::simd_ops::ru64(rsi.wrapping_add(P7))) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 9; continue; } bb = 7; continue; }
            7 => { fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; fl_r = rdi; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } rdx = 0; fl_l = 0; fl_r = 0; rcx = rbx; rax = crate::simd_ops::call_stk(P6, rax, rcx, rdx, r8, r9, rsp); bb = 8; continue; }
            8 => { rbx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rbx; fl_r = rbx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 7; continue; } bb = 9; continue; }
            9 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30566<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rdx; rdi = rcx; let _ = crate::simd_ops::atomic_add_u32(rcx.wrapping_add(P0), (1) as u32); rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P2))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } fl_l = (crate::simd_ops::ru8(rbx.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 3; continue; } rcx = (crate::simd_ops::ru32(rbx)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 1; continue; } fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rdx = 0; fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = ((rdi.wrapping_add(P4)) as u64) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rcx = ((rdi.wrapping_add(P6)) as u64) as u64; rax = 0xffffffff; let xa_src: u32 = (rax as u32); let xa_old: u32 = crate::simd_ops::atomic_xadd_u32(rcx.wrapping_add(P7), xa_src); fl_l = xa_old.wrapping_add(xa_src) as u64; rax = (xa_old) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 4; continue; } rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P8))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            5 => { rdx = (rdx & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 2; continue; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30667<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((xmm1 as u32)) as u32);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30674<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); r9 = rax; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rdx = (crate::simd_ops::ru32(rbx.wrapping_add(P1))) as u32 as u64; rcx = (crate::simd_ops::ru64(rbx)) as u64; rdx = (((rdx as u32)).wrapping_sub(1)) as u32 as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u64) as u64; rdx = ((rcx.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } rdx = ((rdx).wrapping_sub(rax)) as u64; rcx = r9; rax = (0x2aaaaaaaaaaaaaabu64) as u64; let prod = (rax as i64 as i128).wrapping_mul((rdx) as i64 as i128) as u128; rax = prod as u64; rdx = (prod >> 64) as u64; rdx = ((((rdx) as i64).wrapping_shr(2u32) as u64)) as u64; rax = rdx; rax = ((rax).wrapping_shr(63u32)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; r8 = ((rdx.wrapping_add(rdx.wrapping_mul(2))) as u32) as u32 as u64; r8 = (((r8 as u32)).wrapping_shl(3u32)) as u32 as u64; rdx = ((r9.wrapping_add(P2)) as u64) as u64; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { crate::simd_ops::lk_sub_u32(rbx.wrapping_add(P1), (1) as u32); rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            2 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30692<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rdi = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x38; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30707<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = rdx;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30709<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x18u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P1, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30711<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rax = ((rcx.wrapping_add(P1)) as u64) as u64;
    rcx = 0;
    fl_l = 0; fl_r = 0;
    fl_l = rdx; fl_r = rdx;
    rax = (if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { rcx } else { rax }) as u64;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30716<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P2, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30741<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rcx = (crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64;
    rax = ((crate::simd_ops::ru8(rcx.wrapping_add(P2))) as u32) as u32 as u64;
    fl_l = ((((rax as u8)) & (0x7u8))) as u8 as u64; fl_r = fl_l;
    rax = (rax & !0xffu64) | ((((fl_l as u8)) as u8 as u64));
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30771<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rdx);
    crate::simd_ops::wu64(rcx, rax);
    crate::simd_ops::wu64(rcx.wrapping_add(P1), rax);
    rax = rcx;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30780<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru32(rax.wrapping_add(P0))) as u32 as u64; return rax; }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30799<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = rcx; fl_r = (0xa0u64) as u64;
    rcx = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30883<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = 0;
    fl_l = 0; fl_r = 0;
    crate::simd_ops::wu64(rcx.wrapping_add(P0), rax);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30911<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rax = 0x1;
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30914<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0xa0u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30942<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rdx);
    rdx = rsp.wrapping_add(0x38);
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_30994<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) <= ((fl_r as u32)))) { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P1), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); }
    fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = 0x0;
    if !((((fl_l as u32)) <= ((fl_r as u32)))) { fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P3))) as u32 as u64; fl_r = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32); }
    rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = (crate::simd_ops::ru64(rcx)) as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P5))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31043<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rax = P0;
    rdi = ((rdx as u32)) as u32 as u64;
    crate::simd_ops::wu64(rcx, rax);
    rbx = rcx;
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (0x0u64) as u64);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = 0x1; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rcx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), (0x0u64) as u64);
    rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x40; rcx = rbx; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31044<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0x40;
    rdi = ((rcx as u32)) as u32 as u64;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((rdi as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu64(rbx.wrapping_add(P4), (0x0u64) as u64); crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31053<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0x38;
    rdi = ((rcx as u32)) as u32 as u64;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((rdi as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31055<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x48; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31056<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdx = 0x48;
    rdi = ((rcx as u32)) as u32 as u64;
    rcx = ((rdx.wrapping_add(P0)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rdx = ((rdi as u32)) as u32 as u64; rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = P3; crate::simd_ops::wu64(rbx, rax); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31063<const P0: u64, const P1: u64, const P2: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
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
    rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rbx as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) == 0)) { rdx = 0x58; rcx = rdi; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); }
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = rdi;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31123<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu8(rcx.wrapping_add(P0), (0x0u8) as u8);
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31205<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    rcx = rdx;
    rdx = r8;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31214<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    fl_l = (crate::simd_ops::ru8(rcx.wrapping_add(P0))) as u8 as u64; fl_r = (0x0u8) as u8 as u64;
    rax = P1;
    crate::simd_ops::wu64(rcx, rax);
    rbx = rcx;
    rax = P2;
    crate::simd_ops::wu64(rcx.wrapping_add(P3), rax);
    if !((((fl_l as u8)) == ((fl_r as u8)))) { rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; rax = crate::simd_ops::call_stk(P5, rax, rcx, rdx, r8, r9, rsp); }
    rax = P6;
    rcx = rbx;
    crate::simd_ops::wu64(rbx.wrapping_add(P3), rax);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk(P7, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31257<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    crate::simd_ops::wu32(rcx.wrapping_add(P0), ((rdx as u32)) as u32);
    rax = 0x1;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31262<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = rdx; rsi = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = 0x7; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rbx); fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rbx = (0xffffffffffffffffu64) as u64; bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_add(1)) as u64; fl_l = (crate::simd_ops::ru16(rdi.wrapping_add(rbx.wrapping_mul(2)))) as u16 as u64; fl_r = (0x0u16) as u16 as u64; if (((fl_l as u16)) != ((fl_r as u16))) { bb = 2; continue; } rbx = ((rbx).wrapping_add(1)) as u64; rcx = 0x7; rdx = ((rbx.wrapping_add(rbx)) as u64) as u64; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); crate::simd_ops::wu64(rsi.wrapping_add(P0), rax); fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 3; continue; } rax = 0x34; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            3 => { r8 = rbx; rdx = rdi; rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = 0x1; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31272<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = ((rcx.wrapping_add(P0)) as u64) as u64; rdi = rcx; rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp); fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } fl_l = (crate::simd_ops::ru8(rdi.wrapping_add(P3))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rdi = (rdi & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 2; continue; }
            1 => { rdi = (rdi & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 2; continue; }
            2 => { rcx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P4)) as u64, rax, rcx, rdx, r8, r9, rsp); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = (((rdi as u8)) as u32) as u32 as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31292<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = ((rcx.wrapping_add(P0)) as u64) as u64;
    rdi = rcx;
    rcx = rbx;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P1)) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; fl_r = (0x0u64) as u64;
    rcx = rbx;
    rdi = (rdi & !0xffu64) | (((if (((fl_l as u64)) == ((fl_r as u64))) { 1u8 } else { 0u8 }) as u8 as u64));
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rax = (((rdi as u8)) as u32) as u32 as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31302<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { fl_l = (crate::simd_ops::ru8(rdx.wrapping_add(P1))) as u8 as u64; fl_r = (0x7u8) as u8 as u64; rax = 0x0; rax = (if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { rdx } else { rax }) as u64; return rax; }
    rax = 0;
    fl_l = 0; fl_r = 0;
    return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31307<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64) -> u64 {
    let (mut rax, mut r10, mut fl_l, mut fl_r) = (0u64, 0u64, 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx.wrapping_add(P0))) as u32 as u64; r9 = rcx; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x2u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 7; continue; } fl_l = ((r8 as u8)) as u8 as u64; fl_r = ((r8 as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 5; continue; } rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 5; continue; } rcx = 0; fl_l = 0; fl_r = 0; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 1; continue; }
            1 => { fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 2; continue; } rcx = rax; rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 1; continue; } return rax; }
            2 => { rdx = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(P1))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 3; continue; } crate::simd_ops::wu64(r9.wrapping_add(P1), rdx); bb = 4; continue; }
            3 => { crate::simd_ops::wu64(rcx, rdx); bb = 4; continue; }
            4 => { fl_l = rax; fl_r = (crate::simd_ops::ru64(r9.wrapping_add(P2))) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 11; continue; } crate::simd_ops::wu64(r9.wrapping_add(P2), rcx); return rax; }
            5 => { rax = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } rax = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 6; continue; } rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(r9.wrapping_add(P1), rcx); crate::simd_ops::wu64(r9.wrapping_add(P2), rcx); return rax; }
            6 => { crate::simd_ops::wu64(rcx.wrapping_add(P1), rax); return rax; }
            7 => { r10 = (crate::simd_ops::ru64(rcx.wrapping_add(P3))) as u64; rcx = 0; fl_l = 0; fl_r = 0; rax = r10; fl_l = r10; fl_r = r10; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 11; continue; } bb = 8; continue; }
            8 => { r8 = (crate::simd_ops::ru64(rax)) as u64; fl_l = rax; fl_r = rdx; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 9; continue; } rcx = rax; rax = r8; fl_l = r8; fl_r = r8; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 8; continue; } return rax; }
            9 => { fl_l = rax; fl_r = r10; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 10; continue; } crate::simd_ops::wu64(r9.wrapping_add(P3), r8); return rax; }
            10 => { crate::simd_ops::wu64(rcx, r8); bb = 11; continue; }
            11 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31324<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = ((rcx).wrapping_add(0x18u64)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P0)) as u64, rax, rcx, rdx, r8, r9, rsp);
    fl_l = (crate::simd_ops::ru32(rdi.wrapping_add(P1))) as u32 as u64; fl_r = 0x1;
    crate::simd_ops::wu32(rdi.wrapping_add(P1), ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32);
    if !((((fl_l as u32)) != ((fl_r as u32)))) { rcx = (crate::simd_ops::ru64(rdi.wrapping_add(P2))) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(P3)) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rcx = ((rdi.wrapping_add(P4)) as u64) as u64;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return crate::simd_ops::tail_stk((crate::simd_ops::ru64(P5)) as u64, rax, rcx, rdx, r8, r9, rsp);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31368<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 11>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { r11 = rsp; crate::simd_ops::wu64(r11.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(r11.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(r11.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(r11.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x50u64)) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x90u64))) as u64; r10 = ((rcx.wrapping_add(P0)) as u64) as u64; rax = (crate::simd_ops::ru64(rsp.wrapping_add(0x88u64))) as u64; xmm1 = (xmm2) as u128; xmm2 = ((crate::simd_ops::ru32(rcx.wrapping_add(P1))) as u32) as u128; r14 = r9; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x80u64))) as u64; rcx = ((rdx as u32)) as u32 as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x18u64), rbx); rbp = ((rdx as u32)) as u32 as u64; rdi = ((rsi.wrapping_add(rbp.wrapping_mul(4))) as u64) as u64; crate::simd_ops::wu64(r11.wrapping_sub(0x20u64), rdi); crate::simd_ops::wu64(r11.wrapping_sub(0x28u64), rsi); crate::simd_ops::wu64(r11.wrapping_sub(0x30u64), rax); crate::simd_ops::wu64(r11.wrapping_sub(0x38u64), r10); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(r14.wrapping_add(P3))) as u64; r9 = rdi; r8 = rsi; rdx = ((rbp as u32)) as u32 as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru8(r14.wrapping_add(P5))) as u32) as u32 as u64; r9 = rdi; r8 = rsi; crate::simd_ops::wu64(rsp.wrapping_add(0x88u64), rbx); rcx = ((rbp as u32)) as u32 as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x68u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x78u64))) as u64; rsp = ((rsp).wrapping_add(0x50u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return crate::simd_ops::tail_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31429<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    rax = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64;
    fl_l = ((rcx).wrapping_add(0x8u64)) as u64; fl_r = fl_l;
    rcx = ((fl_l as u64)) as u64;
    crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8);
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31525<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = ((rcx).wrapping_add(0x30u64)) as u64;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P1))) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rdx = (crate::simd_ops::ru64(rbx)) as u64;
    fl_l = rdx; fl_r = rdx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = (crate::simd_ops::ru64(P2)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64, rax, rcx, rdx, r8, r9, rsp); }
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31538<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; r8 = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdi = rdx; rsi = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; r8 = ((r8).wrapping_add(1)) as u64; rdx = rcx; rbx = rcx; rax = crate::simd_ops::call_stk(P1, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rbx)) as u64; r8 = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; r14 = ((rax.wrapping_add(r8.wrapping_mul(8))) as u64) as u64; rax = (crate::simd_ops::ru64(rdi)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = rax; rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); rax = (crate::simd_ops::ru64(rdi)) as u64; bb = 1; continue; }
            1 => { rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(P3), rax); rcx = (crate::simd_ops::ru64(rbx.wrapping_add(P0))) as u64; rax = (crate::simd_ops::ru64(rbx)) as u64; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rax = ((rax.wrapping_add(rcx.wrapping_mul(8))) as u64) as u64; rcx = (crate::simd_ops::ru64(rax.wrapping_add(P3))) as u64; rax = ((rax).wrapping_add(0xfffffffffffffff8u64)) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P4), ((rsi as u32)) as u32); rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31540<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbp); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rdi = r8; r14 = rdx; rsi = rcx; rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = r8; fl_r = rcx; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 4; continue; } rbp = rcx; fl_l = rbp; fl_r = r8; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; rax = (crate::simd_ops::ru64(rsi)) as u64; rbx = ((rcx.wrapping_add(P1)) as u64) as u64; rbx = ((rax.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_sub(0x8u64)) as u64; fl_l = rbp; fl_r = (0x1u64) as u64; rbp = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } bb = 3; continue; }
            3 => { rax = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; rax = ((rax).wrapping_shr(1u32)) as u64; fl_l = rdi; fl_r = rax; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 6; continue; } r8 = rdi; rdx = r14; bb = 5; continue; }
            4 => { fl_l = rdi; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(P3))) as u64; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 6; continue; } r8 = ((r8).wrapping_shr(2u32)) as u64; fl_l = ((r8).wrapping_add(rdi)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 5; continue; }
            5 => { rcx = rsi; rax = crate::simd_ops::call_stk(P4, rax, rcx, rdx, r8, r9, rsp); bb = 6; continue; }
            6 => { crate::simd_ops::wu64(rsi.wrapping_add(P0), rdi); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rbp = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x40u64))) as u64; rdi = (crate::simd_ops::ru64(rsp.wrapping_add(0x48u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31603<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let mut rax: u64 = crate::simd_ops::get_rax_in();
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = ((rsp).wrapping_sub(0x28u64)) as u64;
    rdx = (crate::simd_ops::ru64(rcx)) as u64;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P1))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rsp = ((rsp).wrapping_add(0x28u64)) as u64;
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31614<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbx); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rbx = rcx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 1; continue; } rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 1; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P3))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P4))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 3; continue; }
            3 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P5))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rdx = (crate::simd_ops::ru64(rbx.wrapping_add(P6))) as u64; fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 5; continue; } rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 5; continue; }
            5 => { rsp = ((rsp).wrapping_add(0x20u64)) as u64; rbx = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31671<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rcx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = rcx;
    rcx = (crate::simd_ops::ru64(P0)) as u64;
    rax = (crate::simd_ops::ru64(rcx)) as u64;
    r8 = 0;
    fl_l = 0; fl_r = 0;
    rdx = ((r8.wrapping_add(P1)) as u32) as u32 as u64;
    rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp);
    rbx = rax;
    crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax);
    fl_l = rax; fl_r = rax;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rcx = rax; rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp); rax = P4; crate::simd_ops::wu64(rbx, rax); } else { rbx = 0; fl_l = 0; fl_r = 0; }
    crate::simd_ops::wu64(rdi, rbx);
    rax = rdi;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31685<const P0: u64, const P1: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rbx);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rbx = rcx;
    rcx = (crate::simd_ops::ru64(rcx)) as u64;
    fl_l = rcx; fl_r = rcx;
    if !(((((fl_l as u64)) & ((fl_r as u64))) == 0)) { rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp); }
    crate::simd_ops::wu64(rbx, (0x0u64) as u64);
    crate::simd_ops::wu64(rbx.wrapping_add(P1), (0xffffffffffffffffu64) as u64);
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rbx = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31701<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 25>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rsp.wrapping_sub(0x37); rsp = ((rsp).wrapping_sub(0xa0u64)) as u64; rdi = rdx; r13 = rcx; rcx = rdi; rdx = rbp.wrapping_sub(0x9); rsi = r8; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_sub(0x19); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x19u64))) as u64; r9 = rbp.wrapping_add(0x77); rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x11u64))) as u64; r8 = rbp.wrapping_sub(0x19); rbx = ((rbx).wrapping_sub(crate::simd_ops::ru64(rbp.wrapping_sub(0x1u64)))) as u64; rdx = rbp.wrapping_sub(0x9); rax = ((rax).wrapping_sub(crate::simd_ops::ru64(rbp.wrapping_sub(0x9u64)))) as u64; rcx = rbp.wrapping_sub(0x29); xmm0 = (crate::simd_ops::ru128(rsi)) as u128; r14 = 0x1; xmm1 = (crate::simd_ops::ru128(rsi.wrapping_add(0x10u64))) as u128; rax = (((rax) as i64).wrapping_mul((rax) as i64) as u64) as u64; rbx = (((rbx) as i64).wrapping_mul((rbx) as i64) as u64) as u64; crate::simd_ops::wu128(rbp.wrapping_sub(0x29u64), (xmm0) as u128); crate::simd_ops::wu128(rbp.wrapping_sub(0x39u64), (xmm1) as u128); rbx = ((rbx).wrapping_add(rax)) as u64; r15 = rbx; crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), rbx); r15 = (((r15) as i64).wrapping_mul((crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as i64) as u64) as u64; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } rdx = rbp.wrapping_add(0x7); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r14 = 0x4; rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::lk_add_u64(rbp.wrapping_sub(0x29u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::lk_add_u64(rbp.wrapping_sub(0x21u64), (rax) as u64)) as u64; fl_r = fl_l; bb = 1; continue; }
            1 => { r9 = rbp.wrapping_add(0x77); crate::simd_ops::wu64(rbp.wrapping_add(0x77u64), rbx); r8 = rbp.wrapping_sub(0x19); rdx = rbp.wrapping_sub(0x9); rcx = rbp.wrapping_sub(0x39); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = r15; if (((fl_l as u64)) >= ((fl_r as u64))) { bb = 2; continue; } rax = (crate::simd_ops::ru32(rsi.wrapping_add(0x28u64))) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((rax as u32)) as i32) as u32) as u32 as u64; rax = ((((rax as u32)) as i32).wrapping_mul(((r14 as u32)) as i32) as u32) as u32 as u64; fl_l = (crate::simd_ops::ru64(rsi.wrapping_add(0x30u64))) as u64; fl_r = rax; if (((fl_l as u64)) <= ((fl_r as u64))) { bb = 11; continue; } rdx = rbp.wrapping_add(0x7); rcx = rsi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P3, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rax)) as u64; crate::simd_ops::lk_sub_u64(rbp.wrapping_sub(0x39u64), (rcx) as u64); rax = (crate::simd_ops::ru64(rax.wrapping_add(0x8u64))) as u64; fl_l = (crate::simd_ops::ru64(rbp.wrapping_sub(0x31u64))) as u64; fl_r = rax; crate::simd_ops::wu64(rbp.wrapping_sub(0x31u64), ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64); bb = 2; continue; }
            2 => { r9 = rbp.wrapping_sub(0x39); r8 = rbp.wrapping_sub(0x29); rdx = rbp.wrapping_sub(0x19); rcx = rbp.wrapping_sub(0x9); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } rdx = (crate::simd_ops::ru64(r13.wrapping_add(0x10u64))) as u64; rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P5, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 10; continue; } r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), (0xffff3fffu32) as u32); r14 = 0x3fff; crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r12); rdx = rbp.wrapping_add(0x7); crate::simd_ops::wu16(rbp.wrapping_sub(0x41u64), ((r14 as u16)) as u16); rcx = rdi; crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), r12); r15 = 0xffff; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r13 = P7; bb = 3; continue; }
            3 => { rdx = ((crate::simd_ops::ru16(rbp.wrapping_add(0xfu64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rcx = ((rdx as u32)) as u32 as u64; rax = ((((rdx as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_sub(0x2u16)) as u16 as u64)); crate::simd_ops::wu16(rbp.wrapping_sub(0x3fu64), ((r15 as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r8); rax = (((rax as u32)).wrapping_shr(17u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_add(crate::simd_ops::ru16(r13.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x33e1600u64)))) as u16 as u64)); crate::simd_ops::wu16(rbp.wrapping_sub(0x41u64), ((rdx as u16)) as u16); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 4; continue; } fl_l = ((rdx as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rdi.wrapping_add(0x8u64))) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 6; continue; } bb = 4; continue; }
            4 => { rdx = rbp.wrapping_add(0x17); rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x27); rbx = rax; rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; r9 = rbp.wrapping_sub(0x39); r8 = rbp.wrapping_sub(0x29); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 6; continue; } rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x41u64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rdx = ((rax as u32)) as u32 as u64; rdx = (((rdx as u32)).wrapping_shr(29u32)) as u32 as u64; fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x5; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; rdx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } fl_l = ((rdx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 5; continue; } rdx = ((crate::simd_ops::ru16(r8.wrapping_add(0x60u64))) as u32) as u32 as u64; rax = (((rax) & (r14))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x68u64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 10; continue; } fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(0x8u64))) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 10; continue; } bb = 5; continue; }
            5 => { rdx = rbp.wrapping_add(0x7); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), r12); rcx = rbp.wrapping_sub(0x49); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), (0xffff3fffu32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 3; continue; }
            6 => { rcx = rdi; bb = 7; continue; }
            7 => { rdx = rbp.wrapping_add(0x7); crate::simd_ops::wu64(rbp.wrapping_add(0x7u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0xfu64), (0xffff3fffu32) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = ((crate::simd_ops::ru16(rbp.wrapping_add(0xfu64))) as u32) as u32 as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_add(0x7u64))) as u64; rcx = ((rdx as u32)) as u32 as u64; rax = ((((rdx as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x3fu64), ((r15 as u16)) as u16); crate::simd_ops::wu64(rbp.wrapping_sub(0x49u64), r8); rax = (((rax as u32)).wrapping_shr(17u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; rcx = (((rcx as u32)).wrapping_sub((rax as u32))) as u32 as u64; rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_sub(crate::simd_ops::ru16(r13.wrapping_add(rcx.wrapping_mul(4)).wrapping_add(0x33e1568u64)))) as u16 as u64)); rdx = (rdx & !0xffffu64) | (((((rdx as u16)).wrapping_add(0x2u16)) as u16 as u64)); crate::simd_ops::wu16(rbp.wrapping_sub(0x41u64), ((rdx as u16)) as u16); fl_l = r8; fl_r = (crate::simd_ops::ru64(rdi)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 8; continue; } fl_l = ((rdx as u16)) as u16 as u64; fl_r = (crate::simd_ops::ru16(rdi.wrapping_add(0x8u64))) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 11; continue; } bb = 8; continue; }
            8 => { rdx = rbp.wrapping_add(0x27); rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rbp.wrapping_add(0x17); rbx = rax; rcx = rbp.wrapping_sub(0x49); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = rax; r9 = rbp.wrapping_sub(0x39); r8 = rbp.wrapping_sub(0x29); rdx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = rax; fl_r = (crate::simd_ops::ru64(rsi.wrapping_add(0x20u64))) as u64; if (((fl_l as u64)) > ((fl_r as u64))) { bb = 11; continue; } r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x49u64))) as u64; rdx = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x41u64))) as u32) as u32 as u64; rcx = (crate::simd_ops::ru64(r8.wrapping_add(0x20u64))) as u64; rcx = ((rcx).wrapping_add(0x10u64)) as u64; rax = (((crate::simd_ops::ru32(rcx.wrapping_add(0x4u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x4u64))) as u32 as u64; rcx = ((rax as u32)) as u32 as u64; rcx = (((rcx as u32)).wrapping_shr(29u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 9; continue; } rdx = ((crate::simd_ops::ru16(r8.wrapping_add(0x60u64))) as u32) as u32 as u64; rax = (((rax) & (r14))) as u64; rax = ((rax).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(r8)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 10; continue; } fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = r12; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 10; continue; } fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x68u64))) as u16 as u64; fl_r = ((r15 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 10; continue; } fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(0x8u64))) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 10; continue; } bb = 9; continue; }
            9 => { rcx = rbp.wrapping_sub(0x49); bb = 7; continue; }
            10 => { rax = (rax & !0xffu64) | (((0u8) as u8 as u64)); fl_l = 0; fl_r = 0; bb = 12; continue; }
            11 => { rax = (rax & !0xffu64) | (((0x1u8) as u8 as u64)); bb = 12; continue; }
            12 => { r11 = rsp.wrapping_add(0xa0); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x38u64))) as u64; rdi = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31744<const P0: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    crate::simd_ops::forward_incoming_to_outgoing();
    let (mut rax, mut fl_l, mut fl_r) = (crate::simd_ops::get_rax_in(), 0u64, 0u64);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = (crate::simd_ops::ru32(rdx)) as u32 as u64; r9 = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx, ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x4u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x4u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x8u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x8u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0xcu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0xcu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x10u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x10u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x14u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x14u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x18u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x18u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x1cu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x1cu64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x20u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x20u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x24u64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x24u64), ((rax as u32)) as u32); rcx = (crate::simd_ops::ru32(rdx.wrapping_add(0x28u64))) as u32 as u64; fl_l = ((r9 as u32)) as u32 as u64; fl_r = ((r9 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 1; continue; } rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; rcx = (((rcx as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), ((rcx as u32)) as u32); rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } rax = (rax as u32 as i32 as i64) as u64; rdx = ((rdx).wrapping_add(0x2cu64)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; rcx = 0; fl_l = 0; fl_r = 0; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            1 => { fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) != 0) { bb = 2; continue; } r8 = 0; fl_l = 0; fl_r = 0; bb = 3; continue; }
            2 => { rax = (((crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as i32 as i64) as u64) as u64; r8 = rdx.wrapping_add(0x2c); fl_l = ((r8).wrapping_add(rax)) as u64; fl_r = fl_l; r8 = ((fl_l as u64)) as u64; bb = 3; continue; }
            3 => { rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = (crate::simd_ops::ru32(rdx.wrapping_add(0x2cu64))) as u32 as u64; rax = (((rax as u32)).swap_bytes()) as u32 as u64; crate::simd_ops::wu32(rdx.wrapping_add(0x2cu64), ((rax as u32)) as u32); fl_l = ((rcx as u32)) as u32 as u64; fl_r = ((rcx as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 4; continue; } rdx = r8; rcx = ((r9 as u32)) as u32 as u64; crate::simd_ops::replace_rax_in(rax); return crate::exe::call_va8(P0, rcx, rdx, r8, r9, arg5, arg6, arg7, arg8); }
            4 => { return rax; }
            _ => { return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31756<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P0))) as u32 as u64; fl_r = 0x1; rdi = rdx; rbx = rcx; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 3; continue; } rcx = (crate::simd_ops::ru64(rcx.wrapping_add(P1))) as u64; fl_l = (crate::simd_ops::ru32(rcx.wrapping_add(P2))) as u32 as u64; fl_r = 0x7fffffff; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 1; continue; } crate::simd_ops::wu32(rbx.wrapping_add(P0), (0x2u32) as u32); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x2u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            1 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) != 0) { bb = 2; continue; } rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P4))) as u64; bb = 2; continue; }
            2 => { xmm0 = ((crate::simd_ops::ru32(rbx.wrapping_add(P2))) as u32) as u128; r8 = ((rbx.wrapping_add(P5)) as u64) as u64; xmm1 = ((crate::simd_ops::ru32(rbx.wrapping_add(P6))) as u32) as u128; rdx = rsp.wrapping_add(0x30); rcx = ((rcx).wrapping_add(0x58u64)) as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x34u64), ((xmm1 as u32)) as u32); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rdx = rdi; rcx = rbx; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rbx.wrapping_add(P3), (0x2u32) as u32); rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            3 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu32(rcx.wrapping_add(P3), (0x2u32) as u32); rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31766<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64, const P5: u64, const P6: u64, const P7: u64, const P8: u64, const P9: u64, const P10: u64, const P11: u64, const P12: u64, const P13: u64, const P14: u64, const P15: u64, const P16: u64, const P17: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut r10, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r12, mut r13, mut r14, mut r15) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut xmm6: u128 = 0; let mut xmm7: u128 = 0;
    let mut stack = crate::simd_ops::LiftedFrame::<256, 85>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { rax = rsp; crate::simd_ops::wu64(rax.wrapping_add(0x10u64), rbx); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rbp); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r12); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r13); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r14); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, r15); rbp = rax.wrapping_sub(0x1a8); rsp = ((rsp).wrapping_sub(0x270u64)) as u64; crate::simd_ops::wu128(rax.wrapping_sub(0x48u64), (xmm6) as u128); crate::simd_ops::wu128(rax.wrapping_sub(0x58u64), (xmm7) as u128); rbx = rdx; rdi = rcx; r9 = (crate::simd_ops::ru64(rcx.wrapping_add(0x28u64))) as u64; r8 = rdx; rdx = (crate::simd_ops::ru64(rcx.wrapping_add(0x20u64))) as u64; rcx = rbp.wrapping_add(0xe0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P0, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r15 = (crate::simd_ops::ru64(rdi.wrapping_add(0x20u64))) as u64; xmm7 = ((crate::simd_ops::ru32(r15.wrapping_add(0x64u64))) as u32) as u128; rax = rdi.wrapping_add(0xd8); crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), rax); r12 = 0; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), r12); crate::simd_ops::wu32(rbp.wrapping_sub(0x10u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_sub(0x8u64), r12); crate::simd_ops::wu64(rbp, r12); crate::simd_ops::wu32(rbp.wrapping_add(0x8u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x10u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x18u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x20u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x30u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x38u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x40u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x48u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x50u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x58u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x60u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x68u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x70u64), r12); crate::simd_ops::wu64(rbp.wrapping_add(0x78u64), r12); crate::simd_ops::wu32(rbp.wrapping_add(0x80u64), (0xffffffffu32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0x88u64), r12); r9 = (crate::simd_ops::ru32(rdi.wrapping_add(0xd4u64))) as u32 as u64; r8 = rbx; rdx = rbp.wrapping_add(0x1b0); rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P1, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = ((r12 as u8)) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 1; continue; } rax = 0x6; crate::simd_ops::wu32(rdi.wrapping_add(0xd0u64), ((rax as u32)) as u32); bb = 18; continue; }
            1 => { rax = rdi.wrapping_add(0xa8); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rdi.wrapping_add(0x48); r8 = rdi.wrapping_add(0x68); rdx = rdi.wrapping_add(0x98); rcx = rbp.wrapping_add(0xe0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P2, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 2; continue; } rax = 0x2; crate::simd_ops::wu32(rdi.wrapping_add(0xd0u64), ((rax as u32)) as u32); bb = 18; continue; }
            2 => { rax = (crate::simd_ops::ru64(rdi.wrapping_add(0x98u64))) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x80u64), rax); rax = ((crate::simd_ops::ru16(rdi.wrapping_add(0xa0u64))) as u32) as u32 as u64; crate::simd_ops::wu16(rbp.wrapping_sub(0x78u64), ((rax as u16)) as u16); xmm1 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x70u64))) as u32) as u128; xmm2 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x90u64))) as u32) as u128; xmm2 = (xmm2 & !0xffff_ffffu128) | ((f32::from_bits(xmm2 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x6cu64)))).to_bits() as u128); xmm0 = ((crate::simd_ops::ru32(rdi.wrapping_add(0x8cu64))) as u32) as u128; xmm0 = (xmm0 & !0xffff_ffffu128) | ((f32::from_bits(xmm0 as u32) + f32::from_bits(crate::simd_ops::ru32(rdi.wrapping_add(0x68u64)))).to_bits() as u128); crate::simd_ops::wu32(rsp.wrapping_add(0x70u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x74u64), ((xmm2 as u32)) as u32); crate::simd_ops::wu32(rsp.wrapping_add(0x78u64), ((xmm1 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1b0u64), ((xmm0 as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0x1b4u64), ((xmm2 as u32)) as u32); xmm0 = (crate::simd_ops::ru128(P3)) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x60u64), (xmm0) as u128); r8 = rbp.wrapping_sub(0x60); rdx = rbp.wrapping_add(0x1b0); rcx = r15.wrapping_add(0x58); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P4, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((r12 as u32)) as u32); xmm6 = ((crate::simd_ops::ru32(P5)) as u32) as u128; crate::simd_ops::wu32(rsp.wrapping_add(0x30u64), ((xmm6 as u32)) as u32); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0xd4u64))) as u32 as u64; crate::simd_ops::wu32(rsp.wrapping_add(0x28u64), ((rax as u32)) as u32); rax = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = (crate::simd_ops::ru64(rdi.wrapping_add(0x28u64))) as u64; r8 = rsp.wrapping_add(0x70); rdx = rdi.wrapping_add(0x68); rcx = rbp.wrapping_add(0x100); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P6, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = rbp.wrapping_sub(0x80); r8 = (crate::simd_ops::ru32(rdi.wrapping_add(0xd4u64))) as u32 as u64; rdx = rbp.wrapping_add(0x1b0); rcx = rbp.wrapping_add(0x100); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P7, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 3; continue; } rax = 0x6; crate::simd_ops::wu32(rdi.wrapping_add(0xd0u64), ((rax as u32)) as u32); bb = 18; continue; }
            3 => { rdx = (crate::simd_ops::ru32(rdi.wrapping_add(0xd4u64))) as u32 as u64; r13 = ((rdx as u32)) as u32 as u64; r13 = ((((r13 as u32)) & (0x1cu32))) as u32 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0xacu64), (0x0u64) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xb4u64), (0x0u32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xc4u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xc8u64), (0x7f7fffffu32) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xccu64), (0x7f7fffffu32) as u32); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x80u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_add(0x90u64), (xmm0) as u128); xmm0 = ((crate::simd_ops::ru64(rdi.wrapping_add(0x68u64))) as u64) as u128; crate::simd_ops::wu64(rbp.wrapping_add(0xa0u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rdi.wrapping_add(0x70u64))) as u32 as u64; crate::simd_ops::wu32(rbp.wrapping_add(0xa8u64), ((rax as u32)) as u32); crate::simd_ops::wu32(rbp.wrapping_add(0xd0u64), (0x3f800000u32) as u32); crate::simd_ops::wu64(rbp.wrapping_add(0xb8u64), ((xmm0 as u64)) as u64); crate::simd_ops::wu32(rbp.wrapping_add(0xc0u64), ((rax as u32)) as u32); r12 = (((crate::simd_ops::ru32(rdi.wrapping_add(0x48u64))) as i32 as i64) as u64) as u64; r11 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x5cu64))) as u64; rsi = (((crate::simd_ops::ru32(rbp.wrapping_sub(0x60u64))) as i32 as i64) as u64) as u64; fl_l = ((rsi as u32)) as u32 as u64; fl_r = ((r12 as u32)) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } fl_l = ((r11 as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x58u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x54u64))) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { bb = 6; continue; } rax = rbp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x70); r8 = rbp.wrapping_sub(0x20); rcx = rbp.wrapping_add(0x1b0); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P8, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rax)) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { bb = 5; continue; } bb = 4; continue; }
            4 => { rax = 0x6; crate::simd_ops::wu32(rdi.wrapping_add(0xd0u64), ((rax as u32)) as u32); bb = 18; continue; }
            5 => { xmm0 = ((crate::simd_ops::ru64(rsp.wrapping_add(0x70u64))) as u64) as u128; crate::simd_ops::wu64(rdi.wrapping_add(0x80u64), ((xmm0 as u64)) as u64); rax = (crate::simd_ops::ru32(rsp.wrapping_add(0x78u64))) as u32 as u64; crate::simd_ops::wu32(rdi.wrapping_add(0x88u64), ((rax as u32)) as u32); xmm0 = (crate::simd_ops::ru128(rdi.wrapping_add(0x48u64))) as u128; crate::simd_ops::wu128(rdi.wrapping_add(0x58u64), (xmm0) as u128); rcx = rdi.wrapping_add(0xb0); rdx = rdi.wrapping_add(0x98); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P9, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); crate::simd_ops::wu32(rdi.wrapping_add(0xd0u64), (0x9u32) as u32); bb = 18; continue; }
            6 => { rbx = (((crate::simd_ops::ru32(rdi.wrapping_add(0x4cu64))) as i32 as i64) as u64) as u64; rdx = ((r11 as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rdx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rbx as u32) } else { (rdx as u32) }) as u32 as u64; r10 = ((rsi as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; r10 = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (r12 as u32) } else { (r10 as u32) }) as u32 as u64; rax = ((rbx as u32)) as u32 as u64; fl_l = ((rbx as u32)) as u32 as u64; fl_r = ((r11 as u32)) as u32 as u64; rax = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (r11 as u32) } else { (rax as u32) }) as u32 as u64; rcx = ((r12 as u32)) as u32 as u64; fl_l = ((r12 as u32)) as u32 as u64; fl_r = ((rsi as u32)) as u32 as u64; rcx = (if ((((fl_l as u32)) as i32) < (((fl_r as u32)) as i32)) { (rsi as u32) } else { (rcx as u32) }) as u32 as u64; r9 = (((crate::simd_ops::ru32(r15.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; rax = (((rax as u32)).wrapping_add(1)) as u32 as u64; r8 = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((((rdx as u32)) as i32 as i64) as u64) as u64; r8 = ((r8).wrapping_sub(rax)) as u64; rax = ((rcx.wrapping_add(0x1u64)) as u32) as u32 as u64; rdx = ((((rax as u32)) as i32 as i64) as u64) as u64; rax = ((((r10 as u32)) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_sub(rax)) as u64; rcx = r8; rcx = (((rcx) as i64).wrapping_mul((r9) as i64) as u64) as u64; rax = r9; rax = (((rax) as i64).wrapping_mul((rdx) as i64) as u64) as u64; fl_l = rax; fl_r = rcx; r8 = (if ((((fl_l as u64)) as i64) >= (((fl_r as u64)) as i64)) { rdx } else { r8 }) as u64; r8 = (((r8) as i64).wrapping_mul((r9) as i64) as u64) as u64; fl_l = r8; fl_r = (0x100000u64) as u64; if ((((fl_l as u64)) as i64) < (((fl_r as u64)) as i64)) { bb = 7; continue; } crate::simd_ops::wu32(rdi.wrapping_add(0xd0u64), (0x7u32) as u32); bb = 18; continue; }
            7 => { rdx = (((crate::simd_ops::ru32(r15.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; r9 = rbx; r9 = (((r9) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(rdi.wrapping_add(0x54u64))) as i32 as i64) as u64) as u64; r9 = ((r9).wrapping_add(rax)) as u64; r8 = r12; r8 = (((r8) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(rdi.wrapping_add(0x50u64))) as i32 as i64) as u64) as u64; r8 = ((r8).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x50u64), r8); crate::simd_ops::wu64(rbp.wrapping_sub(0x48u64), r9); rdx = (((crate::simd_ops::ru32(r15.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; r10 = ((((r11 as u32)) as i32 as i64) as u64) as u64; r10 = (((r10) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(rbp.wrapping_sub(0x54u64))) as i32 as i64) as u64) as u64; r10 = ((r10).wrapping_add(rax)) as u64; rdx = (((rdx) as i64).wrapping_mul((rsi) as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(rbp.wrapping_sub(0x58u64))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x30u64), rdx); crate::simd_ops::wu64(rbp.wrapping_sub(0x28u64), r10); r10 = ((r10).wrapping_sub(r9)) as u64; rdx = ((rdx).wrapping_sub(r8)) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x40u64), rdx); crate::simd_ops::wu64(rbp.wrapping_sub(0x38u64), r10); r10 = (((r10) as i64).wrapping_mul((r10) as i64) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; r10 = ((r10).wrapping_add(rdx)) as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x1c8u64), r10); r9 = rbp.wrapping_sub(0x30); r8 = rbp.wrapping_sub(0x50); rdx = rbp.wrapping_sub(0x80); rcx = rbp.wrapping_add(0x100); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P10, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); r9 = ((crate::simd_ops::ru16(rbp.wrapping_sub(0x78u64))) as u32) as u32 as u64; rcx = ((rax.wrapping_add(r9.wrapping_mul(2))) as u32) as u32 as u64; r9 = (((r9 as u32)).wrapping_add((rcx as u32))) as u32 as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x80u64))) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), rax); r10 = (((r9 as u16)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x68u64), ((r10 as u16)) as u16); r12 = 0xffff; crate::simd_ops::wu16(rsp.wrapping_add(0x6au64), ((r12 as u16)) as u16); rdx = (((crate::simd_ops::ru32(r15.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; r8 = (((crate::simd_ops::ru32(rax.wrapping_add(0x3cu64))) as i32 as i64) as u64) as u64; r8 = (((r8) as i64).wrapping_mul((rdx) as i64) as u64) as u64; rcx = (((crate::simd_ops::ru32(rax.wrapping_add(0x38u64))) as i32 as i64) as u64) as u64; rcx = (((rcx) as i64).wrapping_mul((rdx) as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rcx); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), r8); rsi = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; rcx = (((crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(r9.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), (0x0u64) as u64); r14 = 0x3fff; crate::simd_ops::wu32(rsp.wrapping_add(0x58u64), (0xffff3fffu32) as u32); xmm0 = ((crate::simd_ops::ru32(P11)) as u32) as u128; crate::simd_ops::wu32(rbp.wrapping_add(0x1c0u64), ((xmm0 as u32)) as u32); rax = rbp.wrapping_add(0x1c8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rbp.wrapping_sub(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rbp.wrapping_sub(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = ((r10 as u32)) as u32 as u64; r8 = rsi; rdx = rbp.wrapping_sub(0x70); rcx = rbp.wrapping_add(0x100); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } bb = 8; continue; }
            8 => { r8 = ((rbx as u32)) as u32 as u64; r8 = (((r8 as u32)).wrapping_shr(29u32)) as u32 as u64; rcx = ((r8 as u32)) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x5; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 16; continue; } fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; rcx = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 10; continue; } rbx = ((((rbx as u32)) & ((r14 as u32)))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = 0x1; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 11; continue; } rcx = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; rax = ((crate::simd_ops::ru16(rcx.wrapping_add(0x60u64))) as u32) as u32 as u64; rdx = ((rbx as u32)) as u32 as u64; rdx = ((rdx).wrapping_shl(4u32)) as u64; rax = ((rax).wrapping_add(rcx)) as u64; rdx = ((rdx).wrapping_add(rax)) as u64; xmm0 = (crate::simd_ops::ru128(rdx)) as u128; crate::simd_ops::wu128(rsp.wrapping_add(0x50u64), (xmm0) as u128); rax = (crate::simd_ops::ru64(rdx)) as u64; fl_l = rax; fl_r = rax; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 16; continue; } fl_l = (crate::simd_ops::ru64(rax.wrapping_add(0x20u64))) as u64; fl_r = (0x0u64) as u64; if (((fl_l as u64)) == ((fl_r as u64))) { bb = 16; continue; } fl_l = (crate::simd_ops::ru16(rax.wrapping_add(0x68u64))) as u16 as u64; fl_r = ((r12 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 16; continue; } fl_l = (crate::simd_ops::ru16(rdx.wrapping_add(0x8u64))) as u16 as u64; fl_r = ((r14 as u16)) as u16 as u64; if (((fl_l as u16)) == ((fl_r as u16))) { bb = 16; continue; } r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x5; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 12; continue; } fl_l = ((r8 as u32)) as u32 as u64; fl_r = 0x4; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 9; continue; } rcx = (((crate::simd_ops::ru32(r15.wrapping_add(0x74u64))) as i32 as i64) as u64) as u64; rdx = (((crate::simd_ops::ru32(r10.wrapping_add(0x3cu64))) as i32 as i64) as u64) as u64; rdx = (((rdx) as i64).wrapping_mul((rcx) as i64) as u64) as u64; rax = (((crate::simd_ops::ru32(r10.wrapping_add(0x38u64))) as i32 as i64) as u64) as u64; rax = (((rax) as i64).wrapping_mul((rcx) as i64) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x70u64), rax); crate::simd_ops::wu64(rbp.wrapping_sub(0x68u64), rdx); bb = 9; continue; }
            9 => { rsi = (crate::simd_ops::ru64(r10.wrapping_add(0x20u64))) as u64; bb = 12; continue; }
            10 => { fl_l = ((((rbx as u32)) & ((r14 as u32)))) as u32 as u64; fl_r = fl_l; rbx = ((fl_l as u32)) as u32 as u64; bb = 11; continue; }
            11 => { r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x60u64))) as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x5au64), ((r12 as u16)) as u16); crate::simd_ops::wu16(rsp.wrapping_add(0x58u64), ((rbx as u16)) as u16); crate::simd_ops::wu64(rsp.wrapping_add(0x50u64), r10); bb = 12; continue; }
            12 => { fl_l = ((r13 as u32)) as u32 as u64; fl_r = ((r13 as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { bb = 13; continue; } rax = rbp.wrapping_add(0x1c0); crate::simd_ops::wu64(rsp.wrapping_add(0x48u64), rax); crate::simd_ops::wu8(rsp.wrapping_add(0x40u64), (0x0u8) as u8); crate::simd_ops::wu32(rsp.wrapping_add(0x38u64), ((xmm6 as u32)) as u32); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rsi); rax = rbp.wrapping_sub(0x70); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rbp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x50); xmm2 = (xmm7) as u128; rdx = rbp.wrapping_add(0x1b0); rcx = rbp.wrapping_add(0x100); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P13, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = (crate::simd_ops::ru8(rbp.wrapping_add(0x1b0u64))) as u8 as u64; fl_r = (0x0u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { bb = 4; continue; } r10 = (crate::simd_ops::ru64(rsp.wrapping_add(0x50u64))) as u64; bb = 13; continue; }
            13 => { rbx = ((crate::simd_ops::ru16(rsp.wrapping_add(0x58u64))) as u32) as u32 as u64; rax = ((((rbx as u32)) as i32).wrapping_mul((0xaaabu32) as i32) as u32) as u32 as u64; rax = (((rax as u32)).wrapping_shr(17u32)) as u32 as u64; rax = ((rax.wrapping_add(rax.wrapping_mul(2))) as u32) as u32 as u64; r14 = ((rbx as u32)) as u32 as u64; r14 = (((r14 as u32)).wrapping_sub((rax as u32))) as u32 as u64; r11 = ((rbx as u32)) as u32 as u64; rcx = P14; r11 = (((r11 as u32)).wrapping_sub(crate::simd_ops::ru32(rcx.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x33e1568u64)))) as u32 as u64; r11 = (((r11 as u32)).wrapping_add(0x2u32)) as u32 as u64; rcx = (((crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; r9 = (crate::simd_ops::ru32(rcx.wrapping_add(r11.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; r9 = ((r9).wrapping_shr(14u32)) as u64; r9 = ((((r9 as u32)) & (0xfffu32))) as u32 as u64; rdx = (((crate::simd_ops::ru32(rsi.wrapping_add(0x1cu64))) as i32 as i64) as u64) as u64; rdx = ((rdx).wrapping_add(rsi)) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(r9.wrapping_mul(2)).wrapping_add(0x1du64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(7u32)) as u64; r8 = (crate::simd_ops::ru64(rbp.wrapping_sub(0x68u64))) as u64; r8 = ((r8).wrapping_add(rcx)) as u64; r8 = ((r8).wrapping_sub(crate::simd_ops::ru64(rbp.wrapping_sub(0x48u64)))) as u64; rcx = ((crate::simd_ops::ru8(rdx.wrapping_add(r9.wrapping_mul(2)).wrapping_add(0x1cu64))) as u32) as u32 as u64; rcx = ((rcx).wrapping_shl(7u32)) as u64; rcx = ((rcx).wrapping_sub(crate::simd_ops::ru64(rbp.wrapping_sub(0x50u64)))) as u64; rcx = ((rcx).wrapping_add(crate::simd_ops::ru64(rbp.wrapping_sub(0x70u64)))) as u64; rcx = (((rcx) as i64).wrapping_mul((crate::simd_ops::ru64(rbp.wrapping_sub(0x38u64))) as i64) as u64) as u64; rax = (crate::simd_ops::ru64(rbp.wrapping_sub(0x40u64))) as u64; rax = (((rax) as i64).wrapping_mul((r8) as i64) as u64) as u64; rcx = ((rcx).wrapping_sub(rax)) as u64; crate::simd_ops::wu64(rsp.wrapping_add(0x60u64), r10); crate::simd_ops::wu16(rsp.wrapping_add(0x6au64), ((r12 as u16)) as u16); fl_l = rcx; fl_r = rcx; if (((((fl_l as u64)) & ((fl_r as u64))) as i64) <= 0) { bb = 14; continue; } crate::simd_ops::wu16(rsp.wrapping_add(0x68u64), ((r11 as u16)) as u16); bb = 15; continue; }
            14 => { rax = P14; rax = (crate::simd_ops::ru32(rax.wrapping_add(r14.wrapping_mul(4)).wrapping_add(0x33e1600u64))) as u32 as u64; rcx = ((rbx.wrapping_sub(0x2u64)) as u32) as u32 as u64; fl_l = (((rcx as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_r = fl_l; rcx = ((fl_l as u32)) as u32 as u64; r11 = (((rcx as u16)) as u32) as u32 as u64; crate::simd_ops::wu16(rsp.wrapping_add(0x68u64), ((rcx as u16)) as u16); bb = 15; continue; }
            15 => { rdx = (((r11 as u16)) as u32) as u32 as u64; rcx = (((crate::simd_ops::ru32(rsi.wrapping_add(0x14u64))) as i32 as i64) as u64) as u64; rcx = ((rcx).wrapping_add(rsi)) as u64; rbx = (crate::simd_ops::ru32(rcx.wrapping_add(rdx.wrapping_mul(4)).wrapping_add(0x14u64))) as u32 as u64; r9 = (((r11 as u16)) as u32) as u32 as u64; rax = rbp.wrapping_add(0x1c8); crate::simd_ops::wu64(rsp.wrapping_add(0x38u64), rax); rax = rbp.wrapping_sub(0x30); crate::simd_ops::wu64(rsp.wrapping_add(0x30u64), rax); rax = rbp.wrapping_sub(0x40); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rbp.wrapping_sub(0x50); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r8 = rsi; rdx = rbp.wrapping_sub(0x70); rcx = rbp.wrapping_add(0x100); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P12, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) != 0) { bb = 17; continue; } r14 = 0x3fff; bb = 8; continue; }
            16 => { rax = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rbp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rbp.wrapping_sub(0x30); r8 = rsp.wrapping_add(0x60); rdx = rbp.wrapping_add(0xe0); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P15, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            17 => { rax = rbp.wrapping_sub(0x20); crate::simd_ops::wu64(rsp.wrapping_add(0x28u64), rax); rax = rbp.wrapping_add(0x90); crate::simd_ops::wu64(rsp.wrapping_add(0x20u64), rax); r9 = rsp.wrapping_add(0x70); r8 = rbp.wrapping_sub(0x60); rdx = rsp.wrapping_add(0x60); rcx = rdi; (rax, xmm0) = crate::simd_ops::call_stk_xmm(P16, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); bb = 18; continue; }
            18 => { rcx = rbp.wrapping_sub(0x20); (rax, xmm0) = crate::simd_ops::call_stk_xmm(P17, rax, rcx, rdx, r8, r9, rsp, [xmm0, xmm1, xmm2, xmm3]); rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x78u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 19; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x80u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 19; continue; }
            19 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x60u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 20; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x68u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 20; continue; }
            20 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x48u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 21; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x50u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 21; continue; }
            21 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x30u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 22; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x38u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 22; continue; }
            22 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 23; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x20u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 23; continue; }
            23 => { rcx = (crate::simd_ops::ru64(rbp)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 24; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_add(0x8u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 24; continue; }
            24 => { rcx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x18u64))) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 25; continue; } rax = (crate::simd_ops::ru32(rbp.wrapping_sub(0x10u64))) as u32 as u64; fl_l = ((rax).wrapping_add(rax)) as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; crate::simd_ops::wu8(rcx.wrapping_add(rax.wrapping_mul(8)).wrapping_add(0x24u64), (0x0u8) as u8); bb = 25; continue; }
            25 => { r11 = rsp.wrapping_add(0x270); rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x48u64))) as u64; xmm6 = (crate::simd_ops::ru128(r11.wrapping_sub(0x10u64))) as u128; xmm7 = (crate::simd_ops::ru128(r11.wrapping_sub(0x20u64))) as u128; rsp = r11; r15 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r14 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r13 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); r12 = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rsi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); rbp = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            _ => { let _ = &stack; crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31775<const P0: u64, const P1: u64, const P2: u64, const P3: u64, const P4: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rsi, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), rsi); rsp = rsp.wrapping_sub(8); crate::simd_ops::wu64(rsp, rdi); rsp = ((rsp).wrapping_sub(0x20u64)) as u64; rsi = rcx; rdi = (crate::simd_ops::ru64(rcx.wrapping_add(P0))) as u64; rdx = (crate::simd_ops::ru64(rcx)) as u64; rbx = ((rdi.wrapping_add(P1)) as u64) as u64; rbx = ((rdx.wrapping_add(rbx.wrapping_mul(8))) as u64) as u64; fl_l = rdi; fl_r = rdi; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 3; continue; } bb = 1; continue; }
            1 => { rcx = (crate::simd_ops::ru64(rbx)) as u64; fl_l = rcx; fl_r = rcx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 2; continue; } rax = crate::simd_ops::call_stk(P2, rax, rcx, rdx, r8, r9, rsp); bb = 2; continue; }
            2 => { rbx = ((rbx).wrapping_sub(0x8u64)) as u64; fl_l = rdi; fl_r = (0x1u64) as u64; rdi = ((fl_l as u64).wrapping_sub(fl_r as u64)) as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 1; continue; } rdx = (crate::simd_ops::ru64(rsi)) as u64; bb = 3; continue; }
            3 => { fl_l = rdx; fl_r = rdx; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { bb = 4; continue; } rcx = (crate::simd_ops::ru64(P3)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P4))) as u64, rax, rcx, rdx, r8, r9, rsp); bb = 4; continue; }
            4 => { rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsi = (crate::simd_ops::ru64(rsp.wrapping_add(0x38u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
            _ => { let _ = &stack; return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn tpl_31780<const P0: u64, const P1: u64, const P2: u64, const P3: u64>(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    let (mut rax, mut fl_l, mut fl_r, mut rbx, mut rdi) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 5>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), rbx);
    rsp = rsp.wrapping_sub(8);
    crate::simd_ops::wu64(rsp, rdi);
    rsp = ((rsp).wrapping_sub(0x20u64)) as u64;
    rdi = ((rdx as u32)) as u32 as u64;
    rbx = rcx;
    rax = crate::simd_ops::call_stk(P0, rax, rcx, rdx, r8, r9, rsp);
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1u8) as u8 as u64;
    if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64;
    if !(((((fl_l as u8)) & ((fl_r as u8))) != 0)) { rcx = (crate::simd_ops::ru64(P1)) as u64; rax = (crate::simd_ops::ru64(rcx)) as u64; rdx = rbx; rax = crate::simd_ops::call_stk((crate::simd_ops::ru64(rax.wrapping_add(P2))) as u64, rax, rcx, rdx, r8, r9, rsp); rax = rbx; rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64; rsp = ((rsp).wrapping_add(0x20u64)) as u64; rdi = crate::simd_ops::ru64(rsp); rsp = rsp.wrapping_add(8); return rax; }
    rdx = 0x18;
    rcx = rbx;
    rax = crate::simd_ops::call_stk(P3, rax, rcx, rdx, r8, r9, rsp);
    rax = rbx;
    rbx = (crate::simd_ops::ru64(rsp.wrapping_add(0x30u64))) as u64;
    rsp = ((rsp).wrapping_add(0x20u64)) as u64;
    rdi = crate::simd_ops::ru64(rsp);
    rsp = rsp.wrapping_add(8);
    let _ = &stack; return rax;
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bed18(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x142646f28; const P2: u64 = 0x142646fd0; const P3: u64 = 0x142647090; const P4: u64 = 0x14266c8c0; const P5: u64 = 0x142647088; const P6: u64 = 0x1414bc7a4; const P7: u64 = 0x1414c2674; const P8: u64 = 0x1414c1b84; const P9: u64 = 0x1414c93a8; const P10: u64 = 0x1414c2524; const P11: u64 = 0x1414c2ca4; const P12: u64 = 0x1426470d8; const P13: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (rcx) as u64; rsi = (r8) as u64; rcx = (r8) as u64; bb = 33; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rdx.wrapping_add(0x2u64)) as u64) as u64; bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 10; continue; }
            10 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x28u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = (rsi) as u64; rcx = (rax) as u64; rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); rax = (rbx) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rdx = ((P4) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 37; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; fl_k = 1; bb = 31; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbx).wrapping_sub(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; fl_k = 6; bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbx) as u64; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 17; continue; }
            17 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi)) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 32; continue; } bb = 18; continue; }
            18 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 32; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 16; continue; }
            20 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), (rax) as u64); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r8 = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            24 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 36; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 36; continue; } bb = 26; continue; }
            26 => { rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 29; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 29; continue; }
            29 => { rcx = ((rdi as u32)) as u32 as u64; r9 = (rsi) as u64; r8 = (rbx) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_k = 4; bb = 31; continue; }
            31 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 37; continue; } bb = 32; continue; }
            32 => { rcx = (rsi) as u64; bb = 33; continue; }
            33 => { rdx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 35; continue; }
            35 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 35; continue; }
            37 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_k = 3; bb = 35; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414bf2cc(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x142646f28; const P2: u64 = 0x142646fd0; const P3: u64 = 0x142647090; const P4: u64 = 0x14266c8f8; const P5: u64 = 0x142647088; const P6: u64 = 0x1414bc80c; const P7: u64 = 0x1414c271c; const P8: u64 = 0x1414c1b84; const P9: u64 = 0x1414c7e28; const P10: u64 = 0x1414c23d4; const P11: u64 = 0x1414c2ca4; const P12: u64 = 0x1426470d8; const P13: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (rcx) as u64; rsi = (r8) as u64; rcx = (r8) as u64; bb = 33; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rdx.wrapping_add(0x2u64)) as u64) as u64; bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 10; continue; }
            10 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x28u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = (rsi) as u64; rcx = (rax) as u64; rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); rax = (rbx) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rdx = ((P4) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 37; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; fl_k = 1; bb = 31; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbx).wrapping_sub(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; fl_k = 6; bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbx) as u64; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 17; continue; }
            17 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi)) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 32; continue; } bb = 18; continue; }
            18 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 32; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 16; continue; }
            20 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), (rax) as u64); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r8 = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            24 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 36; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 36; continue; } bb = 26; continue; }
            26 => { rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 29; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 29; continue; }
            29 => { rcx = ((rdi as u32)) as u32 as u64; r9 = (rsi) as u64; r8 = (rbx) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_k = 4; bb = 31; continue; }
            31 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 37; continue; } bb = 32; continue; }
            32 => { rcx = (rsi) as u64; bb = 33; continue; }
            33 => { rdx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 35; continue; }
            35 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 35; continue; }
            37 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_k = 3; bb = 35; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c9ba8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x142646f28; const P2: u64 = 0x142646fd0; const P3: u64 = 0x142647090; const P4: u64 = 0x14266eca0; const P5: u64 = 0x142647088; const P6: u64 = 0x1414bc73c; const P7: u64 = 0x1414cb8ac; const P8: u64 = 0x1414c1b84; const P9: u64 = 0x1414c93a8; const P10: u64 = 0x1414c2524; const P11: u64 = 0x1414c2ca4; const P12: u64 = 0x1426470d8; const P13: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (rcx) as u64; rsi = (r8) as u64; rcx = (r8) as u64; bb = 33; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rdx.wrapping_add(0x2u64)) as u64) as u64; bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 10; continue; }
            10 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x28u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = (rsi) as u64; rcx = (rax) as u64; rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); rax = (rbx) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rdx = ((P4) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 37; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; fl_k = 1; bb = 31; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbx).wrapping_sub(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; fl_k = 6; bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbx) as u64; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 17; continue; }
            17 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi)) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 32; continue; } bb = 18; continue; }
            18 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 32; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 16; continue; }
            20 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), (rax) as u64); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r8 = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            24 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 36; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 36; continue; } bb = 26; continue; }
            26 => { rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 29; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 29; continue; }
            29 => { rcx = ((rdi as u32)) as u32 as u64; r9 = (rsi) as u64; r8 = (rbx) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_k = 4; bb = 31; continue; }
            31 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 37; continue; } bb = 32; continue; }
            32 => { rcx = (rsi) as u64; bb = 33; continue; }
            33 => { rdx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 35; continue; }
            35 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 35; continue; }
            37 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_k = 3; bb = 35; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414ca144(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x142646f28; const P2: u64 = 0x142646fd0; const P3: u64 = 0x142647090; const P4: u64 = 0x14266ecd8; const P5: u64 = 0x142647088; const P6: u64 = 0x1414bc604; const P7: u64 = 0x1414cb75c; const P8: u64 = 0x1414c1b84; const P9: u64 = 0x1414c7e28; const P10: u64 = 0x1414c23d4; const P11: u64 = 0x1414c2ca4; const P12: u64 = 0x1426470d8; const P13: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (rcx) as u64; rsi = (r8) as u64; rcx = (r8) as u64; bb = 33; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rdx.wrapping_add(0x2u64)) as u64) as u64; bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 10; continue; }
            10 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x28u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = (rsi) as u64; rcx = (rax) as u64; rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); rax = (rbx) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rdx = ((P4) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 37; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; fl_k = 1; bb = 31; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbx).wrapping_sub(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; fl_k = 6; bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbx) as u64; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 17; continue; }
            17 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi)) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 32; continue; } bb = 18; continue; }
            18 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 32; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 16; continue; }
            20 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), (rax) as u64); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r8 = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            24 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 36; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 36; continue; } bb = 26; continue; }
            26 => { rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 29; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 29; continue; }
            29 => { rcx = ((rdi as u32)) as u32 as u64; r9 = (rsi) as u64; r8 = (rbx) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_k = 4; bb = 31; continue; }
            31 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 37; continue; } bb = 32; continue; }
            32 => { rcx = (rsi) as u64; bb = 33; continue; }
            33 => { rdx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 35; continue; }
            35 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 35; continue; }
            37 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_k = 3; bb = 35; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d03c8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x142646f28; const P2: u64 = 0x142646fd0; const P3: u64 = 0x142647090; const P4: u64 = 0x142670a80; const P5: u64 = 0x142647088; const P6: u64 = 0x1414bcce4; const P7: u64 = 0x1414d4050; const P8: u64 = 0x1414c1b84; const P9: u64 = 0x1414c93a8; const P10: u64 = 0x1414c2524; const P11: u64 = 0x1414c2ca4; const P12: u64 = 0x1426470d8; const P13: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (rcx) as u64; rsi = (r8) as u64; rcx = (r8) as u64; bb = 33; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rdx.wrapping_add(0x2u64)) as u64) as u64; bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 10; continue; }
            10 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x28u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = (rsi) as u64; rcx = (rax) as u64; rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); rax = (rbx) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rdx = ((P4) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 37; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; fl_k = 1; bb = 31; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbx).wrapping_sub(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; fl_k = 6; bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbx) as u64; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 17; continue; }
            17 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi)) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 32; continue; } bb = 18; continue; }
            18 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 32; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 16; continue; }
            20 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), (rax) as u64); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r8 = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            24 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 36; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 36; continue; } bb = 26; continue; }
            26 => { rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 29; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 29; continue; }
            29 => { rcx = ((rdi as u32)) as u32 as u64; r9 = (rsi) as u64; r8 = (rbx) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_k = 4; bb = 31; continue; }
            31 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 37; continue; } bb = 32; continue; }
            32 => { rcx = (rsi) as u64; bb = 33; continue; }
            33 => { rdx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 35; continue; }
            35 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 35; continue; }
            37 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_k = 3; bb = 35; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414d0930(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x142646f28; const P2: u64 = 0x142646fd0; const P3: u64 = 0x142647090; const P4: u64 = 0x142670ac0; const P5: u64 = 0x142647088; const P6: u64 = 0x1414bcbac; const P7: u64 = 0x1414d3f00; const P8: u64 = 0x1414c1b84; const P9: u64 = 0x1414c7e28; const P10: u64 = 0x1414c23d4; const P11: u64 = 0x1414c2ca4; const P12: u64 = 0x1426470d8; const P13: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (rcx) as u64; rsi = (r8) as u64; rcx = (r8) as u64; bb = 33; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rdx.wrapping_add(0x2u64)) as u64) as u64; bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 20; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 14; continue; } bb = 8; continue; }
            8 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 24; continue; } bb = 9; continue; }
            9 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x1au8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 10; continue; }
            10 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x28u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = (rsi) as u64; rcx = (rax) as u64; rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); rax = (rbx) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 12; continue; } bb = 11; continue; }
            11 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 12; continue; }
            12 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rdx = ((P4) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 37; continue; } bb = 13; continue; }
            13 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; fl_k = 1; bb = 31; continue; }
            14 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 15; continue; }
            15 => { fl_l = ((rbx).wrapping_sub(1)) as u64 as u64; rbx = ((fl_l as u64)) as u64; fl_k = 6; bb = 16; continue; }
            16 => { rbx = ((rbx).wrapping_add(1)) as u64; rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; r8 = (rbx) as u64; rdx = (rax) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 37; continue; } bb = 17; continue; }
            17 => { fl_l = (rax) as u64 as u64; fl_r = (crate::simd_ops::ru64(rsi)) as u64 as u64; if (((fl_l as u64)) >= ((fl_r as u64))) { fl_k = 1; bb = 32; continue; } bb = 18; continue; }
            18 => { rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P8) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 32; continue; } bb = 19; continue; }
            19 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 16; continue; }
            20 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0xau8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 24; continue; } bb = 21; continue; }
            21 => { fl_l = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; if (((fl_l as u64)) != ((fl_r as u64))) { bb = 23; continue; } bb = 22; continue; }
            22 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P9) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(r14.wrapping_add(0x30u64), (rax) as u64); bb = 23; continue; }
            23 => { rdx = (crate::simd_ops::ru64(r14.wrapping_add(0x30u64))) as u64; r8 = (rbx) as u64; rcx = (rsi) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P10) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            24 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 36; continue; } bb = 25; continue; }
            25 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 36; continue; } bb = 26; continue; }
            26 => { rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 28; continue; } bb = 27; continue; }
            27 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 29; continue; }
            28 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P11) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 29; continue; }
            29 => { rcx = ((rdi as u32)) as u32 as u64; r9 = (rsi) as u64; r8 = (rbx) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P12)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 30; continue; }
            30 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_k = 4; bb = 31; continue; }
            31 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 37; continue; } bb = 32; continue; }
            32 => { rcx = (rsi) as u64; bb = 33; continue; }
            33 => { rdx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P13)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 34; continue; }
            34 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 35; continue; }
            35 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            36 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 35; continue; }
            37 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_k = 3; bb = 35; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c45c8(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x142646f28; const P2: u64 = 0x142646fd0; const P3: u64 = 0x142647090; const P4: u64 = 0x14266cfe0; const P5: u64 = 0x142647088; const P6: u64 = 0x1414c2a54; const P7: u64 = 0x1414c2ca4; const P8: u64 = 0x1426470d8; const P9: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (rcx) as u64; rsi = (r8) as u64; rcx = (r8) as u64; bb = 24; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rdx.wrapping_add(0x2u64)) as u64) as u64; bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 28; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 15; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 15; continue; } bb = 9; continue; }
            9 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = (rsi) as u64; rcx = (rax) as u64; rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); rax = (rbx) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rdx = ((P4) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 28; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; fl_k = 1; bb = 22; continue; }
            13 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rbp.wrapping_add(0x38u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), (rcx) as u64); bb = 21; continue; }
            15 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 27; continue; } bb = 16; continue; }
            16 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 27; continue; } bb = 17; continue; }
            17 => { rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 20; continue; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            20 => { rcx = ((rdi as u32)) as u32 as u64; r9 = (rsi) as u64; r8 = (rbx) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_k = 4; bb = 22; continue; }
            22 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 28; continue; } bb = 23; continue; }
            23 => { rcx = (rsi) as u64; bb = 24; continue; }
            24 => { rdx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 26; continue; }
            26 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 26; continue; }
            28 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_k = 3; bb = 26; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}

#[inline(never)]
pub unsafe extern "win64" fn fn_1414c5720(mut rcx: u64, mut rdx: u64, mut r8: u64, mut r9: u64, arg5: u64, arg6: u64, arg7: u64, arg8: u64) -> u64 {
    const P0: u64 = 0x142647078; const P1: u64 = 0x142646f28; const P2: u64 = 0x142646fd0; const P3: u64 = 0x142647090; const P4: u64 = 0x14266d620; const P5: u64 = 0x142647088; const P6: u64 = 0x1414c2a54; const P7: u64 = 0x1414c2ca4; const P8: u64 = 0x1426470d8; const P9: u64 = 0x142647060;
    let (mut rax, mut r11, mut fl_l, mut fl_r, mut rbx, mut rbp, mut rsi, mut rdi, mut r14) = (crate::simd_ops::get_rax_in(), 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64, 0u64);
    let mut stack = crate::simd_ops::LiftedFrame::<256, 13>::new(); let mut rsp: u64 = crate::simd_ops::init_lifted_stack(&mut stack, arg5, arg6, arg7, arg8);
    let [mut xmm0, mut xmm1, mut xmm2, mut xmm3] = crate::simd_ops::take_xmm_args();
    let mut fl_k: u32 = 0;
    let mut bb: u32 = 0;
    loop {
        match bb {
            0 => { crate::simd_ops::wu64(rsp.wrapping_add(0x8u64), (rbx) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x18u64), (rsi) as u64); crate::simd_ops::wu64(rsp.wrapping_add(0x10u64), (rdx) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rbp) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (rdi) as u64); rsp = rsp.wrapping_sub(8); core::ptr::write_unaligned(rsp as *mut u64, (r14) as u64); rbp = (rsp) as u64; rsp = ((rsp).wrapping_sub(0x50u64)) as u64; r14 = (rcx) as u64; rsi = (r8) as u64; rcx = (r8) as u64; bb = 24; continue; }
            1 => { rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; rdi = ((crate::simd_ops::ru8(rdx)) as u32) as u32 as u64; rbx = ((rdx.wrapping_add(0x1u64)) as u64) as u64; fl_l = ((rdi as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) < ((fl_r as u32))) { bb = 5; continue; } bb = 2; continue; }
            2 => { rcx = ((crate::simd_ops::ru8(rbx)) as u32) as u32 as u64; rdi = (((rdi as u32)).wrapping_add(0xffffff80u32)) as u32 as u64; rax = ((rcx as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shl(7u32)) as u32 as u64; rdi = (((rdi as u32)).wrapping_add((rax as u32))) as u32 as u64; fl_l = ((rcx as u32)) as u32 as u64; fl_r = (0x80u32) as u32 as u64; if (((fl_l as u32)) >= ((fl_r as u32))) { bb = 4; continue; } bb = 3; continue; }
            3 => { rbx = ((rdx.wrapping_add(0x2u64)) as u64) as u64; bb = 5; continue; }
            4 => { r8 = ((rdi as u32)) as u32 as u64; rcx = ((rbp.wrapping_sub(0x30u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P0)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdi = (crate::simd_ops::ru32(rbp.wrapping_sub(0x28u64))) as u32 as u64; rbx = (crate::simd_ops::ru64(rbp.wrapping_sub(0x30u64))) as u64; bb = 5; continue; }
            5 => { crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rbx) as u64); fl_l = (rbx) as u64 as u64; fl_r = (rbx) as u64 as u64; if ((((fl_l as u64)) & ((fl_r as u64))) == 0) { fl_k = 4; bb = 28; continue; } bb = 6; continue; }
            6 => { rax = ((rdi as u32)) as u32 as u64; rax = (((rax as u32)).wrapping_shr(3u32)) as u32 as u64; fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; rax = ((fl_l as u32).wrapping_sub(fl_r as u32)) as u32 as u64; if (((fl_l as u32)) == ((fl_r as u32))) { bb = 13; continue; } bb = 7; continue; }
            7 => { fl_l = ((rax as u32)) as u32 as u64; fl_r = (0x1u32) as u32 as u64; if (((fl_l as u32)) != ((fl_r as u32))) { fl_k = 0; bb = 15; continue; } bb = 8; continue; }
            8 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x12u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 15; continue; } bb = 9; continue; }
            9 => { rcx = (r14) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P1)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = ((r14.wrapping_add(0x10u64)) as u64) as u64; rdx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; r8 = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P2)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rdx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; r8 = (rsi) as u64; rcx = (rax) as u64; rbx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P3)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); rax = (rbx) as u64; fl_l = (crate::simd_ops::ru64(rbx.wrapping_add(0x18u64))) as u64 as u64; fl_r = (0x10u64) as u64 as u64; if (((fl_l as u64)) < ((fl_r as u64))) { bb = 11; continue; } bb = 10; continue; }
            10 => { rax = (crate::simd_ops::ru64(rbx)) as u64; bb = 11; continue; }
            11 => { crate::simd_ops::wu64(rbp.wrapping_sub(0x20u64), (rax) as u64); rdx = ((P4) as u64) as u64; rax = (crate::simd_ops::ru64(rbx.wrapping_add(0x10u64))) as u64; rcx = ((rbp.wrapping_sub(0x10u64)) as u64) as u64; crate::simd_ops::wu64(rbp.wrapping_sub(0x18u64), (rax) as u64); xmm0 = (crate::simd_ops::ru128(rbp.wrapping_sub(0x20u64))) as u128; crate::simd_ops::wu128(rbp.wrapping_sub(0x10u64), (xmm0) as u128); let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P5)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { fl_k = 5; bb = 28; continue; } bb = 12; continue; }
            12 => { fl_l = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64 as u64; fl_r = (0x0u64) as u64 as u64; fl_k = 1; bb = 22; continue; }
            13 => { fl_l = ((rdi as u8)) as u8 as u64; fl_r = (0x8u8) as u8 as u64; if (((fl_l as u8)) != ((fl_r as u8))) { fl_k = 2; bb = 15; continue; } bb = 14; continue; }
            14 => { rdx = ((rbp.wrapping_add(0x38u64)) as u64) as u64; rcx = (rbx) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P6) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; rcx = (crate::simd_ops::ru64(rbp.wrapping_add(0x38u64))) as u64; crate::simd_ops::wu64(r14.wrapping_add(0x18u64), (rcx) as u64); bb = 21; continue; }
            15 => { rax = ((rdi as u32)) as u32 as u64; rax = (rax & !0xffu64) | ((((((rax as u8)) & (0x7u8))) as u8 as u64)); fl_l = ((rax as u8)) as u8 as u64; fl_r = (0x4u8) as u8 as u64; if (((fl_l as u8)) == ((fl_r as u8))) { fl_k = 2; bb = 27; continue; } bb = 16; continue; }
            16 => { fl_l = ((rdi as u32)) as u32 as u64; fl_r = ((rdi as u32)) as u32 as u64; if ((((fl_l as u32)) & ((fl_r as u32))) == 0) { fl_k = 3; bb = 27; continue; } bb = 17; continue; }
            17 => { rcx = ((r14.wrapping_add(0x8u64)) as u64) as u64; fl_l = (crate::simd_ops::ru8(rcx)) as u8 as u64; fl_r = (0x1u8) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 19; continue; } bb = 18; continue; }
            18 => { rax = (crate::simd_ops::ru64(rcx)) as u64; rax = (((rax) & (0xfffffffffffffffcu64))) as u64; fl_l = ((rax).wrapping_add(0x8u64)) as u64 as u64; fl_r = fl_l; rax = ((fl_l as u64)) as u64; bb = 20; continue; }
            19 => { let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm((P7) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 20; continue; }
            20 => { rcx = ((rdi as u32)) as u32 as u64; r9 = (rsi) as u64; r8 = (rbx) as u64; rdx = (rax) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P8)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; bb = 21; continue; }
            21 => { fl_l = (rax) as u64 as u64; fl_r = (rax) as u64 as u64; crate::simd_ops::wu64(rbp.wrapping_add(0x28u64), (rax) as u64); fl_k = 4; bb = 22; continue; }
            22 => { if (match fl_k { 1 => (((fl_l as u64)) == ((fl_r as u64))), 4 => ((((fl_l as u64)) & ((fl_r as u64))) == 0), _ => false }) { bb = 28; continue; } bb = 23; continue; }
            23 => { rcx = (rsi) as u64; bb = 24; continue; }
            24 => { rdx = ((rbp.wrapping_add(0x28u64)) as u64) as u64; let s5 = crate::simd_ops::read_s5_call(rsp, &stack); let s6 = core::ptr::read_unaligned(rsp.wrapping_add(0x28) as *const u64); let s7 = core::ptr::read_unaligned(rsp.wrapping_add(0x30) as *const u64); let s8 = core::ptr::read_unaligned(rsp.wrapping_add(0x38) as *const u64); crate::simd_ops::replace_rax_in(rax); let (c_rax, c_x0) = crate::simd_ops::call_va8_xmm(({ let t = (crate::simd_ops::ru64(P9)) as u64; if t == 0x1_4254_7700 { rax } else { t } }) as u64, rcx, rdx, r8, r9, s5, s6, s7, s8, [xmm0, xmm1, xmm2, xmm3]); rax = c_rax; xmm0 = c_x0; fl_l = ((rax as u8)) as u8 as u64; fl_r = ((rax as u8)) as u8 as u64; if ((((fl_l as u8)) & ((fl_r as u8))) == 0) { bb = 1; continue; } bb = 25; continue; }
            25 => { rbx = (crate::simd_ops::ru64(rbp.wrapping_add(0x28u64))) as u64; fl_k = 5; bb = 26; continue; }
            26 => { r11 = ((rsp.wrapping_add(0x50u64)) as u64) as u64; rax = (rbx) as u64; rbx = (crate::simd_ops::ru64(r11.wrapping_add(0x20u64))) as u64; rsi = (crate::simd_ops::ru64(r11.wrapping_add(0x30u64))) as u64; rsp = (r11) as u64; r14 = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rdi = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); rbp = core::ptr::read_unaligned(rsp as *const u64); rsp = rsp.wrapping_add(8); crate::simd_ops::set_xmm_ret(xmm0); return rax; }
            27 => { rax = ((rdi.wrapping_sub(0x1u64)) as u32) as u32 as u64; crate::simd_ops::wu32(rsi.wrapping_add(0x50u64), ((rax as u32)) as u32); bb = 26; continue; }
            28 => { rbx = (0u32) as u32 as u64; fl_l = 0; fl_r = 0; fl_k = 3; bb = 26; continue; }
            _ => { crate::simd_ops::set_xmm_ret(xmm0); return rax; }
        }
    }
}
